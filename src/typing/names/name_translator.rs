use crate::utils::range::CodeLocationS;

use crate::postparsing::names::*;

use crate::typing::compiler::Compiler;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use std::marker::PhantomData;
use std::mem::discriminant;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn translate_generic_template_function_name(
    &self,
    function_name: IFunctionDeclarationNameS<'s>,
    params: &[KindT<'s, 't>],
  ) -> INameT<'s, 't> {
    match function_name {
      IFunctionDeclarationNameS::LambdaDeclarationName(lambda_name) => {
        let interned = self.typing_interner.intern_lambda_call_function_template_name(
          LambdaCallFunctionTemplateNameValT {
            code_location: lambda_name.code_location,
            param_types: params,
          },
        );
        INameT::LambdaCallFunctionTemplate(interned)
      }
      _ => {
        panic!("vwat: Only templates should call this");
      }
    }
  }

  pub fn translate_generic_function_name(
    &self,
    function_name: IFunctionDeclarationNameS<'s>,
  ) -> IFunctionTemplateNameT<'s, 't> {
    match function_name {
      IFunctionDeclarationNameS::LambdaDeclarationName(_) => {
        panic!("Lambdas are generic templates, not generics");
      }
      IFunctionDeclarationNameS::FunctionName(n) => IFunctionTemplateNameT::FunctionTemplate(
        self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
          human_name: n.imprecise_name.name,
          code_location: self.translate_code_location(n.code_location),
        }),
      ),
      IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(r) => {
        IFunctionTemplateNameT::ForwarderFunctionTemplate(
          self.typing_interner.intern_forwarder_function_template_name(
            ForwarderFunctionTemplateNameT {
              inner: self.translate_generic_function_name(r.inner),
              index: r.index,
            },
          ),
        )
      }
      IFunctionDeclarationNameS::ConstructorName(r) => {
        match r.tlcd {
          ICitizenDeclarationNameS::TopLevelStructDeclarationName(s) => {
            IFunctionTemplateNameT::FunctionTemplate(
              self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                human_name: s.name,
                code_location: self.translate_code_location(s.range.begin),
              }),
            )
          }
          ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(i) => {
            IFunctionTemplateNameT::FunctionTemplate(
              self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                human_name: i.name,
                code_location: self.translate_code_location(i.range.begin),
              }),
            )
          }
          ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn) => {
            // See LNASC.
            let citizen_name = self.translate_citizen_name(
              ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn),
            );
            IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(
              self.typing_interner.intern_anonymous_substruct_constructor_template_name(
                AnonymousSubstructConstructorTemplateNameT { substruct: citizen_name },
              ),
            )
          }
        }
      }
    }
  }

  pub fn translate_struct_name(
    &self,
    name: IStructDeclarationNameS<'s>,
  ) -> IStructTemplateNameT<'s, 't> {
    match name {
      IStructDeclarationNameS::TopLevelStructDeclarationName(top_level) => {
        let struct_template_name = StructTemplateNameT { human_name: top_level.name };
        IStructTemplateNameT::StructTemplate(
          self.typing_interner.intern_struct_template_name(struct_template_name),
        )
      }
      IStructDeclarationNameS::AnonymousSubstructTemplateName(anon) => {
        let interface_template_name = self.translate_interface_name(anon.interface_name);
        IStructTemplateNameT::AnonymousSubstructTemplate(
          self.typing_interner.intern_anonymous_substruct_template_name(
            AnonymousSubstructTemplateNameT { interface: interface_template_name },
          ),
        )
      }
    }
  }

  pub fn translate_interface_name(
    &self,
    name: TopLevelInterfaceDeclarationNameS<'s>,
  ) -> IInterfaceTemplateNameT<'s, 't> {
    let interface_template_name = InterfaceTemplateNameT { human_namee: name.name };
    IInterfaceTemplateNameT::InterfaceTemplate(
      self.typing_interner.intern_interface_template_name(interface_template_name),
    )
  }

  pub fn translate_citizen_name(
    &self,
    name: ICitizenDeclarationNameS<'s>,
  ) -> ICitizenTemplateNameT<'s, 't> {
    match name {
      ICitizenDeclarationNameS::TopLevelStructDeclarationName(n) => {
        ICitizenTemplateNameT::StructTemplate(
          self
            .typing_interner
            .intern_struct_template_name(StructTemplateNameT { human_name: n.name }),
        )
      }
      ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn) => {
        // See LNASC.
        let interface_template_name = self.translate_interface_name(astn.interface_name);
        ICitizenTemplateNameT::AnonymousSubstructTemplate(
          self.typing_interner.intern_anonymous_substruct_template_name(
            AnonymousSubstructTemplateNameT { interface: interface_template_name },
          ),
        )
      }
      ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(n) => {
        ICitizenTemplateNameT::InterfaceTemplate(
          self
            .typing_interner
            .intern_interface_template_name(InterfaceTemplateNameT { human_namee: n.name }),
        )
      }
    }
  }

  pub fn translate_name_step(&self, name: INameS<'s>) -> INameT<'s, 't> {
    match name {
      INameS::LambdaStructDeclaration(_) => {
        panic!("Unimplemented: translate_name_step LambdaStructDeclaration");
        // interner.intern(LambdaCitizenNameT(interner.intern(LambdaCitizenTemplateNameT(translateCodeLocation(codeLocation)))))
      }
      INameS::LetName(_) => {
        panic!("Unimplemented: translate_name_step LetNameS");
        // interner.intern(LetNameT(translateCodeLocation(codeLocation)))
      }
      INameS::ExportAsName(_) => {
        panic!("Unimplemented: translate_name_step ExportAsNameS");
        // interner.intern(ExportAsNameT(translateCodeLocation(codeLocation)))
      }
      INameS::VarName(v) => panic!("Unimplemented: translate_name_step VarName {:?}", v),
      INameS::TopLevelStructDeclaration(s) => {
        match self.translate_struct_name(IStructDeclarationNameS::TopLevelStructDeclarationName(*s))
        {
          IStructTemplateNameT::StructTemplate(r) => INameT::StructTemplate(r),
          IStructTemplateNameT::AnonymousSubstructTemplate(r) => {
            INameT::AnonymousSubstructTemplate(r)
          }
          IStructTemplateNameT::LambdaCitizenTemplate(_) => {
            panic!("Unimplemented: translate_name_step LambdaCitizenTemplate")
          }
        }
      }
      INameS::TopLevelInterfaceDeclaration(i) => match self.translate_interface_name(*i) {
        IInterfaceTemplateNameT::InterfaceTemplate(r) => INameT::InterfaceTemplate(r),
      },
      INameS::AnonymousSubstructTemplateName(n) => {
        // See LNASC.
        let interface_template_name = self.translate_interface_name(n.interface_name);
        INameT::AnonymousSubstructTemplate(
          self.typing_interner.intern_anonymous_substruct_template_name(
            AnonymousSubstructTemplateNameT { interface: interface_template_name },
          ),
        )
      }
      INameS::AnonymousSubstructImplDeclaration(_) => {
        // Impl template names carry the sub-citizen and super-interface imprecise
        // names, which INameS doesn't hold — callers must go through
        // translate_impl_name with the imprecise names from the ImplS.
        panic!("translate_name_step can't build an impl name; use translate_impl_name");
      }
      INameS::ImplDeclaration(_) => {
        panic!("Unimplemented: translate_name_step ImplDeclarationNameS");
        // vimpl()
        // // interner.intern(ImplDeclareNameT(codeLocation))
      }
      INameS::RuneName(_) => panic!("Unimplemented: translate_name_step RuneNameS"),
      INameS::RuntimeSizedArrayDeclarationName(_) => {
        panic!("Unimplemented: translate_name_step RuntimeSizedArrayDeclarationName")
      }
      INameS::StaticSizedArrayDeclarationName(_) => {
        panic!("Unimplemented: translate_name_step StaticSizedArrayDeclarationName")
      }
      INameS::GlobalFunctionFamilyName(_) => {
        panic!("Unimplemented: translate_name_step GlobalFunctionFamilyName")
      }
      INameS::ArbitraryName(_) => panic!("Unimplemented: translate_name_step ArbitraryName"),
      INameS::FunctionDeclaration(fn_decl) => {
        match fn_decl {
          IFunctionDeclarationNameS::LambdaDeclarationName(_) => {
            panic!("Unimplemented: translate_name_step LambdaDeclarationNameS");
            // vcurious()
            // // interner.intern(LambdaTemplateNameT(translateCodeLocation(codeLocation)))
          }
          IFunctionDeclarationNameS::FunctionName(n) => {
            INameT::FunctionTemplate(self.typing_interner.intern_function_template_name(
              FunctionTemplateNameT {
                human_name: n.imprecise_name.name,
                code_location: n.code_location,
              },
            ))
          }
          IFunctionDeclarationNameS::ConstructorName(ctor) => {
            match ctor.tlcd {
              ICitizenDeclarationNameS::TopLevelStructDeclarationName(n) => {
                INameT::FunctionTemplate(self.typing_interner.intern_function_template_name(
                  FunctionTemplateNameT { human_name: n.name, code_location: n.range.begin },
                ))
              }
              ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(_) => {
                panic!("Unimplemented: translate_name_step ConstructorNameS for interface")
              }
              ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn) => {
                // See LNASC.
                let citizen_name = self.translate_citizen_name(
                  ICitizenDeclarationNameS::AnonymousSubstructTemplateName(astn),
                );
                INameT::AnonymousSubstructConstructorTemplate(
                  self.typing_interner.intern_anonymous_substruct_constructor_template_name(
                    AnonymousSubstructConstructorTemplateNameT { substruct: citizen_name },
                  ),
                )
              }
            }
          }
          IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(_) => {
            panic!("Unimplemented: translate_name_step ForwarderFunctionDeclarationName")
          }
        }
      }
    }
  }

  pub fn translate_code_location(&self, s: CodeLocationS<'s>) -> CodeLocationS<'s> {
    s
  }

  pub fn translate_var_name_step(&self, name: IVarDeclarationNameS<'s>) -> IVarNameT<'s, 't> {
    match name {
      IVarDeclarationNameS::CodeVarName(CodeVarNameS { imprecise_name, lid }) => {
        IVarNameT::Local(
          self.typing_interner.intern_local_name(LocalNameT { imprecise_name, life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid) }),
        )
      }
      IVarDeclarationNameS::ClosureParamName(ClosureParamNameDeclarationS { imprecise_name, lid }) => {
        IVarNameT::ClosureParam(
          self.typing_interner.intern_closure_param_name(ClosureParamNameT { imprecise_name, life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid) }),
        )
      }
      IVarDeclarationNameS::MagicParamName(MagicParamNameDeclarationS { imprecise_name, lid }) => {
        IVarNameT::MagicParam(
          self.typing_interner.intern_magic_param_name(MagicParamNameT { imprecise_name, life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid) }),
        )
      }
      IVarDeclarationNameS::SelfName(SelfNameDeclarationS { lid, .. }) => {
        IVarNameT::Self_(self.typing_interner.intern_self_name(SelfNameT { life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid) }))
      }
      IVarDeclarationNameS::ConstructingMemberName(ConstructingMemberNameDeclarationS { imprecise_name, lid }) => {
        IVarNameT::ConstructingMember(
          self.typing_interner.intern_constructing_member_name(ConstructingMemberNameT {
            imprecise_name,
            life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid),
          }),
        )
      }
      IVarDeclarationNameS::IterableName(IterableNameDeclarationS { lid, .. }) => {
        IVarNameT::Iterable(self.typing_interner.intern_iterable_name(IterableNameT { life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid) }))
      }
      IVarDeclarationNameS::IteratorName(IteratorNameDeclarationS { lid, .. }) => {
        IVarNameT::Iterator(self.typing_interner.intern_iterator_name(IteratorNameT { life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid) }))
      }
      IVarDeclarationNameS::IterationOptionName(IterationOptionNameDeclarationS{ lid, .. }) => IVarNameT::IterationOption(
        self.typing_interner.intern_iteration_option_name(IterationOptionNameT { life: LocationInFunctionEnvironmentT::from_lid(self.typing_interner, lid) }),
      ),
      _ => {
        panic!("implement: translate_var_name_step — {:?}", discriminant(&name));
        // vimpl(name.toString)
      }
    }
  }

  pub fn translate_impl_name(
    &self,
    n: IImplDeclarationNameS<'s>,
    sub_citizen_imprecise_name: IImpreciseNameS<'s>,
    super_interface_imprecise_name: IImpreciseNameS<'s>,
  ) -> IImplTemplateNameT<'s, 't> {
    match n {
      IImplDeclarationNameS::ImplDeclarationName(impl_decl) => {
        let impl_template_name = ImplTemplateNameT {
          code_location: self.translate_code_location(impl_decl.code_location),
          sub_citizen_imprecise_name,
          super_interface_imprecise_name,
        };
        IImplTemplateNameT::ImplTemplate(
          self.typing_interner.intern_impl_template_name(impl_template_name),
        )
      }
      IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(anon) => {
        let interface_template_name = self.translate_interface_name(anon.interface);
        let anon_impl_template_name = AnonymousSubstructImplTemplateNameT {
          interface: interface_template_name,
          sub_citizen_imprecise_name,
          super_interface_imprecise_name,
        };
        IImplTemplateNameT::AnonymousSubstructImplTemplate(
          self
            .typing_interner
            .intern_anonymous_substruct_impl_template_name(anon_impl_template_name),
        )
      }
    }
  }
}
