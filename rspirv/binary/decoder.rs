use crate::spirv;

use super::DecodeError as Error;
use std::convert::TryInto;
use std::result;
use std::str;

pub type Result<T> = result::Result<T, Error>;

const WORD_NUM_BYTES: usize = 4;

/// The SPIR-V binary decoder.
///
/// Takes in a vector of bytes, and serves requests for raw SPIR-V words
/// or values of a specific SPIR-V enum type. Successful decoding will
/// surely consume the number of words decoded, while unsuccessful decoding
/// may consume any number of bytes.
///
/// TODO: The decoder should not conume words if an error occurs.
///
/// Different from the [`Parser`](struct.Parser.html),
/// this decoder is low-level; it has no knowledge of the SPIR-V grammar.
/// Given a vector of bytes, it solely responds to word decoding requests
/// via method calls: both raw words requests and decoding the raw words
/// into a value of a specific SPIR-V enum type.
///
/// It also provides a limit mechanism. Users can set a limit, and then
/// requesting words. If that limit is reached before the end of the
/// stream, [`State::LimitReached`](enum.ParseState.html) will be
/// returned.
///
/// # Errors
///
/// For its methods, there can be the following errors:
///
/// * `DecodeError::LimitReached(offset)` if the most recent limit has reached.
/// * `DecodeError::StreamExpected(offset)` if more bytes are needed to decode
///    the next word.
/// * `DecodeError::<spirv-enum>Unknown(offset, value)` if failed to decode the
///    next word as the given `<spirv-enum>`.
///
/// All errors contain the byte offset of the word failed decoding.
///
/// # Examples
///
/// ```
/// extern crate rspirv;
/// extern crate spirv as spirv;
///
/// use rspirv::binary::{Decoder, DecodeError};
/// use spirv::SourceLanguage;
///
/// fn main() {
///     let v = vec![0x12, 0x34, 0x56, 0x78,
///                  0x90, 0xab, 0xcd, 0xef,
///                  0x02, 0x00, 0x00, 0x00];
///     let mut d = Decoder::new(&v);
///
///     assert_eq!(Ok(0x78563412), d.word());
///     assert_eq!(Ok(0xefcdab90), d.word());
///     assert_eq!(Ok(SourceLanguage::GLSL), d.source_language());
///
///     assert_eq!(Err(DecodeError::StreamExpected(12)), d.word());
/// }
/// ```
pub struct Decoder<'a> {
    /// Raw bytes to decode
    bytes: &'a [u8],
    /// Offset for next byte to decode
    offset: usize,
    /// Remaining limit of number of words before error
    limit: Option<usize>,
}

impl<'a> Decoder<'a> {
    /// Creates a new `Decoder` instance.
    pub fn new(bytes: &'a [u8]) -> Decoder<'a> {
        Decoder {
            bytes,
            offset: 0,
            limit: None,
        }
    }

    /// Returns the offset of the byte to decode next.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Decodes and returns the next raw SPIR-V word.
    pub fn word(&mut self) -> Result<spirv::Word> {
        if self.has_limit() {
            if self.limit_reached() {
                return Err(Error::LimitReached(self.offset));
            } else {
                *self.limit.as_mut().unwrap() -= 1
            }
        }

        if self.offset >= self.bytes.len() || self.offset + WORD_NUM_BYTES > self.bytes.len() {
            Err(Error::StreamExpected(self.offset))
        } else {
            self.offset += WORD_NUM_BYTES;
            Ok(spirv::Word::from_le_bytes(
                self.bytes[self.offset - 4..self.offset].try_into().unwrap(),
            ))
        }
    }

    /// Decodes and returns the next `n` raw SPIR-V words.
    pub fn words(&mut self, n: usize) -> Result<Vec<spirv::Word>> {
        let mut words = Vec::new();
        for _ in 0..n {
            words.push(self.word()?);
        }
        Ok(words)
    }
}

impl<'a> Decoder<'a> {
    /// Sets the limit to `num_words` words.
    ///
    /// The decoder will return [`State::LimitReached`](enum.ParseState.html)
    /// after `num_words` words have been requested, if having not consumed
    /// the whole stream.
    pub fn set_limit(&mut self, num_words: usize) {
        self.limit = Some(num_words)
    }

    /// Clear the previously set limit (if any).
    pub fn clear_limit(&mut self) {
        self.limit = None
    }

