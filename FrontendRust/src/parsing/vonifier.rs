use crate::lexing::ast::RangeL;
use crate::parsing::ast::*;
use crate::von::{IVonData, VonObject, VonMember, VonArray, VonStr, VonInt, VonBool, VonFloat};

/// ParserVonifier converts Parser AST to Von (JSON-like) format
/// Mirrors ParserVonifier.scala

pub struct ParserVonifier;

impl ParserVonifier {
    /// Helper to vonify optional values
    /// Mirrors vonifyOptional in ParserVonifier.scala lines 11-16
    pub fn vonify_optional<T, F>(opt: &Option<T>, func: F) -> IVonData
    where
        F: Fn(&T) -> IVonData,
    {
        match opt {
            None => IVonData::Object(VonObject {
                tyype: "None".to_string(),
                id: None,
                members: vec![],
            }),
            Some(value) => IVonData::Object(VonObject {
                tyype: "Some".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "value".to_string(),
                    value: func(value),
                }],
            }),
        }
    }

    /// Vonify a file
    /// Mirrors vonifyFile in ParserVonifier.scala lines 18-30
    pub fn vonify_file(file: &FileP) -> IVonData {
        let FileP {
            file_coord,
            comments_ranges,
            denizens,
        } = file;

        IVonData::Object(VonObject {
            tyype: "File".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "fileCoord".to_string(),
                    value: Self::vonify_file_coord(file_coord),
                },
                VonMember {
                    field_name: "commentsRanges".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: comments_ranges.iter().map(Self::vonify_range).collect(),
                    }),
                },
                VonMember {
                    field_name: "denizens".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: denizens.iter().map(Self::vonify_denizen).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify a denizen (top-level declaration)
    /// Mirrors vonifyDenizen in ParserVonifier.scala lines 32-41
    pub fn vonify_denizen(denizen_p: &IDenizenP) -> IVonData {
        match denizen_p {
            IDenizenP::TopLevelFunction(function) => Self::vonify_function(function),
            IDenizenP::TopLevelStruct(struct_p) => Self::vonify_struct(struct_p),
            IDenizenP::TopLevelInterface(interface) => Self::vonify_interface(interface),
            IDenizenP::TopLevelImpl(impl_p) => Self::vonify_impl(impl_p),
            IDenizenP::TopLevelExportAs(export) => Self::vonify_export_as(export),
            IDenizenP::TopLevelImport(import) => Self::vonify_import(import),
        }
    }

    /// Vonify a file coordinate
    fn vonify_file_coord(coord: &FileCoordinate) -> IVonData {
        IVonData::Object(VonObject {
            tyype: "FileCoordinate".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "packageCoord".to_string(),
                    value: Self::vonify_package_coord(&coord.package_coord),
                },
                VonMember {
                    field_name: "filepath".to_string(),
                    value: IVonData::Str(VonStr {
                        value: coord.filepath.clone(),
                    }),
                },
            ],
        })
    }

    /// Vonify a package coordinate
    fn vonify_package_coord(coord: &PackageCoordinate) -> IVonData {
        IVonData::Object(VonObject {
            tyype: "PackageCoordinate".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "module".to_string(),
                    value: IVonData::Str(VonStr {
                        value: coord.module.str.clone(),
                    }),
                },
                VonMember {
                    field_name: "packages".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: coord.packages.iter()
                            .map(|p| IVonData::Str(VonStr { value: p.str.clone() }))
                            .collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify a range
    fn vonify_range(range: &RangeL) -> IVonData {
        IVonData::Object(VonObject {
            tyype: "Range".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "begin".to_string(),
                    value: IVonData::Int(VonInt {
                        value: range.begin as i64,
                    }),
                },
                VonMember {
                    field_name: "end".to_string(),
                    value: IVonData::Int(VonInt {
                        value: range.end as i64,
                    }),
                },
            ],
        })
    }

    /// Vonify a name
    fn vonify_name(name: &NameP) -> IVonData {
        IVonData::Object(VonObject {
            tyype: "Name".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(&name.range),
                },
                VonMember {
                    field_name: "name".to_string(),
                    value: IVonData::Str(VonStr {
                        value: name.str.str.clone(),
                    }),
                },
            ],
        })
    }

    /// Vonify a struct
    /// Mirrors vonifyStruct in ParserVonifier.scala lines 68-83
    fn vonify_struct(thing: &StructP) -> IVonData {
        let StructP {
            range,
            name,
            attributes,
            mutability,
            identifying_runes,
            template_rules,
            maybe_default_region_rune,
            body_range: _body_range,
            members,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "Struct".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_name(name),
                },
                VonMember {
                    field_name: "attributes".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: attributes.iter().map(Self::vonify_attribute).collect(),
                    }),
                },
                VonMember {
                    field_name: "mutability".to_string(),
                    value: Self::vonify_optional(mutability, Self::vonify_templex),
                },
                VonMember {
                    field_name: "identifyingRunes".to_string(),
                    value: Self::vonify_optional(identifying_runes, Self::vonify_identifying_runes),
                },
                VonMember {
                    field_name: "templateRules".to_string(),
                    value: Self::vonify_optional(template_rules, Self::vonify_template_rules),
                },
                VonMember {
                    field_name: "maybeDefaultRegion".to_string(),
                    value: Self::vonify_optional(maybe_default_region_rune, Self::vonify_region_rune),
                },
                VonMember {
                    field_name: "bodyRange".to_string(),
                    // NOTE: Scala bug - uses range instead of body_range
                    // See ParserVonifier.scala line 81
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "members".to_string(),
                    value: Self::vonify_struct_members(members),
                },
            ],
        })
    }

    /// Vonify a function
    /// Mirrors vonifyFunction in ParserVonifier.scala lines 222-231
    fn vonify_function(thing: &FunctionP) -> IVonData {
        let FunctionP { range, header, body } = thing;

        IVonData::Object(VonObject {
            tyype: "Function".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "header".to_string(),
                    value: Self::vonify_function_header(header),
                },
                VonMember {
                    field_name: "body".to_string(),
                    value: Self::vonify_optional(body, |b| Self::vonify_block(b.as_ref())),
                },
            ],
        })
    }

    /// Vonify an interface
    /// Mirrors vonifyInterface in ParserVonifier.scala lines 135-150
    fn vonify_interface(thing: &InterfaceP) -> IVonData {
        let InterfaceP {
            range,
            name,
            attributes,
            mutability,
            maybe_identifying_runes,
            template_rules,
            maybe_default_region_rune,
            body_range: _body_range,
            members,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "Interface".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_name(name),
                },
                VonMember {
                    field_name: "attributes".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: attributes.iter().map(Self::vonify_attribute).collect(),
                    }),
                },
                VonMember {
                    field_name: "mutability".to_string(),
                    value: Self::vonify_optional(mutability, Self::vonify_templex),
                },
                VonMember {
                    field_name: "maybeIdentifyingRunes".to_string(),
                    value: Self::vonify_optional(maybe_identifying_runes, Self::vonify_identifying_runes),
                },
                VonMember {
                    field_name: "templateRules".to_string(),
                    value: Self::vonify_optional(template_rules, Self::vonify_template_rules),
                },
                VonMember {
                    field_name: "maybeDefaultRegion".to_string(),
                    value: Self::vonify_optional(maybe_default_region_rune, Self::vonify_region_rune),
                },
                VonMember {
                    field_name: "bodyRange".to_string(),
                    // NOTE: Scala bug - uses range instead of body_range
                    // See ParserVonifier.scala line 148
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "members".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: members.iter().map(Self::vonify_function).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify an impl
    /// Mirrors vonifyImpl in ParserVonifier.scala lines 152-165
    fn vonify_impl(thing: &ImplP) -> IVonData {
        let ImplP {
            range,
            generic_params,
            template_rules,
            struct_,
            interface,
            attributes,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "Impl".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "identifyingRunes".to_string(),
                    value: Self::vonify_optional(generic_params, Self::vonify_identifying_runes),
                },
                VonMember {
                    field_name: "attributes".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: attributes.iter().map(Self::vonify_attribute).collect(),
                    }),
                },
                VonMember {
                    field_name: "templateRules".to_string(),
                    value: Self::vonify_optional(template_rules, Self::vonify_template_rules),
                },
                VonMember {
                    field_name: "struct".to_string(),
                    value: Self::vonify_optional(struct_, Self::vonify_templex),
                },
                VonMember {
                    field_name: "interface".to_string(),
                    value: Self::vonify_templex(interface),
                },
            ],
        })
    }

    /// Vonify an export
    /// Mirrors vonifyExportAs in ParserVonifier.scala lines 167-177
    fn vonify_export_as(thing: &ExportAsP) -> IVonData {
        let ExportAsP {
            range,
            struct_,
            exported_name,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "ExportAs".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "struct".to_string(),
                    value: Self::vonify_templex(struct_),
                },
                VonMember {
                    field_name: "exportedName".to_string(),
                    value: Self::vonify_name(exported_name),
                },
            ],
        })
    }

    /// Vonify an import
    /// Mirrors vonifyImport in ParserVonifier.scala lines 179-190
    fn vonify_import(thing: &ImportP) -> IVonData {
        let ImportP {
            range,
            module_name,
            package_steps,
            importee_name,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "Import".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "moduleName".to_string(),
                    value: Self::vonify_name(module_name),
                },
                VonMember {
                    field_name: "packageSteps".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: package_steps.iter().map(Self::vonify_name).collect(),
                    }),
                },
                VonMember {
                    field_name: "importeeName".to_string(),
                    value: Self::vonify_name(importee_name),
                },
            ],
        })
    }

    /// Vonify a function header
    /// Mirrors vonifyFunctionHeader in ParserVonifier.scala lines 233-254
    fn vonify_function_header(thing: &FunctionHeaderP) -> IVonData {
        let FunctionHeaderP {
            range,
            name,
            attributes,
            generic_parameters,
            template_rules,
            params,
            ret,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "FunctionHeader".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_optional(name, Self::vonify_name),
                },
                VonMember {
                    field_name: "attributes".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: attributes.iter().map(Self::vonify_attribute).collect(),
                    }),
                },
                VonMember {
                    field_name: "maybeUserSpecifiedIdentifyingRunes".to_string(),
                    value: Self::vonify_optional(generic_parameters, Self::vonify_identifying_runes),
                },
                VonMember {
                    field_name: "templateRules".to_string(),
                    value: Self::vonify_optional(template_rules, Self::vonify_template_rules),
                },
                VonMember {
                    field_name: "params".to_string(),
                    value: Self::vonify_optional(params, Self::vonify_params),
                },
                VonMember {
                    field_name: "return".to_string(),
                    value: IVonData::Object(VonObject {
                        tyype: "FunctionReturn".to_string(),
                        id: None,
                        members: vec![
                            VonMember {
                                field_name: "range".to_string(),
                                value: Self::vonify_range(&ret.range),
                            },
                            VonMember {
                                field_name: "retType".to_string(),
                                value: Self::vonify_optional(&ret.ret_type, Self::vonify_templex),
                            },
                        ],
                    }),
                },
            ],
        })
    }

    /// Vonify params
    /// Mirrors vonifyParams in ParserVonifier.scala lines 256-264
    fn vonify_params(thing: &ParamsP) -> IVonData {
        let ParamsP { range, params } = thing;

        IVonData::Object(VonObject {
            tyype: "Params".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "params".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: params.iter().map(Self::vonify_parameter).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify a parameter
    /// Mirrors vonifyParameter in ParserVonifier.scala lines 266-277
    fn vonify_parameter(thing: &ParameterP) -> IVonData {
        let ParameterP {
            range,
            virtuality,
            maybe_pre_checked,
            self_borrow,
            pattern,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "Parameter".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "selfBorrow".to_string(),
                    value: Self::vonify_optional(self_borrow, Self::vonify_range),
                },
                VonMember {
                    field_name: "maybePreChecked".to_string(),
                    value: Self::vonify_optional(maybe_pre_checked, Self::vonify_range),
                },
                VonMember {
                    field_name: "virtuality".to_string(),
                    value: Self::vonify_optional(virtuality, Self::vonify_virtuality),
                },
                VonMember {
                    field_name: "pattern".to_string(),
                    value: Self::vonify_optional(pattern, Self::vonify_pattern),
                },
            ],
        })
    }

    /// Vonify a pattern
    /// Mirrors vonifyPattern in ParserVonifier.scala lines 279-289
    fn vonify_pattern(thing: &PatternPP) -> IVonData {
        let PatternPP {
            range,
            destination,
            templex,
            destructure,
        } = thing;

        IVonData::Object(VonObject {
            tyype: "Pattern".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "capture".to_string(),
                    value: Self::vonify_optional(destination, Self::vonify_destination_local),
                },
                VonMember {
                    field_name: "templex".to_string(),
                    value: Self::vonify_optional(templex, Self::vonify_templex),
                },
                VonMember {
                    field_name: "destructure".to_string(),
                    value: Self::vonify_optional(destructure, Self::vonify_destructure),
                },
            ],
        })
    }

    /// Vonify an attribute
    /// Mirrors vonifyAttribute in ParserVonifier.scala lines 381-409
    fn vonify_attribute(thing: &IAttributeP) -> IVonData {
        match thing {
            IAttributeP::WeakableAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "WeakableAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::SealedAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "SealedAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::LinearAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "LinearAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::ExportAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "ExportAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::MacroCall { range, inclusion, name } => {
                IVonData::Object(VonObject {
                    tyype: "MacroCall".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "dontCall".to_string(),
                            value: IVonData::Bool(VonBool {
                                value: matches!(inclusion, IMacroInclusionP::DontCallMacro),
                            }),
                        },
                        VonMember {
                            field_name: "name".to_string(),
                            value: Self::vonify_name(name),
                        },
                    ],
                })
            }
            IAttributeP::AbstractAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "AbstractAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::ExternAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "ExternAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::PureAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "PureAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::AdditiveAttribute(range) => {
                IVonData::Object(VonObject {
                    tyype: "AdditiveAttribute".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IAttributeP::BuiltinAttribute { range, generator_name } => {
                IVonData::Object(VonObject {
                    tyype: "BuiltinAttribute".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "generatorName".to_string(),
                            value: Self::vonify_name(generator_name),
                        },
                    ],
                })
            }
        }
    }

    /// Vonify struct members
    /// Mirrors vonifyStructMembers in ParserVonifier.scala lines 85-93
    fn vonify_struct_members(thing: &StructMembersP) -> IVonData {
        let StructMembersP { range, contents } = thing;
        IVonData::Object(VonObject {
            tyype: "StructMembers".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "members".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: contents.iter().map(Self::vonify_struct_contents).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify struct contents
    /// Mirrors vonifyStructContents in ParserVonifier.scala lines 95-101
    fn vonify_struct_contents(thing: &IStructContent) -> IVonData {
        match thing {
            IStructContent::StructMethod(function) => IVonData::Object(VonObject {
                tyype: "StructMethod".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "function".to_string(),
                    value: Self::vonify_function(function),
                }],
            }),
            IStructContent::NormalStructMember {
                range,
                name,
                variability,
                tyype,
            } => IVonData::Object(VonObject {
                tyype: "NormalStructMember".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "name".to_string(),
                        value: Self::vonify_name(name),
                    },
                    VonMember {
                        field_name: "variability".to_string(),
                        value: Self::vonify_variability(variability),
                    },
                    VonMember {
                        field_name: "type".to_string(),
                        value: Self::vonify_templex(tyype),
                    },
                ],
            }),
            IStructContent::VariadicStructMember {
                range,
                variability,
                tyype,
            } => IVonData::Object(VonObject {
                tyype: "VariadicStructMember".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "variability".to_string(),
                        value: Self::vonify_variability(variability),
                    },
                    VonMember {
                        field_name: "type".to_string(),
                        value: Self::vonify_templex(tyype),
                    },
                ],
            }),
        }
    }

    /// Vonify virtuality
    /// Mirrors vonifyVirtuality in ParserVonifier.scala lines 338-350
    fn vonify_virtuality(thing: &AbstractP) -> IVonData {
        let AbstractP { range } = thing;
        IVonData::Object(VonObject {
            tyype: "Abstract".to_string(),
            id: None,
            members: vec![VonMember {
                field_name: "range".to_string(),
                value: Self::vonify_range(range),
            }],
        })
    }

    /// Vonify destination local
    /// Mirrors vonifyDestinationLocal in ParserVonifier.scala lines 301-309
    fn vonify_destination_local(thing: &DestinationLocalP) -> IVonData {
        let DestinationLocalP { decl, mutate } = thing;
        IVonData::Object(VonObject {
            tyype: "DestinationLocal".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_name_declaration(decl),
                },
                VonMember {
                    field_name: "mutate".to_string(),
                    value: Self::vonify_optional(mutate, Self::vonify_range),
                },
            ],
        })
    }

    /// Vonify name declaration
    /// Mirrors vonifyNameDeclaration in ParserVonifier.scala lines 311-320
    fn vonify_name_declaration(thing: &INameDeclarationP) -> IVonData {
        match thing {
            INameDeclarationP::IgnoredLocalNameDeclaration(range) => {
                IVonData::Object(VonObject {
                    tyype: "IgnoredLocalNameDeclaration".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            INameDeclarationP::LocalNameDeclaration(name) => {
                IVonData::Object(VonObject {
                    tyype: "LocalNameDeclaration".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "name".to_string(),
                        value: Self::vonify_name(name),
                    }],
                })
            }
            INameDeclarationP::ConstructingMemberNameDeclaration(name) => {
                IVonData::Object(VonObject {
                    tyype: "ConstructingMemberNameDeclaration".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "name".to_string(),
                        value: Self::vonify_name(name),
                    }],
                })
            }
            INameDeclarationP::IterableNameDeclaration(range) => {
                IVonData::Object(VonObject {
                    tyype: "IterableNameDeclaration".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            INameDeclarationP::IteratorNameDeclaration(range) => {
                IVonData::Object(VonObject {
                    tyype: "IteratorNameDeclaration".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            INameDeclarationP::IterationOptionNameDeclaration(range) => {
                IVonData::Object(VonObject {
                    tyype: "IterationOptionNameDeclaration".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
        }
    }

    /// Vonify destructure
    /// Mirrors vonifyDestructure in ParserVonifier.scala lines 362-370
    fn vonify_destructure(thing: &DestructureP) -> IVonData {
        let DestructureP { range, patterns } = thing;
        IVonData::Object(VonObject {
            tyype: "Destructure".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "patterns".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: patterns.iter().map(Self::vonify_pattern).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify template rules
    /// Mirrors vonifyTemplateRules in ParserVonifier.scala lines 411-419
    fn vonify_template_rules(thing: &TemplateRulesP) -> IVonData {
        let TemplateRulesP { range, rules } = thing;
        IVonData::Object(VonObject {
            tyype: "TemplateRules".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "rules".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: rules.iter().map(Self::vonify_rule).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify a rule
    /// Mirrors vonifyRule in ParserVonifier.scala lines 421-513
    fn vonify_rule(thing: &IRulexPR) -> IVonData {
        match thing {
            IRulexPR::Equals { range, left, right } => IVonData::Object(VonObject {
                tyype: "EqualsPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "left".to_string(),
                        value: Self::vonify_rule(left),
                    },
                    VonMember {
                        field_name: "right".to_string(),
                        value: Self::vonify_rule(right),
                    },
                ],
            }),
            IRulexPR::Or { range, possibilities } => IVonData::Object(VonObject {
                tyype: "OrPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "possibilities".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: possibilities.iter().map(Self::vonify_rule).collect(),
                        }),
                    },
                ],
            }),
            IRulexPR::Dot { range, container, member_name } => IVonData::Object(VonObject {
                tyype: "DotPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "container".to_string(),
                        value: Self::vonify_rule(container),
                    },
                    VonMember {
                        field_name: "memberName".to_string(),
                        value: Self::vonify_name(member_name),
                    },
                ],
            }),
            IRulexPR::Components { range, container, components } => IVonData::Object(VonObject {
                tyype: "ComponentsPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "container".to_string(),
                        value: Self::vonify_rune_type(container),
                    },
                    VonMember {
                        field_name: "components".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: components.iter().map(Self::vonify_rule).collect(),
                        }),
                    },
                ],
            }),
            IRulexPR::Typed { range, rune, tyype } => IVonData::Object(VonObject {
                tyype: "TypedPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "rune".to_string(),
                        value: Self::vonify_optional(rune, Self::vonify_name),
                    },
                    VonMember {
                        field_name: "type".to_string(),
                        value: Self::vonify_rune_type(tyype),
                    },
                ],
            }),
            IRulexPR::Templex(templex) => IVonData::Object(VonObject {
                tyype: "TemplexPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "templex".to_string(),
                        value: Self::vonify_templex(templex),
                    },
                ],
            }),
            IRulexPR::BuiltinCall { range, name, args } => IVonData::Object(VonObject {
                tyype: "BuiltinCallPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "name".to_string(),
                        value: Self::vonify_name(name),
                    },
                    VonMember {
                        field_name: "args".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: args.iter().map(Self::vonify_rule).collect(),
                        }),
                    },
                ],
            }),
            IRulexPR::Pack { range, elements } => IVonData::Object(VonObject {
                tyype: "PackPR".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "elements".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: elements.iter().map(Self::vonify_rule).collect(),
                        }),
                    },
                ],
            }),
        }
    }

    /// Vonify rune type
    /// Mirrors vonifyRuneType in ParserVonifier.scala lines 515-530
    fn vonify_rune_type(thing: &ITypePR) -> IVonData {
        let tyype = match thing {
            ITypePR::IntType => "IntTypePR",
            ITypePR::BoolType => "BoolTypePR",
            ITypePR::OwnershipType => "OwnershipTypePR",
            ITypePR::MutabilityType => "MutabilityTypePR",
            ITypePR::VariabilityType => "VariabilityTypePR",
            ITypePR::LocationType => "LocationTypePR",
            ITypePR::CoordType => "CoordTypePR",
            ITypePR::CoordListType => "CoordListTypePR",
            ITypePR::PrototypeType => "PrototypeTypePR",
            ITypePR::KindType => "KindTypePR",
            ITypePR::RegionType => "RegionTypePR",
            ITypePR::CitizenTemplateType => "CitizenTemplateTypePR",
        };
        IVonData::Object(VonObject {
            tyype: tyype.to_string(),
            id: None,
            members: vec![],
        })
    }

    /// Vonify identifying runes (generic parameters)
    /// Mirrors vonifyIdentifyingRunes in ParserVonifier.scala lines 532-540
    pub fn vonify_identifying_runes(thing: &GenericParametersP) -> IVonData {
        let GenericParametersP { range, params: identifying_runes_p } = thing;
        IVonData::Object(VonObject {
            tyype: "IdentifyingRunes".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "identifyingRunes".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: identifying_runes_p.iter().map(Self::vonify_generic_parameter).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify generic parameter
    /// Mirrors vonifyGenericParameter in ParserVonifier.scala lines 542-554
    fn vonify_generic_parameter(thing: &GenericParameterP) -> IVonData {
        let GenericParameterP {
            range,
            name,
            maybe_type,
            coord_region,
            attributes,
            maybe_default,
        } = thing;
        IVonData::Object(VonObject {
            tyype: "IdentifyingRune".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_name(name),
                },
                VonMember {
                    field_name: "maybeType".to_string(),
                    value: Self::vonify_optional(maybe_type, Self::vonify_generic_parameter_type),
                },
                VonMember {
                    field_name: "maybeCoordRegion".to_string(),
                    value: Self::vonify_optional(coord_region, Self::vonify_region_rune),
                },
                VonMember {
                    field_name: "attributes".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: attributes.iter().map(Self::vonify_rune_attribute).collect(),
                    }),
                },
                VonMember {
                    field_name: "maybeDefault".to_string(),
                    value: Self::vonify_optional(maybe_default, Self::vonify_templex),
                },
            ],
        })
    }

    /// Vonify generic parameter type
    fn vonify_generic_parameter_type(thing: &GenericParameterTypeP) -> IVonData {
        let GenericParameterTypeP { range, tyype } = thing;
        IVonData::Object(VonObject {
            tyype: "GenericParameterType".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "type".to_string(),
                    value: Self::vonify_rune_type(tyype),
                },
            ],
        })
    }

    /// Vonify rune attribute
    /// Need to check Scala for IRuneAttributeP vonification
    fn vonify_rune_attribute(thing: &IRuneAttributeP) -> IVonData {
        match thing {
            IRuneAttributeP::ImmutableRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "ImmutableRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::MutableRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "MutableRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::ReadOnlyRegionRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "ReadOnlyRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::ReadWriteRegionRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "ReadWriteRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::ImmutableRegionRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "ImmutableRegionRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::AdditiveRegionRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "AdditiveRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::PoolRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "PoolRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::ArenaRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "ArenaRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IRuneAttributeP::BumpRuneAttribute(range) => IVonData::Object(VonObject {
                tyype: "BumpRuneAttribute".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
        }
    }

    /// Vonify region rune
    /// Mirrors vonifyRegionRune in ParserVonifier.scala lines 745-753
    fn vonify_region_rune(region_rune: &RegionRunePT) -> IVonData {
        let RegionRunePT { range, name } = region_rune;
        IVonData::Object(VonObject {
            tyype: "RegionRuneT".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_optional(name, Self::vonify_name),
                },
            ],
        })
    }

    /// Vonify pack
    fn vonify_pack(thing: &PackPT) -> IVonData {
        let PackPT { range, members } = thing;
        IVonData::Object(VonObject {
            tyype: "PackT".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "members".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: members.iter().map(Self::vonify_templex).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify templex (type expression with 24 variants!)
    /// Mirrors vonifyTemplex in ParserVonifier.scala lines 566-743
    fn vonify_templex(thing: &ITemplexPT) -> IVonData {
        match thing {
            ITemplexPT::RegionRune(r) => Self::vonify_region_rune(r),
            ITemplexPT::AnonymousRune(range) => IVonData::Object(VonObject {
                tyype: "AnonymousRuneT".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            ITemplexPT::Point { range, inner } => IVonData::Object(VonObject {
                tyype: "PointT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: Self::vonify_templex(inner),
                    },
                ],
            }),
            ITemplexPT::Bool { range, value } => IVonData::Object(VonObject {
                tyype: "BoolT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "value".to_string(),
                        value: IVonData::Bool(VonBool { value: *value }),
                    },
                ],
            }),
            ITemplexPT::Call { range, template, args } => IVonData::Object(VonObject {
                tyype: "CallT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "template".to_string(),
                        value: Self::vonify_templex(template),
                    },
                    VonMember {
                        field_name: "args".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: args.iter().map(Self::vonify_templex).collect(),
                        }),
                    },
                ],
            }),
            ITemplexPT::Inline { range, inner } => IVonData::Object(VonObject {
                tyype: "InlineT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: Self::vonify_templex(inner),
                    },
                ],
            }),
            ITemplexPT::Int { range, value } => IVonData::Object(VonObject {
                tyype: "IntT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: IVonData::Str(VonStr {
                            value: value.to_string(),
                        }),
                    },
                ],
            }),
            ITemplexPT::Location { range, location } => IVonData::Object(VonObject {
                tyype: "LocationT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "location".to_string(),
                        value: Self::vonify_location(location),
                    },
                ],
            }),
            ITemplexPT::Tuple { range, elements } => IVonData::Object(VonObject {
                tyype: "ManualSequenceT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "members".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: elements.iter().map(Self::vonify_templex).collect(),
                        }),
                    },
                ],
            }),
            ITemplexPT::Mutability { range, mutability } => IVonData::Object(VonObject {
                tyype: "MutabilityT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "mutability".to_string(),
                        value: Self::vonify_mutability(mutability),
                    },
                ],
            }),
            ITemplexPT::NameOrRune(rune) => IVonData::Object(VonObject {
                tyype: "NameOrRuneT".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "rune".to_string(),
                    value: Self::vonify_name(rune),
                }],
            }),
            ITemplexPT::Interpreted {
                range,
                maybe_ownership,
                maybe_region,
                inner,
            } => IVonData::Object(VonObject {
                tyype: "InterpretedT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "maybeOwnership".to_string(),
                        value: match maybe_ownership {
                            Some(o) => Self::vonify_optional(&Some(o.as_ref()), |t| Self::vonify_ownership_pt(t)),
                            None => Self::vonify_optional::<&OwnershipPT, _>(&None, |t| Self::vonify_ownership_pt(t)),
                        },
                    },
                    VonMember {
                        field_name: "maybeRegion".to_string(),
                        value: match maybe_region {
                            Some(r) => Self::vonify_optional(&Some(r.as_ref()), |t| Self::vonify_region_rune(t)),
                            None => Self::vonify_optional::<&RegionRunePT, _>(&None, |t| Self::vonify_region_rune(t)),
                        },
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: Self::vonify_templex(inner),
                    },
                ],
            }),
            ITemplexPT::Ownership { range, ownership } => IVonData::Object(VonObject {
                tyype: "OwnershipT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "ownership".to_string(),
                        value: Self::vonify_ownership(ownership),
                    },
                ],
            }),
            ITemplexPT::StaticSizedArray {
                range,
                mutability,
                variability,
                size,
                element,
            } => IVonData::Object(VonObject {
                tyype: "StaticSizedArrayT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "mutability".to_string(),
                        value: Self::vonify_templex(mutability),
                    },
                    VonMember {
                        field_name: "variability".to_string(),
                        value: Self::vonify_templex(variability),
                    },
                    VonMember {
                        field_name: "size".to_string(),
                        value: Self::vonify_templex(size),
                    },
                    VonMember {
                        field_name: "element".to_string(),
                        value: Self::vonify_templex(element),
                    },
                ],
            }),
            ITemplexPT::RuntimeSizedArray { range, mutability, element } => {
                IVonData::Object(VonObject {
                    tyype: "RuntimeSizedArrayT".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "mutability".to_string(),
                            value: Self::vonify_templex(mutability),
                        },
                        VonMember {
                            field_name: "element".to_string(),
                            value: Self::vonify_templex(element),
                        },
                    ],
                })
            }
            ITemplexPT::Function {
                range,
                mutability,
                parameters,
                return_type,
            } => IVonData::Object(VonObject {
                tyype: "FunctionT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "mutability".to_string(),
                        value: match mutability {
                            Some(m) => Self::vonify_optional(&Some(m.as_ref()), |t| Self::vonify_templex(t)),
                            None => Self::vonify_optional::<&ITemplexPT, _>(&None, |t| Self::vonify_templex(t)),
                        },
                    },
                    VonMember {
                        field_name: "params".to_string(),
                        value: Self::vonify_pack(parameters),
                    },
                    VonMember {
                        field_name: "returnType".to_string(),
                        value: Self::vonify_templex(return_type),
                    },
                ],
            }),
            ITemplexPT::Pack(pack_pt) => Self::vonify_pack(pack_pt),
            ITemplexPT::Func {
                range,
                name,
                params_range,
                parameters,
                return_type,
            } => IVonData::Object(VonObject {
                tyype: "PrototypeT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "name".to_string(),
                        value: Self::vonify_name(name),
                    },
                    VonMember {
                        field_name: "paramsRange".to_string(),
                        value: Self::vonify_range(params_range),
                    },
                    VonMember {
                        field_name: "params".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: parameters.iter().map(Self::vonify_templex).collect(),
                        }),
                    },
                    VonMember {
                        field_name: "returnType".to_string(),
                        value: Self::vonify_templex(return_type),
                    },
                ],
            }),
            ITemplexPT::Share { range, inner } => IVonData::Object(VonObject {
                tyype: "ShareT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: Self::vonify_templex(inner),
                    },
                ],
            }),
            ITemplexPT::String { range, str } => IVonData::Object(VonObject {
                tyype: "StringT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "str".to_string(),
                        value: IVonData::Str(VonStr {
                            value: str.clone(),
                        }),
                    },
                ],
            }),
            ITemplexPT::TypedRune { range, rune, tyype } => IVonData::Object(VonObject {
                tyype: "TypedRuneT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "rune".to_string(),
                        value: Self::vonify_name(rune),
                    },
                    VonMember {
                        field_name: "type".to_string(),
                        value: Self::vonify_rune_type(tyype),
                    },
                ],
            }),
            ITemplexPT::Variability { range, variability } => IVonData::Object(VonObject {
                tyype: "VariabilityT".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "variability".to_string(),
                        value: Self::vonify_variability(variability),
                    },
                ],
            }),
        }
    }

    /// Vonify mutability
    /// Mirrors vonifyMutability in ParserVonifier.scala lines 755-760
    fn vonify_mutability(thing: &MutabilityP) -> IVonData {
        let tyype = match thing {
            MutabilityP::Mutable => "Mutable",
            MutabilityP::Immutable => "Immutable",
        };
        IVonData::Object(VonObject {
            tyype: tyype.to_string(),
            id: None,
            members: vec![],
        })
    }

    /// Vonify location
    /// Mirrors vonifyLocation in ParserVonifier.scala lines 762-767
    fn vonify_location(thing: &LocationP) -> IVonData {
        let tyype = match thing {
            LocationP::Inline => "Inline",
            LocationP::Yonder => "Yonder",
        };
        IVonData::Object(VonObject {
            tyype: tyype.to_string(),
            id: None,
            members: vec![],
        })
    }

    /// Vonify ownership PT (ownership templex)
    fn vonify_ownership_pt(thing: &OwnershipPT) -> IVonData {
        IVonData::Object(VonObject {
            tyype: "OwnershipT".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(&thing.range),
                },
                VonMember {
                    field_name: "ownership".to_string(),
                    value: Self::vonify_ownership(&thing.ownership),
                },
            ],
        })
    }

    /// Vonify ownership
    /// Mirrors vonifyOwnership in ParserVonifier.scala lines 769-776
    fn vonify_ownership(thing: &OwnershipP) -> IVonData {
        let tyype = match thing {
            OwnershipP::Share => "Share",
            OwnershipP::Own => "Own",
            OwnershipP::Borrow => "Borrow",
            OwnershipP::Live => "Live",
            OwnershipP::Weak => "Weak",
        };
        IVonData::Object(VonObject {
            tyype: tyype.to_string(),
            id: None,
            members: vec![],
        })
    }

    /// Vonify load-as
    /// Mirrors vonifyLoadAs in ParserVonifier.scala lines 778-785
    fn _vonify_load_as(thing: &LoadAsP) -> IVonData {
        let tyype = match thing {
            LoadAsP::Use => "Use",
            LoadAsP::Move => "Move",
            LoadAsP::LoadAsBorrow => "LoadAsBorrow",
            LoadAsP::LoadAsWeak => "LoadAsWeak",
        };
        IVonData::Object(VonObject {
            tyype: tyype.to_string(),
            id: None,
            members: vec![],
        })
    }

    /// Vonify variability
    /// Mirrors vonifyVariability in ParserVonifier.scala lines 331-336
    fn vonify_variability(thing: &VariabilityP) -> IVonData {
        let tyype = match thing {
            VariabilityP::Final => "Final",
            VariabilityP::Varying => "Varying",
        };
        IVonData::Object(VonObject {
            tyype: tyype.to_string(),
            id: None,
            members: vec![],
        })
    }

    /// Vonify unit
    /// Mirrors vonifyUnit in ParserVonifier.scala lines 372-379
    fn vonify_unit(thing: &UnitP) -> IVonData {
        let UnitP { range } = thing;
        IVonData::Object(VonObject {
            tyype: "Unit".to_string(),
            id: None,
            members: vec![VonMember {
                field_name: "range".to_string(),
                value: Self::vonify_range(range),
            }],
        })
    }

    /// Vonify lookup (needed for MethodCallPE)
    fn vonify_lookup(thing: &LookupPE) -> IVonData {
        let LookupPE { name, template_args } = thing;
        IVonData::Object(VonObject {
            tyype: "Lookup".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_imprecise_name(name),
                },
                VonMember {
                    field_name: "templateArgs".to_string(),
                    value: Self::vonify_optional(template_args, Self::vonify_template_args),
                },
            ],
        })
    }

    /// Vonify imprecise name
    /// Mirrors vonifyImpreciseName in ParserVonifier.scala lines 322-329
    fn vonify_imprecise_name(thing: &IImpreciseNameP) -> IVonData {
        match thing {
            IImpreciseNameP::LookupName(name) => IVonData::Object(VonObject {
                tyype: "LookupName".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "name".to_string(),
                    value: Self::vonify_name(name),
                }],
            }),
            IImpreciseNameP::IterableName(range) => IVonData::Object(VonObject {
                tyype: "IterableName".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IImpreciseNameP::IteratorName(range) => IVonData::Object(VonObject {
                tyype: "IteratorName".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IImpreciseNameP::IterationOptionName(range) => IVonData::Object(VonObject {
                tyype: "IterationOptionName".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
        }
    }

    /// Vonify block
    /// Mirrors vonifyBlock in ParserVonifier.scala lines 787-797
    fn vonify_block(thing: &BlockPE) -> IVonData {
        let BlockPE {
            range,
            maybe_pure,
            maybe_default_region,
            inner,
        } = thing;
        IVonData::Object(VonObject {
            tyype: "Block".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "maybePure".to_string(),
                    value: Self::vonify_optional(maybe_pure, Self::vonify_range),
                },
                VonMember {
                    field_name: "maybeDefaultRegion".to_string(),
                    value: Self::vonify_optional(maybe_default_region, Self::vonify_region_rune),
                },
                VonMember {
                    field_name: "inner".to_string(),
                    value: Self::vonify_expression(inner),
                },
            ],
        })
    }

    /// Vonify consecutor
    /// Mirrors vonifyConsecutor in ParserVonifier.scala lines 799-806
    fn vonify_consecutor(thing: &ConsecutorPE) -> IVonData {
        let ConsecutorPE { inners } = thing;
        IVonData::Object(VonObject {
            tyype: "Consecutor".to_string(),
            id: None,
            members: vec![VonMember {
                field_name: "inners".to_string(),
                value: IVonData::Array(VonArray {
                    id: None,
                    members: inners.iter().map(Self::vonify_expression).collect(),
                }),
            }],
        })
    }

    /// Vonify template args
    /// Mirrors vonifyTemplateArgs in ParserVonifier.scala lines 1174-1182
    fn vonify_template_args(thing: &TemplateArgsP) -> IVonData {
        let TemplateArgsP { range, args } = thing;
        IVonData::Object(VonObject {
            tyype: "TemplateArgs".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "args".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: args.iter().map(Self::vonify_templex).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify array size
    /// Mirrors vonifyArraySize in ParserVonifier.scala lines 1144-1156
    fn vonify_array_size(obj: &IArraySizeP) -> IVonData {
        match obj {
            IArraySizeP::RuntimeSized => IVonData::Object(VonObject {
                tyype: "RuntimeSized".to_string(),
                id: None,
                members: vec![],
            }),
            IArraySizeP::StaticSized { size_pt } => IVonData::Object(VonObject {
                tyype: "StaticSized".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "size".to_string(),
                    value: Self::vonify_optional(size_pt, Self::vonify_templex),
                }],
            }),
        }
    }

    /// Vonify construct array
    /// Mirrors vonifyConstructArray in ParserVonifier.scala lines 1158-1172
    fn vonify_construct_array(ca: &ConstructArrayPE) -> IVonData {
        let ConstructArrayPE {
            range,
            type_pt,
            mutability_pt,
            variability_pt,
            size,
            initializing_individual_elements,
            args,
        } = ca;

        IVonData::Object(VonObject {
            tyype: "ConstructArray".to_string(),
            id: None,
            members: vec![
                VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                },
                VonMember {
                    field_name: "type".to_string(),
                    value: Self::vonify_optional(type_pt, Self::vonify_templex),
                },
                VonMember {
                    field_name: "mutability".to_string(),
                    value: Self::vonify_optional(mutability_pt, Self::vonify_templex),
                },
                VonMember {
                    field_name: "variability".to_string(),
                    value: Self::vonify_optional(variability_pt, Self::vonify_templex),
                },
                VonMember {
                    field_name: "size".to_string(),
                    value: Self::vonify_array_size(size),
                },
                VonMember {
                    field_name: "initializingIndividualElements".to_string(),
                    value: IVonData::Bool(VonBool {
                        value: *initializing_individual_elements,
                    }),
                },
                VonMember {
                    field_name: "args".to_string(),
                    value: IVonData::Array(VonArray {
                        id: None,
                        members: args.iter().map(Self::vonify_expression).collect(),
                    }),
                },
            ],
        })
    }

    /// Vonify expression (38 variants!)
    /// Mirrors vonifyExpression in ParserVonifier.scala lines 808-1142
    fn vonify_expression(thing: &IExpressionPE) -> IVonData {
        match thing {
            IExpressionPE::ConstantBool(ConstantBoolPE { range, value }) => {
                IVonData::Object(VonObject {
                    tyype: "ConstantBool".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "value".to_string(),
                            value: IVonData::Bool(VonBool { value: *value }),
                        },
                    ],
                })
            }
            IExpressionPE::Dot(DotPE {
                range,
                left,
                operator_range,
                member,
            }) => IVonData::Object(VonObject {
                tyype: "Dot".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "left".to_string(),
                        value: Self::vonify_expression(left),
                    },
                    VonMember {
                        field_name: "operatorRange".to_string(),
                        value: Self::vonify_range(operator_range),
                    },
                    VonMember {
                        field_name: "member".to_string(),
                        value: Self::vonify_name(member),
                    },
                ],
            }),
            IExpressionPE::ConstantFloat(ConstantFloatPE { range, value }) => {
                IVonData::Object(VonObject {
                    tyype: "ConstantFloat".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "value".to_string(),
                            value: IVonData::Float(VonFloat { value: *value }),
                        },
                    ],
                })
            }
            IExpressionPE::Not(NotPE { range, inner }) => IVonData::Object(VonObject {
                tyype: "Not".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "innerExpr".to_string(),
                        value: Self::vonify_expression(inner),
                    },
                ],
            }),
            IExpressionPE::Range(RangePE {
                range,
                from_expr,
                to_expr,
            }) => IVonData::Object(VonObject {
                tyype: "Range".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "begin".to_string(),
                        value: Self::vonify_expression(from_expr),
                    },
                    VonMember {
                        field_name: "end".to_string(),
                        value: Self::vonify_expression(to_expr),
                    },
                ],
            }),
            IExpressionPE::FunctionCall(FunctionCallPE {
                range,
                operator_range,
                callable_expr,
                arg_exprs,
            }) => IVonData::Object(VonObject {
                tyype: "FunctionCall".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "operatorRange".to_string(),
                        value: Self::vonify_range(operator_range),
                    },
                    VonMember {
                        field_name: "callableExpr".to_string(),
                        value: Self::vonify_expression(callable_expr),
                    },
                    VonMember {
                        field_name: "argExprs".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: arg_exprs.iter().map(Self::vonify_expression).collect(),
                        }),
                    },
                ],
            }),
            IExpressionPE::BraceCall(BraceCallPE {
                range,
                operator_range,
                subject_expr,
                arg_exprs,
                callable_readwrite,
            }) => IVonData::Object(VonObject {
                tyype: "BraceCall".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "operatorRange".to_string(),
                        value: Self::vonify_range(operator_range),
                    },
                    VonMember {
                        field_name: "callableExpr".to_string(),
                        value: Self::vonify_expression(subject_expr),
                    },
                    VonMember {
                        field_name: "argExprs".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: arg_exprs.iter().map(Self::vonify_expression).collect(),
                        }),
                    },
                    VonMember {
                        field_name: "callableReadwrite".to_string(),
                        value: IVonData::Bool(VonBool {
                            value: *callable_readwrite,
                        }),
                    },
                ],
            }),
            IExpressionPE::BinaryCall(BinaryCallPE {
                range,
                function_name,
                left_expr,
                right_expr,
            }) => IVonData::Object(VonObject {
                tyype: "BinaryCall".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "functionName".to_string(),
                        value: Self::vonify_name(function_name),
                    },
                    VonMember {
                        field_name: "leftExpr".to_string(),
                        value: Self::vonify_expression(left_expr),
                    },
                    VonMember {
                        field_name: "rightExpr".to_string(),
                        value: Self::vonify_expression(right_expr),
                    },
                ],
            }),
            IExpressionPE::SubExpression(SubExpressionPE { range, inner }) => {
                IVonData::Object(VonObject {
                    tyype: "SubExpression".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "innerExpr".to_string(),
                            value: Self::vonify_expression(inner),
                        },
                    ],
                })
            }
            IExpressionPE::Each(EachPE {
                range,
                maybe_pure,
                entry_pattern,
                in_keyword_range,
                iterable_expr,
                body,
            }) => IVonData::Object(VonObject {
                tyype: "Each".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "maybePure".to_string(),
                        value: Self::vonify_optional(maybe_pure, Self::vonify_range),
                    },
                    VonMember {
                        field_name: "entryPattern".to_string(),
                        value: Self::vonify_pattern(entry_pattern),
                    },
                    VonMember {
                        field_name: "inRange".to_string(),
                        value: Self::vonify_range(in_keyword_range),
                    },
                    VonMember {
                        field_name: "iterableExpr".to_string(),
                        value: Self::vonify_expression(iterable_expr),
                    },
                    VonMember {
                        field_name: "body".to_string(),
                        value: Self::vonify_block(body),
                    },
                ],
            }),
            IExpressionPE::Pack(PackPE { range, inners }) => IVonData::Object(VonObject {
                tyype: "Pack".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "innerExprs".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: inners.iter().map(Self::vonify_expression).collect(),
                        }),
                    },
                ],
            }),
            IExpressionPE::MethodCall(MethodCallPE {
                range,
                subject_expr,
                operator_range,
                method_lookup,
                arg_exprs,
            }) => IVonData::Object(VonObject {
                tyype: "MethodCall".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "operatorRange".to_string(),
                        value: Self::vonify_range(operator_range),
                    },
                    VonMember {
                        field_name: "subjectExpr".to_string(),
                        value: Self::vonify_expression(subject_expr),
                    },
                    VonMember {
                        field_name: "method".to_string(),
                        value: Self::vonify_lookup(method_lookup),
                    },
                    VonMember {
                        field_name: "argExprs".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: arg_exprs.iter().map(Self::vonify_expression).collect(),
                        }),
                    },
                ],
            }),
            IExpressionPE::Shortcall(ShortcallPE { range, arg_exprs }) => {
                IVonData::Object(VonObject {
                    tyype: "Shortcall".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "argExprs".to_string(),
                            value: IVonData::Array(VonArray {
                                id: None,
                                members: arg_exprs.iter().map(Self::vonify_expression).collect(),
                            }),
                        },
                    ],
                })
            }
            IExpressionPE::If(IfPE {
                range,
                condition,
                then_body,
                else_body,
            }) => IVonData::Object(VonObject {
                tyype: "If".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "condition".to_string(),
                        value: Self::vonify_expression(condition),
                    },
                    VonMember {
                        field_name: "thenBody".to_string(),
                        value: Self::vonify_block(then_body),
                    },
                    VonMember {
                        field_name: "elseBody".to_string(),
                        value: Self::vonify_block(else_body),
                    },
                ],
            }),
            IExpressionPE::Index(IndexPE { range, left, args }) => IVonData::Object(VonObject {
                tyype: "Index".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "left".to_string(),
                        value: Self::vonify_expression(left),
                    },
                    VonMember {
                        field_name: "args".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: args.iter().map(Self::vonify_expression).collect(),
                        }),
                    },
                ],
            }),
            IExpressionPE::ConstantInt(ConstantIntPE { range, value, bits }) => {
                IVonData::Object(VonObject {
                    tyype: "ConstantInt".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "value".to_string(),
                            value: IVonData::Str(VonStr {
                                value: value.to_string(),
                            }),
                        },
                        VonMember {
                            field_name: "bits".to_string(),
                            value: Self::vonify_optional(bits, |b| {
                                IVonData::Int(VonInt { value: *b as i64 })
                            }),
                        },
                    ],
                })
            }
            IExpressionPE::Augment(AugmentPE {
                range,
                target_ownership,
                inner,
            }) => IVonData::Object(VonObject {
                tyype: "Augment".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "targetOwnership".to_string(),
                        value: Self::vonify_ownership(target_ownership),
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: Self::vonify_expression(inner),
                    },
                ],
            }),
            IExpressionPE::Transmigrate(TransmigratePE {
                range,
                target_region,
                inner,
            }) => IVonData::Object(VonObject {
                tyype: "Transmigrate".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "targetRegion".to_string(),
                        value: Self::vonify_name(target_region),
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: Self::vonify_expression(inner),
                    },
                ],
            }),
            IExpressionPE::Let(LetPE {
                range,
                pattern,
                source,
            }) => IVonData::Object(VonObject {
                tyype: "Let".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "pattern".to_string(),
                        value: Self::vonify_pattern(pattern),
                    },
                    VonMember {
                        field_name: "source".to_string(),
                        value: Self::vonify_expression(source),
                    },
                ],
            }),
            IExpressionPE::Lookup(LookupPE {
                name,
                template_args,
            }) => IVonData::Object(VonObject {
                tyype: "Lookup".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "name".to_string(),
                        value: Self::vonify_imprecise_name(name),
                    },
                    VonMember {
                        field_name: "templateArgs".to_string(),
                        value: Self::vonify_optional(template_args, Self::vonify_template_args),
                    },
                ],
            }),
            IExpressionPE::Mutate(MutatePE {
                range,
                mutatee,
                source,
            }) => IVonData::Object(VonObject {
                tyype: "Mutate".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "source".to_string(),
                        value: Self::vonify_expression(source),
                    },
                    VonMember {
                        field_name: "mutatee".to_string(),
                        value: Self::vonify_expression(mutatee),
                    },
                ],
            }),
            IExpressionPE::Return(ReturnPE { range, expr }) => IVonData::Object(VonObject {
                tyype: "Return".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "expr".to_string(),
                        value: Self::vonify_expression(expr),
                    },
                ],
            }),
            IExpressionPE::Break(BreakPE { range }) => IVonData::Object(VonObject {
                tyype: "Break".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IExpressionPE::ConstantStr(ConstantStrPE { range, value }) => {
                IVonData::Object(VonObject {
                    tyype: "ConstantStr".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "value".to_string(),
                            value: IVonData::Str(VonStr {
                                value: value.clone(),
                            }),
                        },
                    ],
                })
            }
            IExpressionPE::StrInterpolate(StrInterpolatePE { range, parts }) => {
                IVonData::Object(VonObject {
                    tyype: "StrInterpolate".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "range".to_string(),
                            value: Self::vonify_range(range),
                        },
                        VonMember {
                            field_name: "parts".to_string(),
                            value: IVonData::Array(VonArray {
                                id: None,
                                members: parts.iter().map(Self::vonify_expression).collect(),
                            }),
                        },
                    ],
                })
            }
            IExpressionPE::And(AndPE { range, left, right }) => IVonData::Object(VonObject {
                tyype: "And".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "left".to_string(),
                        value: Self::vonify_expression(left),
                    },
                    VonMember {
                        field_name: "right".to_string(),
                        value: Self::vonify_block(right),
                    },
                ],
            }),
            IExpressionPE::Or(OrPE { range, left, right }) => IVonData::Object(VonObject {
                tyype: "Or".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "left".to_string(),
                        value: Self::vonify_expression(left),
                    },
                    VonMember {
                        field_name: "right".to_string(),
                        value: Self::vonify_block(right),
                    },
                ],
            }),
            IExpressionPE::Block(b) => Self::vonify_block(b),
            IExpressionPE::Consecutor(c) => Self::vonify_consecutor(c),
            IExpressionPE::Destruct(DestructPE { range, inner }) => IVonData::Object(VonObject {
                tyype: "Destruct".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "inner".to_string(),
                        value: Self::vonify_expression(inner),
                    },
                ],
            }),
            IExpressionPE::Unlet(UnletPE { range, name }) => IVonData::Object(VonObject {
                tyype: "Unlet".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "localName".to_string(),
                        value: Self::vonify_imprecise_name(name),
                    },
                ],
            }),
            IExpressionPE::Lambda(LambdaPE { captures, function }) => {
                IVonData::Object(VonObject {
                    tyype: "Lambda".to_string(),
                    id: None,
                    members: vec![
                        VonMember {
                            field_name: "captures".to_string(),
                            value: Self::vonify_optional(captures, Self::vonify_unit),
                        },
                        VonMember {
                            field_name: "function".to_string(),
                            value: Self::vonify_function(function),
                        },
                    ],
                })
            }
            IExpressionPE::MagicParamLookup(MagicParamLookupPE { range }) => {
                IVonData::Object(VonObject {
                    tyype: "MagicParamLookup".to_string(),
                    id: None,
                    members: vec![VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    }],
                })
            }
            IExpressionPE::Tuple(TuplePE { range, elements }) => IVonData::Object(VonObject {
                tyype: "Tuple".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "elements".to_string(),
                        value: IVonData::Array(VonArray {
                            id: None,
                            members: elements.iter().map(Self::vonify_expression).collect(),
                        }),
                    },
                ],
            }),
            IExpressionPE::ConstructArray(ca) => Self::vonify_construct_array(ca),
            IExpressionPE::Void(VoidPE { range }) => IVonData::Object(VonObject {
                tyype: "Void".to_string(),
                id: None,
                members: vec![VonMember {
                    field_name: "range".to_string(),
                    value: Self::vonify_range(range),
                }],
            }),
            IExpressionPE::While(WhilePE {
                range,
                condition,
                body,
            }) => IVonData::Object(VonObject {
                tyype: "While".to_string(),
                id: None,
                members: vec![
                    VonMember {
                        field_name: "range".to_string(),
                        value: Self::vonify_range(range),
                    },
                    VonMember {
                        field_name: "condition".to_string(),
                        value: Self::vonify_expression(condition),
                    },
                    VonMember {
                        field_name: "body".to_string(),
                        value: Self::vonify_block(body),
                    },
                ],
            }),
        }
    }
}

