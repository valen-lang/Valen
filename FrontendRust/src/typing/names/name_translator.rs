use crate::utils::range::CodeLocationS;

use crate::postparsing::names::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::compiler::Compiler;

/*
package dev.vale.typing.names

import dev.vale.{CodeLocationS, Interner, vcurious, vfail, vimpl, vwat}
import dev.vale.postparsing._
import dev.vale.typing.types.{CoordT, ICitizenTT}
import dev.vale.highertyping._
import dev.vale.postparsing._

import scala.collection.mutable

*/
/*
class NameTranslator(interner: Interner) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_generic_template_function_name(&self, function_name: IFunctionDeclarationNameS<'s>, params: &[CoordT<'s, 't>]) -> INameT<'s, 't> {
        match function_name {
            IFunctionDeclarationNameS::LambdaDeclarationName(lambda_name) => {
                let interned = self.typing_interner.intern_lambda_call_function_template_name(LambdaCallFunctionTemplateNameValT {
                    code_location: lambda_name.code_location,
                    param_types: params,
                });
                INameT::LambdaCallFunctionTemplate(interned)
            }
            _ => { panic!("vwat: Only templates should call this"); }
        }
    }
/*
Guardian: temp-disable: SPDMX — Scala's translateCodeLocation is a documented identity function (CodeLocationS(line,col) => CodeLocationS(line,col)). Rust has no NameTranslator on Compiler — using code_location directly is semantically identical. Same pattern as the temp-disable on evaluate_templated_function_from_call_for_prototype. — FrontendRust/guardian-logs/request-1792-1777947462984/hook-1792/translate_generic_template_function_name--27.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateGenericTemplateFunctionName(
    functionName: IFunctionDeclarationNameS,
    params: Vector[CoordT]):
  IFunctionTemplateNameT = {
    functionName match {
      case LambdaDeclarationNameS(codeLocation) => {
        interner.intern(LambdaCallFunctionTemplateNameT(translateCodeLocation(codeLocation), params))
      }
      case other => vwat(other) // Only templates should call this
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_generic_function_name(&self, function_name: IFunctionDeclarationNameS<'s>) -> IFunctionTemplateNameT<'s, 't> {
        match function_name {
            IFunctionDeclarationNameS::LambdaDeclarationName(_) => {
                panic!("Lambdas are generic templates, not generics");
            }
            IFunctionDeclarationNameS::FunctionName(n) => {
                IFunctionTemplateNameT::FunctionTemplate(
                    self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                        human_name: n.name,
                        code_location: self.translate_code_location(n.code_location),
                        _phantom: std::marker::PhantomData,
                    })
                )
            }
            IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(r) => {
                IFunctionTemplateNameT::ForwarderFunctionTemplate(
                    self.typing_interner.intern_forwarder_function_template_name(ForwarderFunctionTemplateNameT {
                        inner: self.translate_generic_function_name(r.inner),
                        index: r.index,
                    })
                )
            }
            IFunctionDeclarationNameS::ConstructorName(r) => {
                let (name, code_location) = match r.tlcd {
                    TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(s) => (s.name, s.range.begin),
                    TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(i) => (i.name, i.range.begin),
                };
                IFunctionTemplateNameT::FunctionTemplate(
                    self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                        human_name: name,
                        code_location: self.translate_code_location(code_location),
                        _phantom: std::marker::PhantomData,
                    })
                )
            }
            IFunctionDeclarationNameS::ImmConcreteDestructorName(_) => panic!("Unimplemented: ImmConcreteDestructorName in translate_generic_function_name"),
            IFunctionDeclarationNameS::ImmInterfaceDestructorName(_) => panic!("Unimplemented: ImmInterfaceDestructorName in translate_generic_function_name"),
        }
    }
/*
  def translateGenericFunctionName(functionName: IFunctionDeclarationNameS): IFunctionTemplateNameT = {
    functionName match {
      case LambdaDeclarationNameS(codeLocation) => {
        vfail() // Lambdas are generic templates, not generics
      }
      case FunctionNameS(name, codeLocation) => {
        interner.intern(FunctionTemplateNameT(name, translateCodeLocation(codeLocation)))
      }
      case ForwarderFunctionDeclarationNameS(inner, index) => {
        interner.intern(ForwarderFunctionTemplateNameT(translateGenericFunctionName(inner), index))
      }
      case ConstructorNameS(TopLevelCitizenDeclarationNameS(name, codeLocation)) => {
        interner.intern(FunctionTemplateNameT(name, translateCodeLocation(codeLocation.begin)))
      }
      case ConstructorNameS(thing @ AnonymousSubstructTemplateNameS(_)) => {
        interner.intern(AnonymousSubstructConstructorTemplateNameT(translateCitizenName(thing)))
      }
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_struct_name(&self, name: IStructDeclarationNameS<'s>) -> IStructTemplateNameT<'s, 't> {
        panic!("Unimplemented: translate_struct_name");
    }
/*
  def translateStructName(name: IStructDeclarationNameS): IStructTemplateNameT = {
    name match {
      case TopLevelCitizenDeclarationNameS(humanName, codeLocation) => {
        interner.intern(StructTemplateNameT(humanName))
      }
      case AnonymousSubstructTemplateNameS(interfaceName) => {
        // Now strip it off, stuff it inside our new name. See LNASC.
        interner.intern(AnonymousSubstructTemplateNameT(translateInterfaceName(interfaceName)))
      }
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_interface_name(&self, name: IStructDeclarationNameS<'s>) -> IInterfaceTemplateNameT<'s, 't> {
        panic!("Unimplemented: translate_interface_name");
    }
/*
  def translateInterfaceName(name: IInterfaceDeclarationNameS): IInterfaceTemplateNameT = {
    name match {
      case TopLevelCitizenDeclarationNameS(humanName, codeLocation) => {
        interner.intern(InterfaceTemplateNameT(humanName))
      }
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_citizen_name(&self, name: IFunctionDeclarationNameS<'s>) -> ICitizenTemplateNameT<'s, 't> {
        panic!("Unimplemented: translate_citizen_name");
    }
/*
  def translateCitizenName(name: ICitizenDeclarationNameS): ICitizenTemplateNameT = {
    name match {
      case TopLevelCitizenDeclarationNameS(humanName, codeLocation) => {
        interner.intern(StructTemplateNameT(humanName))
      }
      case AnonymousSubstructTemplateNameS(interfaceName) => {
        // Now strip it off, stuff it inside our new name. See LNASC.
        interner.intern(AnonymousSubstructTemplateNameT(translateInterfaceName(interfaceName)))
      }
      case TopLevelCitizenDeclarationNameS(humanName, codeLocation) => {
        interner.intern(InterfaceTemplateNameT(humanName))
      }
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_name_step(&self, name: INameS) -> INameT<'_, '_> {
        panic!("Unimplemented: translate_name_step");
    }
/*
  def translateNameStep(name: INameS): INameT = {
    name match {
      case LambdaStructDeclarationNameS(LambdaDeclarationNameS(codeLocation)) => interner.intern(LambdaCitizenNameT(interner.intern(LambdaCitizenTemplateNameT(translateCodeLocation(codeLocation)))))
      case LetNameS(codeLocation) => interner.intern(LetNameT(translateCodeLocation(codeLocation)))
      case ExportAsNameS(codeLocation) => interner.intern(ExportAsNameT(translateCodeLocation(codeLocation)))
      case ClosureParamNameS(codeLocation) => interner.intern(ClosureParamNameT(codeLocation))
      case MagicParamNameS(codeLocation) => interner.intern(MagicParamNameT(translateCodeLocation(codeLocation)))
      case CodeVarNameS(name) => interner.intern(CodeVarNameT(name))
      case s @ TopLevelStructDeclarationNameS(_, _) => translateStructName(s)
      case s @ TopLevelInterfaceDeclarationNameS(_, _) => translateInterfaceName(s)
      case LambdaDeclarationNameS(codeLocation) => {
        vcurious()
//        interner.intern(LambdaTemplateNameT(translateCodeLocation(codeLocation)))
      }
      case FunctionNameS(name, codeLocation) => {
        interner.intern(FunctionTemplateNameT(name, translateCodeLocation(codeLocation)))
      }
      case ConstructorNameS(TopLevelCitizenDeclarationNameS(name, codeLocation)) => {
        interner.intern(FunctionTemplateNameT(name, translateCodeLocation(codeLocation.begin)))
      }
      case ConstructorNameS(astn @ AnonymousSubstructTemplateNameS(_)) => {
        // See LNASC.
        interner.intern(AnonymousSubstructConstructorTemplateNameT(translateCitizenName(astn)))
      }
      case AnonymousSubstructTemplateNameS(tlcd) => {
        // See LNASC.
        interner.intern(AnonymousSubstructTemplateNameT(translateInterfaceName(tlcd)))
      }
      case AnonymousSubstructImplDeclarationNameS(tlcd) => {
        // See LNASC.
        interner.intern(AnonymousSubstructImplTemplateNameT(translateInterfaceName(tlcd)))
      }
      case ImplDeclarationNameS(codeLocation) => {
        vimpl()
//        interner.intern(ImplDeclareNameT(codeLocation))
      }
      case _ => vimpl(name.toString)
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_code_location(&self, s: CodeLocationS<'s>) -> CodeLocationS<'s> {
        s
    }
/*
  def translateCodeLocation(s: CodeLocationS): CodeLocationS = {
    val CodeLocationS(line, col) = s
    CodeLocationS(line, col)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_var_name_step(&self, name: IVarNameS<'s>) -> IVarNameT<'s, 't> {
        match name {
            IVarNameS::CodeVarName(name_str) => {
                IVarNameT::CodeVar(self.typing_interner.intern_code_var_name(
                    CodeVarNameT { name: name_str, _phantom: std::marker::PhantomData }))
            }
            IVarNameS::ClosureParamName(closure_param_name_s) => {
                IVarNameT::ClosureParam(self.typing_interner.intern_closure_param_name(
                    ClosureParamNameT { code_location: closure_param_name_s.code_location, _phantom: std::marker::PhantomData }))
            }
            _ => {
                panic!("implement: translate_var_name_step — {:?}", std::mem::discriminant(&name));
            }
        }
    }
/*
  def translateVarNameStep(name: IVarNameS): IVarNameT = {
    name match {
      //      case UnnamedLocalNameS(codeLocation) => UnnamedLocalNameT(translateCodeLocation(codeLocation))
      case ClosureParamNameS(codeLocation) => interner.intern(ClosureParamNameT(codeLocation))
      case SelfNameS() => interner.intern(SelfNameT())
      case IterableNameS(range) => interner.intern(IterableNameT(range))
      case IteratorNameS(range) => interner.intern(IteratorNameT(range))
      case IterationOptionNameS(range) => interner.intern(IterationOptionNameT(range))
      case MagicParamNameS(codeLocation) => interner.intern(MagicParamNameT(translateCodeLocation(codeLocation)))
      case ConstructingMemberNameS(n) => interner.intern(ConstructingMemberNameT(n))
      case WhileCondResultNameS(range) => interner.intern(WhileCondResultNameT(range))
      case CodeVarNameS(name) => interner.intern(CodeVarNameT(name))
      case AnonymousSubstructMemberNameS(index) => interner.intern(AnonymousSubstructMemberNameT(index))
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_impl_name(&self, n: IImplDeclarationNameS) -> IImplTemplateNameT<'_, '_> {
        panic!("Unimplemented: translate_impl_name");
    }
/*
  def translateImplName(n: IImplDeclarationNameS): IImplTemplateNameT = {
    n match {
      case ImplDeclarationNameS(l) => {
        interner.intern(ImplTemplateNameT(translateCodeLocation(l)))
      }
      case AnonymousSubstructImplDeclarationNameS(interfaceName) => {
        interner.intern(AnonymousSubstructImplTemplateNameT(translateInterfaceName(interfaceName)))
      }
    }
  }
}
*/
}