    /// Returns true if a limit has been set on this decoder.
    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    /// Returns true if the previously set limit has been reached.
    ///
    /// This will always return false if no limit has been ever set.
    pub fn limit_reached(&self) -> bool {
        if let Some(left) = self.limit {
            left == 0
        } else {
            false
        }
    }
}

impl<'a> Decoder<'a> {
    /// Decodes and returns the next SPIR-V word as an id.
    pub fn id(&mut self) -> Result<spirv::Word> {
        self.word()
    }

    /// Decodes and returns a literal string.
    ///
    /// This method will consume as many words as necessary until finding a
    /// null character (`\0`), or reaching the limit or end of the stream
    /// and erroring out.
    pub fn string(&mut self) -> Result<String> {
        // If we have a limit, then don't search further than we need to.
        let slice = match self.limit {
            Some(limit) => &self.bytes[self.offset..(self.offset + limit * WORD_NUM_BYTES)],
            None => &self.bytes[self.offset..],
        };
        // Find the null terminator.
        let first_null_byte =
            slice
                .iter()
                .position(|&c| c == 0)
                .ok_or_else(|| match self.limit {
                    Some(_) => Error::LimitReached(self.offset + slice.len()),
                    None => Error::StreamExpected(self.offset),
                })?;
        // Validate the string is utf8.
        let result = str::from_utf8(&slice[..first_null_byte])
            .map_err(|e| Error::DecodeStringFailed(self.offset, format!("{}", e)))?;
        // Round up consumed words to include null byte(s).
        let consumed_words = (first_null_byte / WORD_NUM_BYTES) + 1;
        self.offset += consumed_words * WORD_NUM_BYTES;
        if let Some(ref mut limit) = self.limit {
            // This is guaranteed to be enough due to the slice limit above.
            *limit -= consumed_words;
        }
        Ok(result.to_string())
    }

    /// Decodes and returns the next SPIR-V word as a 32-bit
    /// literal integer.
    pub fn int32(&mut self) -> Result<u32> {
        self.word()
    }

    /// Decodes and returns the next two SPIR-V words as a 64-bit
    /// literal integer.
    pub fn int64(&mut self) -> Result<u64> {
        let low = u64::from(self.word()?);
        let high = u64::from(self.word()?);
        Ok((high << 32) | low)
    }

    /// Decodes and returns the next SPIR-V word as a 32-bit
    /// literal floating point number.
    pub fn float32(&mut self) -> Result<f32> {
        let val = self.word()?;
        Ok(f32::from_bits(val))
    }

    /// Decodes and returns the next two SPIR-V words as a 64-bit
    /// literal floating point number.
    pub fn float64(&mut self) -> Result<f64> {
        let low = u64::from(self.word()?);
        let high = u64::from(self.word()?);
        let val = (high << 32) | low;
        Ok(f64::from_bits(val))
    }

    /// Decodes and returns the next SPIR-V word as a 32-bit
    /// extended-instruction-set number.
    pub fn ext_inst_integer(&mut self) -> Result<u32> {
        self.word()
    }
}

include!("autogen_decode_operand.rs");

#[cfg(test)]
mod tests {
    use crate::spirv;

    use super::Decoder;
    use crate::binary::DecodeError as Error;

    #[test]
    fn test_decoding_word_from_one_bytes() {
        let b = vec![1];
        let mut d = Decoder::new(&b);
        assert_eq!(Err(Error::StreamExpected(0)), d.word());
    }

    #[test]
    fn test_decoding_word_from_two_bytes() {
        let b = vec![1, 2];
        let mut d = Decoder::new(&b);
        assert_eq!(Err(Error::StreamExpected(0)), d.word());
    }

    #[test]
    fn test_decoding_word_from_three_bytes() {
        let b = vec![1, 2, 3];
        let mut d = Decoder::new(&b);
        assert_eq!(Err(Error::StreamExpected(0)), d.word());
    }

    #[test]
    fn test_decoding_word_from_four_bytes() {
        let b = vec![0x12, 0x34, 0x56, 0x78];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(0x78563412), d.word());
    }

