// From Frontend/SimplifyingPass/src/dev/vale/simplifying/VonHammer.scala
//
// Per typing-pass `Compiler` precedent, `VonHammer` is not a Rust struct.
// Methods become `impl Hammer { ... }` blocks colocated here.

use crate::final_ast::ast::{
    EdgeH, FunctionH, FunctionRefH, IdH, InterfaceDefinitionH, InterfaceMethodH, PackageH,
    ProgramH, PrototypeH, RegionH, StructDefinitionH, StructMemberH,
};
use crate::final_ast::instructions::{ExpressionH, Local, VariableIdH};
use crate::final_ast::types::{
    CodeLocation, CoordH, InterfaceHT, KindHT, LocationH, Mutability, OwnershipH,
    RuntimeSizedArrayDefinitionHT, SimpleId, SimpleIdStep, StaticSizedArrayDefinitionHT, StructHT,
    Variability,
};
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::von::ast::{IVonData, VonMember};
use crate::simplifying::hammer::Hammer;

/*
package dev.vale.simplifying

import dev.vale.{CodeLocationS, PackageCoordinate, RangeS, vimpl}
import dev.vale.finalast._
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.{finalast => m}
import dev.vale.postparsing._
import dev.vale.von.{IVonData, VonArray, VonBool, VonFloat, VonInt, VonMember, VonObject, VonStr}

class VonHammer(nameHammer: NameHammer, typeHammer: TypeHammer) {
*/

// mig: fn vonify_program
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_program(&self, program: &ProgramH<'s, 'h>) -> IVonData {
        let ProgramH { packages } = program;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Program".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "packages".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: packages.package_coord_to_contents.iter().map(|(package_coord, paackage)| {
                            crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                                tyype: "Entry".to_string(),
                                id: None,
                                members: vec![
                                    crate::von::ast::VonMember {
                                        field_name: "packageCoordinate".to_string(),
                                        value: crate::von::ast::IVonData::Object(crate::simplifying::name_hammer::translate_package_coordinate(*package_coord)),
                                    },
                                    crate::von::ast::VonMember {
                                        field_name: "package".to_string(),
                                        value: self.vonify_package(**package_coord, paackage),
                                    },
                                ],
                            })
                        }).collect(),
                    }),
                },
            ],
        })
    }
}
/*
  def vonifyProgram(program: ProgramH): IVonData = {
    val ProgramH(packages) = program

    VonObject(
      "Program",
      None,
      Vector(
        VonMember(
          "packages",
          VonArray(
            None,
            packages.flatMap({ case (packageCoord, paackage) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("packageCoordinate", NameHammer.translatePackageCoordinate(packageCoord)),
                  VonMember("package", vonifyPackage(packageCoord, paackage))
                ))
            }).toVector))))
  }
*/

// mig: fn vonify_simple_id
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_simple_id(&self, simple_id: SimpleId<'s, 'h>) -> IVonData {
        IVonData::object(
            "Id".to_string(),
            vec![
                VonMember::new(
                    "steps".to_string(),
                    IVonData::array(
                        simple_id.steps.iter().map(|step| self.vonify_simple_id_step(*step)).collect(),
                    ),
                ),
            ],
        )
    }
}
/*
  def vonifySimpleId(simpleId: SimpleId): IVonData = {
    VonObject(
      "Id",
      None,
      Vector(
        VonMember(
          "steps",
          VonArray(
            None,
            simpleId.steps.map(vonifySimpleIdStep)))))
  }
*/

// mig: fn vonify_simple_id_step
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_simple_id_step(&self, step: SimpleIdStep<'s, 'h>) -> IVonData {
        let SimpleIdStep { name, template_args } = step;
        IVonData::object(
            "IdStep".to_string(),
            vec![
                VonMember::new("name".to_string(), IVonData::str(name.0.to_string())),
                VonMember::new(
                    "templateArgs".to_string(),
                    IVonData::array(
                        template_args.iter().map(|a| self.vonify_simple_id(*a)).collect(),
                    ),
                ),
            ],
        )
    }
}
/*
  def vonifySimpleIdStep(step: SimpleIdStep): IVonData = {
    val SimpleIdStep(name, templateArgs) = step
    VonObject(
      "IdStep",
      None,
      Vector(
        VonMember("name", VonStr(name)),
        VonMember("templateArgs", VonArray(None, templateArgs.map(vonifySimpleId)))))
  }
*/

// mig: fn vonify_package
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_package(
        &self,
        package_coord: PackageCoordinate<'s>,
        paackage: &PackageH<'s, 'h>,
    ) -> IVonData {
        let PackageH {
            interfaces, structs, functions, static_sized_arrays, runtime_sized_arrays,
            export_name_to_function, export_name_to_kind, prototype_to_extern, kind_to_extern,
        } = paackage;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Package".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "packageCoordinate".to_string(),
                    value: crate::von::ast::IVonData::Object(crate::simplifying::name_hammer::translate_package_coordinate(&package_coord)),
                },
                crate::von::ast::VonMember {
                    field_name: "interfaces".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: interfaces.iter().map(|i| self.vonify_interface_def(i)).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "structs".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: structs.iter().map(|s| self.vonify_struct(s)).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "functions".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: functions.iter().map(|f| self.vonify_function(f)).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "staticSizedArrays".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: static_sized_arrays.iter().map(|a| self.vonify_static_sized_array_definition(a)).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "runtimeSizedArrays".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: runtime_sized_arrays.iter().map(|a| self.vonify_runtime_sized_array_definition(a)).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "exportNameToFunction".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: export_name_to_function.iter().map(|(export_name, prototype)| {
                            crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                                tyype: "Entry".to_string(),
                                id: None,
                                members: vec![
                                    crate::von::ast::VonMember {
                                        field_name: "exportName".to_string(),
                                        value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: export_name.0.to_string() }),
                                    },
                                    crate::von::ast::VonMember {
                                        field_name: "prototype".to_string(),
                                        value: self.vonify_prototype(prototype),
                                    },
                                ],
                            })
                        }).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "exportNameToKind".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: export_name_to_kind.iter().map(|(export_name, kind)| {
                            crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                                tyype: "Entry".to_string(),
                                id: None,
                                members: vec![
                                    crate::von::ast::VonMember {
                                        field_name: "exportName".to_string(),
                                        value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: export_name.0.to_string() }),
                                    },
                                    crate::von::ast::VonMember {
                                        field_name: "kind".to_string(),
                                        value: self.vonify_kind(*kind),
                                    },
                                ],
                            })
                        }).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "prototypeToExtern".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: prototype_to_extern.iter().map(|(_, extern_)| {
                            let crate::final_ast::types::HamutsFunctionExtern { maybe_extern_name, prototype, simple_id } = extern_;
                            crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                                tyype: "ExternFunction".to_string(),
                                id: None,
                                members: vec![
                                    crate::von::ast::VonMember {
                                        field_name: "mangledName".to_string(),
                                        value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: maybe_extern_name.0.to_string() }),
                                    },
                                    crate::von::ast::VonMember {
                                        field_name: "id".to_string(),
                                        value: self.vonify_simple_id(*simple_id),
                                    },
                                    crate::von::ast::VonMember {
                                        field_name: "prototype".to_string(),
                                        value: self.vonify_prototype(prototype),
                                    },
                                ],
                            })
                        }).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "kindToExtern".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: kind_to_extern.iter().map(|(_, extern_)| {
                            let crate::final_ast::types::HamutsKindExtern { maybe_extern_name, kind, simple_id } = extern_;
                            crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                                tyype: "ExternKind".to_string(),
                                id: None,
                                members: vec![
                                    crate::von::ast::VonMember {
                                        field_name: "mangledName".to_string(),
                                        value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: maybe_extern_name.0.to_string() }),
                                    },
                                    crate::von::ast::VonMember {
                                        field_name: "id".to_string(),
                                        value: self.vonify_simple_id(*simple_id),
                                    },
                                    crate::von::ast::VonMember {
                                        field_name: "kind".to_string(),
                                        value: self.vonify_kind(*kind),
                                    },
                                ],
                            })
                        }).collect(),
                    }),
                },
            ],
        })
    }
}
/*
  def vonifyPackage(packageCoord: PackageCoordinate, paackage: PackageH): IVonData = {
    val PackageH(
      interfaces,
      structs,
      functions,
      staticSizedArrays,
      runtimeSizedArrays,
//      immDestructorsByKind,
      exportNameToFunction,
      exportNameToKind,
      externNameToFunction,
      externNameToKind,
    ) = paackage

    VonObject(
      "Package",
      None,
      Vector(
        VonMember("packageCoordinate", NameHammer.translatePackageCoordinate(packageCoord)),
        VonMember("interfaces", VonArray(None, interfaces.map(vonifyInterface).toVector)),
        VonMember("structs", VonArray(None, structs.map(vonfiyStruct).toVector)),
        VonMember("functions", VonArray(None, functions.map(vonifyFunction).toVector)),
        VonMember("staticSizedArrays", VonArray(None, staticSizedArrays.map(vonifyStaticSizedArrayDefinition).toVector)),
        VonMember("runtimeSizedArrays", VonArray(None, runtimeSizedArrays.map(vonifyRuntimeSizedArrayDefinition).toVector)),
//        VonMember(
//          "immDestructorsByKind",
//          VonArray(
//            None,
//            immDestructorsByKind.toVector.map({ case (kind, destructor) =>
//              VonObject(
//                "Entry",
//                None,
//                Vector(
//                  VonMember("kind", vonifyKind(kind)),
//                  VonMember("destructor", vonifyPrototype(destructor))))
//            }))),
        VonMember(
          "exportNameToFunction",
          VonArray(
            None,
            exportNameToFunction.toVector.map({ case (exportName, prototype) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("exportName", VonStr(exportName.str)),
                  VonMember("prototype", vonifyPrototype(prototype))))
            }))),
        VonMember(
          "exportNameToKind",
          VonArray(
            None,
            exportNameToKind.toVector.map({ case (exportName, kind) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("exportName", VonStr(exportName.str)),
                  VonMember("kind", vonifyKind(kind))))
            }))),
        VonMember(
          "prototypeToExtern",
          VonArray(
            None,
            externNameToFunction.toVector.map({ case (_, HamutsFunctionExtern(maybeExternName, prototype, simpleId)) =>
              VonObject(
                "ExternFunction",
                None,
                Vector(
                  VonMember("mangledName", VonStr(maybeExternName)),
                  VonMember("id", vonifySimpleId(simpleId)),
                  VonMember("prototype", vonifyPrototype(prototype))))
            }))),
        VonMember(
          "kindToExtern", // DO NOT SUBMIT rename plz
          VonArray(
            None,
            externNameToKind.toVector.map({ case (_, HamutsKindExtern(maybeExternName, kind, simpleId)) =>
              VonObject(
                "ExternKind",
                None,
                Vector(
                  VonMember("mangledName", VonStr(maybeExternName)),
                  VonMember("id", vonifySimpleId(simpleId)),
                  VonMember("kind", vonifyKind(kind))))
            })))))
  }
*/

