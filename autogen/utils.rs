use crate::structs;

use std::fs;
use std::io::Write;

use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[rustfmt::skip]
static AUTOGEN_COMMENT : &'static str = "\
// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!";

pub fn write_autogen_comment(file: &mut fs::File) {
    file.write_all(AUTOGEN_COMMENT.as_bytes()).unwrap();
    file.write_all(b"\n\n").unwrap();
}

/// Converts the given string into an `Ident`, with call-site span.
pub fn as_ident(ident: &str) -> Ident {
    Ident::new(ident, Span::call_site())
}

/// Returns the corresponding operand kind in data representation for the
/// given operand `kind` in the grammar.
pub fn get_dr_operand_kind(kind: &str) -> Ident {
    as_ident(if kind == "LiteralInteger" {
        "LiteralInt32"
    } else if kind == "LiteralContextDependentNumber" {
        // TODO: should use the correct type to decode
        "LiteralInt32"
    } else {
        kind
    })
}

/// Returns the underlying type used in operand kind enums for the operand
/// kind `kind` in the grammar.
pub fn get_enum_underlying_type(kind: &str, generic_string: bool) -> TokenStream {
    if kind.starts_with("Id") {
        quote! { spirv::Word }
    } else if kind == "LiteralInteger" || kind == "LiteralExtInstInteger" {
        quote! { u32 }
    } else if kind == "LiteralSpecConstantOpInteger" {
        quote! { spirv::Op }
    } else if kind == "LiteralContextDependentNumber" {
        panic!("this kind is not expected to be handled here")
    } else if kind == "LiteralString" {
        if generic_string { quote! { T } } else { quote! { String } }
    } else if kind == "PairLiteralIntegerIdRef" {
        quote! { (u32, spirv::Word) }
    } else if kind == "PairIdRefLiteralInteger" {
        quote! { (spirv::Word, u32) }
    } else if kind == "PairIdRefIdRef" {
        quote! { (spirv::Word, spirv::Word) }
    } else {
        let kind = as_ident(kind);
        quote! { spirv::#kind }
    }
}

/// Returns a suitable name for the given parameter.
pub fn get_param_name(param: &structs::Operand) -> Ident {
    let owned;
    let mut name = if param.name.len() == 0 {
        if param.kind == "IdResultType" {
            "result_type"
        } else {
            owned = param.kind.to_snake_case();
            &owned
        }
    } else {
        owned = param.name.to_snake_case();
        &owned
    };

    if name == "type" {
        name = "ty";
    }
    if name.starts_with("member_0_type") {
        name = "members";
    }

    as_ident(name)
}
