/*
package dev.vale.typing.names

import dev.vale.{CodeLocationS, Interner, vcurious, vfail, vimpl, vwat}
import dev.vale.postparsing._
import dev.vale.typing.types.{CoordT, ICitizenTT}
import dev.vale.highertyping._
import dev.vale.postparsing._

import scala.collection.mutable

*/
use crate::utils::range::CodeLocationS;

use crate::postparsing::names::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::compiler::Compiler;

// mig: struct NameTranslator
// vestigial: kept until Step 8 cleanup because sub-compilers still hold `name_translator: NameTranslator<'s>` fields
pub struct NameTranslator<'s>(pub std::marker::PhantomData<&'s ()>);
// mig: impl NameTranslator
/*
class NameTranslator(interner: Interner) {
*/
// mig: fn translate_generic_template_function_name
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_generic_template_function_name(&self, function_name: IFunctionDeclarationNameS<'s>, params: Vec<CoordT<'s, 't>>) -> IFunctionTemplateNameT<'s, 't> {
        panic!("Unimplemented: translate_generic_template_function_name");
    }
/*
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

// mig: fn translate_generic_function_name
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_generic_function_name(&self, function_name: IFunctionDeclarationNameS<'s>) -> IFunctionTemplateNameT<'s, 't> {
        panic!("Unimplemented: translate_generic_function_name");
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

// mig: fn translate_struct_name
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

// mig: fn translate_interface_name
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

// mig: fn translate_citizen_name
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

// mig: fn translate_name_step
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

// mig: fn translate_code_location
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_code_location(&self, s: CodeLocationS) -> CodeLocationS<'_> {
        panic!("Unimplemented: translate_code_location");
    }
/*
  def translateCodeLocation(s: CodeLocationS): CodeLocationS = {
    val CodeLocationS(line, col) = s
    CodeLocationS(line, col)
  }
*/
}

// mig: fn translate_var_name_step
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_var_name_step(&self, name: IVarNameS) -> IVarNameT<'_, '_> {
        panic!("Unimplemented: translate_var_name_step");
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

// mig: fn translate_impl_name
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
