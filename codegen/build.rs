// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod binary;
mod header;
mod mr;
mod sr;
mod structs;
mod table;
mod utils;

use std::{
    env,
    fs,
    io::{Read, Write},
    path::PathBuf,
    process,
};
use utils::write_copyright_autogen_comment;

fn write(path: &PathBuf, contents: String) {
    let mut f = fs::File::create(path)
        .expect(&format!("cannot open file: {:?}", path));
    write_copyright_autogen_comment(&mut f);
    write!(f, "{}", contents).unwrap()
}

fn write_formatted(path: &PathBuf, contents: String) {
    write(path, contents);
    match process::Command::new("rustfmt")
        .arg(path)
        .status() {
            Ok(status) if !status.success() => {
                println!("cargo:warning=failed to rustfmt {:?}", path);
            }
            Ok(_) => {}
            Err(_) => {
                println!("cargo:warning=failed to execute rustfmt");
            }
        };
}

fn main() {
    // Path to the SPIR-V core grammar file.
    let env_var = env::var("CARGO_MANIFEST_DIR").unwrap();
    let codegen_src_dir = PathBuf::from(&env_var);

    if !codegen_src_dir.join("external/SPIRV-Headers").exists() {
        panic!("SPIRV-Headers missing - please checkout using git submodule");
    }

    let mut contents = String::new();

    {
        let path = codegen_src_dir.join("external/spirv.core.grammar.json");
        let mut file = fs::File::open(path).unwrap();
        file.read_to_string(&mut contents).unwrap();
    }
    let grammar: structs::Grammar = serde_json::from_str(&contents).unwrap();

    // For GLSLstd450 extended instruction set.
    {
        let path = codegen_src_dir.join(
            "external/SPIRV-Headers/include/spirv/unified1/extinst.glsl.std.450.grammar.json");
        let mut file = fs::File::open(path).unwrap();
        contents.clear();
        file.read_to_string(&mut contents).unwrap();
    }
    let gl_grammar: structs::ExtInstSetGrammar = serde_json::from_str(&contents).unwrap();

    // For OpenCL extended instruction set.
    {
        let path = codegen_src_dir.join(
            "external/SPIRV-Headers/include/spirv/unified1/extinst.opencl.std.100.grammar.json");
        let mut file = fs::File::open(path).unwrap();
        contents.clear();
        file.read_to_string(&mut contents).unwrap();
    }
    let cl_grammar: structs::ExtInstSetGrammar = serde_json::from_str(&contents).unwrap();

    // Path to the generated SPIR-V header file.
    write(
        &codegen_src_dir.join("../spirv/autogen_spirv.rs"),
        {
            let core = header::gen_spirv_header(&grammar);
            let gl = header::gen_glsl_std_450_opcodes(&gl_grammar);
            let cl = header::gen_opencl_std_opcodes(&cl_grammar);
            format!("{}\n{}\n{}", core, gl, cl)
        }
    );

    // Path to the generated instruction table.
    write(
        &codegen_src_dir.join("../rspirv/grammar/autogen_table.rs"),
        table::gen_grammar_inst_table_operand_kinds(&grammar),
    );
    // Path to the generated GLSLstd450 extended instruction set header.
    write(
        &codegen_src_dir.join("../rspirv/grammar/autogen_glsl_std_450.rs"),
        table::gen_glsl_std_450_inst_table(&gl_grammar),
    );
    write(
        &codegen_src_dir.join("../rspirv/grammar/autogen_opencl_std_100.rs"),
        table::gen_opencl_std_100_inst_table(&cl_grammar),
    );

    // Path to the generated operands kind in data representation.
    write(
        &codegen_src_dir.join("../rspirv/mr/autogen_operand.rs"),
        mr::gen_mr_operand_kinds(&grammar.operand_kinds),
    );
    // Path to the generated builder for data representation.
    write(
        &codegen_src_dir.join("../rspirv/mr/build/autogen_type.rs"),
        mr::gen_mr_builder_types(&grammar),
    );
    // Path to the generated builder for data representation.
    write(
        &codegen_src_dir.join("../rspirv/mr/build/autogen_terminator.rs"),
        mr::gen_mr_builder_terminator(&grammar),
    );
    // Path to the generated builder for data representation.
    write(
        &codegen_src_dir.join("../rspirv/mr/build/autogen_annotation.rs"),
        mr::gen_mr_builder_annotation(&grammar),
    );
    // Path to the generated builder for data representation.
    write(
        &codegen_src_dir.join("../rspirv/mr/build/autogen_constant.rs"),
        mr::gen_mr_builder_constants(&grammar),
    );
    // Path to the generated builder for data representation.
    write(
        &codegen_src_dir.join("../rspirv/mr/build/autogen_debug.rs"),
        mr::gen_mr_builder_debug(&grammar),
    );
    // Path to the generated builder for data representation.
    write(
        &codegen_src_dir.join("../rspirv/mr/build/autogen_norm_insts.rs"),
        mr::gen_mr_builder_normal_insts(&grammar),
    );

    // Path to the generated decoding errors.
    write(
        &codegen_src_dir.join("../rspirv/binary/autogen_error.rs"),
        binary::gen_operand_decode_errors(&grammar.operand_kinds),
    );
    // Path to the generated operand decoding methods.
    write(
        &codegen_src_dir.join("../rspirv/binary/autogen_decode_operand.rs"),
        binary::gen_operand_decode_methods(&grammar.operand_kinds),
    );
    // Path to the generated operand parsing methods.
    write(
        &codegen_src_dir.join("../rspirv/binary/autogen_parse_operand.rs"),
        binary::gen_operand_parse_methods(&grammar.operand_kinds),
    );
    // Path to the generated operand parsing methods.
    write(
        &codegen_src_dir.join("../rspirv/binary/autogen_disas_operand.rs"),
        binary::gen_disas_bit_enum_operands(&grammar.operand_kinds),
    );

    write_formatted(
        &codegen_src_dir.join("../rspirv/sr/autogen_decoration.rs"),
        sr::gen_sr_decoration(&grammar),
    );
    write_formatted(
        &codegen_src_dir.join("../rspirv/sr/autogen_type_enum_check.rs"),
        sr::gen_sr_type_check(&grammar),
    );
    write_formatted(
        &codegen_src_dir.join("../rspirv/sr/autogen_type_creation.rs"),
        sr::gen_sr_type_creation(&grammar),
    );
    let (inst_enums, inst_structs) = sr::gen_sr_instruction(&grammar);
    write_formatted(
        &codegen_src_dir.join("../rspirv/sr/autogen_instruction_enums.rs"),
        inst_enums,
    );
    write_formatted(
        &codegen_src_dir.join("../rspirv/sr/autogen_instruction_structs.rs"),
        inst_structs,
    );
}