// mig: fn vonify_region
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_region(&self, region: RegionH) -> IVonData {
        panic!("Unimplemented: vonify_region");
    }
}
/*
  def vonifyRegion(region: RegionH): IVonData = {
    val RegionH() = region

    VonObject(
      "Region",
      None,
      Vector())
//      Vector(
//        VonMember(
//          "kinds",
//          VonArray(
//            None,
//            kinds.map(vonifyKind).toVector))))
  }
*/

// mig: fn vonify_struct_h
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_struct_h(&self, r#ref: &StructHT<'s, 'h>) -> IVonData {
        let StructHT { id: full_name, .. } = *r#ref;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "StructId".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember { field_name: "name".to_string(), value: self.vonify_name(full_name) },
            ],
        })
    }
}
/*
  def vonifyStructH(ref: StructHT): IVonData = {
    val StructHT(fullName) = ref

    VonObject(
      "StructId",
      None,
      Vector(
        VonMember("name", vonifyName(fullName))))
  }
*/

// mig: fn vonify_interface (Scala overload — by InterfaceHT)
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_interface(&self, r#ref: &InterfaceHT<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_interface");
    }
}
/*
  def vonifyInterface(ref: InterfaceHT): IVonData = {
    val InterfaceHT(fullName) = ref

    VonObject(
      "InterfaceId",
      None,
      Vector(
        VonMember("name", vonifyName(fullName))))
  }
*/

// mig: fn vonify_interface_method
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_interface_method(
        &self,
        interface_method_h: &InterfaceMethodH<'s, 'h>,
    ) -> IVonData {
        panic!("Unimplemented: vonify_interface_method");
    }
}
/*
  def vonifyInterfaceMethod(interfaceMethodH: InterfaceMethodH): IVonData = {
    val InterfaceMethodH(prototype, virtualParamIndex) = interfaceMethodH

    VonObject(
      "InterfaceMethod",
      None,
      Vector(
        VonMember("prototype", vonifyPrototype(prototype)),
        VonMember("virtualParamIndex", VonInt(virtualParamIndex))))
  }
*/

// mig: fn vonify_interface_def (Scala overload — by InterfaceDefinitionH;
// disambiguated per overload-suffix pattern.)
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_interface_def(&self, interface: &InterfaceDefinitionH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_interface_def");
    }
}
/*
  def vonifyInterface(interface: InterfaceDefinitionH): IVonData = {
    val InterfaceDefinitionH(fullName, weakable, mutability, superInterfaces, prototypes) = interface

    VonObject(
      "Interface",
      None,
      Vector(
        VonMember("name", vonifyName(fullName)),
        VonMember("kind", vonifyInterface(interface.getRef)),
        VonMember("weakable", VonBool(weakable)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("superInterfaces", VonArray(None, superInterfaces.map(vonifyInterface).toVector)),
        VonMember("methods", VonArray(None, prototypes.map(vonifyInterfaceMethod).toVector))))
  }
*/

// mig: fn vonify_struct
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_struct(&self, r#struct: &StructDefinitionH<'s, 'h>) -> IVonData {
        let StructDefinitionH { id: full_name, weakable, extern_, mutability, edges, members } = *r#struct;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Struct".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember { field_name: "name".to_string(), value: self.vonify_name(full_name) },
                crate::von::ast::VonMember { field_name: "kind".to_string(), value: self.vonify_struct_h(r#struct.get_ref(self.interner)) },
                crate::von::ast::VonMember { field_name: "weakable".to_string(), value: crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: weakable }) },
                crate::von::ast::VonMember { field_name: "extern".to_string(), value: crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: extern_ }) },
                crate::von::ast::VonMember { field_name: "mutability".to_string(), value: self.vonify_mutability(mutability) },
                crate::von::ast::VonMember { field_name: "edges".to_string(), value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray { id: None, members: edges.iter().map(|edge| self.vonify_edge(edge)).collect() }) },
                crate::von::ast::VonMember { field_name: "members".to_string(), value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray { id: None, members: members.iter().map(|m| self.vonify_struct_member(m)).collect() }) },
            ],
        })
    }
}
/*
  def vonfiyStruct(struct: StructDefinitionH): IVonData = {
    val StructDefinitionH(fullName, weakable, extern, mutability, edges, members) = struct

    VonObject(
      "Struct",
      None,
      Vector(
        VonMember("name", vonifyName(fullName)),
        VonMember("kind", vonifyStructH(struct.getRef)),
        VonMember("weakable", VonBool(weakable)),
        VonMember("extern", VonBool(extern)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("edges", VonArray(None, edges.map(edge => vonifyEdge(edge)).toVector)),
        VonMember("members", VonArray(None, members.map(vonifyStructMember).toVector))))
  }
*/

// mig: fn vonify_mutability
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_mutability(&self, mutability: Mutability) -> IVonData {
        match mutability {
            Mutability::Immutable => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Immutable".to_string(), id: None, members: vec![] }),
            Mutability::Mutable => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Mutable".to_string(), id: None, members: vec![] }),
        }
    }
}
/*
  def vonifyMutability(mutability: Mutability): IVonData = {
    mutability match {
      case Immutable => VonObject("Immutable", None, Vector())
      case Mutable => VonObject("Mutable", None, Vector())
    }
  }
*/

// mig: fn vonify_location
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_location(&self, location: LocationH) -> IVonData {
        match location {
            LocationH::InlineH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Inline".to_string(), id: None, members: vec![] }),
            LocationH::YonderH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Yonder".to_string(), id: None, members: vec![] }),
        }
    }
}
/*
  def vonifyLocation(location: LocationH): IVonData = {
    location match {
      case InlineH => VonObject("Inline", None, Vector())
      case YonderH => VonObject("Yonder", None, Vector())
    }
  }
*/

