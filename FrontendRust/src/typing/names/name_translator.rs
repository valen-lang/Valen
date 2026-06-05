use crate::utils::range::CodeLocationS;

use crate::postparsing::names::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::compiler::Compiler;
use std::marker::PhantomData;

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
                match r.tlcd {
                    ICitizenDeclarationNameS::TopLevelStructDeclarationName(s) => {
                        IFunctionTemplateNameT::FunctionTemplate(
                            self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                                human_name: s.name,
                                code_location: self.translate_code_location(s.range.begin),
                                _phantom: std::marker::PhantomData,
                            })
                        )
                    }
                    ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(i) => {
                        IFunctionTemplateNameT::FunctionTemplate(
                            self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                                human_name: i.name,
                                code_location: self.translate_code_location(i.range.begin),
                                _phantom: std::marker::PhantomData,
                            })
                        )
                    }
                    ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn) => {
                        // See LNASC.
                        let citizen_name = self.translate_citizen_name(ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn));
                        IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(
                            self.typing_interner.intern_anonymous_substruct_constructor_template_name(
                                AnonymousSubstructConstructorTemplateNameT { substruct: citizen_name }
                            )
                        )
                    }
                }
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
        match name {
            IStructDeclarationNameS::TopLevelStructDeclarationName(top_level) => {
                let struct_template_name = StructTemplateNameT {
                    human_name: top_level.name,
                    _phantom: std::marker::PhantomData,
                };
                IStructTemplateNameT::StructTemplate(
                    self.typing_interner.intern_struct_template_name(struct_template_name)
                )
            }
            IStructDeclarationNameS::AnonymousSubstructTemplateName(anon) => {
                let interface_template_name = self.translate_interface_name(anon.interface_name);
                IStructTemplateNameT::AnonymousSubstructTemplate(
                    self.typing_interner.intern_anonymous_substruct_template_name(
                        AnonymousSubstructTemplateNameT { interface: interface_template_name }
                    )
                )
            }
        }
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
    pub fn translate_interface_name(&self, name: TopLevelInterfaceDeclarationNameS<'s>) -> IInterfaceTemplateNameT<'s, 't> {
        let interface_template_name = InterfaceTemplateNameT {
            human_namee: name.name,
            _phantom: std::marker::PhantomData,
        };
        IInterfaceTemplateNameT::InterfaceTemplate(
            self.typing_interner.intern_interface_template_name(interface_template_name)
        )
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
    pub fn translate_citizen_name(&self, name: ICitizenDeclarationNameS<'s>) -> ICitizenTemplateNameT<'s, 't> {
        match name {
            ICitizenDeclarationNameS::TopLevelStructDeclarationName(n) => {
                ICitizenTemplateNameT::StructTemplate(
                    self.typing_interner.intern_struct_template_name(StructTemplateNameT {
                        human_name: n.name,
                        _phantom: std::marker::PhantomData,
                    })
                )
            }
            ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn) => {
                // See LNASC.
                let interface_template_name = self.translate_interface_name(astn.interface_name);
                ICitizenTemplateNameT::AnonymousSubstructTemplate(
                    self.typing_interner.intern_anonymous_substruct_template_name(
                        AnonymousSubstructTemplateNameT { interface: interface_template_name }
                    )
                )
            }
            ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(n) => {
                ICitizenTemplateNameT::InterfaceTemplate(
                    self.typing_interner.intern_interface_template_name(InterfaceTemplateNameT {
                        human_namee: n.name,
                        _phantom: std::marker::PhantomData,
                    })
                )
            }
        }
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
    pub fn translate_name_step(&self, name: INameS<'s>) -> INameT<'s, 't> {
        match name {
            INameS::LambdaStructDeclaration(_) => panic!("Unimplemented: translate_name_step LambdaStructDeclaration"),
            INameS::LetName(_) => panic!("Unimplemented: translate_name_step LetNameS"),
            INameS::ExportAsName(_) => panic!("Unimplemented: translate_name_step ExportAsNameS"),
            INameS::VarName(v) => panic!("Unimplemented: translate_name_step VarName {:?}", v),
            INameS::TopLevelStructDeclaration(s) => {
                match self.translate_struct_name(IStructDeclarationNameS::TopLevelStructDeclarationName(*s)) {
                    IStructTemplateNameT::StructTemplate(r) => INameT::StructTemplate(r),
                    IStructTemplateNameT::AnonymousSubstructTemplate(r) => INameT::AnonymousSubstructTemplate(r),
                    IStructTemplateNameT::LambdaCitizenTemplate(_) => panic!("Unimplemented: translate_name_step LambdaCitizenTemplate"),
                }
            }
            INameS::TopLevelInterfaceDeclaration(i) => {
                match self.translate_interface_name(*i) {
                    IInterfaceTemplateNameT::InterfaceTemplate(r) => INameT::InterfaceTemplate(r),
                }
            }
            INameS::AnonymousSubstructTemplateName(n) => {
                // See LNASC.
                let interface_template_name = self.translate_interface_name(n.interface_name);
                INameT::AnonymousSubstructTemplate(
                    self.typing_interner.intern_anonymous_substruct_template_name(
                        AnonymousSubstructTemplateNameT { interface: interface_template_name }
                    )
                )
            }
            INameS::AnonymousSubstructImplDeclaration(n) => {
                // See LNASC.
                let interface_template_name = self.translate_interface_name(n.interface);
                INameT::AnonymousSubstructImplTemplate(
                    self.typing_interner.intern_anonymous_substruct_impl_template_name(
                        AnonymousSubstructImplTemplateNameT { interface: interface_template_name }
                    )
                )
            }
            INameS::ImplDeclaration(_) => panic!("Unimplemented: translate_name_step ImplDeclarationNameS"),
            INameS::RuneName(_) => panic!("Unimplemented: translate_name_step RuneNameS"),
            INameS::RuntimeSizedArrayDeclarationName(_) => panic!("Unimplemented: translate_name_step RuntimeSizedArrayDeclarationName"),
            INameS::StaticSizedArrayDeclarationName(_) => panic!("Unimplemented: translate_name_step StaticSizedArrayDeclarationName"),
            INameS::GlobalFunctionFamilyName(_) => panic!("Unimplemented: translate_name_step GlobalFunctionFamilyName"),
            INameS::ArbitraryName(_) => panic!("Unimplemented: translate_name_step ArbitraryName"),
            INameS::FunctionDeclaration(fn_decl) => {
                match fn_decl {
                    IFunctionDeclarationNameS::LambdaDeclarationName(_) => panic!("Unimplemented: translate_name_step LambdaDeclarationNameS"),
                    IFunctionDeclarationNameS::FunctionName(n) => {
                        INameT::FunctionTemplate(self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                            human_name: n.name,
                            code_location: n.code_location,
                            _phantom: PhantomData,
                        }))
                    }
                    IFunctionDeclarationNameS::ConstructorName(ctor) => {
                        match ctor.tlcd {
                            ICitizenDeclarationNameS::TopLevelStructDeclarationName(n) => {
                                INameT::FunctionTemplate(self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                                    human_name: n.name,
                                    code_location: n.range.begin,
                                    _phantom: PhantomData,
                                }))
                            }
                            ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(_) => {
                                panic!("Unimplemented: translate_name_step ConstructorNameS for interface")
                            }
                            ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn) => {
                                // See LNASC.
                                let citizen_name = self.translate_citizen_name(ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn));
                                INameT::AnonymousSubstructConstructorTemplate(
                                    self.typing_interner.intern_anonymous_substruct_constructor_template_name(
                                        AnonymousSubstructConstructorTemplateNameT { substruct: citizen_name }
                                    )
                                )
                            }
                        }
                    }
                    IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(_) => panic!("Unimplemented: translate_name_step ForwarderFunctionDeclarationName"),
                    IFunctionDeclarationNameS::ImmConcreteDestructorName(_) => panic!("Unimplemented: translate_name_step ImmConcreteDestructorName"),
                    IFunctionDeclarationNameS::ImmInterfaceDestructorName(_) => panic!("Unimplemented: translate_name_step ImmInterfaceDestructorName"),
                }
            }
        }
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
            IVarNameS::MagicParamName(code_location) => {
                IVarNameT::MagicParam(self.typing_interner.intern_magic_param_name(
                    MagicParamNameT { code_location2: self.translate_code_location(code_location), _phantom: std::marker::PhantomData }))
            }
            IVarNameS::SelfName => {
                IVarNameT::Self_(self.typing_interner.intern_self_name(
                    SelfNameT { _phantom: std::marker::PhantomData }))
            }
            IVarNameS::ConstructingMemberName(n) => {
                IVarNameT::ConstructingMember(self.typing_interner.intern_constructing_member_name(
                    ConstructingMemberNameT { name: n, _phantom: std::marker::PhantomData }))
            }
            IVarNameS::IterableName(range) => {
                IVarNameT::Iterable(self.typing_interner.intern_iterable_name(
                    IterableNameT { range, _phantom: std::marker::PhantomData }))
            }
            IVarNameS::IteratorName(range) => {
                IVarNameT::Iterator(self.typing_interner.intern_iterator_name(
                    IteratorNameT { range, _phantom: std::marker::PhantomData }))
            }
            IVarNameS::IterationOptionName(range) => {
                IVarNameT::IterationOption(self.typing_interner.intern_iteration_option_name(
                    IterationOptionNameT { range, _phantom: std::marker::PhantomData }))
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
    pub fn translate_impl_name(&self, n: IImplDeclarationNameS<'s>) -> IImplTemplateNameT<'s, 't> {
        match n {
            IImplDeclarationNameS::ImplDeclarationName(impl_decl) => {
                let impl_template_name = ImplTemplateNameT {
                    code_location_s: self.translate_code_location(impl_decl.code_location),
                    _phantom: std::marker::PhantomData,
                };
                IImplTemplateNameT::ImplTemplate(
                    self.typing_interner.intern_impl_template_name(impl_template_name)
                )
            }
            IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(anon) => {
                let interface_template_name = self.translate_interface_name(anon.interface);
                let anon_impl_template_name = AnonymousSubstructImplTemplateNameT {
                    interface: interface_template_name,
                };
                IImplTemplateNameT::AnonymousSubstructImplTemplate(
                    self.typing_interner.intern_anonymous_substruct_impl_template_name(anon_impl_template_name)
                )
            }
        }
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
