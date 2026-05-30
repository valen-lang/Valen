use crate::interner::StrI;
use crate::utils::range::CodeLocationS;
use crate::instantiating::ast::types::{CoordI, KindIT};
use crate::instantiating::ast::names::{IdI, INameI};
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::ast::SignatureI;

/*
package dev.vale.instantiating

import dev.vale._
import dev.vale.instantiating.ast._

object InstantiatedHumanizer {
*/
// mig: fn humanize_templata
pub fn humanize_templata<'s, 'i, R>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    templata: &ITemplataI<'s, 'i, R>,
) -> String {
    panic!("Unimplemented: humanize_templata");
}
/*
  def humanizeTemplata[R <: IRegionsModeI](
    codeMap: CodeLocationS => String,
    templata: ITemplataI[R]):
  String = {
    templata match {
      case RuntimeSizedArrayTemplateTemplataI() => "Array"
      case StaticSizedArrayTemplateTemplataI() => "StaticArray"
      case InterfaceDefinitionTemplataI(envId, tyype) => humanizeId(codeMap, envId, None)
      case StructDefinitionTemplataI(envId, tyype) => humanizeId(codeMap, envId, None)
      case VariabilityTemplataI(variability) => {
        variability match {
          case FinalI => "final"
          case VaryingI => "vary"
        }
      }
      case IntegerTemplataI(value) => value.toString
      case MutabilityTemplataI(mutability) => {
        mutability match {
          case MutableI => "mut"
          case ImmutableI => "imm"
        }
      }
      case OwnershipTemplataI(ownership) => {
        ownership match {
          case OwnI => "own"
          case ImmutableBorrowI => "i&"
          case MutableBorrowI => "&"
          case ImmutableShareI => "i*"
          case MutableShareI => "*"
          case WeakI => "weak"
        }
      }
      case PrototypeTemplataI(range, prototype) => {
        humanizeId(codeMap, prototype.id)
      }
      case CoordTemplataI(region, coord) => {
        humanizeCoord(codeMap, coord)
      }
      case KindTemplataI(kind) => {
        humanizeKind(codeMap, kind)
      }
      case CoordListTemplataI(coords) => {
        "(" + coords.map(humanizeCoord(codeMap, _)).mkString(",") + ")"
      }
      case StringTemplataI(value) => "\"" + value + "\""
      case RegionTemplataI(pureHeight) => pureHeight.toString
      case other => vimpl(other)
    }
  }
*/
// mig: fn humanize_coord
pub fn humanize_coord<'s, 'i, R>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    coord: &'i CoordI<'s, 'i, R>,
) -> String {
    panic!("Unimplemented: humanize_coord");
}
/*
  private def humanizeCoord[R <: IRegionsModeI](
    codeMap: CodeLocationS => String,
    coord: CoordI[R]
  ): String = {
    val CoordI(ownership, kind) = coord

    val ownershipStr =
      ownership match {
        case OwnI => ""
        case MutableShareI => ""
        case MutableBorrowI => "&"
        case ImmutableShareI => "#"
        case ImmutableBorrowI => "&#"
        case WeakI => "weak&"
      }
    val kindStr = humanizeKind(codeMap, kind)
    ownershipStr + kindStr
  }
*/
// mig: fn humanize_kind
pub fn humanize_kind<'s, 'i, R>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    kind: &'i KindIT<'s, 'i, R>,
) -> String {
    panic!("Unimplemented: humanize_kind");
}
/*
  private def humanizeKind[R <: IRegionsModeI](
    codeMap: CodeLocationS => String,
    kind: KindIT[R]
  ) = {
    kind match {
      case IntIT(bits) => "i" + bits
      case BoolIT() => "bool"
      case StrIT() => "str"
      case NeverIT(_) => "never"
      case VoidIT() => "void"
      case FloatIT() => "float"
      case InterfaceIT(name) => humanizeId(codeMap, name)
      case StructIT(name) => humanizeId(codeMap, name)
      case RuntimeSizedArrayIT(name) => humanizeId(codeMap, name)
      case StaticSizedArrayIT(name) => humanizeId(codeMap, name)
    }
  }
*/
// mig: fn humanize_id
pub fn humanize_id<'s, 'i, R: Copy + PartialEq>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    name: &IdI<'s, 'i, R>,
    containing_region: Option<&ITemplataI<'s, 'i, R>>,
) -> String {
    let prefix = if !name.init_steps.is_empty() {
        name.init_steps.iter().map(|n| humanize_name(code_map, *n, None)).collect::<Vec<_>>().join(".") + "."
    } else {
        String::new()
    };
    prefix + &humanize_name(code_map, name.local_name, containing_region)
}
/*
  def humanizeId[R <: IRegionsModeI, I <: INameI[R]](
    codeMap: CodeLocationS => String,
    name: IdI[R, I],
    containingRegion: Option[ITemplataI[R]] = None):
  String = {
    (if (name.initSteps.nonEmpty) {
      name.initSteps.map(n => humanizeName(codeMap, n)).mkString(".") + "."
    } else {
      ""
    }) +
      humanizeName(codeMap, name.localName, containingRegion)
  }
*/
// mig: fn humanize_name
pub fn humanize_name<'s, 'i, R: Copy + PartialEq>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    name: INameI<'s, 'i, R>,
    containing_region: Option<&ITemplataI<'s, 'i, R>>,
) -> String {
    match name {
        INameI::FunctionNameIX(f) => {
            let template_str = humanize_name(code_map, INameI::FunctionTemplate(&f.template), None);
            let args_str = humanize_generic_args(code_map, f.template_args, containing_region);
            let params_str = if !f.parameters.is_empty() {
                "(".to_string() + &f.parameters.iter().map(|c| humanize_coord(code_map, c)).collect::<Vec<_>>().join(",") + ")"
            } else {
                String::new()
            };
            template_str + &args_str + &params_str
        }
        INameI::FunctionTemplate(f) => f.human_name.0.to_string(),
        INameI::ExternFunction(f) => f.human_name.0.to_string() + &humanize_generic_args(code_map, f.template_args, containing_region),
        INameI::PackageTopLevel(_) => panic!("humanize_name: PackageTopLevel branch"),
        INameI::CodeVar(c) => c.name.0.to_string(),
        INameI::TypingPassBlockResultVar(b) => panic!("humanize_name: TypingPassBlockResultVar branch"),
        INameI::TypingPassFunctionResultVar(_) => panic!("humanize_name: TypingPassFunctionResultVar branch"),
        INameI::TypingPassTemporaryVar(t) => panic!("humanize_name: TypingPassTemporaryVar branch"),
        other => panic!("humanize_name: unimplemented variant {:?}", std::mem::discriminant(&other)),
    }
}
/*
  def humanizeName[R <: IRegionsModeI, I <: INameI[R]](
    codeMap: CodeLocationS => String,
    name: INameI[R],
    containingRegion: Option[ITemplataI[R]] = None):
  String = {
    name match {
      case AnonymousSubstructConstructorNameI(template, templateArgs, parameters) => {
        humanizeName(codeMap, template) +
          "<" + templateArgs.map(humanizeTemplata(codeMap, _)).mkString(",") + ">" +
          "(" + parameters.map(humanizeCoord(codeMap, _)).mkString(",") + ")"
      }
      case AnonymousSubstructConstructorTemplateNameI(substruct) => {
        "asc:" + humanizeName(codeMap, substruct)
      }
      case SelfNameI() => "self"
      case IteratorNameI(range) => "it:" + codeMap(range.begin)
      case IterableNameI(range) => "ib:" + codeMap(range.begin)
      case IterationOptionNameI(range) => "io:" + codeMap(range.begin)
      case ForwarderFunctionNameI(_, inner) => humanizeName(codeMap, inner)
      case ForwarderFunctionTemplateNameI(inner, index) => "fwd" + index + ":" + humanizeName(codeMap, inner)
      case MagicParamNameI(codeLoc) => "mp:" + codeMap(codeLoc)
      case ClosureParamNameI(codeLocation) => "λP:" + codeMap(codeLocation)
      case ConstructingMemberNameI(name) => "cm:" + name
      case TypingPassBlockResultVarNameI(life) => "b:" + life
      case TypingPassFunctionResultVarNameI() => "(result)"
      case TypingPassTemporaryVarNameI(life) => "t:" + life
      case FunctionBoundTemplateNameI(humanName) => humanName.str
      case LambdaCallFunctionTemplateNameI(codeLocation, _) => "λF:" + codeMap(codeLocation)
      case LambdaCitizenTemplateNameI(codeLocation) => "λC:" + codeMap(codeLocation)
      case LambdaCallFunctionNameI(template, templateArgs, parameters) => {
        humanizeName(codeMap, template) +
          humanizeGenericArgs(codeMap, templateArgs, None) +
          "(" + parameters.map(humanizeCoord(codeMap, _)).mkString(",") + ")"
      }
      case FunctionBoundNameI(template, templateArgs, parameters) => {
        humanizeName(codeMap, template) +
          humanizeGenericArgs(codeMap, templateArgs, None) +
          "(" + parameters.map(humanizeCoord(codeMap, _)).mkString(",") + ")"
      }
      case CodeVarNameI(name) => name.str
      case LambdaCitizenNameI(template) => humanizeName(codeMap, template) + "<>"
      case FunctionTemplateNameI(humanName, codeLoc) => humanName.str
      case ExternFunctionNameI(humanName, templateArgs, parameters) =>
        humanName.str + humanizeGenericArgs(codeMap, templateArgs, containingRegion)
      case FunctionNameIX(templateName, templateArgs, parameters) => {
        humanizeName(codeMap, templateName) +
          humanizeGenericArgs(codeMap, templateArgs, containingRegion) +
          (if (parameters.nonEmpty) {
            "(" + parameters.map(humanizeCoord(codeMap, _)).mkString(",") + ")"
          } else {
            ""
          })
      }
      case CitizenNameI(humanName, templateArgs) => {
        humanizeName(codeMap, humanName) +
          humanizeGenericArgs(codeMap, templateArgs, containingRegion)
      }
      case RuntimeSizedArrayNameI(RuntimeSizedArrayTemplateNameI(), RawArrayNameI(mutability, elementType, region)) => {
        ("[]<" +
          (mutability match { case ImmutableI => "i" case MutableI => "m" }) + "," +
          humanizeTemplata(codeMap, region) + ">" +
          humanizeTemplata(codeMap, elementType))
      }
      case StaticSizedArrayNameI(StaticSizedArrayTemplateNameI(), size, variability, RawArrayNameI(mutability, elementType, region)) => {
        ("[]<" +
          humanizeTemplata(codeMap, IntegerTemplataI(size)) + "," +
          humanizeTemplata(codeMap, MutabilityTemplataI(mutability)) + "," +
          humanizeTemplata(codeMap, VariabilityTemplataI(variability)) + "," +
          humanizeTemplata(codeMap, region) + ">" +
          humanizeTemplata(codeMap, elementType))
      }
      case AnonymousSubstructNameI(interface, templateArgs) => {
        humanizeName(codeMap, interface) +
          "<" + templateArgs.map(humanizeTemplata(codeMap, _)).mkString(",") + ">"
      }
      case AnonymousSubstructTemplateNameI(interface) => {
        humanizeName(codeMap, interface) + ".anonymous"
      }
      case StructTemplateNameI(humanName) => humanName.str
      case InterfaceTemplateNameI(humanName) => humanName.str
    }
  }
*/
// mig: fn humanize_generic_args
pub fn humanize_generic_args<'s, 'i, R: Copy + PartialEq>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    template_args: &[ITemplataI<'s, 'i, R>],
    containing_region: Option<&ITemplataI<'s, 'i, R>>,
) -> String {
    if !template_args.is_empty() {
        let (last, init) = template_args.split_last().unwrap();
        let init_strs: Vec<String> = init.iter().map(|t| humanize_templata(code_map, t)).collect();
        let last_str = match containing_region {
            None => humanize_templata(code_map, last),
            Some(r) => { assert!(r == last); "_".to_string() }
        };
        let mut all = init_strs;
        all.push(last_str);
        "<".to_string() + &all.join(",") + ">"
    } else {
        String::new()
    }
}
/*
  private def humanizeGenericArgs[R <: IRegionsModeI](
    codeMap: CodeLocationS => String,
    templateArgs: Vector[ITemplataI[R]],
    containingRegion: Option[ITemplataI[R]]
  ) = {
    (
      if (templateArgs.nonEmpty) {
        "<" +
          (templateArgs.init.map(humanizeTemplata(codeMap, _)) ++
              templateArgs.lastOption.map(region => {
                containingRegion match {
                  case None => humanizeTemplata(codeMap, region)
                  case Some(r) => vassert(r == region); "_"
                }
              })).mkString(",") +
          ">"
      } else {
        ""
      })
  }
*/
// mig: fn humanize_signature
pub fn humanize_signature<'s, 'i, R>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    signature: &'i SignatureI<'s, 'i, R>,
) -> String {
    panic!("Unimplemented: humanize_signature");
}
/*
  def humanizeSignature[R <: IRegionsModeI](codeMap: CodeLocationS => String, signature: SignatureI[R]): String = {
    humanizeId(codeMap, signature.id)
  }
}
*/