// mig: fn vonify_variability
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_variability(&self, variability: Variability) -> IVonData {
        match variability {
            Variability::Varying => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Varying".to_string(), id: None, members: vec![] }),
            Variability::Final => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Final".to_string(), id: None, members: vec![] }),
        }
    }
}
/*
  def vonifyVariability(variability: Variability): IVonData = {
    variability match {
      case Varying => VonObject("Varying", None, Vector())
      case Final => VonObject("Final", None, Vector())
    }
  }
*/

// mig: fn vonify_prototype
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_prototype(&self, prototype: &PrototypeH<'s, 'h>) -> IVonData {
        let PrototypeH { id: full_name, params, return_type, _must_intern: _ } = prototype;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Prototype".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "name".to_string(),
                    value: self.vonify_name(full_name),
                },
                crate::von::ast::VonMember {
                    field_name: "params".to_string(),
                    value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                        id: None,
                        members: params.iter().map(|c| self.vonify_coord(*c)).collect(),
                    }),
                },
                crate::von::ast::VonMember {
                    field_name: "return".to_string(),
                    value: self.vonify_coord(*return_type),
                },
            ],
        })
    }
}
/*
  def vonifyPrototype(prototype: PrototypeH): IVonData = {
    val PrototypeH(fullName, params, returnType) = prototype

    VonObject(
      "Prototype",
      None,
      Vector(
        VonMember("name", vonifyName(fullName)),
        VonMember("params", VonArray(None, params.map(vonifyCoord).toVector)),
        VonMember("return", vonifyCoord(returnType))))
  }
*/

// mig: fn vonify_coord
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_coord(&self, coord: CoordH<'s, 'h>) -> IVonData {
        let CoordH { ownership, location, kind } = coord;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Ref".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "ownership".to_string(),
                    value: self.vonify_ownership(ownership),
                },
                crate::von::ast::VonMember {
                    field_name: "location".to_string(),
                    value: self.vonify_location(location),
                },
                crate::von::ast::VonMember {
                    field_name: "kind".to_string(),
                    value: self.vonify_kind(kind),
                },
            ],
        })
    }
}
/*
  def vonifyCoord(coord: CoordH[KindHT]): IVonData = {
    val CoordH(ownership, location, kind) = coord

    VonObject(
      "Ref",
      None,
      Vector(
        VonMember("ownership", vonifyOwnership(ownership)),
        VonMember("location", vonifyLocation(location)),
        VonMember("kind", vonifyKind(kind))))
  }
*/

// mig: fn vonify_edge
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_edge(&self, edge_h: &EdgeH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_edge");
    }
}
/*
  def vonifyEdge(edgeH: EdgeH): IVonData = {
    val EdgeH(struct, interface, structPrototypesByInterfacePrototype) = edgeH

    VonObject(
      "Edge",
      None,
      Vector(
        VonMember("structName", vonifyStructH(struct)),
        VonMember("interfaceName", vonifyInterface(interface)),
        VonMember(
          "methods",
          VonArray(
            None,
            structPrototypesByInterfacePrototype.toVector.map({ case (interfaceMethod, structPrototype) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("method", vonifyInterfaceMethod(interfaceMethod)),
                  VonMember("override", vonifyPrototype(structPrototype))))
            })))))
  }
*/

// mig: fn vonify_ownership
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_ownership(&self, ownership: OwnershipH) -> IVonData {
        match ownership {
            OwnershipH::OwnH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Own".to_string(), id: None, members: vec![] }),
            OwnershipH::ImmutableBorrowH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "ImmutableBorrow".to_string(), id: None, members: vec![] }),
            OwnershipH::MutableBorrowH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "MutableBorrow".to_string(), id: None, members: vec![] }),
            OwnershipH::ImmutableShareH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "ImmutableShare".to_string(), id: None, members: vec![] }),
            OwnershipH::MutableShareH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "MutableShare".to_string(), id: None, members: vec![] }),
            OwnershipH::WeakH => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Weak".to_string(), id: None, members: vec![] }),
        }
    }
}
/*
  def vonifyOwnership(ownership: OwnershipH): IVonData = {
    ownership match {
      case OwnH => VonObject("Own", None, Vector())
      case ImmutableBorrowH => VonObject("ImmutableBorrow", None, Vector())
      case MutableBorrowH => VonObject("MutableBorrow", None, Vector())
      case ImmutableShareH => VonObject("ImmutableShare", None, Vector())
      case MutableShareH => VonObject("MutableShare", None, Vector())
      case WeakH => VonObject("Weak", None, Vector())
    }
  }
*/

// mig: fn vonify_struct_member
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_struct_member(&self, struct_member_h: &StructMemberH<'s, 'h>) -> IVonData {
        let StructMemberH { name, variability, tyype } = *struct_member_h;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "StructMember".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember { field_name: "fullName".to_string(), value: self.vonify_name(name) },
                crate::von::ast::VonMember { field_name: "name".to_string(), value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: name.local_name.0.to_string() }) },
                crate::von::ast::VonMember { field_name: "variability".to_string(), value: self.vonify_variability(variability) },
                crate::von::ast::VonMember { field_name: "type".to_string(), value: self.vonify_coord(tyype) },
            ],
        })
    }
}
/*
  def vonifyStructMember(structMemberH: StructMemberH): IVonData = {
    val StructMemberH(name, variability, tyype) = structMemberH

    VonObject(
      "StructMember",
      None,
      Vector(
        VonMember("fullName", vonifyName(name)),
        VonMember("name", VonStr(name.localName)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("type", vonifyCoord(tyype))))
  }
*/

// mig: fn vonify_runtime_sized_array_definition
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_runtime_sized_array_definition(
        &self,
        rsa_def: &RuntimeSizedArrayDefinitionHT<'s, 'h>,
    ) -> IVonData {
        panic!("Unimplemented: vonify_runtime_sized_array_definition");
    }
}
/*
  def vonifyRuntimeSizedArrayDefinition(rsaDef: RuntimeSizedArrayDefinitionHT): IVonData = {
    val RuntimeSizedArrayDefinitionHT(name, mutability, elementType) = rsaDef
    VonObject(
      "RuntimeSizedArrayDefinition",
      None,
      Vector(
        VonMember("name", vonifyName(name)),
        VonMember("kind", vonifyKind(rsaDef.kind)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("elementType", vonifyCoord(elementType))))
  }
*/

// mig: fn vonify_static_sized_array_definition
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_static_sized_array_definition(
        &self,
        ssa_def: &StaticSizedArrayDefinitionHT<'s, 'h>,
    ) -> IVonData {
        panic!("Unimplemented: vonify_static_sized_array_definition");
    }
}
/*
  def vonifyStaticSizedArrayDefinition(ssaDef: StaticSizedArrayDefinitionHT): IVonData = {
    val StaticSizedArrayDefinitionHT(name, size, mutability, variability, elementType) = ssaDef
    VonObject(
      "StaticSizedArrayDefinition",
      None,
      Vector(
        VonMember("name", vonifyName(name)),
        VonMember("kind", vonifyKind(ssaDef.kind)),
        VonMember("size", VonInt(size)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("elementType", vonifyCoord(elementType))))
  }
*/

