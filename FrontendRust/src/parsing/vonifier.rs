use crate::lexing::ast::RangeL;
use crate::parsing::ast::*;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::von::{IVonData, VonArray, VonBool, VonFloat, VonInt, VonMember, VonObject, VonStr};
use std::marker::PhantomData;

/// ParserVonifier converts Parser AST to Von (JSON-like) format
/// Mirrors ParserVonifier.scala

pub struct ParserVonifier<'a> {
  _marker: PhantomData<&'a ()>,
}
impl<'a> ParserVonifier<'a> {
  /// Helper to vonify optional values
  /// Mirrors vonifyOptional in ParserVonifier.scala lines 11-16
  pub fn vonify_optional_ref<T, F>(opt: &Option<&'a T>, func: F) -> IVonData
  where
    F: Fn(&'a T) -> IVonData,
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

  /// Helper to vonify optional values
  /// Mirrors vonifyOptional in ParserVonifier.scala lines 11-16
  pub fn vonify_optional_owned<T, F>(opt: &Option<T>, func: F) -> IVonData
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
  pub fn vonify_file(file: &'a FileP<'a>) -> IVonData {
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
  pub fn vonify_denizen(denizen_p: &'a IDenizenP<'a>) -> IVonData {
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
  fn vonify_file_coord(coord: &'a FileCoordinate<'a>) -> IVonData {
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
            value: coord.filepath.to_string(),
          }),
        },
      ],
    })
  }

  /// Vonify a package coordinate
  fn vonify_package_coord(coord: &'a PackageCoordinate<'a>) -> IVonData {
    IVonData::Object(VonObject {
      tyype: "PackageCoordinate".to_string(),
      id: None,
      members: vec![
        VonMember {
          field_name: "module".to_string(),
          value: IVonData::Str(VonStr {
            value: coord.module.str.to_string(),
          }),
        },
        VonMember {
          field_name: "packages".to_string(),
          value: IVonData::Array(VonArray {
            id: None,
            members: coord
              .packages
              .iter()
              .map(|p| {
                IVonData::Str(VonStr {
                  value: p.str.to_string(),
                })
              })
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
  fn vonify_name(name: &NameP<'a>) -> IVonData {
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
            value: name.str.str.to_string(),
          }),
        },
      ],
    })
  }

  /// Vonify a struct
  /// Mirrors vonifyStruct in ParserVonifier.scala lines 68-83
  fn vonify_struct(thing: &'a StructP<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(mutability, Self::vonify_templex),
        },
        VonMember {
          field_name: "identifyingRunes".to_string(),
          value: Self::vonify_optional_owned(identifying_runes, Self::vonify_identifying_runes),
        },
        VonMember {
          field_name: "templateRules".to_string(),
          value: Self::vonify_optional_owned(template_rules, Self::vonify_template_rules),
        },
        VonMember {
          field_name: "maybeDefaultRegion".to_string(),
          value: Self::vonify_optional_owned(maybe_default_region_rune, Self::vonify_region_rune),
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
  fn vonify_function(thing: &FunctionP<'a>) -> IVonData {
    let FunctionP {
      range,
      header,
      body,
    } = thing;

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
          value: Self::vonify_optional_owned(body, |b| Self::vonify_block(b.as_ref())),
        },
      ],
    })
  }

  /// Vonify an interface
  /// Mirrors vonifyInterface in ParserVonifier.scala lines 135-150
  fn vonify_interface(thing: &'a InterfaceP<'a>) -> IVonData {
    let InterfaceP::<'a> {
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
          value: Self::vonify_optional_owned(mutability, Self::vonify_templex),
        },
        VonMember {
          field_name: "maybeIdentifyingRunes".to_string(),
          value: Self::vonify_optional_owned(maybe_identifying_runes, Self::vonify_identifying_runes),
        },
        VonMember {
          field_name: "templateRules".to_string(),
          value: Self::vonify_optional_owned(template_rules, Self::vonify_template_rules),
        },
        VonMember {
          field_name: "maybeDefaultRegion".to_string(),
          value: Self::vonify_optional_owned(maybe_default_region_rune, Self::vonify_region_rune),
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
  fn vonify_impl(thing: &'a ImplP<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(generic_params, Self::vonify_identifying_runes),
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
          value: Self::vonify_optional_owned(template_rules, Self::vonify_template_rules),
        },
        VonMember {
          field_name: "struct".to_string(),
          value: Self::vonify_optional_owned(struct_, Self::vonify_templex),
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
  fn vonify_export_as(thing: &'a ExportAsP<'a>) -> IVonData {
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
  fn vonify_import(thing: &'a ImportP<'a>) -> IVonData {
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
  fn vonify_function_header(thing: &FunctionHeaderP<'a>) -> IVonData {
    let FunctionHeaderP::<'a> {
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
          value: Self::vonify_optional_owned(name, Self::vonify_name),
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
          value: Self::vonify_optional_owned(generic_parameters, Self::vonify_identifying_runes),
        },
        VonMember {
          field_name: "templateRules".to_string(),
          value: Self::vonify_optional_owned(template_rules, Self::vonify_template_rules),
        },
        VonMember {
          field_name: "params".to_string(),
          value: Self::vonify_optional_owned(params, Self::vonify_params),
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
                value: Self::vonify_optional_owned(&ret.ret_type, Self::vonify_templex),
              },
            ],
          }),
        },
      ],
    })
  }

  /// Vonify params
  /// Mirrors vonifyParams in ParserVonifier.scala lines 256-264
  fn vonify_params(thing: &ParamsP<'a>) -> IVonData {
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
  fn vonify_parameter(thing: &ParameterP<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(self_borrow, Self::vonify_range),
        },
        VonMember {
          field_name: "maybePreChecked".to_string(),
          value: Self::vonify_optional_owned(maybe_pre_checked, Self::vonify_range),
        },
        VonMember {
          field_name: "virtuality".to_string(),
          value: Self::vonify_optional_owned(virtuality, Self::vonify_virtuality),
        },
        VonMember {
          field_name: "pattern".to_string(),
          value: Self::vonify_optional_owned(pattern, Self::vonify_pattern),
        },
      ],
    })
  }

  /// Vonify a pattern
  /// Mirrors vonifyPattern in ParserVonifier.scala lines 279-289
  fn vonify_pattern(thing: &PatternPP<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(destination, Self::vonify_destination_local),
        },
        VonMember {
          field_name: "templex".to_string(),
          value: Self::vonify_optional_owned(templex, Self::vonify_templex),
        },
        VonMember {
          field_name: "destructure".to_string(),
          value: Self::vonify_optional_owned(destructure, Self::vonify_destructure),
        },
      ],
    })
  }

  /// Vonify an attribute
  /// Mirrors vonifyAttribute in ParserVonifier.scala lines 381-409
  fn vonify_attribute(thing: &IAttributeP<'a>) -> IVonData {
    match thing {
      IAttributeP::WeakableAttribute(WeakableAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "WeakableAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::SealedAttribute(SealedAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "SealedAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::LinearAttribute(LinearAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "LinearAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::ExportAttribute(ExportAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "ExportAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::MacroCall(MacroCallP {
        range,
        inclusion,
        name,
      }) => IVonData::Object(VonObject {
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
      }),
      IAttributeP::AbstractAttribute(AbstractAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "AbstractAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::ExternAttribute(ExternAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "ExternAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::PureAttribute(PureAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "PureAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::AdditiveAttribute(AdditiveAttributeP { range }) => IVonData::Object(VonObject {
        tyype: "AdditiveAttribute".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      IAttributeP::BuiltinAttribute(BuiltinAttributeP {
        range,
        generator_name,
      }) => IVonData::Object(VonObject {
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
      }),
    }
  }

  /// Vonify struct members
  /// Mirrors vonifyStructMembers in ParserVonifier.scala lines 85-93
  fn vonify_struct_members(thing: &StructMembersP<'a>) -> IVonData {
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
  fn vonify_struct_contents(thing: &IStructContent<'a>) -> IVonData {
    match thing {
      IStructContent::StructMethod(function) => IVonData::Object(VonObject {
        tyype: "StructMethod".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "function".to_string(),
          value: Self::vonify_function(function),
        }],
      }),
      IStructContent::NormalStructMember(NormalStructMemberP {
        range,
        name,
        variability,
        tyype,
      }) => IVonData::Object(VonObject {
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
      IStructContent::VariadicStructMember(VariadicStructMemberP {
        range,
        variability,
        tyype,
      }) => IVonData::Object(VonObject {
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
  fn vonify_destination_local(thing: &DestinationLocalP<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(mutate, Self::vonify_range),
        },
      ],
    })
  }

  /// Vonify name declaration
  /// Mirrors vonifyNameDeclaration in ParserVonifier.scala lines 311-320
  fn vonify_name_declaration(thing: &INameDeclarationP<'a>) -> IVonData {
    match thing {
      INameDeclarationP::IgnoredLocalNameDeclaration(range) => IVonData::Object(VonObject {
        tyype: "IgnoredLocalNameDeclaration".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      INameDeclarationP::LocalNameDeclaration(name) => IVonData::Object(VonObject {
        tyype: "LocalNameDeclaration".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "name".to_string(),
          value: Self::vonify_name(name),
        }],
      }),
      INameDeclarationP::ConstructingMemberNameDeclaration(name) => IVonData::Object(VonObject {
        tyype: "ConstructingMemberNameDeclaration".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "name".to_string(),
          value: Self::vonify_name(name),
        }],
      }),
      INameDeclarationP::IterableNameDeclaration(range) => IVonData::Object(VonObject {
        tyype: "IterableNameDeclaration".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      INameDeclarationP::IteratorNameDeclaration(range) => IVonData::Object(VonObject {
        tyype: "IteratorNameDeclaration".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      INameDeclarationP::IterationOptionNameDeclaration(range) => IVonData::Object(VonObject {
        tyype: "IterationOptionNameDeclaration".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
    }
  }

  /// Vonify destructure
  /// Mirrors vonifyDestructure in ParserVonifier.scala lines 362-370
  fn vonify_destructure(thing: &DestructureP<'a>) -> IVonData {
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
  fn vonify_template_rules(thing: &TemplateRulesP<'a>) -> IVonData {
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
  fn vonify_rule(thing: &IRulexPR<'a>) -> IVonData {
    match thing {
      IRulexPR::Equals(EqualsPR { range, left, right }) => IVonData::Object(VonObject {
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
      IRulexPR::Or(OrPR {
        range,
        possibilities,
      }) => IVonData::Object(VonObject {
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
      IRulexPR::Dot(DotPR {
        range,
        container,
        member_name,
      }) => IVonData::Object(VonObject {
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
      IRulexPR::Components(ComponentsPR {
        range,
        container,
        components,
      }) => IVonData::Object(VonObject {
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
      IRulexPR::Typed(TypedPR { range, rune, tyype }) => IVonData::Object(VonObject {
        tyype: "TypedPR".to_string(),
        id: None,
        members: vec![
          VonMember {
            field_name: "range".to_string(),
            value: Self::vonify_range(range),
          },
          VonMember {
            field_name: "rune".to_string(),
            value: Self::vonify_optional_owned(rune, Self::vonify_name),
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
        members: vec![VonMember {
          field_name: "templex".to_string(),
          value: Self::vonify_templex(templex),
        }],
      }),
      IRulexPR::BuiltinCall(BuiltinCallPR { range, name, args }) => IVonData::Object(VonObject {
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
      IRulexPR::Pack(PackPR { range, elements }) => IVonData::Object(VonObject {
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
  pub fn vonify_identifying_runes(thing: &GenericParametersP<'a>) -> IVonData {
    let GenericParametersP {
      range,
      params: identifying_runes_p,
    } = thing;
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
            members: identifying_runes_p
              .iter()
              .map(Self::vonify_generic_parameter)
              .collect(),
          }),
        },
      ],
    })
  }

  /// Vonify generic parameter
  /// Mirrors vonifyGenericParameter in ParserVonifier.scala lines 542-554
  fn vonify_generic_parameter(thing: &GenericParameterP<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(maybe_type, Self::vonify_generic_parameter_type),
        },
        VonMember {
          field_name: "maybeCoordRegion".to_string(),
          value: Self::vonify_optional_owned(coord_region, Self::vonify_region_rune),
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
          value: Self::vonify_optional_owned(maybe_default, Self::vonify_templex),
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
  fn vonify_region_rune(region_rune: &RegionRunePT<'a>) -> IVonData {
    let RegionRunePT::<'a> { range, name } = region_rune;
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
          value: Self::vonify_optional_owned(name, Self::vonify_name),
        },
      ],
    })
  }

  /// Vonify pack
  fn vonify_pack(thing: &PackPT<'a>) -> IVonData {
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
  fn vonify_templex(thing: &ITemplexPT<'a>) -> IVonData {
    match thing {
      ITemplexPT::RegionRune(r) => Self::vonify_region_rune(r),
      ITemplexPT::AnonymousRune(AnonymousRunePT { range }) => IVonData::Object(VonObject {
        tyype: "AnonymousRuneT".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "range".to_string(),
          value: Self::vonify_range(range),
        }],
      }),
      ITemplexPT::Point(PointPT { range, inner }) => IVonData::Object(VonObject {
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
      ITemplexPT::Bool(BoolPT { range, value }) => IVonData::Object(VonObject {
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
      ITemplexPT::Call(CallPT {
        range,
        template,
        args,
      }) => IVonData::Object(VonObject {
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
      ITemplexPT::Inline(InlinePT { range, inner }) => IVonData::Object(VonObject {
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
      ITemplexPT::Int(IntPT { range, value }) => IVonData::Object(VonObject {
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
      ITemplexPT::Location(LocationPT { range, location }) => IVonData::Object(VonObject {
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
      ITemplexPT::Tuple(TuplePT { range, elements }) => IVonData::Object(VonObject {
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
      ITemplexPT::Mutability(MutabilityPT { range, mutability }) => IVonData::Object(VonObject {
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
      ITemplexPT::NameOrRune(NameOrRunePT { name: rune }) => IVonData::Object(VonObject {
        tyype: "NameOrRuneT".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "rune".to_string(),
          value: Self::vonify_name(rune),
        }],
      }),
      ITemplexPT::Interpreted(InterpretedPT {
        range,
        maybe_ownership,
        maybe_region,
        inner,
      }) => IVonData::Object(VonObject {
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
              Some(o) => Self::vonify_optional_owned(&Some(o.as_ref()), |t| Self::vonify_ownership_pt(t)),
              None => {
                Self::vonify_optional_owned::<&OwnershipPT, _>(&None, |t| Self::vonify_ownership_pt(t))
              }
            },
          },
          VonMember {
            field_name: "maybeRegion".to_string(),
            value: match maybe_region {
              Some(r) => Self::vonify_optional_owned(&Some(r.as_ref()), |t| Self::vonify_region_rune(t)),
              None => {
                Self::vonify_optional_owned::<&RegionRunePT, _>(&None, |t| Self::vonify_region_rune(t))
              }
            },
          },
          VonMember {
            field_name: "inner".to_string(),
            value: Self::vonify_templex(inner),
          },
        ],
      }),
      ITemplexPT::Ownership(OwnershipPT { range, ownership }) => IVonData::Object(VonObject {
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
      ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
        range,
        mutability,
        variability,
        size,
        element,
      }) => IVonData::Object(VonObject {
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
      ITemplexPT::RuntimeSizedArray(RuntimeSizedArrayPT {
        range,
        mutability,
        element,
      }) => IVonData::Object(VonObject {
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
      }),
      ITemplexPT::Function(FunctionPT {
        range,
        mutability,
        parameters,
        return_type,
      }) => IVonData::Object(VonObject {
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
              Some(m) => Self::vonify_optional_owned(&Some(m.as_ref()), |t| Self::vonify_templex(t)),
              None => Self::vonify_optional_owned::<&ITemplexPT, _>(&None, |t| Self::vonify_templex(t)),
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
      ITemplexPT::Func(FuncPT {
        range,
        name,
        params_range,
        parameters,
        return_type,
      }) => IVonData::Object(VonObject {
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
      ITemplexPT::Share(SharePT { range, inner }) => IVonData::Object(VonObject {
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
      ITemplexPT::String(StringPT { range, str }) => IVonData::Object(VonObject {
        tyype: "StringT".to_string(),
        id: None,
        members: vec![
          VonMember {
            field_name: "range".to_string(),
            value: Self::vonify_range(range),
          },
          VonMember {
            field_name: "str".to_string(),
            value: IVonData::Str(VonStr { value: str.clone() }),
          },
        ],
      }),
      ITemplexPT::TypedRune(TypedRunePT { range, rune, tyype }) => IVonData::Object(VonObject {
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
      ITemplexPT::Variability(VariabilityPT { range, variability }) => {
        IVonData::Object(VonObject {
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
        })
      }
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
  fn vonify_lookup(thing: &LookupPE<'a>) -> IVonData {
    let LookupPE {
      name,
      template_args,
    } = thing;
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
          value: Self::vonify_optional_owned(template_args, Self::vonify_template_args),
        },
      ],
    })
  }

  /// Vonify imprecise name
  /// Mirrors vonifyImpreciseName in ParserVonifier.scala lines 322-329
  fn vonify_imprecise_name(thing: &IImpreciseNameP<'a>) -> IVonData {
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
  fn vonify_block(thing: &BlockPE<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(maybe_pure, Self::vonify_range),
        },
        VonMember {
          field_name: "maybeDefaultRegion".to_string(),
          value: Self::vonify_optional_owned(maybe_default_region, Self::vonify_region_rune),
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
  fn vonify_consecutor(thing: &ConsecutorPE<'a>) -> IVonData {
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
  fn vonify_template_args(thing: &TemplateArgsP<'a>) -> IVonData {
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
  fn vonify_array_size(obj: &IArraySizeP<'a>) -> IVonData {
    match obj {
      IArraySizeP::RuntimeSized => IVonData::Object(VonObject {
        tyype: "RuntimeSized".to_string(),
        id: None,
        members: vec![],
      }),
      IArraySizeP::StaticSized(static_sized) => IVonData::Object(VonObject {
        tyype: "StaticSized".to_string(),
        id: None,
        members: vec![VonMember {
          field_name: "size".to_string(),
          value: Self::vonify_optional_owned(&static_sized.size_pt, Self::vonify_templex),
        }],
      }),
    }
  }

  /// Vonify construct array
  /// Mirrors vonifyConstructArray in ParserVonifier.scala lines 1158-1172
  fn vonify_construct_array(ca: &ConstructArrayPE<'a>) -> IVonData {
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
          value: Self::vonify_optional_owned(type_pt, Self::vonify_templex),
        },
        VonMember {
          field_name: "mutability".to_string(),
          value: Self::vonify_optional_owned(mutability_pt, Self::vonify_templex),
        },
        VonMember {
          field_name: "variability".to_string(),
          value: Self::vonify_optional_owned(variability_pt, Self::vonify_templex),
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
  fn vonify_expression(thing: &IExpressionPE<'a>) -> IVonData {
    match thing {
      IExpressionPE::ConstantBool(ConstantBoolPE { range, value }) => IVonData::Object(VonObject {
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
      }),
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
            value: Self::vonify_optional_owned(maybe_pure, Self::vonify_range),
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
      IExpressionPE::Shortcall(ShortcallPE { range, arg_exprs }) => IVonData::Object(VonObject {
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
      }),
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
              value: Self::vonify_optional_owned(bits, |b| IVonData::Int(VonInt { value: *b as i64 })),
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
            value: Self::vonify_optional_owned(template_args, Self::vonify_template_args),
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
      IExpressionPE::ConstantStr(ConstantStrPE { range, value }) => IVonData::Object(VonObject {
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
      }),
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
      IExpressionPE::Lambda(LambdaPE { captures, function }) => IVonData::Object(VonObject {
        tyype: "Lambda".to_string(),
        id: None,
        members: vec![
          VonMember {
            field_name: "captures".to_string(),
            value: Self::vonify_optional_owned(captures, Self::vonify_unit),
          },
          VonMember {
            field_name: "function".to_string(),
            value: Self::vonify_function(function),
          },
        ],
      }),
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

/*
package dev.vale.parsing

import dev.vale.lexing.RangeL
import dev.vale.parsing.ast._
import dev.vale.{FileCoordinate, PackageCoordinate, Profiler, vimpl}
import dev.vale.parsing.ast._
import dev.vale.von.{IVonData, VonArray, VonBool, VonFloat, VonInt, VonMember, VonObject, VonStr}

object ParserVonifier {

  def vonifyOptional[T](opt: Option[T], func: (T) => IVonData): IVonData = {
    opt match {
      case None => VonObject("None", None, Vector())
      case Some(value) => VonObject("Some", None, Vector(VonMember("value", func(value))))
    }
  }

  def vonifyFile(file: FileP): IVonData = {
    Profiler.frame(() => {
      val FileP(fileCoord, commentsRanges, denizens) = file

      VonObject(
        "File",
        None,
        Vector(
          VonMember("fileCoord", vonifyFileCoord(fileCoord)),
          VonMember("commentsRanges", VonArray(None, commentsRanges.map(vonifyRange).toVector)),
          VonMember("denizens", VonArray(None, denizens.map(vonifyDenizen).toVector))))
    })
  }

  def vonifyDenizen(denizenP: IDenizenP): VonObject = {
    denizenP match {
      case TopLevelFunctionP(function) => vonifyFunction(function)
      case TopLevelStructP(struct) => vonifyStruct(struct)
      case TopLevelInterfaceP(interface) => vonifyInterface(interface)
      case TopLevelImplP(impl) => vonifyImpl(impl)
      case TopLevelExportAsP(impl) => vonifyExportAs(impl)
      case TopLevelImportP(impl) => vonifyImport(impl)
    }
  }

  def vonifyGenericParameterType(thing: GenericParameterTypeP): VonObject = {
    val GenericParameterTypeP(range, tyype) = thing
    VonObject(
      "GenericParameterType",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("type", vonifyRuneType(tyype))))
  }

  def vonifyRuneAttribute(thing: IRuneAttributeP): VonObject = {
    thing match {
      case ReadOnlyRegionRuneAttributeP(range) => VonObject("ReadOnlyRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case ReadWriteRegionRuneAttributeP(range) => VonObject("ReadWriteRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case AdditiveRegionRuneAttributeP(range) => VonObject("AdditiveRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case ImmutableRegionRuneAttributeP(range) => VonObject("ImmutableRegionRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case PoolRuneAttributeP(range) => VonObject("PoolRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case ArenaRuneAttributeP(range) => VonObject("ArenaRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case BumpRuneAttributeP(range) => VonObject("BumpRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case ImmutableRuneAttributeP(range) => VonObject("ImmutableRuneAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case x => vimpl(x.toString)
    }
  }

  def vonifyStruct(thing: StructP): VonObject = {
    val StructP(range, name, attributes, mutability, identifyingRunes, templateRules, maybeDefaultRegionRuneP, bodyRange, members) = thing
    VonObject(
      "Struct",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("name", vonifyName(name)),
        VonMember("attributes", VonArray(None, attributes.map(vonifyAttribute).toVector)),
        VonMember("mutability", vonifyOptional(mutability, vonifyTemplex)),
        VonMember("identifyingRunes", vonifyOptional(identifyingRunes, vonifyIdentifyingRunes)),
        VonMember("templateRules", vonifyOptional(templateRules, vonifyTemplateRules)),
        VonMember("maybeDefaultRegion", vonifyOptional(maybeDefaultRegionRuneP, vonifyRegionRune)),
        VonMember("bodyRange", vonifyRange(range)),
        VonMember("members", vonifyStructMembers(members))))
  }

  def vonifyStructMembers(thing: StructMembersP) = {
    val StructMembersP(range, contents) = thing
    VonObject(
      "StructMembers",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("members", VonArray(None, contents.map(vonifyStructContents).toVector))))
  }

  def vonifyStructContents(thing: IStructContent) = {
    thing match {
      case sm @ StructMethodP(_) => vonifyStructMethod(sm)
      case sm @ NormalStructMemberP(_, _, _, _) => vonifyStructMember(sm)
      case sm @ VariadicStructMemberP(_, _, _) => vonifyVariadicStructMember(sm)
    }
  }

  def vonifyStructMember(thing: NormalStructMemberP) = {
    val NormalStructMemberP(range, name, variability, tyype) = thing
    VonObject(
      "NormalStructMember",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("name", vonifyName(name)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("type", vonifyTemplex(tyype))))
  }

  def vonifyVariadicStructMember(thing: VariadicStructMemberP) = {
    val VariadicStructMemberP(range, variability, tyype) = thing
    VonObject(
      "VariadicStructMember",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("type", vonifyTemplex(tyype))))
  }

  def vonifyStructMethod(thing: StructMethodP) = {
    val StructMethodP(function) = thing
    VonObject(
      "StructMethod",
      None,
      Vector(
        VonMember("function", vonifyFunction(function))))
  }

  def vonifyInterface(thing: InterfaceP): VonObject = {
    val InterfaceP(range, name, attributes, mutability, maybeIdentifyingRunes, templateRules, maybeDefaultRegionRuneP, bodyRange, members) = thing
    VonObject(
      "Interface",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("name", vonifyName(name)),
        VonMember("attributes", VonArray(None, attributes.map(vonifyAttribute).toVector)),
        VonMember("mutability", vonifyOptional(mutability, vonifyTemplex)),
        VonMember("maybeIdentifyingRunes", vonifyOptional(maybeIdentifyingRunes, vonifyIdentifyingRunes)),
        VonMember("templateRules", vonifyOptional(templateRules, vonifyTemplateRules)),
        VonMember("maybeDefaultRegion", vonifyOptional(maybeDefaultRegionRuneP, vonifyRegionRune)),
        VonMember("bodyRange", vonifyRange(range)),
        VonMember("members", VonArray(None, members.map(vonifyFunction).toVector))))
  }

  def vonifyImpl(impl: ImplP): VonObject = {
    val ImplP(range, identifyingRunes, templateRules, struct, interface, attributes) = impl

    VonObject(
      "Impl",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("identifyingRunes", vonifyOptional(identifyingRunes, vonifyIdentifyingRunes)),
        VonMember("attributes", VonArray(None, attributes.map(vonifyAttribute).toVector)),
        VonMember("templateRules", vonifyOptional(templateRules, vonifyTemplateRules)),
        VonMember("struct", vonifyOptional(struct, vonifyTemplex)),
        VonMember("interface", vonifyTemplex(interface))))
  }

  def vonifyExportAs(exportAs: ExportAsP): VonObject = {
    val ExportAsP(range, struct, exportedName) = exportAs

    VonObject(
      "ExportAs",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("struct", vonifyTemplex(struct)),
        VonMember("exportedName", vonifyName(exportedName))))
  }

  def vonifyImport(exportAs: ImportP): VonObject = {
    val ImportP(range, moduleName, packageSteps, importeeName) = exportAs

    VonObject(
      "Import",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("moduleName", vonifyName(moduleName)),
        VonMember("packageSteps", VonArray(None, packageSteps.map(vonifyName).toVector)),
        VonMember("importeeName", vonifyName(importeeName))))
  }

  def vonifyFileCoord(range: FileCoordinate): VonObject = {
    val FileCoordinate(packageCoord, filepath) = range
    VonObject(
      "FileCoordinate",
      None,
      Vector(
        VonMember("packageCoord", vonifyPackageCoord(packageCoord)),
        VonMember("filepath", VonStr(filepath))))
  }

  def vonifyPackageCoord(range: PackageCoordinate): VonObject = {
    val PackageCoordinate(module, packages) = range
    VonObject(
      "PackageCoordinate",
      None,
      Vector(
        VonMember("module", VonStr(module.str)),
        VonMember("packages", VonArray(None, packages.map(p => VonStr(p.str))))))
  }

  def vonifyRange(range: RangeL): VonObject = {
    val RangeL(begin, end) = range
    VonObject(
      "Range",
      None,
      Vector(
        VonMember("begin", VonInt(begin)),
        VonMember("end", VonInt(end))))
  }

  def vonifyFunction(thing: FunctionP): VonObject = {
    val FunctionP(range, header, body) = thing
    VonObject(
      "Function",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("header", vonifyFunctionHeader(header)),
        VonMember("body", vonifyOptional(body, vonifyBlock))))
  }

  def vonifyFunctionHeader(thing: FunctionHeaderP): VonObject = {
    val FunctionHeaderP(range, name, attributes, maybeUserSpecifiedIdentifyingRunes, templateRules, params, FunctionReturnP(retRange, retType)) = thing
    VonObject(
      "FunctionHeader",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("name", vonifyOptional(name, vonifyName)),
        VonMember("attributes", VonArray(None, attributes.map(vonifyAttribute).toVector)),
        VonMember("maybeUserSpecifiedIdentifyingRunes", vonifyOptional(maybeUserSpecifiedIdentifyingRunes, vonifyIdentifyingRunes)),
        VonMember("templateRules", vonifyOptional(templateRules, vonifyTemplateRules)),
        VonMember("params", vonifyOptional(params, vonifyParams)),
        VonMember(
          "return",
          VonObject(
            "FunctionReturn",
            None,
            Vector(
              VonMember("range", vonifyRange(retRange)),
              VonMember("retType", vonifyOptional(retType, vonifyTemplex)))))))
//        VonMember("maybeDefaultRegion", vonifyOptional(maybeDefaultRegion, vonifyName))))
  }

  def vonifyParams(thing: ParamsP): VonObject = {
    val ParamsP(range, patterns) = thing
    VonObject(
      "Params",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("params", VonArray(None, patterns.map(vonifyParameter)))))
  }

  def vonifyParameter(thing: ParameterP): VonObject = {
    val ParameterP(range, virtuality, maybePreChecked, selfBorrow, pattern) = thing
    VonObject(
      "Parameter",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("selfBorrow", vonifyOptional(selfBorrow, vonifyRange)),
        VonMember("maybePreChecked", vonifyOptional(maybePreChecked, vonifyRange)),
        VonMember("virtuality", vonifyOptional(virtuality, vonifyVirtuality)),
        VonMember("pattern", vonifyOptional(pattern, vonifyPattern))))
  }

  def vonifyPattern(thing: PatternPP): VonObject = {
    val PatternPP(range, capture, templex, destructure) = thing
    VonObject(
      "Pattern",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("capture", vonifyOptional(capture, vonifyDestinationLocal)),
        VonMember("templex", vonifyOptional(templex, vonifyTemplex)),
        VonMember("destructure", vonifyOptional(destructure, vonifyDestructure))))
  }

//  def vonifyCapture(thing: INameDeclarationP): VonObject = {
//    val INameDeclarationP(range, captureName) = thing
//    VonObject(
//      "Capture",
//      None,
//      Vector(
//        VonMember("range", vonifyRange(range)),
//        VonMember("captureName", vonifyCaptureName(captureName))))
//  }

  def vonifyDestinationLocal(thing: DestinationLocalP): VonObject = {
    val DestinationLocalP(decl, mutate) = thing
    VonObject(
      "DestinationLocal",
      None,
      Vector(
        VonMember("name", vonifyNameDeclaration(decl)),
        VonMember("mutate", vonifyOptional(mutate, vonifyRange))))
  }

  def vonifyNameDeclaration(thing: INameDeclarationP): VonObject = {
    thing match {
      case IgnoredLocalNameDeclarationP(range) => VonObject("IgnoredLocalNameDeclaration", None, Vector(VonMember("range", vonifyRange(range))))
      case LocalNameDeclarationP(name) => VonObject("LocalNameDeclaration", None, Vector(VonMember("name", vonifyName(name))))
      case ConstructingMemberNameDeclarationP(name) => VonObject("ConstructingMemberNameDeclaration", None, Vector(VonMember("name", vonifyName(name))))
      case IterableNameDeclarationP(range) => VonObject("IterableNameDeclaration", None, Vector(VonMember("range", vonifyRange(range))))
      case IteratorNameDeclarationP(range) => VonObject("IteratorNameDeclaration", None, Vector(VonMember("range", vonifyRange(range))))
      case IterationOptionNameDeclarationP(range) => VonObject("IterationOptionNameDeclaration", None, Vector(VonMember("range", vonifyRange(range))))
    }
  }

  def vonifyImpreciseName(thing: IImpreciseNameP): VonObject = {
    thing match {
      case LookupNameP(name) => VonObject("LookupName", None, Vector(VonMember("name", vonifyName(name))))
      case IterableNameP(range) => VonObject("IterableName", None, Vector(VonMember("range", vonifyRange(range))))
      case IteratorNameP(range) => VonObject("IteratorName", None, Vector(VonMember("range", vonifyRange(range))))
      case IterationOptionNameP(range) => VonObject("IterationOptionName", None, Vector(VonMember("range", vonifyRange(range))))
    }
  }

  def vonifyVariability(thing: VariabilityP): VonObject = {
    thing match {
      case FinalP => VonObject("Final", None, Vector())
      case VaryingP => VonObject("Varying", None, Vector())
    }
  }

  def vonifyVirtuality(thing: AbstractP): VonObject = {
//    thing match {
//      case AbstractP(range) => {
    val AbstractP(range) = thing
        VonObject(
          "Abstract",
          None,
          Vector(
            VonMember("range", vonifyRange(range))))
//      }
//      case ov @ OverrideP(_, _) => vonifyOverride(ov)
//    }
  }

//  def vonifyOverride(thing: OverrideP): VonObject = {
//    val OverrideP(range, tyype) = thing
//    VonObject(
//      "Override",
//      None,
//      Vector(
//        VonMember("range", vonifyRange(range)),
//        VonMember("type", vonifyTemplex(tyype))))
//  }

  def vonifyDestructure(thing: DestructureP): VonObject = {
    val DestructureP(range, patterns) = thing
    VonObject(
      "Destructure",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("patterns", VonArray(None, patterns.map(vonifyPattern).toVector))))
  }

  def vonifyUnit(thing: UnitP): VonObject = {
    val UnitP(range) = thing
    VonObject(
      "Unit",
      None,
      Vector(
        VonMember("range", vonifyRange(range))))
  }

  def vonifyAttribute(thing: IAttributeP): VonObject = {
    thing match {
      case WeakableAttributeP(range) => VonObject("WeakableAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case SealedAttributeP(range) => VonObject("SealedAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case LinearAttributeP(range) => VonObject("LinearAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case ExportAttributeP(range) => VonObject("ExportAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case MacroCallP(range, dontCall, name) => {
        VonObject(
          "MacroCall",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("dontCall", VonBool(dontCall == DontCallMacroP)),
            VonMember("name", vonifyName(name))))
      }
      case AbstractAttributeP(range) => VonObject("AbstractAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case ExternAttributeP(range) => VonObject("ExternAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case PureAttributeP(range) => VonObject("PureAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case AdditiveAttributeP(range) => VonObject("AdditiveAttribute", None, Vector(VonMember("range", vonifyRange(range))))
      case BuiltinAttributeP(range, generatorName) => {
        VonObject(
          "BuiltinAttribute",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("generatorName", vonifyName(generatorName))))
      }
    }
  }

  def vonifyTemplateRules(thing: TemplateRulesP): VonObject = {
    val TemplateRulesP(range, rules) = thing
    VonObject(
      "TemplateRules",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("rules", VonArray(None, rules.map(vonifyRule).toVector))))
  }

  def vonifyRule(thing: IRulexPR): VonObject = {
    thing match {
      case EqualsPR(range, left, right) => {
        VonObject(
          "EqualsPR",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("left", vonifyRule(left)),
            VonMember("right", vonifyRule(right))
          )
        )
      }
      case OrPR(range, possibilities) => {
        VonObject(
          "OrPR",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("possibilities", VonArray(None, possibilities.map(vonifyRule).toVector))
          )
        )
      }
      case DotPR(range, container, memberName) => {
        VonObject(
          "DotPR",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("container", vonifyRule(container)),
            VonMember("memberName", vonifyName(memberName))))
      }
      case ComponentsPR(range, container, components) => {
        VonObject(
          "ComponentsPR",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("container", vonifyRuneType(container)),
            VonMember("components", VonArray(None, components.map(vonifyRule).toVector))
          )
        )
      }
      case TypedPR(range, rune, tyype) => {
        VonObject(
          "TypedPR",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("rune", vonifyOptional(rune, vonifyName)),
            VonMember("type", vonifyRuneType(tyype))))
      }
      case TemplexPR(templex) => {
        VonObject(
          "TemplexPR",
          None,
          Vector(
            VonMember("templex", vonifyTemplex(templex))))
      }
      case BuiltinCallPR(range, name, args) => {
        VonObject(
          "BuiltinCallPR",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("name", vonifyName(name)),
            VonMember("args", VonArray(None, args.map(vonifyRule).toVector))
          )
        )
      }
//      case ResolveSignaturePR(range, nameStrRule, argsPackRule) => {
//        VonObject(
//          "ResolveSignaturePR",
//          None,
//          Vector(
//            VonMember("range", vonifyRange(range)),
//            VonMember("nameStrRule", vonifyRule(nameStrRule)),
//            VonMember("argsPackRule", vonifyRule(argsPackRule)),
//          )
//        )
//      }
      case PackPR(range, elements) => {
        VonObject(
          "PackPR",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("elements", VonArray(None, elements.map(vonifyRule).toVector))
          )
        )
      }
    }
  }

  def vonifyRuneType(thing: ITypePR): VonObject = {
    thing match {
      case IntTypePR => VonObject("IntTypePR", None, Vector())
      case BoolTypePR => VonObject("BoolTypePR", None, Vector())
      case OwnershipTypePR => VonObject("OwnershipTypePR", None, Vector())
      case MutabilityTypePR => VonObject("MutabilityTypePR", None, Vector())
      case VariabilityTypePR => VonObject("VariabilityTypePR", None, Vector())
      case LocationTypePR => VonObject("LocationTypePR", None, Vector())
      case CoordTypePR => VonObject("CoordTypePR", None, Vector())
      case CoordListTypePR => VonObject("CoordListTypePR", None, Vector())
      case PrototypeTypePR => VonObject("PrototypeTypePR", None, Vector())
      case KindTypePR => VonObject("KindTypePR", None, Vector())
      case RegionTypePR => VonObject("RegionTypePR", None, Vector())
      case CitizenTemplateTypePR => VonObject("CitizenTemplateTypePR", None, Vector())
    }
  }

  def vonifyIdentifyingRunes(thing: GenericParametersP): VonObject = {
    val GenericParametersP(range, identifyingRunesP) = thing
    VonObject(
      "IdentifyingRunes",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("identifyingRunes", VonArray(None, identifyingRunesP.map(vonifyGenericParameter).toVector))))
  }

  def vonifyGenericParameter(thing: GenericParameterP): VonObject = {
    val GenericParameterP(range, name, maybeType, maybeCoordRegion, attributes, maybeDefault) = thing
    VonObject(
      "IdentifyingRune",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("name", vonifyName(name)),
        VonMember("maybeType", vonifyOptional(maybeType, vonifyGenericParameterType)),
        VonMember("maybeCoordRegion", vonifyOptional(maybeCoordRegion, vonifyRegionRune)),
        VonMember("attributes", VonArray(None, attributes.map(vonifyRuneAttribute).toVector)),
        VonMember("maybeDefault", vonifyOptional(maybeDefault, vonifyTemplex))))
  }

  def vonifyName(thing: NameP): VonObject = {
    val NameP(range, name) = thing
    VonObject(
      "Name",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("name", VonStr(name.str))))
  }

  def vonifyTemplex(thing: ITemplexPT): VonObject = {
    thing match {
      case r @ RegionRunePT(_, _) => {
        vonifyRegionRune(r)
      }
      case AnonymousRunePT(range) => {
        VonObject(
          "AnonymousRuneT",
          None,
          Vector(
            VonMember("range", vonifyRange(range))))
      }
      case BoolPT(range, value) => {
        VonObject(
          "BoolT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("value", VonBool(value))))
      }
      case CallPT(range, template, args) => {
        VonObject(
          "CallT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("template", vonifyTemplex(template)),
            VonMember("args", VonArray(None, args.map(vonifyTemplex).toVector))))
      }
      case InlinePT(range, inner) => {
        VonObject(
          "InlineT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("inner", vonifyTemplex(inner))))
      }
      case IntPT(range, value) => {
        VonObject(
          "IntT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("inner", VonStr(value.toString))))
      }
      case LocationPT(range, location) => {
        VonObject(
          "LocationT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("location", vonifyLocation(location))))
      }
      case TuplePT(range, members) => {
        VonObject(
          "ManualSequenceT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("members", VonArray(None, members.map(vonifyTemplex).toVector))))
      }
      case MutabilityPT(range, mutability) => {
        VonObject(
          "MutabilityT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("mutability", vonifyMutability(mutability))))
      }
      case NameOrRunePT(rune) => {
        VonObject(
          "NameOrRuneT",
          None,
          Vector(
            VonMember("rune", vonifyName(rune))))
      }
      case InterpretedPT(range, maybeOwnership, maybeRegion, inner) => {
        VonObject(
          "InterpretedT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("maybeOwnership", vonifyOptional(maybeOwnership, vonifyTemplex)),
            VonMember("maybeRegion", vonifyOptional(maybeRegion, vonifyRegionRune)),
            VonMember("inner", vonifyTemplex(inner))))
      }
      case OwnershipPT(range, ownership) => {
        VonObject(
          "OwnershipT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("ownership", vonifyOwnership(ownership))))
      }
      case StaticSizedArrayPT(range, mutability, variability, size, element) => {
        VonObject(
          "StaticSizedArrayT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("mutability", vonifyTemplex(mutability)),
            VonMember("variability", vonifyTemplex(variability)),
            VonMember("size", vonifyTemplex(size)),
            VonMember("element", vonifyTemplex(element))))
      }
      case RuntimeSizedArrayPT(range, mutability, element) => {
        VonObject(
          "RuntimeSizedArrayT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("mutability", vonifyTemplex(mutability)),
            VonMember("element", vonifyTemplex(element))))
      }
      case FunctionPT(range, mutability, parameters, returnType) => {
        VonObject(
          "FunctionT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("mutability", vonifyOptional(mutability, vonifyTemplex)),
            VonMember("params", vonifyTemplex(parameters)),
            VonMember("returnType", vonifyTemplex(returnType))))
      }
      case PackPT(range, members) => {
        VonObject(
          "PackT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("members", VonArray(None, members.map(vonifyTemplex).toVector))))
      }
      case FuncPT(range, name, paramsRange, parameters, returnType) => {
        VonObject(
          "PrototypeT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("name", vonifyName(name)),
            VonMember("paramsRange", vonifyRange(paramsRange)),
            VonMember("params", VonArray(None, parameters.map(vonifyTemplex).toVector)),
            VonMember("returnType", vonifyTemplex(returnType))))
      }
      case SharePT(range, inner) => {
        VonObject(
          "ShareT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("inner", vonifyTemplex(inner))))
      }
      case StringPT(range, str) => {
        VonObject(
          "StringT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("str", VonStr(str))))
      }
      case TypedRunePT(range, rune, tyype) => {
        VonObject(
          "TypedRuneT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("rune", vonifyName(rune)),
            VonMember("type", vonifyRuneType(tyype))))
      }
      case VariabilityPT(range, variability) => {
        VonObject(
          "VariabilityT",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("variability", vonifyVariability(variability))))
      }
    }
  }

  private def vonifyRegionRune(regionRune: RegionRunePT): VonObject = {
    val RegionRunePT(range, name) = regionRune
    VonObject(
      "RegionRuneT",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("name", vonifyOptional(name, x => vonifyName(x)))))
  }

  def vonifyMutability(thing: MutabilityP): IVonData = {
    thing match {
      case MutableP => VonObject("Mutable", None, Vector())
      case ImmutableP => VonObject("Immutable", None, Vector())
    }
  }

  def vonifyLocation(thing: LocationP): VonObject = {
    thing match {
      case InlineP => VonObject("Inline", None, Vector())
      case YonderP => VonObject("Yonder", None, Vector())
    }
  }

  def vonifyOwnership(thing: OwnershipP): VonObject = {
    thing match {
      case ShareP => VonObject("Share", None, Vector())
      case OwnP => VonObject("Own", None, Vector())
      case BorrowP => VonObject("Borrow", None, Vector())
      case WeakP => VonObject("Weak", None, Vector())
    }
  }

  def vonifyLoadAs(thing: LoadAsP): VonObject = {
    thing match {
      case UseP => VonObject("Use", None, Vector())
      case MoveP => VonObject("Move", None, Vector())
      case LoadAsBorrowP => VonObject("LoadAsBorrow", None, Vector())
      case LoadAsWeakP => VonObject("LoadAsWeak", None, Vector())
    }
  }

  def vonifyBlock(thing: BlockPE): VonObject = {
    val BlockPE(range, maybePure, maybeDefaultRegion, inner) = thing
    VonObject(
      "Block",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("maybePure", vonifyOptional(maybePure, vonifyRange)),
        VonMember("maybeDefaultRegion", vonifyOptional(maybeDefaultRegion, vonifyRegionRune)),
        VonMember("inner", vonifyExpression(inner))))
  }

  def vonifyConsecutor(thing: ConsecutorPE): VonObject = {
    val ConsecutorPE(inners) = thing
    VonObject(
      "Consecutor",
      None,
      Vector(
        VonMember("inners", VonArray(None, inners.map(vonifyExpression).toVector))))
  }

  def vonifyExpression(thing: IExpressionPE): VonObject = {
    thing match {
      case ConstantBoolPE(range, value) => {
        VonObject(
          "ConstantBool",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("value", VonBool(value))))
      }
      case DotPE(range, left, operatorRange, member) => {
        VonObject(
          "Dot",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("left", vonifyExpression(left)),
            VonMember("operatorRange", vonifyRange(operatorRange)),
            VonMember("member", vonifyName(member))))
      }
      case ConstantFloatPE(range, value) => {
        VonObject(
          "ConstantFloat",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("value", VonFloat(value))))
      }
      case NotPE(range, inner) => {
        VonObject(
          "Not",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("innerExpr", vonifyExpression(inner))))
      }
      case RangePE(range, begin, end) => {
        VonObject(
          "Range",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("begin", vonifyExpression(begin)),
            VonMember("end", vonifyExpression(end))))
      }
      case FunctionCallPE(range, operatorRange, callableExpr, argExprs) => {
        VonObject(
          "FunctionCall",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("operatorRange", vonifyRange(operatorRange)),
            VonMember("callableExpr", vonifyExpression(callableExpr)),
            VonMember("argExprs", VonArray(None, argExprs.map(vonifyExpression).toVector))))
      }
      case BraceCallPE(range, operatorRange, callableExpr, argExprs, callableReadwrite) => {
        VonObject(
          "BraceCall",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("operatorRange", vonifyRange(operatorRange)),
            VonMember("callableExpr", vonifyExpression(callableExpr)),
            VonMember("argExprs", VonArray(None, argExprs.map(vonifyExpression).toVector)),
            VonMember("callableReadwrite", VonBool(callableReadwrite))))
      }
      case BinaryCallPE(range, name, leftExpr, rightExpr) => {
        VonObject(
          "BinaryCall",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("functionName", vonifyName(name)),
            VonMember("leftExpr", vonifyExpression(leftExpr)),
            VonMember("rightExpr", vonifyExpression(rightExpr))))
      }
      case SubExpressionPE(range, inner) => {
        VonObject(
          "SubExpression",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("innerExpr", vonifyExpression(inner))))
      }
      case EachPE(range, maybePure, entryPattern, inRange, iterableExpr, body) => {
        VonObject(
          "Each",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("maybePure", vonifyOptional(maybePure, vonifyRange)),
            VonMember("entryPattern", vonifyPattern(entryPattern)),
            VonMember("inRange", vonifyRange(inRange)),
            VonMember("iterableExpr", vonifyExpression(iterableExpr)),
            VonMember("body", vonifyBlock(body))))
      }
      case PackPE(range, innersPE) => {
        VonObject(
          "Pack",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("innerExprs", VonArray(None, innersPE.map(vonifyExpression).toVector))))
      }
      case MethodCallPE(range, subjectExpr, operatorRange, methodLookup, argExprs) => {
        VonObject(
          "MethodCall",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("operatorRange", vonifyRange(operatorRange)),
            VonMember("subjectExpr", vonifyExpression(subjectExpr)),
            VonMember("method", vonifyExpression(methodLookup)),
            VonMember("argExprs", VonArray(None, argExprs.map(vonifyExpression).toVector))))
      }
      case ShortcallPE(range, argExprs) => {
        VonObject(
          "Shortcall",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("argExprs", VonArray(None, argExprs.map(vonifyExpression).toVector))))
      }
      case IfPE(range, condition, thenBody, elseBody) => {
        VonObject(
          "If",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("condition", vonifyExpression(condition)),
            VonMember("thenBody", vonifyExpression(thenBody)),
            VonMember("elseBody", vonifyExpression(elseBody))))
      }
      case IndexPE(range, left, args) => {
        VonObject(
          "Index",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("left", vonifyExpression(left)),
            VonMember("args", VonArray(None, args.map(vonifyExpression).toVector))))
      }
//      case ResultPE(range, source) => {
//        VonObject(
//          "Result",
//          None,
//          Vector(
//            VonMember("range", vonifyRange(range)),
//            VonMember("source", vonifyExpression(source))))
//      }
      case ConstantIntPE(range, value, bits) => {
        VonObject(
          "ConstantInt",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("value", VonStr(value.toString)),
            VonMember("bits", vonifyOptional(bits, VonInt(_)))))
      }
      case AugmentPE(range, targetOwnership, inner) => {
        VonObject(
          "Augment",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("targetOwnership", vonifyOwnership(targetOwnership)),
            VonMember("inner", vonifyExpression(inner))))
      }
      case TransmigratePE(range, targetRegion, inner) => {
        VonObject(
          "Transmigrate",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("targetRegion", vonifyName(targetRegion)),
            VonMember("inner", vonifyExpression(inner))))
      }
      case LetPE(range, pattern, source) => {
        VonObject(
          "Let",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
//            VonMember("templateRules", vonifyOptional(templateRules, vonifyTemplateRules)),
            VonMember("pattern", vonifyPattern(pattern)),
            VonMember("source", vonifyExpression(source))))
      }
//      case BadLetPE(range) => {
//        VonObject(
//          "BadLet",
//          None,
//          Vector(
//            VonMember("range", vonifyRange(range))))
//      }
      case LookupPE(name, templateArgs) => {
        VonObject(
          "Lookup",
          None,
          Vector(
            VonMember("name", vonifyImpreciseName(name)),
            VonMember("templateArgs", vonifyOptional(templateArgs, vonifyTemplateArgs))))
      }
      case MutatePE(range, mutatee, expr) => {
        VonObject(
          "Mutate",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("source", vonifyExpression(expr)),
            VonMember("mutatee", vonifyExpression(mutatee))))
      }
      case ReturnPE(range, expr) => {
        VonObject(
          "Return",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("expr", vonifyExpression(expr))))
      }
      case BreakPE(range) => {
        VonObject(
          "Break",
          None,
          Vector(
            VonMember("range", vonifyRange(range))))
      }
      case ConstantStrPE(range, value) => {
        VonObject(
          "ConstantStr",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("value", VonStr(value))))
      }
      case StrInterpolatePE(range, parts) => {
        VonObject(
          "StrInterpolate",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("parts", VonArray(None, parts.toVector.map(vonifyExpression)))))
      }
      case AndPE(range, left, right) => {
        VonObject(
          "And",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("left", vonifyExpression(left)),
            VonMember("right", vonifyExpression(right))))
      }
      case OrPE(range, left, right) => {
        VonObject(
          "Or",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("left", vonifyExpression(left)),
            VonMember("right", vonifyExpression(right))))
      }
      case b @ BlockPE(_, _, _, _) => {
        vonifyBlock(b)
      }
      case c @ ConsecutorPE(_) => {
        vonifyConsecutor(c)
      }
      case DestructPE(range, inner) => {
        VonObject(
          "Destruct",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("inner", vonifyExpression(inner))))
      }
      case UnletPE(range, inner) => {
        VonObject(
          "Unlet",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("localName", vonifyImpreciseName(inner))))
      }
      case LambdaPE(captures, function) => {
        VonObject(
          "Lambda",
          None,
          Vector(
            VonMember("captures", vonifyOptional(captures, vonifyUnit)),
            VonMember("function", vonifyFunction(function))))
      }
      case MagicParamLookupPE(range) => {
        VonObject(
          "MagicParamLookup",
          None,
          Vector(
            VonMember("range", vonifyRange(range))))
      }
//      case MatchPE(range, condition, lambdas) => {
//        VonObject(
//          "Match",
//          None,
//          Vector(
//            VonMember("range", vonifyRange(range)),
//            VonMember("condition", vonifyExpression(condition)),
//            VonMember("lambdas", VonArray(None, lambdas.map(vonifyExpression).toVector))))
//      }
      case TuplePE(range, elements) => {
        VonObject(
          "Tuple",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("elements", VonArray(None, elements.map(vonifyExpression).toVector))))
      }
      case ca @ ConstructArrayPE(range, tyype, mutability, variability, size, initializingIndividualElements, args) => {
        vonifyConstructArray(ca)
      }
      case VoidPE(range) => {
        VonObject(
          "Void",
          None,
          Vector(
            VonMember("range", vonifyRange(range))))
      }
      case WhilePE(range, condition, body) => {
        VonObject(
          "While",
          None,
          Vector(
            VonMember("range", vonifyRange(range)),
            VonMember("condition", vonifyExpression(condition)),
            VonMember("body", vonifyBlock(body))))
      }
    }
  }

  private def vonifyArraySize(obj: IArraySizeP): VonObject = {
    obj match {
      case RuntimeSizedP => VonObject("RuntimeSized", None, Vector())
      case StaticSizedP(maybeSize) => {
        VonObject(
          "StaticSized",
          None,
          Vector(
            VonMember("size", vonifyOptional(maybeSize, vonifyTemplex))
          ))
      }
    }
  }

  private def vonifyConstructArray(ca: ConstructArrayPE): VonObject = {
    val ConstructArrayPE(range, tyype, mutability, variability, size, initializingIndividualElements, args) = ca

    VonObject(
      "ConstructArray",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("type", vonifyOptional(tyype, vonifyTemplex)),
        VonMember("mutability", vonifyOptional(mutability, vonifyTemplex)),
        VonMember("variability", vonifyOptional(variability, vonifyTemplex)),
        VonMember("size", vonifyArraySize(size)),
        VonMember("initializingIndividualElements", VonBool(initializingIndividualElements)),
        VonMember("args", VonArray(None, args.map(vonifyExpression).toVector))))
  }

  def vonifyTemplateArgs(thing: TemplateArgsP): IVonData = {
    val TemplateArgsP(range, args) = thing
    VonObject(
      "TemplateArgs",
      None,
      Vector(
        VonMember("range", vonifyRange(range)),
        VonMember("args", VonArray(None, args.map(vonifyTemplex).toVector))))
  }
}
*/
