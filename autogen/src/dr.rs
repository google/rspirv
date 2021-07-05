use std::collections::HashSet;

use crate::structs;
use crate::utils::*;

use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;

/// Returns true if the given operand kind can potentially have additional
/// parameters.
#[inline(always)]
pub fn has_additional_params(grammar: &structs::OperandKind) -> bool {
    grammar.enumerants.iter().any(|e| !e.parameters.is_empty())
}

/// Returns true if the given operand can potentially have additional
/// parameters.
pub fn operand_has_additional_params(
    operand: &structs::Operand,
    kinds: &[structs::OperandKind],
) -> bool {
    kinds
        .iter()
        .find(|kind| kind.kind == operand.kind)
        .map_or(false, |kind| has_additional_params(kind))
}

fn get_param_or_arg_list(
    params: &[structs::Operand],
    keep_result_id: bool,
    kinds: &[structs::OperandKind],
    is_params: bool,
) -> Vec<TokenStream> {
    let mut list: Vec<_> = params
        .iter()
        .enumerate()
        .filter_map(|(param_index, param)| {
            let name = get_param_name(params, param_index);
            let kind = get_enum_underlying_type(&param.kind, true);
            if param.kind == "IdResult" {
                if keep_result_id {
                    if is_params {
                        Some(quote! { result_id: Option<spirv::Word> })
                    } else {
                        Some(quote! { result_id })
                    }
                } else {
                    None
                }
            } else if is_params {
                Some(match param.quantifier {
                    structs::Quantifier::One => quote! { #name: #kind },
                    structs::Quantifier::ZeroOrOne => quote! { #name: Option<#kind> },
                    structs::Quantifier::ZeroOrMore => {
                        quote! { #name: impl IntoIterator<Item = #kind> }
                    }
                })
            } else {
                Some(quote! { #name })
            }
        })
        .collect();
    // The last operand may require additional parameters.
    if let Some(o) = params.last() {
        if operand_has_additional_params(o, kinds) {
            if is_params {
                list.push(quote! { additional_params: impl IntoIterator<Item = dr::Operand> });
            } else {
                list.push(quote! { additional_params });
            }
        }
    }
    list
}

/// Returns the parameter list excluding result id.
fn get_param_list(
    params: &[structs::Operand],
    keep_result_id: bool,
    kinds: &[structs::OperandKind],
) -> Vec<TokenStream> {
    get_param_or_arg_list(params, keep_result_id, kinds, true)
}

fn get_arg_list(
    params: &[structs::Operand],
    keep_result_id: bool,
    kinds: &[structs::OperandKind],
) -> Vec<TokenStream> {
    get_param_or_arg_list(params, keep_result_id, kinds, false)
}

/// Returns a suitable function name for the given `opname`.
fn get_function_name(opname: &str) -> TokenStream {
    if opname == "OpReturn" {
        quote! { ret }
    } else if opname == "OpReturnValue" {
        quote! { ret_value }
    } else {
        let name = as_ident(&opname[2..].to_snake_case());
        quote! { #name }
    }
}

/// Returns a suitable function name for the given `opname`.
fn get_function_name_with_prepend(prepend: &str, opname: &str) -> TokenStream {
    let name = if opname == "OpReturn" {
        as_ident(&format!("{}ret", prepend))
    } else if opname == "OpReturnValue" {
        as_ident(&format!("{}ret_value", prepend))
    } else {
        as_ident(&format!("{}{}", prepend, &opname[2..].to_snake_case()))
    };

    quote! { #name }
}

/// Returns the initializer list for all the parameters required to appear
/// once and only once.
fn get_init_list(params: &[structs::Operand]) -> Vec<TokenStream> {
    params
        .iter()
        .enumerate()
        .filter_map(|(param_index, param)| {
            if param.quantifier == structs::Quantifier::One {
                if param.kind == "IdResult" || param.kind == "IdResultType" {
                    // These two operands are not stored in the operands field.
                    None
                } else {
                    let name = get_param_name(params, param_index);
                    let kind = get_dr_operand_kind(&param.kind);
                    let value = if kind == "LiteralString" {
                        quote! { #name.into() }
                    } else {
                        quote! { #name }
                    };
                    Some(quote! { dr::Operand::#kind(#value) })
                }
            } else {
                None
            }
        })
        .collect()
}

fn get_push_extras(
    params: &[structs::Operand],
    kinds: &[structs::OperandKind],
    container: TokenStream,
) -> Vec<TokenStream> {
    let mut list: Vec<_> = params
        .iter()
        .enumerate()
        .filter_map(|(param_index, param)| {
            let name = get_param_name(params, param_index);
            match param.quantifier {
                structs::Quantifier::One => None,
                structs::Quantifier::ZeroOrOne => {
                    let kind = get_dr_operand_kind(&param.kind);
                    let value = if kind == "LiteralString" {
                        quote! { v.into() }
                    } else {
                        quote! { v }
                    };
                    Some(quote! {
                        if let Some(v) = #name {
                            #container.push(dr::Operand::#kind(#value));
                        }
                    })
                }
                structs::Quantifier::ZeroOrMore => {
                    if param.kind == "PairLiteralIntegerIdRef" {
                        Some(quote! {
                            for v in #name {
                                #container.push(v.0);
                                #container.push(dr::Operand::IdRef(v.1));
                            }
                        })
                    } else if param.kind == "PairIdRefLiteralInteger" {
                        Some(quote! {
                            for v in #name {
                                #container.push(dr::Operand::IdRef(v.0));
                                #container.push(dr::Operand::LiteralInt32(v.1));
                            }
                        })
                    } else if param.kind == "PairIdRefIdRef" {
                        Some(quote! {
                            for v in #name {
                                #container.push(dr::Operand::IdRef(v.0));
                                #container.push(dr::Operand::IdRef(v.1));
                            }
                        })
                    } else {
                        let kind = get_dr_operand_kind(&param.kind);
                        Some(quote! {
                            #container.extend(#name.into_iter().map(dr::Operand::#kind));
                        })
                    }
                }
            }
        })
        .collect();
    // The last operand may require additional parameters.
    if let Some(o) = params.last() {
        if operand_has_additional_params(o, kinds) {
            list.push(quote! {
                #container.extend(additional_params);
            });
        }
    }
    list
}

/// Returns the generated dr::Operand and its fmt::Display implementation by
/// walking the given SPIR-V operand kinds `grammar`.
pub fn gen_dr_operand_kinds(grammar: &[structs::OperandKind]) -> TokenStream {
    let kinds: Vec<_> = grammar
        .iter()
        .map(|element| element.kind.as_str())
        .filter(|element| {
            // Pair kinds are not used in dr::Operand.
            // LiteralContextDependentNumber is replaced by suitable literals.
            // LiteralInteger is replaced by LiteralInt32.
            // IdResult and IdResultType are not stored as operands in `dr`.
            !(element.starts_with("Pair")
                || *element == "LiteralContextDependentNumber"
                || *element == "LiteralInteger"
                || *element == "IdResult"
                || *element == "IdResultType")
        })
        .map(as_ident)
        .collect();

    let kind_to_enum: Vec<_> = grammar
        .iter()
        .map(|element| {
            (
                element.kind.as_str(),
                element.category,
                element.enumerants.clone(),
            )
        })
        .collect();

    let kind_and_ty = {
        let id_kinds = kinds
            .iter()
            .filter(|element| element.to_string().starts_with("Id"))
            .map(|element| (element.clone(), quote! { spirv::Word }));

        let num_kinds = vec![
            (format_ident!("LiteralInt32"), quote! {u32}),
            (format_ident!("LiteralInt64"), quote! {u64}),
            (format_ident!("LiteralFloat32"), quote! {f32}),
            (format_ident!("LiteralFloat64"), quote! {f64}),
            (format_ident!("LiteralExtInstInteger"), quote! {u32}),
            (
                format_ident!("LiteralSpecConstantOpInteger"),
                quote! {spirv::Op},
            ),
        ];

        let str_kinds = kinds
            .iter()
            .filter(|element| element.to_string().ends_with("String"))
            .map(|element| (element.clone(), quote! {String}));

        let enum_kinds = kinds
            .iter()
            .filter(|element| {
                let element = element.to_string();
                !(element.starts_with("Id")
                    || element.ends_with("String")
                    || element.ends_with("Integer")
                    || element.ends_with("Number"))
            })
            .map(|element| (element.clone(), quote! {spirv::#element}));

        enum_kinds
            .chain(id_kinds)
            .chain(num_kinds.into_iter())
            .chain(str_kinds)
            .collect::<Vec<_>>()
    };

    let kind_enum = {
        let mut types_seen = HashSet::new();

        // To prevent ambiguity we don't want to generate an implementation for `Word` at all:
        types_seen.insert(quote!(spirv::Word).to_string());

        let kinds = kind_and_ty.iter().map(|(kind, ty)| {
            let v = quote!(#kind(#ty));
            if types_seen.insert(ty.to_string()) {
                v
            } else {
                quote!(#[from(ignore)] #v)
            }
        });
        quote! {
            #[doc = "Data representation of a SPIR-V operand."]
            #[derive(Clone, Debug, PartialEq, From)]
            #[allow(clippy::upper_case_acronyms)]
            pub enum Operand {
                #(#kinds,)*
            }
        }
    };

    let translate_quant = |quant: crate::structs::Quantifier| match quant {
        structs::Quantifier::One => {
            quote! { crate::grammar::OperandQuantifier::One }
        }
        structs::Quantifier::ZeroOrOne => {
            quote! { crate::grammar::OperandQuantifier::ZeroOrOne }
        }
        structs::Quantifier::ZeroOrMore => {
            quote! { crate::grammar::OperandQuantifier::ZeroOrMore }
        }
    };

    let impl_code = {
        // impl fmt::Display for dr::Operand.
        let mut kinds = kinds;
        kinds.extend(
            [
                "LiteralInt32",
                "LiteralInt64",
                "LiteralFloat32",
                "LiteralFloat64",
            ]
            .iter()
            .cloned()
            .map(as_ident),
        );
        let cases = kinds.iter().map(|element| {
            if element == "Dim" {
                // Skip the "Dim" prefix, which is only used in the API to
                // avoid having an enumerant name starting with a number
                quote! {
                    Operand::#element(ref v) => write!(f, "{}", &format!("{:?}", v)[3..])
                }
            } else if element == "IdRef" || element == "IdScope" || element == "IdMemorySemantics" {
                // Prefix operands with a % so they're distinguishable from e.g. integer arguments.
                quote! {
                    Operand::#element(ref v) => write!(f, "%{}", v)
                }
            } else {
                quote! {
                    Operand::#element(ref v) => write!(f, "{:?}", v)
                }
            }
        });
        let unwraps = kind_and_ty.iter().map(|(kind, ty)| {
            let unwrap_kind = format_ident!("unwrap_{}", kind.to_string().to_snake_case());
            let (ret_ty, self_prefix) = if ty.to_string() == "String" {
                (quote! {&str}, quote! {})
            } else {
                (ty.clone(), quote!(*))
            };
            let panic_arg = format!("Expected Operand::{}, got {{}} instead", kind);
            quote! {
                pub fn #unwrap_kind(&self) -> #ret_ty {
                    match #self_prefix self {
                        Self::#kind(v) => v,
                        ref other => panic!(#panic_arg, other),
                    }
                }
            }
        });

        let operand_metadata = kind_to_enum
            .iter()
            .filter_map(|(kind, category, enumerators)| {
                if enumerators.is_empty() {
                    return None;
                }

                let mut capability_clauses = BTreeMap::new();
                let mut extension_clauses = BTreeMap::new();
                let mut operand_clauses = BTreeMap::new();

                let kind = as_ident(kind);
                let mut seen_discriminator = BTreeMap::new();

                for e in enumerators {
                    if seen_discriminator.get(&e.value).is_none() {
                        let name = match category {
                            structs::Category::BitEnum => {
                                use heck::ShoutySnakeCase;

                                as_ident(&e.symbol.to_shouty_snake_case().replace("NA_N", "NAN"))
                            }
                            structs::Category::ValueEnum => {
                                let name_str = if kind == "Dim" {
                                    let mut name = "Dim".to_string();
                                    name.push_str(&e.symbol);
                                    name
                                } else {
                                    e.symbol.to_string()
                                };

                                as_ident(&name_str)
                            }
                            _ => panic!("Unexpected operand type"),
                        };

                        seen_discriminator.insert(e.value, name.clone());

                        capability_clauses
                            .entry(&e.capabilities)
                            .or_insert_with(Vec::new)
                            .push(name.clone());

                        extension_clauses
                            .entry(&e.extensions)
                            .or_insert_with(Vec::new)
                            .push(name.clone());

                        if !e.parameters.is_empty() {
                            operand_clauses
                                .entry(&e.parameters)
                                .or_insert_with(Vec::new)
                                .push(name.clone())
                        }
                    }
                }

                let extensions = if category == &structs::Category::BitEnum {
                    let extensions = extension_clauses
                            .into_iter()
                            .filter(|(k, _)| !k.is_empty())
                            .map(|(k, v)| {
                                let kinds = std::iter::repeat(quote! { s::#kind });

                                quote! {
                                    if v.intersects(#(#kinds::#v)|*) {
                                        result.extend_from_slice(&[#( #k ),*])
                                    }
                                }
                            }).collect::<Vec<_>>();

                        if extensions.is_empty() {
                            quote! {}
                        } else {
                            quote! {
                                Self::#kind(v) => {
                                    let mut result = vec![];
                                    #( #extensions );*;
                                    result
                                }
                            }
                        }
                } else {
                    let extensions = extension_clauses.into_iter().map(|(k, v)| {
                        let kinds = std::iter::repeat(quote! { s::#kind });
                        quote! {
                            #( #kinds::#v )|* => vec![#( #k ),*]
                        }
                    });

                    quote! {
                        Self::#kind(v) => match v {
                            #( #extensions ),*
                        },
                    }
                };

                let capabilities = if category == &structs::Category::BitEnum {
                    let capabilities = capability_clauses
                            .into_iter()
                            .filter(|(k, _)| !k.is_empty())
                            .map(|(k, v)| {
                                let kinds = std::iter::repeat(quote! { s::#kind });
                                let capabilities = k.iter().map(|cap| as_ident(cap));

                                quote! {
                                    if v.intersects(#(#kinds::#v)|*) {
                                        result.extend_from_slice(&[#( spirv::Capability::#capabilities ),*])
                                    }
                                }
                            }).collect::<Vec<_>>();

                        if capabilities.is_empty() {
                            quote! {}
                        } else {
                            quote! {
                                Self::#kind(v) => {
                                    let mut result = vec![];
                                    #( #capabilities );*;
                                    result
                                }
                            }
                        }
                } else {
                    let capabilities = capability_clauses.into_iter().map(|(k, v)| {
                        let kinds = std::iter::repeat(quote! { s::#kind });
                        let capabilities = k.iter().map(|cap| as_ident(cap));
                        quote! {
                            #( #kinds::#v )|* => vec![#( spirv::Capability::#capabilities ),*]
                        }
                    });

                    quote! {
                        Self::#kind(v) => match v {
                            #( #capabilities ),*
                        },
                    }
                };

                let operands = if operand_clauses.is_empty() {
                    quote! {}
                } else if category == &structs::Category::BitEnum {
                    let operands = operand_clauses
                        .into_iter()
                        .map(|(k, v)| {
                            let operands = k.iter().map(|op| {
                                let kind = as_ident(&op.kind);
                                let quant = translate_quant(op.quantifier);

                                quote! {
                                    crate::grammar::LogicalOperand {
                                        kind: crate::grammar::OperandKind::#kind,
                                        quantifier: #quant
                                    }
                                }
                            });

                            let kinds = std::iter::repeat(quote! { s::#kind });

                            quote! {
                                result.extend([#(#kinds::#v,)*].iter().filter(|arg| {
                                    v.contains(**arg)
                                }).flat_map(|_| { [#( #operands ),*].iter().cloned() }))
                            }
                        });

                    quote! {
                        Self::#kind(v) => {
                            let mut result = vec![];
                            #( #operands );*;
                            result
                        }
                    }
                } else {
                    let operands = operand_clauses
                        .into_iter()
                        .map(|(k, v)| {
                            let operands = k.iter().map(|op| {
                                let kind = as_ident(&op.kind);
                                let quant = translate_quant(op.quantifier);

                                quote! {
                                    crate::grammar::LogicalOperand {
                                        kind: crate::grammar::OperandKind::#kind,
                                        quantifier: #quant
                                    }
                                }
                            });

                            let kinds = std::iter::repeat(quote! { s::#kind });

                            quote! {
                                #( #kinds::#v )|* => vec![#( #operands ),*]
                            }
                        });

                    quote! {
                        Self::#kind(v) => match v {
                            #( #operands ),*,
                            _ => vec![]
                        },
                    }
                };

                Some((extensions, capabilities, operands))
            })
            .collect::<Vec<_>>();

        let required_extensions = operand_metadata.iter().map(|e| &e.0);
        let required_capabilities = operand_metadata.iter().map(|e| &e.1);
        let additional_params = operand_metadata.iter().map(|e| &e.2);

        quote! {
            impl fmt::Display for Operand {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match *self {
                        #(#cases),*
                    }
                }
            }

            impl Operand {
                #(#unwraps)*
                pub fn id_ref_any(&self) -> Option<spirv::Word> {
                    match *self {
                        Self::IdRef(v) | Self::IdScope(v) | Self::IdMemorySemantics(v) => Some(v),
                        _ => None,
                    }
                }
                pub fn id_ref_any_mut(&mut self) -> Option<&mut spirv::Word> {
                    match self {
                        Self::IdRef(v) | Self::IdScope(v) | Self::IdMemorySemantics(v) => Some(v),
                        _ => None,
                    }
                }

                pub fn required_capabilities(&self) -> Vec<spirv::Capability> {
                    use spirv as s;
                    match self {
                        #(#required_capabilities)*
                        _ => vec![]
                    }
                }

                pub fn required_extensions(&self) -> Vec<&'static str> {
                    use spirv as s;
                    match self {
                        #(#required_extensions)*
                        _ => vec![]
                    }
                }

                pub fn additional_operands(&self) -> Vec<crate::grammar::LogicalOperand> {
                    use spirv as s;
                    match self {
                       #(#additional_params)*
                       _ => vec![]
                    }
                }
            }
        }
    };

    quote! {
        #kind_enum
        #impl_code
    }
}

/// Returns the generated build methods for SPIR-V types by walking the given
/// SPIR-V instructions `grammar`.
pub fn gen_dr_builder_types(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all types.
    let elements = grammar.instructions.iter().filter(|inst| {
        inst.class == Some(structs::Class::Type) && inst.opname != "OpTypeForwardPointer" &&
            inst.opname != "OpTypePointer" && inst.opname != "OpTypeOpaque"
    }).map(|inst| {
        // Parameter list for this build method.
        let param_list = get_param_list(&inst.operands, false, kinds);
        let arg_list = get_arg_list(&inst.operands, false, kinds);
        // Initializer list for constructing the operands parameter
        // for Instruction.
        let init_list = get_init_list(&inst.operands[1..]);
        // Parameters that are not single values thus need special treatment.
        let extras = get_push_extras(&inst.operands[1..],
                                     kinds,
                                     quote! { inst.operands });
        let opcode = as_ident(&inst.opname[2..]);
        let name = as_ident(&inst.opname[2..].to_snake_case());

        let comment = format!("Appends an Op{} instruction and returns the result id, or return the existing id if the instruction was already present.", opcode);
        let name_id = format_ident!("{}_id", name);
        quote! {
            #[doc = #comment]
            pub fn #name(&mut self,#(#param_list),*) -> spirv::Word {
                self.#name_id(None, #(#arg_list),*)
            }

            #[doc = #comment]
            pub fn #name_id(&mut self, result_id: Option<spirv::Word>,#(#param_list),*) -> spirv::Word {
                let mut inst = dr::Instruction::new(spirv::Op::#opcode, None, result_id, vec![#(#init_list),*]);
                #(#extras)*
                if let Some(result_id) = result_id {
                    self.module.types_global_values.push(inst);
                    result_id
                } else if let Some(id) = self.dedup_insert_type(&inst) {
                    id
                } else {
                    let new_id = self.id();
                    inst.result_id = Some(new_id);
                    self.module.types_global_values.push(inst);
                    new_id
                }
            }
        }
    });
    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}

fn is_terminator_instruction(inst: &structs::Instruction) -> bool {
    match inst.class {
        Some(structs::Class::Reserved) => matches!(
            inst.opname.as_str(),
            "OpTerminateRayKHR" | "OpIgnoreIntersectionKHR"
        ),
        Some(structs::Class::Branch) => !matches!(inst.opname.as_str(), "OpPhi" | "OpLabel" | "OpSelectionMerge"),
        _ => false,
    }
}

pub fn gen_dr_builder_terminator(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all types.
    let elements = grammar
        .instructions
        .iter()
        .filter(|inst| is_terminator_instruction(inst))
        .map(|inst| {
            let params = get_param_list(&inst.operands, false, kinds);
            let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
            let opcode = as_ident(&inst.opname[2..]);
            let comment = format!(
                "Appends an Op{} instruction and ends the current block.",
                opcode
            );
            let insert_comment = format!(
                "Insert an Op{} instruction and ends the current block.",
                opcode
            );
            let name = get_function_name(&inst.opname);
            let init = get_init_list(&inst.operands);
            let insert_name = get_function_name_with_prepend("insert_", &inst.opname);

            quote! {
                #[doc = #comment]
                pub fn #name(&mut self,#(#params),*) -> BuildResult<()> {
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, None, None, vec![#(#init),*]);
                    #(#extras)*
                    self.end_block(inst)
                }

                #[doc = #insert_comment]
                pub fn #insert_name(&mut self, insert_point: InsertPoint, #(#params),*) -> BuildResult<()> {
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, None, None, vec![#(#init),*]);
                    #(#extras)*
                    self.insert_end_block(insert_point, inst)
                }
            }
        });
    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_normal_insts(grammar: &structs::Grammar) -> TokenStream {
    use crate::structs::Class::*;

    let kinds = &grammar.operand_kinds;
    // Generate build methods for all normal instructions (instructions must be
    // in some block).
    let elements = grammar.instructions.iter().filter(|inst| {
        let skip =
            inst.class == Some(Type) ||
            inst.class == Some(Constant) ||
            inst.class == Some(ExtensionDecl) ||
            (inst.class == Some(FunctionStruct) && inst.opname != "OpFunctionCall") ||
            inst.class == Some(Debug) ||
            inst.class == Some(Annotation) ||
            is_terminator_instruction(inst) ||
            // Labels should not be inserted but attached instead.
            inst.opname == "OpLabel" ||
            inst.class == Some(ModeSetting) ||
            inst.class == Some(Exclude) ||
            inst.opname == "OpTypeForwardPointer" ||
            inst.opname == "OpTypePointer" ||
            inst.opname == "OpTypeOpaque" ||
            inst.opname == "OpUndef" ||
            inst.opname == "OpVariable" ||
            inst.opname.starts_with("OpType");
        !skip
    }).map(|inst| {
        let params = get_param_list(&inst.operands, true, kinds);
        let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
        let opcode = as_ident(&inst.opname[2..]);
        let comment = format!("Appends an Op{} instruction to the current block.", opcode);
        let insert_comment = format!("Appends an Op{} instruction to the current block.", opcode);
        let name = get_function_name(&inst.opname);
        let insert_name = get_function_name_with_prepend("insert_", &inst.opname);
        let init = get_init_list(&inst.operands);

        if !inst.operands.is_empty() && inst.operands[0].kind == "IdResultType" {
            // For normal instructions, they either have both result type and
            // result id or have none.
            quote! {
                #[doc = #comment]
                pub fn #name(&mut self,#(#params),*) -> BuildResult<spirv::Word> {
                    let _id = result_id.unwrap_or_else(|| self.id());
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, Some(result_type), Some(_id), vec![#(#init),*]);
                    #(#extras)*
                    self.insert_into_block(InsertPoint::End, inst)?;
                    Ok(_id)
                }

                #[doc = #insert_comment]
                pub fn #insert_name(&mut self,insert_point: InsertPoint, #(#params),*) -> BuildResult<spirv::Word> {
                    let _id = result_id.unwrap_or_else(|| self.id());
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, Some(result_type), Some(_id), vec![#(#init),*]);
                    #(#extras)*
                    self.insert_into_block(insert_point, inst)?;
                    Ok(_id)
                }
            }
        } else {
            quote! {
                #[doc = #comment]
                pub fn #name(&mut self,#(#params),*) -> BuildResult<()> {
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, None, None, vec![#(#init),*]);
                    #(#extras)*
                    self.insert_into_block(InsertPoint::End, inst)?;
                    Ok(())
                }

                #[doc = #comment]
                pub fn #insert_name(&mut self,insert_point: InsertPoint, #(#params),*) -> BuildResult<()> {
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, None, None, vec![#(#init),*]);
                    #(#extras)*
                    self.insert_into_block(insert_point, inst)?;
                    Ok(())
                }
            }
        }
    });
    quote! {
        #[allow(clippy::identity_conversion, clippy::too_many_arguments)]
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_constants(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements = grammar
        .instructions
        .iter()
        .filter(|inst| {
            inst.class == Some(structs::Class::Constant)
                && inst.opname != "OpConstant"
                && inst.opname != "OpSpecConstant"
        })
        .map(|inst| {
            let params = get_param_list(&inst.operands, false, kinds);
            let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
            let opcode = as_ident(&inst.opname[2..]);
            let comment = format!("Appends an Op{} instruction.", opcode);
            let name = get_function_name(&inst.opname);
            let init = get_init_list(&inst.operands);
            quote! {
                #[doc = #comment]
                pub fn #name(&mut self,#(#params),*) -> spirv::Word {
                    let id = self.id();
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, Some(result_type), Some(id), vec![#(#init),*]);
                    #(#extras)*
                    self.module.types_global_values.push(inst);
                    id
                }
            }
        });

    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_debug(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements = grammar
        .instructions
        .iter()
        .filter(|inst| {
            inst.class == Some(structs::Class::Debug)
                && !matches!(inst.opname.as_str(), "OpString" | "OpLine" | "OpNoLine")
        })
        .map(|inst| {
            let params = get_param_list(&inst.operands, false, kinds);
            let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
            let opcode = as_ident(&inst.opname[2..]);
            let comment = format!("Appends an Op{} instruction.", opcode);
            let name = get_function_name(&inst.opname);
            let init = get_init_list(&inst.operands);
            // The debug section is split into three subsections
            let section = match inst.opname.as_str() {
                "OpSourceExtension" | "OpSource" | "OpSourceContinued" => {
                    quote! { debug_string_source }
                }
                "OpName" | "OpMemberName" => quote! { debug_names },
                "OpModuleProcessed" => quote! { debug_module_processed },
                other => panic!("Debug section instruction {} not handled", other),
            };
            quote! {
                #[doc = #comment]
                pub fn #name(&mut self,#(#params),*) {
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, None, None, vec![#(#init),*]);
                    #(#extras)*
                    self.module.#section.push(inst);
                }
            }
        });
    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}

pub fn gen_dr_builder_annotation(grammar: &structs::Grammar) -> TokenStream {
    let kinds = &grammar.operand_kinds;
    // Generate build methods for all constants.
    let elements = grammar
        .instructions
        .iter()
        .filter(|inst| {
            inst.class == Some(structs::Class::Annotation) && inst.opname != "OpDecorationGroup"
        })
        .map(|inst| {
            let params = get_param_list(&inst.operands, false, kinds);
            let extras = get_push_extras(&inst.operands, kinds, quote! { inst.operands });
            let opcode = as_ident(&inst.opname[2..]);
            let comment = format!("Appends an Op{} instruction.", opcode);
            let name = get_function_name(&inst.opname);
            let init = get_init_list(&inst.operands);

            quote! {
                #[doc = #comment]
                pub fn #name(&mut self,#(#params),*) {
                    #[allow(unused_mut)]
                    let mut inst = dr::Instruction::new(
                        spirv::Op::#opcode, None, None, vec![#(#init),*]);
                    #(#extras)*
                    self.module.annotations.push(inst);
                }
            }
        });
    quote! {
        impl Builder {
            #(#elements)*
        }
    }
}