// mig: fn vonify_kind
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_kind(&self, kind: KindHT<'s, 'h>) -> IVonData {
        match kind {
            KindHT::NeverHT(_) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Never".to_string(), id: None, members: vec![] }),
            KindHT::IntHT(crate::final_ast::types::IntHT { bits }) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                tyype: "Int".to_string(),
                id: None,
                members: vec![
                    crate::von::ast::VonMember {
                        field_name: "bits".to_string(),
                        value: crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: bits as i64 }),
                    },
                ],
            }),
            KindHT::BoolHT(_) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Bool".to_string(), id: None, members: vec![] }),
            KindHT::StrHT(_) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Str".to_string(), id: None, members: vec![] }),
            KindHT::VoidHT(_) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Void".to_string(), id: None, members: vec![] }),
            KindHT::FloatHT(_) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "Float".to_string(), id: None, members: vec![] }),
            KindHT::InterfaceHT(ir) => self.vonify_interface(ir),
            KindHT::StructHT(sr) => self.vonify_struct_h(sr),
            KindHT::RuntimeSizedArrayHT(rsa) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                tyype: "RuntimeSizedArray".to_string(),
                id: None,
                members: vec![
                    crate::von::ast::VonMember {
                        field_name: "name".to_string(),
                        value: self.vonify_name(rsa.name),
                    },
                ],
            }),
            KindHT::StaticSizedArrayHT(crate::final_ast::types::StaticSizedArrayHT { id: name, _must_intern: _ }) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                tyype: "StaticSizedArray".to_string(),
                id: None,
                members: vec![
                    crate::von::ast::VonMember {
                        field_name: "name".to_string(),
                        value: self.vonify_name(name),
                    },
                ],
            }),
            KindHT::OpaqueHT(opaque) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                tyype: "Opaque".to_string(),
                id: None,
                members: vec![
                    crate::von::ast::VonMember {
                        field_name: "structId".to_string(),
                        value: self.vonify_name(opaque.struct_id),
                    },
                    crate::von::ast::VonMember {
                        field_name: "structSimpleId".to_string(),
                        value: self.vonify_simple_id(opaque.simple_id),
                    },
                ],
            }),
        }
    }
}
/*
  def vonifyKind(kind: KindHT): IVonData = {
    kind match {
      case NeverHT(_) => VonObject("Never", None, Vector())
      case IntHT(bits) => VonObject("Int", None, Vector(VonMember("bits", VonInt(bits))))
      case BoolHT() => VonObject("Bool", None, Vector())
      case StrHT() => VonObject("Str", None, Vector())
      case VoidHT() => VonObject("Void", None, Vector())
      case FloatHT() => VonObject("Float", None, Vector())
      case ir @ InterfaceHT(_) => vonifyInterface(ir)
      case sr @ StructHT(_) => vonifyStructH(sr)
      case RuntimeSizedArrayHT(name) => {
        VonObject(
          "RuntimeSizedArray",
          None,
          Vector(
            VonMember("name", vonifyName(name))))
      }
      case StaticSizedArrayHT(name) => {
        VonObject(
          "StaticSizedArray",
          None,
          Vector(
            VonMember("name", vonifyName(name))))
      }
      case OpaqueHT(_, structId, simpleId) => {
        VonObject(
          "Opaque",
          None,
          Vector(
            VonMember("structId", vonifyName(structId)),
            VonMember("structSimpleId", vonifySimpleId(simpleId))))
      }
    }
  }
*/

// mig: fn vonify_function
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_function(&self, function_h: &FunctionH<'s, 'h>) -> IVonData {
        let FunctionH { prototype, is_abstract: _, is_extern: _, attributes: _, body } = function_h;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Function".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "prototype".to_string(),
                    value: self.vonify_prototype(prototype),
                },
                crate::von::ast::VonMember {
                    field_name: "block".to_string(),
                    value: self.vonify_expression(*body),
                },
            ],
        })
    }
}
/*
  def vonifyFunction(functionH: FunctionH): IVonData = {
    val FunctionH(prototype, _, _, _, body) = functionH

    VonObject(
      "Function",
      None,
      Vector(
        VonMember("prototype", vonifyPrototype(prototype)),
        // TODO: rename block to body
        VonMember("block", vonifyExpression(body))))
  }
*/

