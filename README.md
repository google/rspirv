rspirv
======

[![Actions Status](https://github.com/gfx-rs/rspirv/workflows/Continuous%20integration/badge.svg)](https://github.com/gfx-rs/rspirv/actions)
[![Matrix Room](https://img.shields.io/badge/Matrix-%23gfx%3Amatrix.org-blueviolet.svg)](https://matrix.to/#/#gfx:matrix.org)

**R**u**s**t implementation of S**PIR**-**V** module processing functionalities.
It aims to provide:

* APIs for processing SPIR-V modules
* Command line tools building on top of the APIs for common processing tasks

rspirv defines a common SPIR-V [data representation][doc-dr] (DR) as the
medium for various purposes. rspirv also provides a [builder][doc-builder] to
build the DR interactively and a [parser][doc-parser] to parse a given SPIR-V
binary module into its DR. A higher level structured representation is currently
under developing.

[SPIR-V][spirv] is a common intermediate language for representing graphics
shaders and compute kernels for multiple Khronos APIs, such as [Vulkan][vulkan],
[OpenGL][opengl], and [OpenCL][opencl].

[SPIRV-Tools][spirv-tools] is the Khronos Group's official C++ implementation of
SPIR-V binary parser, assembler, disassembler, optimizer, and validator. rspirv
is not a Rust language binding for that project; it is a complete rewrite using
Rust.

Documentation
-------------

The current implementation supports SPIR-V 1.4 (Revision 1).

Multiple crates are published from this project:

|  Name  |   Crate   |   Docs   |
| :----: | :-------: | :------: |
| rspirv | [![Crate][img-crate-rspirv]][crate-rspirv]   | [![Documentation][img-doc-rspirv]][doc-rspirv]   |
| spirv  | [![Crate][img-crate-headers]][crate-headers] | [![Documentation][img-doc-headers]][doc-headers] |

In total rspirv APIs contains:
* The [SPIR-V header][doc-headers] (all SPIR-V structs, enums, and constants)
* The whole [SPIR-V grammar][doc-grammar] (instruction layouts and their
  operands)
* A [data representation][doc-dr] of SPIR-V modules and its loader and builder
* SPIR-V [binary][doc-binary] module decoding and parsing functionalities

The Khronos SPIR-V [JSON grammar][json-grammar] is leveraged to generate parts
of the source code using [`rspirv-autogen`][autogen].

Please see the links to docs.rs for detailed documentation.

Status
------

I plan to implement several functionalities:

- [x] SPIR-V [data representation][doc-dr] (DR)
- [x] SPIR-V module [builder][doc-builder]
- [ ] SPIR-V module assembler
- [x] SPIR-V binary module [parser][doc-parser]
- [x] SPIR-V binary module disassembler
- [ ] HLSL/GLSL to SPIR-V frontend (maybe)
- [ ] SPIR-V DR to LLVM IR transformation (maybe)

The DR doesn't handle `OpLine` and `OpNoLine` well right now.

The SPIR-V binary module parser is feature complete.

Usage
-----

This project uses associated constants, which became available in the stable channel
since [1.20][rust-1.20]. So to compile with a compiler from the stable channel,
please make sure that the version is >= 1.20.

Examples
--------

Building a SPIR-V module, assembling it, parsing it, and then disassembling it:

```rust
extern crate rspirv;
extern crate spirv as spirv;

use rspirv::binary::Assemble;
use rspirv::binary::Disassemble;

fn main() {
    // Building
    let mut b = rspirv::dr::Builder::new();
    b.set_version(1, 0);
    b.capability(spirv::Capability::Shader);
    b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);
    let void = b.type_void();
    let voidf = b.type_function(void, vec![]);
    let fun = b
        .begin_function(
            void,
            None,
            spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::CONST,
            voidf,
        )
        .unwrap();
    b.begin_basic_block(None).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(spirv::ExecutionModel::Vertex, fun, "foo", vec![]);
    let module = b.module();

    // Assembling
    let code = module.assemble();
    assert!(code.len() > 20); // Module header contains 5 words
    assert_eq!(spirv::MAGIC_NUMBER, code[0]);

    // Parsing
    let mut loader = rspirv::dr::Loader::new();
    rspirv::binary::parse_words(&code, &mut loader).unwrap();
    let module = loader.module();

    // Disassembling
    assert_eq!(
        module.disassemble(),
        "; SPIR-V\n\
         ; Version: 1.0\n\
         ; Generator: rspirv\n\
         ; Bound: 5\n\
         OpCapability Shader\n\
         OpMemoryModel Logical GLSL450\n\
         OpEntryPoint Vertex %3 \"foo\"\n\
         %1 = OpTypeVoid\n\
         %2 = OpTypeFunction %1\n\
         %3 = OpFunction  %1  DontInline|Const %2\n\
         %4 = OpLabel\n\
         OpReturn\n\
         OpFunctionEnd"
    );
}
```

Directory Organization
----------------------

There are multiple crates inside this repo:

- `autogen/`: Crate to generate various Rust code snippets used in the modules
  in `spirv/` and `rspirv/`, from SPIR-V's JSON grammar. If you are not
  modifying `spirv/` or `rspirv/`, you don't need to care about this directory.
- `spirv/`: The `spirv` crate.
- `rspirv/`: The core `rspirv` crate.
- `dis/`: A binary SPIR-V disassembler based on the `rspirv` crate.
- `spirv-blobs`: SPIR-V blobs provided by the user for testing.

Build
-----

```sh
git clone https://github.com/gfx-rs/rspirv.git /path/to/rspirv
```

If you just want to compile and use the `spirv` crate:

```sh
cd /path/to/rspirv/spirv
cargo build
```

If you just want to compile and use the `rspirv` crate:

```sh
cd /path/to/rspirv/rspirv
cargo build
```

If you want to refresh the `spirv` or `rspirv` crate with new code
snippets generated from SPIR-V's JSON grammar:

```sh
cd /path/to/rspirv
# Clone the SPIRV-Headers repo
git submodule update --init
cargo run -p rspirv-autogen
```

Test
----

Running `cargo test` would scan `spirv-blobs` folder and its subfolders
(with symlinks followed) for any SPIR-V binary blobs. The test will try to
load them, disassemble them, and then compose back from the internal
representations, ensuring the smooth round-trip.

Contributions
-------------

This project is licensed under the Apache License, Version 2.0
([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).
Please see [CONTRIBUTING](CONTRIBUTING.md) before contributing.

### Authors

This project is initialized Lei Zhang ([@antiagainst][me]) and currently
developed by the gfx-rs [Translators][github-translators] team.

[autogen]: ./autogen/README.md
[img-crate-rspirv]: https://img.shields.io/crates/v/rspirv.svg
[img-doc-rspirv]: https://docs.rs/rspirv/badge.svg
[crate-rspirv]: https://crates.io/crates/rspirv
[doc-rspirv]: https://docs.rs/rspirv
[img-crate-headers]: https://img.shields.io/crates/v/spirv.svg
[img-doc-headers]: https://docs.rs/spirv/badge.svg
[crate-headers]: https://crates.io/crates/spirv
[doc-headers]: https://docs.rs/spirv
[spirv]: https://www.khronos.org/registry/spir-v/
[vulkan]: https://www.khronos.org/vulkan/
[opengl]: https://www.opengl.org/
[opencl]: https://www.khronos.org/opencl/
[me]: https://github.com/antiagainst
[github-translators]: https://github.com/orgs/gfx-rs/teams/translators
[json-grammar]: https://github.com/KhronosGroup/SPIRV-Headers/tree/master/include/spirv
[spirv-tools]: https://github.com/KhronosGroup/SPIRV-Tools
[doc-dr]: https://docs.rs/rspirv/*/rspirv/dr/index.html
[doc-builder]: https://docs.rs/rspirv/*/rspirv/dr/struct.Builder.html
[doc-parser]: https://docs.rs/rspirv/*/rspirv/binary/struct.Parser.html
[doc-grammar]: https://docs.rs/rspirv/*/rspirv/grammar/index.html
[doc-binary]: https://docs.rs/rspirv/*/rspirv/binary/index.html
[rust-1.20]: https://blog.rust-lang.org/2017/08/31/Rust-1.20.html