    #[test]
    #[rustfmt::skip]
    fn test_decoding_words() {
        let b = vec![0x12, 0x34, 0x56, 0x78,
                     0x90, 0xab, 0xcd, 0xef,
                     0x01, 0x23, 0x45, 0x67,
                     0x89, 0xfe, 0xdc, 0xba];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(vec![0x78563412, 0xefcdab90]), d.words(2));
        assert_eq!(Ok(vec![0x67452301]), d.words(1));
        assert_eq!(Ok(vec![0xbadcfe89]), d.words(1));
    }

    #[test]
    fn test_decoding_string() {
        {
            let b = vec![0x00, 0x00, 0x00, 0x00];
            let mut d = Decoder::new(&b);
            assert_eq!(Ok(String::new()), d.string());
        }
        {
            let b = b"ok".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Err(Error::StreamExpected(0)), d.string());
        }
        {
            let b = b"ok\0\0".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Ok("ok".to_string()), d.string());
        }
        {
            let b = b"ok\0\0rust\0\0\0\0rocks\0\0\0".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Ok("ok".to_string()), d.string());
            assert_eq!(Ok("rust".to_string()), d.string());
            assert_eq!(Ok("rocks".to_string()), d.string());
        }
        {
            let b = b"I..don't know..\0".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Ok("I..don't know..".to_string()), d.string());
        }
    }

    #[test]
    fn test_decoding_source_language() {
        let b = vec![0x02, 0x00, 0x00, 0x00];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(spirv::SourceLanguage::GLSL), d.source_language());
    }

    #[test]
    fn test_decoding_unknown_execution_model() {
        let b = vec![0xef, 0xbe, 0xad, 0xde];
        let mut d = Decoder::new(&b);
        assert_eq!(
            Err(Error::ExecutionModelUnknown(0, 0xdeadbeef)),
            d.execution_model()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_offset() {
        let b = vec![0x12, 0x34, 0x56, 0x78,
                     0x90, 0xab, 0xcd, 0xef,
                     0x01, 0x23, 0x45, 0x67,
                     0x89, 0xfe, 0xdc, 0xba,
                     0x01, 0x00, 0x00, 0x00,
                     0xff, 0xff, 0xff, 0xff];
        let mut d = Decoder::new(&b);

        assert_eq!(0, d.offset());
        assert!(d.words(1).is_ok());
        assert_eq!(4, d.offset());
        assert!(d.words(2).is_ok());
        assert_eq!(12, d.offset());
        assert!(d.words(1).is_ok());
        assert_eq!(16, d.offset());

        assert!(d.source_language().is_ok());
        assert_eq!(20, d.offset());

        assert!(d.execution_model().is_err());
        assert_eq!(24, d.offset());
    }

    #[test]
    fn test_decoding_after_errors() {
        let b = vec![0x12, 0x34, 0x56, 0x78];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(0x78563412), d.word());
        assert_eq!(Err(Error::StreamExpected(4)), d.word());
        assert_eq!(Err(Error::StreamExpected(4)), d.word());
        assert_eq!(Err(Error::StreamExpected(4)), d.word());
    }

    #[test]
    fn test_limit() {
        let mut v = vec![];
        for _ in 0..12 {
            v.push(0xff);
        }
        let mut d = Decoder::new(&v);

        assert!(!d.has_limit());
        assert!(!d.limit_reached());

        d.set_limit(4);
        assert!(d.has_limit());
        assert!(!d.limit_reached());

        d.clear_limit();
        assert!(!d.has_limit());
        assert!(!d.limit_reached());

        d.set_limit(2);
        assert!(d.has_limit());
        assert!(!d.limit_reached());
        assert_eq!(Ok(0xffffffff), d.word());
        assert!(d.has_limit());
        assert!(!d.limit_reached());
        assert_eq!(Ok(0xffffffff), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());
        assert_eq!(Err(Error::LimitReached(8)), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());
        assert_eq!(Err(Error::LimitReached(8)), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());

        d.clear_limit();
        assert_eq!(Ok(0xffffffff), d.word());
        assert!(!d.has_limit());
        assert!(!d.limit_reached());

        d.set_limit(0);
        assert_eq!(Err(Error::LimitReached(12)), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());

        d.clear_limit();
        assert_eq!(Err(Error::StreamExpected(12)), d.word());
    }

    #[test]
    fn test_decode_int64() {
        let b = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(0xefcdab9078563412), d.int64());
    }

    #[test]
    fn test_decode_float32() {
        let b = [0x14, 0xAE, 0x29, 0x42];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(42.42), d.float32());

        let b = [0xA4, 0x70, 0x45, 0xC1];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(-12.34), d.float32());
    }

    #[test]
    fn test_decode_float64() {
        let b = [0xF6, 0x28, 0x5C, 0x8F, 0xC2, 0x35, 0x45, 0x40];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(42.42), d.float64());

        let b = [0xAE, 0x47, 0xE1, 0x7A, 0x14, 0xAE, 0x28, 0xC0];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(-12.34), d.float64());
    }
}