// mig: fn vonify_expression
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_expression(&self, node: ExpressionH<'s, 'h>) -> IVonData {
        match node {
            ExpressionH::ConstantVoidH(_) => {
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "ConstantVoid".to_string(),
                    id: None,
                    members: vec![],
                })
            }
            ExpressionH::ConstantBoolH(c) => {
                let crate::final_ast::instructions::ConstantBoolH { value } = *c;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "ConstantBool".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "value".to_string(),
                            value: crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value }),
                        },
                    ],
                })
            }
            ExpressionH::ConstantIntH(c) => {
                let crate::final_ast::instructions::ConstantIntH { value, bits } = *c;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "ConstantInt".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "value".to_string(),
                            value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: value.to_string() }),
                        },
                        crate::von::ast::VonMember {
                            field_name: "bits".to_string(),
                            value: crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: bits as i64 }),
                        },
                    ],
                })
            }
            ExpressionH::ConstantStrH(_) => panic!("vonify_expression: ConstantStrH"),
            ExpressionH::ConstantF64H(c) => {
                let crate::final_ast::instructions::ConstantF64H { value } = *c;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "ConstantF64".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "value".to_string(),
                            value: crate::von::ast::IVonData::Float(crate::von::ast::VonFloat { value }),
                        },
                    ],
                })
            }
            ExpressionH::ArgumentH(a) => {
                let crate::final_ast::instructions::ArgumentH { result_type, argument_index } = *a;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Argument".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "resultType".to_string(),
                            value: self.vonify_coord(result_type),
                        },
                        crate::von::ast::VonMember {
                            field_name: "argumentIndex".to_string(),
                            value: crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: argument_index as i64 }),
                        },
                    ],
                })
            }
            ExpressionH::StackifyH(s) => {
                let crate::final_ast::instructions::StackifyH { source_expr, local, name } = *s;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Stackify".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "sourceExpr".to_string(),
                            value: self.vonify_expression(source_expr),
                        },
                        crate::von::ast::VonMember {
                            field_name: "local".to_string(),
                            value: self.vonify_local(local),
                        },
                        crate::von::ast::VonMember {
                            field_name: "knownLive".to_string(),
                            value: crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: false }),
                        },
                        crate::von::ast::VonMember {
                            field_name: "optName".to_string(),
                            value: self.vonify_optional(name, |n| self.vonify_name(n)),
                        },
                    ],
                })
            }
            ExpressionH::RestackifyH(_) => panic!("vonify_expression: RestackifyH"),
            ExpressionH::UnstackifyH(u) => {
                let crate::final_ast::instructions::UnstackifyH { local } = *u;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Unstackify".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "local".to_string(),
                            value: self.vonify_local(local),
                        },
                    ],
                })
            }
            ExpressionH::DestroyH(d) => {
                let crate::final_ast::instructions::DestroyH { struct_expression: struct_expr, local_types, local_indices: locals } = *d;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Destroy".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember { field_name: "structExpr".to_string(), value: self.vonify_expression(struct_expr) },
                        crate::von::ast::VonMember { field_name: "structType".to_string(), value: self.vonify_coord(struct_expr.result_type()) },
                        crate::von::ast::VonMember { field_name: "localTypes".to_string(), value: crate::von::ast::IVonData::array(local_types.iter().map(|lt| self.vonify_coord(*lt)).collect()) },
                        crate::von::ast::VonMember { field_name: "localIndices".to_string(), value: crate::von::ast::IVonData::array(locals.iter().map(|l| self.vonify_local(*l)).collect()) },
                        crate::von::ast::VonMember { field_name: "localsKnownLives".to_string(), value: crate::von::ast::IVonData::array(locals.iter().map(|_| crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: false })).collect()) },
                    ],
                })
            }
            ExpressionH::DestroyStaticSizedArrayIntoLocalsH(_) => panic!("vonify_expression: DestroyStaticSizedArrayIntoLocalsH"),
            ExpressionH::StructToInterfaceUpcastH(_) => panic!("vonify_expression: StructToInterfaceUpcastH"),
            ExpressionH::InterfaceToInterfaceUpcastH(_) => panic!("vonify_expression: InterfaceToInterfaceUpcastH"),
            ExpressionH::LocalStoreH(s) => {
                let crate::final_ast::instructions::LocalStoreH { local, source_expression: source_expr, local_name } = *s;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "LocalStore".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember { field_name: "local".to_string(), value: self.vonify_local(local) },
                        crate::von::ast::VonMember { field_name: "sourceExpr".to_string(), value: self.vonify_expression(source_expr) },
                        crate::von::ast::VonMember { field_name: "localName".to_string(), value: self.vonify_name(local_name) },
                        crate::von::ast::VonMember { field_name: "knownLive".to_string(), value: crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: false }) },
                    ],
                })
            }
            ExpressionH::LocalLoadH(l) => {
                let crate::final_ast::instructions::LocalLoadH { local, target_ownership, local_name } = *l;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "LocalLoad".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember { field_name: "local".to_string(), value: self.vonify_local(local) },
                        crate::von::ast::VonMember { field_name: "targetOwnership".to_string(), value: self.vonify_ownership(target_ownership) },
                        crate::von::ast::VonMember { field_name: "localName".to_string(), value: self.vonify_name(local_name) },
                    ],
                })
            }
            ExpressionH::MemberStoreH(_) => panic!("vonify_expression: MemberStoreH"),
            ExpressionH::MemberLoadH(ml) => {
                let crate::final_ast::instructions::MemberLoadH { struct_expression: struct_expr, member_index, expected_member_type, result_type, member_name } = *ml;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "MemberLoad".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember { field_name: "structExpr".to_string(), value: self.vonify_expression(struct_expr) },
                        crate::von::ast::VonMember { field_name: "structId".to_string(), value: self.vonify_struct_h(struct_expr.result_type().kind.expect_struct_h()) },
                        crate::von::ast::VonMember { field_name: "structType".to_string(), value: self.vonify_coord(struct_expr.result_type()) },
                        crate::von::ast::VonMember { field_name: "structKnownLive".to_string(), value: crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: false }) },
                        crate::von::ast::VonMember { field_name: "memberIndex".to_string(), value: crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: member_index as i64 }) },
                        crate::von::ast::VonMember { field_name: "targetOwnership".to_string(), value: self.vonify_ownership(result_type.ownership) },
                        crate::von::ast::VonMember { field_name: "expectedMemberType".to_string(), value: self.vonify_coord(expected_member_type) },
                        crate::von::ast::VonMember { field_name: "expectedResultType".to_string(), value: self.vonify_coord(result_type) },
                        crate::von::ast::VonMember { field_name: "memberName".to_string(), value: self.vonify_name(member_name) },
                    ],
                })
            }
            ExpressionH::NewArrayFromValuesH(_) => panic!("vonify_expression: NewArrayFromValuesH"),
            ExpressionH::StaticSizedArrayStoreH(_) => panic!("vonify_expression: StaticSizedArrayStoreH"),
            ExpressionH::RuntimeSizedArrayStoreH(_) => panic!("vonify_expression: RuntimeSizedArrayStoreH"),
            ExpressionH::RuntimeSizedArrayLoadH(_) => panic!("vonify_expression: RuntimeSizedArrayLoadH"),
            ExpressionH::StaticSizedArrayLoadH(_) => panic!("vonify_expression: StaticSizedArrayLoadH"),
            ExpressionH::CallH(c) => {
                let crate::final_ast::instructions::CallH { function: function_expr, args_expressions: args_exprs } = *c;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Call".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "function".to_string(),
                            value: self.vonify_prototype(function_expr),
                        },
                        crate::von::ast::VonMember {
                            field_name: "argExprs".to_string(),
                            value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray { id: None, members: args_exprs.iter().map(|e| self.vonify_expression(*e)).collect() }),
                        },
                    ],
                })
            }
            ExpressionH::ExternCallH(e) => {
                let crate::final_ast::instructions::ExternCallH { function: function_expr, args_expressions: args_exprs } = *e;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "ExternCall".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember { field_name: "function".to_string(), value: self.vonify_prototype(function_expr) },
                        crate::von::ast::VonMember { field_name: "argExprs".to_string(), value: crate::von::ast::IVonData::array(args_exprs.iter().map(|a| self.vonify_expression(*a)).collect()) },
                        crate::von::ast::VonMember { field_name: "argTypes".to_string(), value: crate::von::ast::IVonData::array(args_exprs.iter().map(|a| self.vonify_coord(a.result_type())).collect()) },
                    ],
                })
            }
            ExpressionH::InterfaceCallH(_) => panic!("vonify_expression: InterfaceCallH"),
            ExpressionH::IfH(i) => {
                let crate::final_ast::instructions::IfH { condition_block, then_block, else_block, common_supertype } = *i;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "If".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember { field_name: "conditionBlock".to_string(), value: self.vonify_expression(condition_block) },
                        crate::von::ast::VonMember { field_name: "thenBlock".to_string(), value: self.vonify_expression(then_block) },
                        crate::von::ast::VonMember { field_name: "thenResultType".to_string(), value: self.vonify_coord(then_block.result_type()) },
                        crate::von::ast::VonMember { field_name: "elseBlock".to_string(), value: self.vonify_expression(else_block) },
                        crate::von::ast::VonMember { field_name: "elseResultType".to_string(), value: self.vonify_coord(else_block.result_type()) },
                        crate::von::ast::VonMember { field_name: "commonSupertype".to_string(), value: self.vonify_coord(common_supertype) },
                    ],
                })
            }
            ExpressionH::WhileH(_) => panic!("vonify_expression: WhileH"),
            ExpressionH::ConsecutorH(c) => {
                let crate::final_ast::instructions::ConsecutorH { exprs: nodes } = *c;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Consecutor".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "exprs".to_string(),
                            value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                                id: None,
                                members: nodes.iter().map(|node| self.vonify_expression(*node)).collect(),
                            }),
                        },
                    ],
                })
            }
            ExpressionH::BlockH(b) => {
                let crate::final_ast::instructions::BlockH { inner } = *b;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Block".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "innerExpr".to_string(),
                            value: self.vonify_expression(inner),
                        },
                        crate::von::ast::VonMember {
                            field_name: "innerType".to_string(),
                            value: self.vonify_coord(inner.result_type()),
                        },
                    ],
                })
            }
            ExpressionH::MutabilifyH(_) => panic!("vonify_expression: MutabilifyH"),
            ExpressionH::ImmutabilifyH(_) => panic!("vonify_expression: ImmutabilifyH"),
            ExpressionH::ReturnH(r) => {
                let crate::final_ast::instructions::ReturnH { source_expression } = *r;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Return".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "sourceExpr".to_string(),
                            value: self.vonify_expression(source_expression),
                        },
                        crate::von::ast::VonMember {
                            field_name: "sourceType".to_string(),
                            value: self.vonify_coord(source_expression.result_type()),
                        },
                    ],
                })
            }
            ExpressionH::NewImmRuntimeSizedArrayH(_) => panic!("vonify_expression: NewImmRuntimeSizedArrayH"),
            ExpressionH::NewMutRuntimeSizedArrayH(_) => panic!("vonify_expression: NewMutRuntimeSizedArrayH"),
            ExpressionH::PushRuntimeSizedArrayH(_) => panic!("vonify_expression: PushRuntimeSizedArrayH"),
            ExpressionH::PopRuntimeSizedArrayH(_) => panic!("vonify_expression: PopRuntimeSizedArrayH"),
            ExpressionH::StaticArrayFromCallableH(_) => panic!("vonify_expression: StaticArrayFromCallableH"),
            ExpressionH::DestroyStaticSizedArrayIntoFunctionH(_) => panic!("vonify_expression: DestroyStaticSizedArrayIntoFunctionH"),
            ExpressionH::DestroyImmRuntimeSizedArrayH(_) => panic!("vonify_expression: DestroyImmRuntimeSizedArrayH"),
            ExpressionH::DestroyMutRuntimeSizedArrayH(_) => panic!("vonify_expression: DestroyMutRuntimeSizedArrayH"),
            ExpressionH::BreakH(_) => panic!("vonify_expression: BreakH"),
            ExpressionH::NewStructH(n) => {
                let crate::final_ast::instructions::NewStructH { source_expressions, target_member_names, result_type } = *n;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "NewStruct".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember { field_name: "sourceExprs".to_string(), value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray { id: None, members: source_expressions.iter().map(|e| self.vonify_expression(*e)).collect() }) },
                        crate::von::ast::VonMember { field_name: "memberNames".to_string(), value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray { id: None, members: target_member_names.iter().map(|n| self.vonify_name(n)).collect() }) },
                        crate::von::ast::VonMember { field_name: "resultType".to_string(), value: self.vonify_coord(result_type) },
                    ],
                })
            }
            ExpressionH::ArrayLengthH(_) => panic!("vonify_expression: ArrayLengthH"),
            ExpressionH::ArrayCapacityH(_) => panic!("vonify_expression: ArrayCapacityH"),
            ExpressionH::BorrowToWeakH(_) => panic!("vonify_expression: BorrowToWeakH"),
            ExpressionH::IsSameInstanceH(_) => panic!("vonify_expression: IsSameInstanceH"),
            ExpressionH::AsSubtypeH(_) => panic!("vonify_expression: AsSubtypeH"),
            ExpressionH::LockWeakH(_) => panic!("vonify_expression: LockWeakH"),
            ExpressionH::DiscardH(d) => {
                let crate::final_ast::instructions::DiscardH { source_expression: source_expr } = *d;
                crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                    tyype: "Discard".to_string(),
                    id: None,
                    members: vec![
                        crate::von::ast::VonMember {
                            field_name: "sourceExpr".to_string(),
                            value: self.vonify_expression(source_expr),
                        },
                        crate::von::ast::VonMember {
                            field_name: "sourceResultType".to_string(),
                            value: self.vonify_coord(source_expr.result_type()),
                        },
                    ],
                })
            }
            ExpressionH::PreCheckBorrowH(_) => panic!("vonify_expression: PreCheckBorrowH"),
        }
    }
}
/*
Guardian: temp-disable: SPDMX — Rust narrowing recovery. Scala's MemberLoadH.structExpression has type ExpressionH[StructHT] which statically narrows the kind, so vonifyStructH(structExpr.resultType.kind) typechecks. Rust's ExpressionH erases that, so the match-at-call-site recovers the narrowing. Identical pattern at von_hammer.rs:879 inside vonify_kind. — FrontendRust/guardian-logs/request-669-1780499551710/hook-669/vonify_expression--994.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def vonifyExpression(node: ExpressionH[KindHT]): IVonData = {
    node match {
      case ConstantVoidH() => {
        VonObject("ConstantVoid", None, Vector())
      }
      case ConstantBoolH(value) => {
        VonObject(
          "ConstantBool",
          None,
          Vector(
            VonMember("value", VonBool(value))))
      }
      case ConstantIntH(value, bits) => {
        VonObject(
          "ConstantInt",
          None,
          Vector(
            VonMember("value", VonStr(value.toString)),
            VonMember("bits", VonInt(bits))))
      }
      case ConstantStrH(value) => {
        VonObject(
          "ConstantStr",
          None,
          Vector(
            VonMember("value", VonStr(value))))
      }
      case ConstantF64H(value) => {
        VonObject(
          "ConstantF64",
          None,
          Vector(
            VonMember("value", VonFloat(value))))
      }
      case ArrayCapacityH(sourceExpr) => {
        VonObject(
          "ArrayCapacity",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false))))
      }
      case ArrayLengthH(sourceExpr) => {
        VonObject(
          "ArrayLength",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false))))
      }
      case wa @ BorrowToWeakH(sourceExpr) => {
        VonObject(
          "BorrowToWeak",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKind", vonifyKind(sourceExpr.resultType.kind)),
            VonMember("resultType", vonifyCoord(wa.resultType)),
            VonMember("resultKind", vonifyKind(wa.resultType.kind))))
      }
      case AsSubtypeH(sourceExpr, targetType, resultResultType, okConstructor, errConstructor) => {
        VonObject(
          "AsSubtype",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("targetKind", vonifyKind(targetType)),
            VonMember("okConstructor", vonifyPrototype(okConstructor)),
            VonMember("okType", vonifyCoord(okConstructor.returnType)),
            VonMember("okKind", vonifyKind(okConstructor.returnType.kind)),
            VonMember("errConstructor", vonifyPrototype(errConstructor)),
            VonMember("errType", vonifyCoord(errConstructor.returnType)),
            VonMember("errKind", vonifyKind(errConstructor.returnType.kind)),
            VonMember("resultResultType", vonifyCoord(resultResultType)),
            VonMember("resultResultKind", vonifyKind(resultResultType.kind))))
      }
      case LockWeakH(sourceExpr, resultOptType, someConstructor, noneConstructor) => {
        VonObject(
          "LockWeak",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("someConstructor", vonifyPrototype(someConstructor)),
            VonMember("someType", vonifyCoord(someConstructor.returnType)),
            VonMember("someKind", vonifyKind(someConstructor.returnType.kind)),
            VonMember("noneConstructor", vonifyPrototype(noneConstructor)),
            VonMember("noneType", vonifyCoord(noneConstructor.returnType)),
            VonMember("noneKind", vonifyKind(noneConstructor.returnType.kind)),
            VonMember("resultOptType", vonifyCoord(resultOptType)),
            VonMember("resultOptKind", vonifyKind(resultOptType.kind))))
      }
      case ReturnH(sourceExpr) => {
        VonObject(
          "Return",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType))))
      }
      case BreakH() => {
        VonObject(
          "Break",
          None,
          Vector())
      }
      case DiscardH(sourceExpr) => {
        VonObject(
          "Discard",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceResultType", vonifyCoord(sourceExpr.resultType))))
      }
      case PreCheckBorrowH(sourceExpr) => {
        VonObject(
          "PreCheckBorrow",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceResultType", vonifyCoord(sourceExpr.resultType))))
      }
      case ArgumentH(resultReference, argumentIndex) => {
        VonObject(
          "Argument",
          None,
          Vector(
            VonMember("resultType", vonifyCoord(resultReference)),
            VonMember("argumentIndex", VonInt(argumentIndex))))
      }
      case NewArrayFromValuesH(resultType, sourceExprs) => {
        VonObject(
          "NewArrayFromValues",
          None,
          Vector(
            VonMember("sourceExprs", VonArray(None, sourceExprs.map(vonifyExpression).toVector)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("resultKind", vonifyKind(resultType.kind))))
      }
      case NewStructH(sourceExprs, targetMemberNames, resultType) => {
        VonObject(
          "NewStruct",
          None,
          Vector(
            VonMember(
              "sourceExprs",
              VonArray(None, sourceExprs.map(vonifyExpression).toVector)),
            VonMember(
              "memberNames",
              VonArray(None, targetMemberNames.map(n => vonifyName(n)).toVector)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case StackifyH(sourceExpr, local, name) => {
        VonObject(
          "Stackify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("local", vonifyLocal(local)),
            VonMember("knownLive", VonBool(false)),
            VonMember("optName", vonifyOptional[IdH](name, n => vonifyName(n)))))
      }
      case RestackifyH(sourceExpr, local, name) => {
        VonObject(
          "Restackify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("local", vonifyLocal(local)),
            VonMember("knownLive", VonBool(false)),
            VonMember("optName", vonifyOptional[IdH](name, n => vonifyName(n)))))
      }
      case UnstackifyH(local) => {
        VonObject(
          "Unstackify",
          None,
          Vector(
            VonMember("local", vonifyLocal(local))))
      }
      case DestroyStaticSizedArrayIntoFunctionH(arrayExpr, consumerExpr, consumerMethod, arrayElementType, arraySize) => {
        VonObject(
          "DestroyStaticSizedArrayIntoFunction",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("consumerExpr", vonifyExpression(consumerExpr)),
            VonMember("consumerType", vonifyCoord(consumerExpr.resultType)),
            VonMember("consumerMethod", vonifyPrototype(consumerMethod)),
            VonMember("consumerKnownLive", VonBool(false)),
            VonMember("arrayElementType", vonifyCoord(arrayElementType)),
            VonMember("arraySize", VonInt(arraySize))))
      }
      case DestroyStaticSizedArrayIntoLocalsH(structExpr, localTypes, localIndices) => {
        VonObject(
          "DestroyStaticSizedArrayIntoLocals",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(structExpr)),
            VonMember("arrayType", vonifyCoord(structExpr.resultType)),
            VonMember(
              "localTypes",
              VonArray(None, localTypes.map(localType => vonifyCoord(localType)).toVector)),
            VonMember(
              "localIndices",
              VonArray(None, localIndices.map(local => vonifyLocal(local))))))
      }
      case DestroyH(structExpr, localTypes, locals) => {
        VonObject(
          "Destroy",
          None,
          Vector(
            VonMember("structExpr", vonifyExpression(structExpr)),
            VonMember("structType", vonifyCoord(structExpr.resultType)),
            VonMember(
              "localTypes",
              VonArray(None, localTypes.map(localType => vonifyCoord(localType)).toVector)),
            VonMember(
              "localIndices",
              VonArray(None, locals.map(local => vonifyLocal(local)))),
            VonMember(
              "localsKnownLives",
              VonArray(None, locals.map(local => VonBool(false))))))
      }
      case PushRuntimeSizedArrayH(arrayExpr, newcomerExpr) => {
        VonObject(
          "PushRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("newcomerExpr", vonifyExpression(newcomerExpr)),
            VonMember("newcomerType", vonifyCoord(newcomerExpr.resultType)),
            VonMember("newcomerKind", vonifyKind(newcomerExpr.resultType.kind)),
            VonMember("consumerKnownLive", VonBool(false))))
      }
      case PopRuntimeSizedArrayH(arrayExpr, arrayElementType) => {
        VonObject(
          "PopRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayElementType", vonifyCoord(arrayElementType)),
            VonMember("consumerKnownLive", VonBool(false))))
      }
      case DestroyMutRuntimeSizedArrayH(arrayExpr) => {
        VonObject(
          "DestroyMutRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind))))
      }
      case DestroyImmRuntimeSizedArrayH(arrayExpr, consumerExpr, consumerMethod, arrayElementType) => {
        VonObject(
          "DestroyImmRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("consumerExpr", vonifyExpression(consumerExpr)),
            VonMember("consumerType", vonifyCoord(consumerExpr.resultType)),
            VonMember("consumerKind", vonifyKind(consumerExpr.resultType.kind)),
            VonMember("consumerMethod", vonifyPrototype(consumerMethod)),
            VonMember("arrayElementType", vonifyCoord(arrayElementType)),
            VonMember("consumerKnownLive", VonBool(false))))
      }
      case si @ StructToInterfaceUpcastH(sourceExpr, targetInterfaceRef) => {
        VonObject(
          "StructToInterfaceUpcast",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceStructType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceStructKind", vonifyStructH(sourceExpr.resultType.kind)),
            VonMember("targetInterfaceType", vonifyCoord(si.resultType)),
            VonMember("targetInterfaceKind", vonifyInterface(targetInterfaceRef))))
      }
      case InterfaceToInterfaceUpcastH(sourceExpr, targetInterfaceRef) => {
        vimpl()
      }
      case LocalStoreH(local, sourceExpr,localName) => {
        VonObject(
          "LocalStore",
          None,
          Vector(
            VonMember("local", vonifyLocal(local)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("localName", vonifyName(localName)),
            VonMember("knownLive", VonBool(false))))
      }
      case LocalLoadH(local, targetOwnership, localName) => {
        VonObject(
          "LocalLoad",
          None,
          Vector(
            VonMember("local", vonifyLocal(local)),
            VonMember("targetOwnership", vonifyOwnership(targetOwnership)),
            VonMember("localName", vonifyName(localName))))
      }
      case MemberStoreH(resultType, structExpr, memberIndex, sourceExpr, memberName) => {
        VonObject(
          "MemberStore",
          None,
          Vector(
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("structExpr", vonifyExpression(structExpr)),
            VonMember("structType", vonifyCoord(structExpr.resultType)),
            VonMember("structKnownLive", VonBool(false)),
            VonMember("memberIndex", VonInt(memberIndex)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("memberName", vonifyName(memberName))))
      }
      case ml @ MemberLoadH(structExpr, memberIndex, expectedMemberType, resultType, memberName) => {
        VonObject(
          "MemberLoad",
          None,
          Vector(
            VonMember("structExpr", vonifyExpression(structExpr)),
            VonMember("structId", vonifyStructH(structExpr.resultType.kind)),
            VonMember("structType", vonifyCoord(structExpr.resultType)),
            VonMember("structKnownLive", VonBool(false)),
            VonMember("memberIndex", VonInt(memberIndex)),
            VonMember("targetOwnership", vonifyOwnership(resultType.ownership)),
            VonMember("expectedMemberType", vonifyCoord(expectedMemberType)),
            VonMember("expectedResultType", vonifyCoord(resultType)),
            VonMember("memberName", vonifyName(memberName))))
      }
      case StaticSizedArrayStoreH(arrayExpr, indexExpr, sourceExpr, resultType) => {
        VonObject(
          "StaticSizedArrayStore",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case RuntimeSizedArrayStoreH(arrayExpr, indexExpr, sourceExpr, resultType) => {
        VonObject(
          "RuntimeSizedArrayStore",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("indexType", vonifyCoord(indexExpr.resultType)),
            VonMember("indexKind", vonifyKind(indexExpr.resultType.kind)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKind", vonifyKind(sourceExpr.resultType.kind)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case rsal @ RuntimeSizedArrayLoadH(arrayExpr, indexExpr, targetOwnership, expectedElementType, resultType) => {
        VonObject(
          "RuntimeSizedArrayLoad",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("indexType", vonifyCoord(indexExpr.resultType)),
            VonMember("indexKind", vonifyKind(indexExpr.resultType.kind)),
            VonMember("resultType", vonifyCoord(rsal.resultType)),
            VonMember("targetOwnership", vonifyOwnership(targetOwnership)),
            VonMember("expectedElementType", vonifyCoord(expectedElementType)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case NewMutRuntimeSizedArrayH(capacityExpr, elementType, resultType) => {
        VonObject(
          "NewMutRuntimeSizedArray",
          None,
          Vector(
            VonMember("capacityExpr", vonifyExpression(capacityExpr)),
            VonMember("capacityType", vonifyCoord(capacityExpr.resultType)),
            VonMember("capacityKind", vonifyKind(capacityExpr.resultType.kind)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("elementType", vonifyCoord(elementType))))
      }
      case NewImmRuntimeSizedArrayH(sizeExpr, generatorExpr, generatorMethod, elementType, resultType) => {
        VonObject(
          "NewImmRuntimeSizedArray",
          None,
          Vector(
            VonMember("sizeExpr", vonifyExpression(sizeExpr)),
            VonMember("sizeType", vonifyCoord(sizeExpr.resultType)),
            VonMember("sizeKind", vonifyKind(sizeExpr.resultType.kind)),
            VonMember("generatorExpr", vonifyExpression(generatorExpr)),
            VonMember("generatorType", vonifyCoord(generatorExpr.resultType)),
            VonMember("generatorKind", vonifyKind(generatorExpr.resultType.kind)),
            VonMember("generatorMethod", vonifyPrototype(generatorMethod)),
            VonMember("generatorKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("elementType", vonifyCoord(resultType))))
      }
      case StaticArrayFromCallableH(generatorExpr, generatorMethod, elementType, resultType) => {
        VonObject(
          "StaticArrayFromCallable",
          None,
          Vector(
            VonMember("generatorExpr", vonifyExpression(generatorExpr)),
            VonMember("generatorType", vonifyCoord(generatorExpr.resultType)),
            VonMember("generatorKind", vonifyKind(generatorExpr.resultType.kind)),
            VonMember("generatorMethod", vonifyPrototype(generatorMethod)),
            VonMember("generatorKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("elementType", vonifyCoord(resultType))))
      }
      case ssal @ StaticSizedArrayLoadH(arrayExpr, indexExpr, targetOwnership, expectedElementType, arraySize, resultType) => {
        VonObject(
          "StaticSizedArrayLoad",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("resultType", vonifyCoord(ssal.resultType)),
            VonMember("targetOwnership", vonifyOwnership(targetOwnership)),
            VonMember("expectedElementType", vonifyCoord(expectedElementType)),
            VonMember("arraySize", VonInt(arraySize)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case ExternCallH(functionExpr, argsExprs) => {
        VonObject(
          "ExternCall",
          None,
          Vector(
            VonMember("function", vonifyPrototype(functionExpr)),
            VonMember("argExprs", VonArray(None, argsExprs.toVector.map(vonifyExpression))),
            VonMember("argTypes", VonArray(None, argsExprs.toVector.map(_.resultType).map(vonifyCoord)))))
      }
      case CallH(functionExpr, argsExprs) => {
        VonObject(
          "Call",
          None,
          Vector(
            VonMember("function", vonifyPrototype(functionExpr)),
            VonMember("argExprs", VonArray(None, argsExprs.toVector.map(vonifyExpression)))))
      }
      case InterfaceCallH(argsExprs, virtualParamIndex, interfaceRefH, indexInEdge, functionType) => {
        VonObject(
          "InterfaceCall",
          None,
          Vector(
            VonMember("argExprs", VonArray(None, argsExprs.toVector.map(vonifyExpression))),
            VonMember("virtualParamIndex", VonInt(virtualParamIndex)),
            VonMember("interfaceRef", vonifyInterface(interfaceRefH)),
            VonMember("indexInEdge", VonInt(indexInEdge)),
            VonMember("functionType", vonifyPrototype(functionType))))
      }
      case IfH(conditionBlock, thenBlock, elseBlock, commonSupertype) => {
        VonObject(
          "If",
          None,
          Vector(
            VonMember("conditionBlock", vonifyExpression(conditionBlock)),
            VonMember("thenBlock", vonifyExpression(thenBlock)),
            VonMember("thenResultType", vonifyCoord(thenBlock.resultType)),
            VonMember("elseBlock", vonifyExpression(elseBlock)),
            VonMember("elseResultType", vonifyCoord(elseBlock.resultType)),
            VonMember("commonSupertype", vonifyCoord(commonSupertype))))
      }
      case WhileH(bodyBlock) => {
        VonObject(
          "While",
          None,
          Vector(
            VonMember("bodyBlock", vonifyExpression(bodyBlock))))
      }
      case ConsecutorH(nodes) => {
        VonObject(
          "Consecutor",
          None,
          Vector(
            VonMember("exprs", VonArray(None, nodes.map(node => vonifyExpression(node)).toVector))))
      }
      case m @ MutabilifyH(inner) => {
        VonObject(
          "Mutabilify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(inner)),
            VonMember("sourceType", vonifyCoord(inner.resultType)),
            VonMember("resultType", vonifyCoord(m.resultType))))
      }
      case m @ ImmutabilifyH(inner) => {
        VonObject(
          "Immutabilify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(inner)),
            VonMember("sourceType", vonifyCoord(inner.resultType)),
            VonMember("resultType", vonifyCoord(m.resultType))))
      }
      case BlockH(inner) => {
        VonObject(
          "Block",
          None,
          Vector(
            VonMember("innerExpr", vonifyExpression(inner)),
            VonMember("innerType", vonifyCoord(inner.resultType))))
      }
      case IsSameInstanceH(left, right) => {
        VonObject(
          "Is",
          None,
          Vector(
            VonMember("leftExpr", vonifyExpression(left)),
            VonMember("leftExprType", vonifyCoord(left.resultType)),
            VonMember("rightExpr", vonifyExpression(right)),
            VonMember("rightExprType", vonifyCoord(right.resultType))))
      }
    }
  }
*/

// mig: fn vonify_function_ref
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_function_ref(&self, r#ref: &FunctionRefH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_function_ref");
    }
}
/*
  def vonifyFunctionRef(ref: FunctionRefH): IVonData = {
    val FunctionRefH(prototype) = ref

    VonObject(
      "FunctionRef",
      None,
      Vector(
        VonMember("prototype", vonifyPrototype(prototype))))
  }
*/

// mig: fn vonify_local
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_local(&self, local: Local<'s, 'h>) -> IVonData {
        let Local { id, variability, type_h: tyype } = local;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Local".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "id".to_string(),
                    value: self.vonify_variable_id(id),
                },
                crate::von::ast::VonMember {
                    field_name: "variability".to_string(),
                    value: self.vonify_variability(variability),
                },
                crate::von::ast::VonMember {
                    field_name: "type".to_string(),
                    value: self.vonify_coord(tyype),
                },
            ],
        })
    }
}
/*
  def vonifyLocal(local: Local): IVonData = {
    val Local(id, variability, tyype) = local

    VonObject(
      "Local",
      None,
      Vector(
        VonMember("id", vonifyVariableId(id)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("type", vonifyCoord(tyype))))
  }
*/

// mig: fn vonify_variable_id
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_variable_id(&self, id: VariableIdH<'s, 'h>) -> IVonData {
        let VariableIdH { number, height: _, name: maybe_name } = id;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "VariableId".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "number".to_string(),
                    value: crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: number as i64 }),
                },
                crate::von::ast::VonMember {
                    field_name: "height".to_string(),
                    value: crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: number as i64 }),
                },
                crate::von::ast::VonMember {
                    field_name: "optName".to_string(),
                    value: self.vonify_optional(maybe_name, |x| self.vonify_name(x)),
                },
            ],
        })
    }
}
/*
  def vonifyVariableId(id: VariableIdH): IVonData = {
    val VariableIdH(number, height, maybeName) = id

    VonObject(
      "VariableId",
      None,
      Vector(
        VonMember("number", VonInt(number)),
        VonMember("height", VonInt(number)),
        VonMember(
          "optName",
          vonifyOptional[IdH](maybeName, x => vonifyName(x)))))
  }
*/

// mig: fn vonify_optional
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_optional<I>(&self, opt: Option<I>, func: impl Fn(I) -> IVonData) -> IVonData {
        match opt {
            None => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "None".to_string(), id: None, members: vec![] }),
            Some(value) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
                tyype: "Some".to_string(),
                id: None,
                members: vec![
                    crate::von::ast::VonMember {
                        field_name: "value".to_string(),
                        value: func(value),
                    },
                ],
            }),
        }
    }
}
/*
  def vonifyOptional[I](opt: Option[I], func: (I) => IVonData): IVonData = {
    opt match {
      case None => VonObject("None", None, Vector())
      case Some(value) => VonObject("Some", None, Vector(VonMember("value", func(value))))
    }
  }
*/

// mig: fn vonify_code_location
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_code_location(&self, code_location: CodeLocation) -> IVonData {
        panic!("Unimplemented: vonify_code_location");
    }
}
/*
  def vonifyCodeLocation(codeLocation: CodeLocation): IVonData = {
    val CodeLocation(file, offset) = codeLocation
    VonObject(
      "CodeLocation",
      None,
      Vector(
        VonMember("file", NameHammer.translateFileCoordinate(file)),
        VonMember("offset", VonInt(offset))))
  }
*/

// mig: fn vonify_code_location2
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_code_location2(&self, code_location: &CodeLocationS<'s>) -> IVonData {
        panic!("Unimplemented: vonify_code_location2");
    }
}
/*
  def vonifyCodeLocation2(codeLocation: CodeLocationS): IVonData = {
    val CodeLocationS(file, offset) = codeLocation
    VonObject(
      "CodeLocation",
      None,
      Vector(
        VonMember("file", NameHammer.translateFileCoordinate(file)),
        VonMember("offset", VonInt(offset))))
  }
*/

// mig: fn vonify_ranges
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_ranges(&self, ranges: &[RangeS<'s>]) -> IVonData {
        panic!("Unimplemented: vonify_ranges");
    }
}
/*
  def vonifyRanges(ranges: List[RangeS]): IVonData = {
    VonArray(
      None,
      ranges.map(vonifyRange).toVector)
  }
*/

// mig: fn vonify_range
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_range(&self, range: RangeS<'s>) -> IVonData {
        panic!("Unimplemented: vonify_range");
    }
}
/*
  def vonifyRange(range: RangeS): IVonData = {
    val RangeS(begin, end) = range
    VonObject(
      "Range",
      None,
      Vector(
        VonMember("begin", NameHammer.translateCodeLocation(begin)),
        VonMember("end", NameHammer.translateCodeLocation(end))))
  }
*/

// mig: fn vonify_name
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_name(&self, h: &IdH<'s, 'h>) -> IVonData {
        let IdH { local_name, package_coordinate, shortened_name, fully_qualified_name: _, _must_intern: _, _phantom_h: _ } = h;
        crate::von::ast::IVonData::Object(crate::von::ast::VonObject {
            tyype: "Name".to_string(),
            id: None,
            members: vec![
                crate::von::ast::VonMember {
                    field_name: "localName".to_string(),
                    value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: local_name.0.to_string() }),
                },
                crate::von::ast::VonMember {
                    field_name: "packageCoordinate".to_string(),
                    value: crate::von::ast::IVonData::Object(crate::simplifying::name_hammer::translate_package_coordinate(package_coordinate)),
                },
                crate::von::ast::VonMember {
                    field_name: "shortenedName".to_string(),
                    value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: shortened_name.0.to_string() }),
                },
            ],
        })
    }
}
/*
  def vonifyName(h: IdH): IVonData = {
    val IdH(localName, packageCoordinate, shortenedName, fullyQualifiedName) = h
    VonObject(
      "Name",
      None,
      Vector(
        VonMember("localName", VonStr(localName)),
        VonMember("packageCoordinate", NameHammer.translatePackageCoordinate(packageCoordinate)),
        VonMember("shortenedName", VonStr(shortenedName))))
  }
}
*/
