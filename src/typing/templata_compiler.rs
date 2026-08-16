use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::env::environment::*;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::typing_interner::{MustIntern, TypingInterner};
use crate::keywords::Keywords;
use crate::typing::hinputs_t::{InstantiationBoundArgumentsT, InstantiationReachableBoundArgumentsT};
use crate::postparsing::names::{IRuneS, IImpreciseNameS};
use crate::postparsing::ast::{GenericParameterS, LocationInDenizen};
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rules::rules::{EqualsSR, IRulexSR, ResolveSR, RuneUsage};
use crate::typing::infer_compiler::{collect_bound_search_kinds, include_rule_in_call_site_solve};
use crate::typing::rune_typing::rune_type_solver::IRuneTypeSolverEnv;
use crate::utils::range::RangeS;
use crate::utils::fx::IndexMap;
use crate::utils::fx::HashMap;
use crate::typing::types::types::{KindPlaceholderT, KindT};
use crate::typing::names::names::IInstantiationNameT;
use crate::typing::names::names::{ISuperKindNameT, ITemplateNameT};
use crate::typing::names::names::{INameValT, IdValT};
use crate::typing::names::names::StructNameValT;
use crate::typing::names::names::INameT;
use crate::typing::types::types::StructTTValT;
use crate::typing::names::names::FunctionBoundNameT;
use crate::typing::names::names::ImplBoundNameT;
use crate::typing::names::names::InterfaceNameValT;
use crate::typing::types::types::InterfaceTTValT;
use crate::typing::names::names::IPlaceholderNameT;
use crate::typing::names::names::IFunctionNameT;
use crate::typing::ast::ast::PrototypeValT;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::postparsing::itemplatatype::KindTemplataType;
use crate::postparsing::ast::IGenericParameterTypeS;
use crate::postparsing::ast::KindGenericParameterTypeS;
use crate::scout_arena::ScoutArena;
use crate::typing::rune_typing::rune_type_solver::IRuneTypeSolverLookupResult;
use crate::typing::rune_typing::rune_type_solver::IRuneTypingLookupFailedError;
use crate::typing::rune_typing::rune_type_solver::TemplataLookupResult;
use crate::typing::env::environment::ILookupContext;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::rune_typing::rune_type_solver::CitizenRuneTypeSolverLookupResult;
use crate::typing::rune_typing::rune_type_solver::RuneTypingCouldntFindType;
use crate::typing::rune_typing::rune_type_solver::citizen_or_templata_rune_type_lookup;
use crate::utils::fx::HashSet;
use std::iter::empty;
use std::marker::PhantomData;
use crate::parsing::SharednessP;

#[derive(Copy, Clone)]
pub enum IBoundArgumentsSource<'s, 't> {
    InheritBoundsFromTypeItself,
    UseBoundsFromContainer {
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
        instantiation_bound_arguments: &'t InstantiationBoundArgumentsT<'s, 't>,
    },
}

// V: ideas for where to put this?
pub fn is_ref(kind: KindT) -> bool {
    match kind {
        KindT::BorrowRef(b) => true,
        KindT::OwnRef(h) => true,
        KindT::ShareRef(s) => true,
        KindT::WeakRef(w) => true,
        _ => false,
    }
}

// Strips one reference layer, yielding what the reference points at (which may itself be a
// reference). Returns None for a bare kind, since there is nothing to dereference.
pub fn peel_one_reference<'x, 's, 't>(kind: &'x KindT<'s, 't>) -> Option<KindT<'s, 't>> {
    match kind {
        KindT::BorrowRef(b) => Some(b.inner),
        KindT::OwnRef(h) => Some(h.inner),
        KindT::ShareRef(s) => Some(s.inner),
        KindT::WeakRef(w) => Some(w.inner),
        _ => None,
    }
}

// Strips every reference layer, yielding the underlying citizen or primitive regardless of
// how it is referenced. Total: a bare kind is returned unchanged.
pub fn peel_all_references<'s, 't>(kind: KindT<'s, 't>) -> KindT<'s, 't> {
    let mut current = kind;
    while let Some(inner) = peel_one_reference(&current) {
        current = inner;
    }
    current
}

// Strips exactly `n` reference layers, or None if the kind has fewer than `n`. Peeling an argument
// by the parameter's own written wrap depth (rather than all the way) keeps any references the value
// itself carries, so a `&Ship` argument at a bare-rune parameter binds its rune to `&Ship` rather
// than over-peeling to `Ship`. None is the shape mismatch where the argument is shallower than the
// parameter's written wraps, e.g. a bare value at a `&T` parameter.
// VCOORD: get rid of this, this is temporary
pub fn peel_n_references<'s, 't>(kind: KindT<'s, 't>, n: usize) -> Option<KindT<'s, 't>> {
    let mut current = kind;
    for _ in 0..n {
        current = peel_one_reference(&current)?;
    }
    Some(current)
}

// Rebuilds `full_type_with_refs`'s chain of reference layers around `new_value_type`, so the result
// refers to the new value type exactly the way the original referred to its own. Borrow regions
// carry over layer for layer. A bare `full_type_with_refs` has no layers to rebuild, so the new value
// type is returned as-is.
//
// Used where a type's shape is fixed but the citizen inside it changes: an override's parameter
// against the abstract one it implements, or an upcast's result against the expression it wraps.
pub fn replace_value_type_in_ref<'s, 't>(
    interner: &TypingInterner<'s, 't>,
    full_type_maybe_with_refs: KindT<'s, 't>,
    new_value_type: KindT<'s, 't>,
) -> KindT<'s, 't> {
    match full_type_maybe_with_refs {
        KindT::BorrowRef(b) => KindT::BorrowRef(interner.alloc(BorrowRefT {
            inner: replace_value_type_in_ref(interner, b.inner, new_value_type),
            region: b.region,
        })),
        KindT::OwnRef(o) => KindT::OwnRef(interner.alloc(OwnRefT {
            inner: replace_value_type_in_ref(interner, o.inner, new_value_type),
        })),
        KindT::ShareRef(s) => KindT::ShareRef(interner.alloc(ShareRefT {
            inner: replace_value_type_in_ref(interner, s.inner, new_value_type),
        })),
        KindT::WeakRef(w) => KindT::WeakRef(interner.alloc(WeakRefT {
            inner: replace_value_type_in_ref(interner, w.inner, new_value_type),
        })),
        _ => new_value_type,
    }
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_top_level_denizen_id(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let steps = id.steps();
        let is_instantiation_name = |name: &INameT<'s, 't>| -> bool {
            IInstantiationNameT::try_from(*name).is_ok()
        };
        let index = steps.iter().position(is_instantiation_name);
        let index = index.expect("get_top_level_denizen_id: no IInstantiationNameT found in steps");
        let last_step = steps[index];
        assert!(is_instantiation_name(&last_step), "get_top_level_denizen_id: step at index is not IInstantiationNameT");
        let init_steps_slice = self.typing_interner.alloc_slice_copy(&steps[..index]);
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: init_steps_slice,
            local_name: last_step,
        })
    }

    pub fn get_placeholder_templata_id(
        impl_placeholder: ITemplataT<'s, 't>,
    ) -> IdT<'s, 't> {
        match impl_placeholder {
            ITemplataT::Placeholder(pt) => pt.id,
            ITemplataT::Kind(kt) => match kt.kind {
                KindT::KindPlaceholder(kp) => kp.id,
                _ => panic!("vwat: get_placeholder_templata_id unexpected kind: {:?}", kt.kind),
            },
            other => panic!("vwat: get_placeholder_templata_id unexpected templata: {:?}", other),
        }
    }

    // See SFWPRL. Per @DRSINI, this is the only place that eagerly adds default rules.
    // Safe because prediction has no actual arguments being inferred that could conflict.
    pub fn assemble_predict_rules(
        &self,
        generic_parameters: &'s [&'s GenericParameterS<'s>],
        num_explicit_template_args: i32,
    ) -> Vec<IRulexSR<'s>> {
        let mut result: Vec<IRulexSR<'s>> = Vec::new();
        for (index, generic_param) in generic_parameters.iter().enumerate() {
            if (index as i32) >= num_explicit_template_args {
                match &generic_param.default {
                    Some(x) => {
                        for rule in x.rules.iter() {
                            result.push(**rule);
                        }
                    }
                    None => {}
                }
            }
        }
        result
    }

    // Per @DRSINI, default rules are no longer added eagerly here. They're added
    // incrementally by solveForResolving and evaluateGenericFunctionFromCallForPrototype
    // only for runes that remain unsolved after argument inference.
    pub fn assemble_call_site_rules(
        &self,
        rules: &'s [IRulexSR<'s>],
    ) -> Vec<IRulexSR<'s>> {
        rules.iter().copied().filter(|r| include_rule_in_call_site_solve(r)).collect()
    }

    pub fn get_function_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let func_name = IFunctionNameT::try_from(id.local_name)
            .unwrap_or_else(|_| panic!("get_function_template: not a function name: {:?}", id.local_name));
        let template_local: INameT<'s, 't> = ITemplateNameT::from(func_name.template()).into();
        *interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_local,
        })
    }

    pub fn get_citizen_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let local_name = match id.local_name {
            INameT::Struct(s) => {
                match s.template {
                    IStructTemplateNameT::StructTemplate(tmpl) => INameT::StructTemplate(tmpl),
                    IStructTemplateNameT::LambdaCitizenTemplate(tmpl) => INameT::LambdaCitizenTemplate(tmpl),
                    IStructTemplateNameT::AnonymousSubstructTemplate(tmpl) => INameT::AnonymousSubstructTemplate(tmpl),
                }
            }
            INameT::LambdaCitizen(lc) => INameT::LambdaCitizenTemplate(lc.template),
            INameT::Interface(i) => INameT::InterfaceTemplate(i.template),
            INameT::AnonymousSubstruct(a) => INameT::AnonymousSubstructTemplate(a.template),
            _ => panic!("get_citizen_template called with non-citizen name: {:?}", id.local_name),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
    }

    pub fn get_name_template(
        name: INameT<'s, 't>,
    ) -> INameT<'s, 't> {
        match IInstantiationNameT::try_from(name) {
            Ok(x) => INameT::from(x.template()),
            Err(_) => name,
        }
    }

    pub fn get_super_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let new_init_steps: Vec<INameT<'s, 't>> =
            id.init_steps.iter().map(|n| Self::get_name_template(*n)).collect();
        let new_local_name = Self::get_name_template(id.local_name);
        *interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: &new_init_steps,
            local_name: new_local_name,
        })
    }

    pub fn get_root_super_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let mut tentative_id = Self::get_super_template(interner, id);
        loop {
            let contains_lambda = tentative_id.init_steps.iter().any(|n| {
                match n {
                    INameT::LambdaCitizenTemplate(_) => true,
                    INameT::LambdaCallFunctionTemplate(_) => true,
                    INameT::OverrideDispatcherCase(_) => true,
                    _ => false,
                }
            }) || match tentative_id.local_name {
                INameT::LambdaCitizenTemplate(_) => true,
                INameT::LambdaCallFunctionTemplate(_) => true,
                INameT::OverrideDispatcherCase(_) => true,
                _ => false,
            };
            if contains_lambda {
                tentative_id = tentative_id.init_id(interner);
            } else {
                return tentative_id;
            }
        }
    }

    pub fn get_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> &'t IdT<'s, 't> {
        let last = IInstantiationNameT::try_from(id.local_name).unwrap();
        interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps, //.map(getNameTemplate), // See GLIOGN for why we map the initSteps names too
            local_name: INameT::from(last.template()),
        })
    }

    pub fn get_sub_kind_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let last = IInstantiationNameT::try_from(id.local_name)
            .unwrap_or_else(|_| panic!("get_sub_kind_template: unexpected local_name {:?}", id.local_name));
        let template_name = INameT::from(last.template());
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_name,
        })
    }

    pub fn get_super_kind_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let last = ISuperKindNameT::try_from(id.local_name)
            .unwrap_or_else(|_| panic!("get_super_kind_template: unexpected local_name {:?}", id.local_name));
        let template_name = INameT::from(ITemplateNameT::from(last.template()));
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_name,
        })
    }

    pub fn get_struct_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let local_name = match id.local_name {
            INameT::Struct(s) => {
                match s.template {
                    IStructTemplateNameT::StructTemplate(tmpl) => INameT::StructTemplate(tmpl),
                    IStructTemplateNameT::LambdaCitizenTemplate(tmpl) => INameT::LambdaCitizenTemplate(tmpl),
                    IStructTemplateNameT::AnonymousSubstructTemplate(tmpl) => INameT::AnonymousSubstructTemplate(tmpl),
                }
            }
            INameT::LambdaCitizen(lc) => INameT::LambdaCitizenTemplate(lc.template),
            INameT::AnonymousSubstruct(a) => INameT::AnonymousSubstructTemplate(a.template),
            _ => panic!("get_struct_template called with non-struct name: {:?}", id.local_name),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
    }

    pub fn get_interface_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let local_name = match id.local_name {
            INameT::Interface(i) => INameT::InterfaceTemplate(i.template),
            _ => panic!("get_interface_template called with non-interface name"),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name,
        })
    }

    pub fn get_export_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10");
        // val IdT(packageCoord, initSteps, last) = id
        // IdT(packageCoord, initSteps, last.template)
    }

    pub fn get_extern_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10");
        // val IdT(packageCoord, initSteps, last) = id
        // IdT(packageCoord, initSteps, last.template)
    }

    pub fn get_impl_template(
        interner: &TypingInterner<'s, 't>,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        let IdT { package_coord, init_steps, local_name, .. } = id;
        let impl_name = IImplNameT::try_from(local_name).expect("get_impl_template: not an impl name");
        let template = INameT::from(impl_name.template());
        *interner.intern_id(crate::typing::names::names::IdValT { package_coord, init_steps, local_name: template })
    }

    pub fn get_placeholder_template(
        &self,
        id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        // val IdT(packageCoord, initSteps, last) = id
        // IdT(packageCoord, initSteps, last.template)
        let template_name = match id.local_name {
            INameT::KindPlaceholder(kp) => INameT::KindPlaceholderTemplate(kp.template),
            _ => panic!("get_placeholder_template: unexpected local_name"),
        };
        *self.typing_interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: template_name,
        })
    }

    pub fn assemble_rune_to_function_bound(
        &self,
        templatas: &'t TemplatasStoreT<'s, 't>,
    ) -> HashMap<IRuneS<'s>, &'t PrototypeT<'s, 't>> {
        let mut result = HashMap::default();
        for (name, entry) in templatas.name_to_entry.iter() {
            match (name, entry) {
                (INameT::Rune(rune_name), IEnvEntryT::Templata(ITemplataT::Prototype(proto_templata))) => {
                    match &proto_templata.prototype.id.local_name {
                        INameT::FunctionBound(_) => {
                            result.insert(rune_name.rune, proto_templata.prototype);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        result
    }

    pub fn assemble_rune_to_impl_bound(
        &self,
        templatas: &'t TemplatasStoreT<'s, 't>,
    ) -> HashMap<IRuneS<'s>, IdT<'s, 't>> {
        let mut result = HashMap::default();
        for (name, entry) in templatas.name_to_entry.iter() {
            match (name, entry) {
                (INameT::Rune(rune_name), IEnvEntryT::Templata(ITemplataT::Isa(isa))) => {
                    match isa.impl_name.local_name {
                        INameT::ImplBound(_) => {
                            result.insert(rune_name.rune, isa.impl_name);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        result
    }

    pub fn substitute_templatas_in_kind(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        kind: KindT<'s, 't>,
    ) -> KindT<'s, 't> {
        match kind {
            KindT::Int(_) => kind,
            KindT::Bool(_) => kind,
            KindT::Str(_) => kind,
            KindT::Float(_) => kind,
            KindT::USize(_) => kind,
            KindT::Void(_) => kind,
            KindT::Never(_) => kind,
            KindT::RuntimeSizedArray(rsa) => {
                let INameT::RuntimeSizedArray(rsa_name) = rsa.name.local_name else { panic!("vwat") };
                let new_arr_name = interner.intern_raw_array_name(RawArrayNameT {
                    element_type: Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, rsa_name.arr.element_type),
                    self_region: RegionT::Default,
                });
                let new_rsa_name = interner.intern_runtime_sized_array_name(RuntimeSizedArrayNameT {
                    template: rsa_name.template,
                    arr: new_arr_name,
                });
                let new_id = *interner.intern_id(IdValT {
                    package_coord: rsa.name.package_coord,
                    init_steps: rsa.name.init_steps,
                    local_name: INameT::RuntimeSizedArray(new_rsa_name),
                });
                let new_rsa = interner.intern_runtime_sized_array_tt(RuntimeSizedArrayTTValT { name: new_id });
                KindT::RuntimeSizedArray(new_rsa)
            }
            KindT::StaticSizedArray(ssa) => {
                let INameT::StaticSizedArray(ssa_name) = ssa.name.local_name else { panic!("vwat") };
                let new_arr_name = interner.intern_raw_array_name(RawArrayNameT {
                    element_type: Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, ssa_name.arr.element_type),
                    self_region: RegionT::Default,
                });
                let new_ssa_name = interner.intern_static_sized_array_name(StaticSizedArrayNameT {
                    template: ssa_name.template,
                    size: expect_integer(Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, ssa_name.size)),
                    arr: new_arr_name,
                });
                let new_id = *interner.intern_id(IdValT {
                    package_coord: ssa.name.package_coord,
                    init_steps: ssa.name.init_steps,
                    local_name: INameT::StaticSizedArray(new_ssa_name),
                });
                let new_ssa = interner.intern_static_sized_array_tt(StaticSizedArrayTTValT { name: new_id });
                KindT::StaticSizedArray(new_ssa)
            }
            KindT::KindPlaceholder(p) => {
                let index = match p.id.local_name {
                    INameT::KindPlaceholder(kp) => kp.template.index,
                    _ => panic!("KindPlaceholderT has non-KindPlaceholder local_name"),
                };
                if p.id.init_id(interner) == needle_template_name {
                    expect_kind_templata(new_substituting_templatas[index as usize]).kind
                } else {
                    // VCOORD: investigate what this case is for... suspicious fallback-looking thing
                    kind
                }
            }
            KindT::Struct(s) => {
                let new_struct = Compiler::substitute_templatas_in_struct(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, s);
                KindT::Struct(new_struct)
            }
            KindT::Interface(i) => {
                let new_interface = Compiler::substitute_templatas_in_interface(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, i);
                KindT::Interface(new_interface)
            }
            KindT::OverloadSet(_) => unreachable!("an OverloadSet cannot appear as a substantive kind here"),
            // Substitution reaches through a reference without disturbing it, so each wrap is
            // rebuilt around the substituted inner. A borrow keeps its own region: substituting
            // `T` in `&T` changes what is pointed at, never which group points at it.
            KindT::BorrowRef(b) => KindT::BorrowRef(interner.alloc(BorrowRefT {
                inner: Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, b.inner),
                region: b.region,
            })),
            KindT::OwnRef(o) => KindT::OwnRef(interner.alloc(OwnRefT {
                inner: Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, o.inner),
            })),
            KindT::ShareRef(s) => KindT::ShareRef(interner.alloc(ShareRefT {
                inner: Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, s.inner),
            })),
            KindT::WeakRef(w) => KindT::WeakRef(interner.alloc(WeakRefT {
                inner: Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, w.inner),
            })),

            // // VCOORD: revisit
            // // Composition of substituted ownership. `Borrow + share-kind` is distinct
            // // from `Share T` — Borrow-over-Share preserves the Borrow flavor (`&Share T`),
            // // Share-over-anything stays Share, Own-over-Share stays Share (no way to Own
            // // a shared kind).
            // let result_ownership = match (ownership, c.coord.ownership) {
            //     (OwnershipT::Share, _) => OwnershipT::Share,
            //     (OwnershipT::Own, OwnershipT::Share) => OwnershipT::Share,
            //     (OwnershipT::Borrow, OwnershipT::Share) => OwnershipT::Borrow,
            //     (OwnershipT::Own, OwnershipT::Own) => OwnershipT::Own,
            //     (OwnershipT::Own, OwnershipT::Borrow) => OwnershipT::Borrow,
            //     (OwnershipT::Borrow, OwnershipT::Own) => OwnershipT::Borrow,
            //     (OwnershipT::Borrow, OwnershipT::Borrow) => OwnershipT::Borrow,
            //     _ => unreachable!("remaining Weak-on-substituting-side ownership pairs are degenerate"),
            // };
            // KindT::new(result_ownership, result_region, c.coord.kind)
        }
    }

    pub fn substitute_templatas_in_struct(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        struct_tt: &'t StructTT<'s, 't>,
    ) -> &'t StructTT<'s, 't> {
        let id = struct_tt.id;
        let new_local_name = match id.local_name {
            INameT::AnonymousSubstruct(asub_name_t) => {
                let new_template_args: Vec<ITemplataT<'s, 't>> = asub_name_t.template_args.iter()
                    .map(|templata| Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata))
                    .collect();
                let new_template_args_ref = interner.alloc_slice_from_vec(new_template_args);
                interner.intern_name(INameValT::AnonymousSubstruct(AnonymousSubstructNameValT {
                    template: asub_name_t.template,
                    template_args: new_template_args_ref,
                }))
            }
            INameT::Struct(struct_name_t) => {
                let new_template_args: Vec<ITemplataT<'s, 't>> = struct_name_t.template_args.iter()
                    .map(|templata| Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata))
                    .collect();
                let new_template_args_ref = interner.alloc_slice_from_vec(new_template_args);
                interner.intern_name(INameValT::Struct(StructNameValT {
                    template: struct_name_t.template,
                    template_args: new_template_args_ref,
                }))
            }
            INameT::LambdaCitizen(lambda_citizen_name_t) => {
                INameT::LambdaCitizen(lambda_citizen_name_t)
            }
            _ => unreachable!("exhaustive over AnonymousSubstructNameT/StructNameT/LambdaCitizenNameT"),
        };
        let new_id = interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: new_local_name,
        });
        let new_struct = interner.intern_struct_tt(StructTTValT { id: *new_id });
        // See SBITAFD, we need to register bounds for these new instantiations.
        let instantiation_bound_args = coutputs.get_instantiation_bounds(interner, struct_tt.id).unwrap();
        let translated_bounds = interner.alloc(Self::translate_instantiation_bounds(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, instantiation_bound_args));
        coutputs.add_instantiation_bounds(
            sanity_check, interner,
            original_calling_denizen_id,
            new_struct.id,
            translated_bounds);
        new_struct
    }

    pub fn translate_instantiation_bounds(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        instantiation_bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> InstantiationBoundArgumentsT<'s, 't> {
        match bound_arguments_source {
            IBoundArgumentsSource::InheritBoundsFromTypeItself => {
                let x = Self::substitute_templatas_in_bounds(
                    coutputs, sanity_check, interner, keywords,
                    original_calling_denizen_id, needle_template_name,
                    new_substituting_templatas, bound_arguments_source,
                    instantiation_bound_args);
                x
            }
            IBoundArgumentsSource::UseBoundsFromContainer { instantiation_bound_params: container_instantiation_bound_params, instantiation_bound_arguments: container_instantiation_bound_args } => {
                let container_func_bound_to_bound_arg: HashMap<PrototypeT<'s, 't>, PrototypeT<'s, 't>> =
                    container_instantiation_bound_args.rune_to_bound_prototype.iter()
                        .map(|(rune, container_func_bound_arg)| {
                            let param_proto = *container_instantiation_bound_params.rune_to_bound_prototype.get(rune).unwrap();
                            (param_proto, *container_func_bound_arg)
                        })
                        .collect();
                let container_impl_bound_to_bound_arg: HashMap<IdT<'s, 't>, IdT<'s, 't>> =
                    container_instantiation_bound_args.rune_to_bound_impl.iter()
                        .map(|(rune, container_impl_bound_arg)| {
                            let param_impl = *container_instantiation_bound_params.rune_to_bound_impl.get(rune).unwrap();
                            (param_impl, *container_impl_bound_arg)
                        })
                        .collect();
                let rune_to_bound_prototype = interner.alloc_index_map_from_iter(
                    instantiation_bound_args.rune_to_bound_prototype.iter().map(|(rune, func_bound_arg)| {
                        let new_val = match func_bound_arg.id.local_name {
                            INameT::FunctionBound(_) => {
                                *container_func_bound_to_bound_arg.get(func_bound_arg).unwrap()
                            }
                            _ => {
                                // Not sure if this call is really necessary...
                                *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, func_bound_arg)
                            }
                        };
                        (*rune, new_val)
                    }));
                let rune_to_citizen_rune_to_reachable_prototype = interner.alloc_index_map_from_iter(
                    instantiation_bound_args.rune_to_citizen_rune_to_reachable_prototype.iter().map(|(callee_rune, reachable_bound_args)| {
                        let new_citizen = interner.alloc_index_map_from_iter(
                            reachable_bound_args.citizen_rune_to_reachable_prototype.iter().map(|(citizen_rune, reachable_prototype)| {
                                let new_val = match reachable_prototype.id.local_name {
                                    INameT::FunctionBound(_) => {
                                        *container_func_bound_to_bound_arg.get(reachable_prototype).unwrap()
                                    }
                                    _ => {
                                        // Not sure if this call is really necessary...
                                        *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, reachable_prototype)
                                    }
                                };
                                (*citizen_rune, new_val)
                            }));
                        let new_reachable: &'t InstantiationReachableBoundArgumentsT<'s, 't> = interner.alloc(InstantiationReachableBoundArgumentsT { citizen_rune_to_reachable_prototype: new_citizen });
                        (*callee_rune, new_reachable)
                    }));
                let rune_to_bound_impl = interner.alloc_index_map_from_iter(
                    instantiation_bound_args.rune_to_bound_impl.iter().map(|(rune, impl_bound_arg)| {
                        let new_val = match impl_bound_arg.local_name {
                            INameT::ImplBound(_) => {
                                *container_impl_bound_to_bound_arg.get(impl_bound_arg).unwrap()
                            }
                            _ => {
                                // Not sure if this call is really necessary...
                                Self::substitute_templatas_in_impl_id(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *impl_bound_arg)
                            }
                        };
                        (*rune, new_val)
                    }));
                InstantiationBoundArgumentsT {
                    rune_to_bound_prototype,
                    rune_to_citizen_rune_to_reachable_prototype,
                    rune_to_bound_impl,
                }
            }
        }
    }

    pub fn substitute_templatas_in_impl_id(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        impl_id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10");
        // val IdT(packageCoord, initSteps, last) = implId
        // val substitutedImplId = IdT(packageCoord, initSteps, last match {
        //   case ImplNameT(template, templateArgs, subCitizen) => interner.intern(ImplNameT(template,
        //     templateArgs.map(t => substituteTemplatasInTemplata(...)),
        //     expectKindTemplata(substituteTemplatasInKind(...)).kind.expectCitizen()))
        //   case other => vimpl(other)
        // })
        // // ... addInstantiationBounds, return substitutedImplId
    }

    pub fn substitute_templatas_in_bounds(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> InstantiationBoundArgumentsT<'s, 't> {
        let rune_to_bound_prototype = interner.alloc_index_map_from_iter(
            bound_args.rune_to_bound_prototype.iter().map(|(rune, func_bound_arg)| {
                (*rune, *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, func_bound_arg))
            }));
        let rune_to_citizen_rune_to_reachable_prototype = interner.alloc_index_map_from_iter(
            bound_args.rune_to_citizen_rune_to_reachable_prototype.iter().map(|(caller_rune, reachable_bound_args)| {
                let new_citizen_rune_to_reachable_prototype = interner.alloc_index_map_from_iter(
                    reachable_bound_args.citizen_rune_to_reachable_prototype.iter().map(|(citizen_rune, reachable_prototype)| {
                        (*citizen_rune, *Self::substitute_templatas_in_prototype(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, reachable_prototype))
                    }));
                let new_reachable: &'t InstantiationReachableBoundArgumentsT<'s, 't> = interner.alloc(InstantiationReachableBoundArgumentsT { citizen_rune_to_reachable_prototype: new_citizen_rune_to_reachable_prototype });
                (*caller_rune, new_reachable)
            }));
        let rune_to_bound_impl = interner.alloc_index_map_from_iter(
            bound_args.rune_to_bound_impl.iter().map(|(rune, impl_bound_arg)| {
                (*rune, Self::substitute_templatas_in_impl_id(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *impl_bound_arg))
            }));
        InstantiationBoundArgumentsT {
            rune_to_bound_prototype,
            rune_to_citizen_rune_to_reachable_prototype,
            rune_to_bound_impl,
        }
    }

    pub fn substitute_templatas_in_interface(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        interface_tt: &'t InterfaceTT<'s, 't>,
    ) -> &'t InterfaceTT<'s, 't> {
        let id = interface_tt.id;
        let new_local_name = match id.local_name {
            INameT::Interface(interface_name_t) => {
                let new_template_args: Vec<ITemplataT<'s, 't>> = interface_name_t.template_args.iter()
                    .map(|templata| Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata))
                    .collect();
                let new_template_args_ref = interner.alloc_slice_from_vec(new_template_args);
                interner.intern_name(INameValT::Interface(InterfaceNameValT {
                    template: interface_name_t.template,
                    template_args: new_template_args_ref,
                }))
            }
            _ => unreachable!("exhaustive over InterfaceNameT only"),
        };
        let new_id = interner.intern_id(IdValT {
            package_coord: id.package_coord,
            init_steps: id.init_steps,
            local_name: new_local_name,
        });
        let new_interface = interner.intern_interface_tt(InterfaceTTValT { id: *new_id });
        // See SBITAFD, we need to register bounds for these new instantiations.
        let instantiation_bound_args = coutputs.get_instantiation_bounds(interner, interface_tt.id).unwrap();
        let translated_bounds = interner.alloc(Self::translate_instantiation_bounds(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, instantiation_bound_args));
        coutputs.add_instantiation_bounds(
            sanity_check, interner,
            original_calling_denizen_id,
            new_interface.id,
            translated_bounds);
        new_interface
    }

    pub fn substitute_templatas_in_templata(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        templata: ITemplataT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        match templata {
            ITemplataT::Kind(c) => ITemplataT::Kind(interner.alloc(KindTemplataT { kind: Compiler::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, c.kind) })),
            ITemplataT::Placeholder(p) => {
                let pn = IPlaceholderNameT::try_from(p.id.local_name).unwrap();
                if p.id.init_id(interner) == needle_template_name {
                    new_substituting_templatas[pn.index() as usize]
                } else {
                    templata
                }
            }
            ITemplataT::Integer(_) => templata,
            ITemplataT::Boolean(_) => templata,
            ITemplataT::Prototype(p) => {
                panic!("Unimplemented: substitute_templatas_in_templata Prototype");
                // PrototypeTemplataT(substituteTemplatasInPrototype(...))
            }
            _ => panic!("vimpl: substitute_templatas_in_templata unexpected templata"),
        }
    }

    pub fn substitute_templatas_in_prototype(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        original_prototype: &'t PrototypeT<'s, 't>,
    ) -> &'t PrototypeT<'s, 't> {
        let package_coord = original_prototype.id.package_coord;
        let init_steps = original_prototype.id.init_steps;
        let func_name = IFunctionNameT::try_from(original_prototype.id.local_name).unwrap();
        let substituted_template_args_vec: Vec<ITemplataT<'s, 't>> = func_name.template_args().iter().map(|templata| {
            Self::substitute_templatas_in_templata(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *templata)
        }).collect();
        let substituted_template_args = interner.alloc_slice_from_vec(substituted_template_args_vec);
        let substituted_params_vec: Vec<KindT<'s, 't>> = func_name.parameters().iter().map(|coord| {
            Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, *coord)
        }).collect();
        let substituted_params = interner.alloc_slice_from_vec(substituted_params_vec);
        let substituted_return_type = Self::substitute_templatas_in_kind(coutputs, sanity_check, interner, keywords, original_calling_denizen_id, needle_template_name, new_substituting_templatas, bound_arguments_source, original_prototype.return_type);
        let substituted_func_name = func_name.template().make_function_name(interner, keywords, substituted_template_args, substituted_params);
        let tentative_id = *interner.intern_id(IdValT { package_coord, init_steps, local_name: substituted_func_name });
        let perhaps_imported_id = match tentative_id.local_name {
            INameT::FunctionBound(n) => {
                // Always import a seen function bound into our own environment, see MFBFDP.
                let imported_id = *original_calling_denizen_id.add_step(interner, INameT::FunctionBound(n));
                // It's a function bound, it has no function bounds of its own.
                coutputs.add_instantiation_bounds(
                    sanity_check,
                    interner,
                    original_calling_denizen_id,
                    imported_id,
                    interner.alloc(InstantiationBoundArgumentsT {
                        rune_to_bound_prototype: interner.alloc_index_map_from_iter(empty()),
                        rune_to_citizen_rune_to_reachable_prototype: interner.alloc_index_map_from_iter(empty()),
                        rune_to_bound_impl: interner.alloc_index_map_from_iter(empty()),
                    }),
                );
                imported_id
            }
            _ => {
                // Not really sure if we're supposed to add bounds or something here.
                assert!(coutputs.get_instantiation_bounds(interner, tentative_id).is_some());
                tentative_id
            }
        };
        interner.intern_prototype(PrototypeValT {
            id: IdValT { package_coord: perhaps_imported_id.package_coord, init_steps: perhaps_imported_id.init_steps, local_name: perhaps_imported_id.local_name },
            return_type: substituted_return_type,
        })
    }

    pub fn substitute_templatas_in_function_bound_id(
        coutputs: &mut CompilerOutputs<'s, 't>,
        sanity_check: bool,
        interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        original_calling_denizen_id: IdT<'s, 't>,
        needle_template_name: IdT<'s, 't>,
        new_substituting_templatas: &[ITemplataT<'s, 't>],
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
        original: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 10");
        // val IdT(packageCoord, initSteps, funcName) = original
        // val substitutedTemplateArgs =
        //   funcName.templateArgs.map((templata: ITemplataT[ITemplataType]) => substituteTemplatasInTemplata(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, templata))
        // val substitutedParams =
        //   funcName.parameters.map((coord: CoordT) => substituteTemplatasInCoord(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, coord))
        // val substitutedFuncName = funcName.template.makeFunctionName(interner, keywords, substitutedTemplateArgs, substitutedParams)
        // val newId = IdT(packageCoord, initSteps, substitutedFuncName)
        // coutputs.addInstantiationBounds(
        //   sanityCheck, interner, originalCallingDenizenId, newId,
        //   InstantiationBoundArgumentsT.make(Map(), Map(), Map()))
        // newId
    }
}

// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)

pub struct IPlaceholderSubstituter<'s, 'ctx, 't> {
    pub sanity_check: bool,
    pub interner: &'ctx TypingInterner<'s, 't>,
    pub keywords: &'ctx Keywords<'s>,
    pub original_calling_denizen_id: IdT<'s, 't>,
    pub needle_template_name: IdT<'s, 't>,
    pub new_substituting_templatas: &'t [ITemplataT<'s, 't>],
    pub bound_arguments_source: IBoundArgumentsSource<'s, 't>,
}
impl<'s, 'ctx, 't> IPlaceholderSubstituter<'s, 'ctx, 't> {
    
    pub fn substitute_for_kind(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        coord_t: KindT<'s, 't>,
    ) -> KindT<'s, 't> {
        Compiler::substitute_templatas_in_kind(
            coutputs,
            self.sanity_check,
            self.interner,
            self.keywords,
            self.original_calling_denizen_id,
            self.needle_template_name,
            self.new_substituting_templatas,
            self.bound_arguments_source,
            coord_t,
        )
    }
    
    pub fn substitute_for_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        interface_tt: InterfaceTT<'s, 't>,
    ) -> InterfaceTT<'s, 't> {
        panic!("Unimplemented: Slab 15");
        // Compiler.substituteTemplatasInInterface(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, interfaceTT)
    }
    
    pub fn substitute_for_templata(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        templata: ITemplataT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        Compiler::substitute_templatas_in_templata(
            coutputs,
            self.sanity_check,
            self.interner,
            self.keywords,
            self.original_calling_denizen_id,
            self.needle_template_name,
            self.new_substituting_templatas,
            self.bound_arguments_source,
            templata,
        )
    }
    
    pub fn substitute_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        proto: &'t PrototypeT<'s, 't>,
    ) -> &'t PrototypeT<'s, 't> {
        Compiler::substitute_templatas_in_prototype(
            coutputs,
            self.sanity_check,
            self.interner,
            self.keywords,
            self.original_calling_denizen_id,
            self.needle_template_name,
            self.new_substituting_templatas,
            self.bound_arguments_source,
            proto,
        )
    }
    
    pub fn substitute_for_impl_id(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        impl_id: IdT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 15");
        // Compiler.substituteTemplatasInImplId(coutputs, sanityCheck, interner, keywords, originalCallingDenizenId, needleTemplateName, newSubstitutingTemplatas, boundArgumentsSource, implId)
    }
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_placeholder_substituter(
        &self,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        name: IdT<'s, 't>,
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
    ) -> IPlaceholderSubstituter<'s, 'ctx, 't> {
        let top_level_denizen_id = self.get_top_level_denizen_id(name);
        let top_level_local_name: IInstantiationNameT<'s, 't> =
            top_level_denizen_id.local_name.try_into()
                .unwrap_or_else(|_| panic!("get_placeholder_substituter: topLevelDenizenId.localName must be IInstantiationNameT, got {:?}", top_level_denizen_id.local_name));
        let template_args: &[ITemplataT<'s, 't>] = top_level_local_name.template_args();
        let top_level_denizen_template_id = Compiler::get_template(self.typing_interner, top_level_denizen_id);
        let needle_template_name = *top_level_denizen_template_id;
        IPlaceholderSubstituter {
            sanity_check,
            interner: self.typing_interner,
            keywords: self.keywords,
            original_calling_denizen_id,
            needle_template_name,
            new_substituting_templatas: template_args,
            bound_arguments_source,
        }
    }

    pub fn get_reachable_bounds(
        &self,
        sanity_check: bool,
        original_calling_denizen_id: IdT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        citizen: ICitizenTT<'s, 't>,
    ) -> (InstantiationReachableBoundArgumentsT<'s, 't>, IndexMap<IRuneS<'s>, Vec<KindT<'s, 't>>>) {
        let citizen_id = match citizen {
            ICitizenTT::Struct(s) => s.id,
            ICitizenTT::Interface(i) => i.id,
        };
        let substituter =
            self.get_placeholder_substituter(
                sanity_check,
                original_calling_denizen_id,
                citizen_id,
                IBoundArgumentsSource::InheritBoundsFromTypeItself,
            );
        let citizen_template_id = self.get_citizen_template(citizen_id);

        // VCOORD: turn this into a helper
        let (foreign_generic_runes, foreign_citizen_header_rules) = match citizen {
            ICitizenTT::Struct(_) => {
                let struct_s = coutputs.get_postparsed_struct(&citizen_template_id);
                let runes: Vec<_> = struct_s.generic_params.iter().map(|generic_param| generic_param.rune.rune).collect();
                (runes, struct_s.header_rules)
            }
            ICitizenTT::Interface(_) => {
                let interface_s = coutputs.get_postparsed_interface(&citizen_template_id);
                let runes: Vec<_> = interface_s.generic_params.iter().map(|generic_param| generic_param.rune.rune).collect();
                (runes, interface_s.rules)
            }
        };
        // VCOORD: it's weird that here we're making a map of foreign rune to local conclusions,
        // because that's kind of what a substituter *is*. Substituter just works off of
        // indices though. We should either make a new ByNameSusbtituter, or augment Substituter,
        // or do something here.
        let foreign_rune_to_conclusions: IndexMap<IRuneS<'s>, ITemplataT<'s, 't>> =
            foreign_generic_runes.into_iter()
                .zip(substituter.new_substituting_templatas.iter().map(|t| *t))
                .collect();
        // This map contains, for each foreign resolve rule (well, its result_rune which identifies it)
        // which local conclusions its type uses.
        let foreign_resolve_rule_rune_to_mentioned_conclusions =
            foreign_citizen_header_rules
                .iter()
                .filter_map(|rule| match rule {
                    IRulexSR::Resolve(resolve_sr) => {
                        // For this foreign ResolveSR rule, here's the local conclusions mentioned in it.
                        let bound_search_kinds =
                            collect_bound_search_kinds(resolve_sr, &foreign_rune_to_conclusions);
                        Some((resolve_sr.result_rune.rune, bound_search_kinds))
                    },
                    _ => None,
                })
                .collect();

        let inner_env = coutputs.get_inner_env_for_type(citizen_template_id);
        let citizen_rune_to_reachable_prototype: Vec<(IRuneS<'s>, PrototypeT<'s, 't>)> =
            inner_env.templatas().name_to_entry.iter()
                .filter_map(|(name, entry)| {
                    match (name, entry) {
                        (INameT::Rune(rune_name), IEnvEntryT::Templata(ITemplataT::Prototype(proto_tt))) => {
                            match proto_tt.prototype.id.local_name {
                                INameT::FunctionBound(_) => {
                                    let substituted = substituter.substitute_for_prototype(coutputs, proto_tt.prototype);
                                    Some((rune_name.rune, *substituted))
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                })
                .collect();

        let reachable = InstantiationReachableBoundArgumentsT {
            citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map_from_iter(
                citizen_rune_to_reachable_prototype.into_iter()),
        };
        (reachable, foreign_resolve_rule_rune_to_mentioned_conclusions)
    }

    pub fn get_first_unsolved_identifying_rune(
        &self,
        generic_parameters: &'s [&'s GenericParameterS<'s>],
        is_solved: impl Fn(IRuneS<'s>) -> bool,
    ) -> Option<(&'s GenericParameterS<'s>, i32)> {
        generic_parameters.iter().enumerate()
            .map(|(index, generic_param)| (generic_param, index as i32, is_solved(generic_param.rune.rune)))
            .filter(|(_, _, solved)| !solved)
            .map(|(generic_param, index, _)| (*generic_param, index))
            .next()
    }

    pub fn create_rune_type_solver_env(
        &self,
        parent_env: IInDenizenEnvironmentT<'s, 't>,
    ) -> TemplataCompilerRuneTypeSolverEnv<'_, 's, 't> {
        TemplataCompilerRuneTypeSolverEnv {
            parent_env,
            typing_interner: self.typing_interner,
            scout_arena: self.scout_arena,
        }
    }

}


pub struct TemplataCompilerRuneTypeSolverEnv<'a, 's, 't>
where
    's: 't,
{
    parent_env: IInDenizenEnvironmentT<'s, 't>,
    typing_interner: &'a TypingInterner<'s, 't>,
    scout_arena: &'a ScoutArena<'s>,
}


impl<'a, 's, 't> IRuneTypeSolverEnv<'s, 't>
for TemplataCompilerRuneTypeSolverEnv<'a, 's, 't>
where
    's: 't,
{
    fn lookup(
        &self,
        coutputs: &CompilerOutputs<'s, 't>,
        range: RangeS<'s>,
        parts: &[IImpreciseNameS<'s>],
    ) -> Result<
        IRuneTypeSolverLookupResult<'s>,
        IRuneTypingLookupFailedError<'s>,
    > {
        // The last segment names the item; only diagnostics and the lambda-struct arm need it
        // separately from the path.
        let name_s = *parts.last().expect("vwat: an empty lookup path");
        match name_s {
            // VCOORD: remove this entire branch and see if it just works, it might
            IImpreciseNameS::LambdaStructImpreciseName(_) => {
                Ok(IRuneTypeSolverLookupResult::Templata(
                    TemplataLookupResult {
                        templata: ITemplataType::KindTemplataType(
                            KindTemplataType {},
                        ),
                    },
                ))
            }
            _ => {
                let mut filter = HashSet::default();
                filter.insert(ILookupContext::TemplataLookupContext);
                let found = lookup_nearest_with_path(
                    IEnvironmentT::from(self.parent_env), parts, filter, self.typing_interner);
                citizen_or_templata_rune_type_lookup(coutputs, self.scout_arena, found, range, name_s)
            }

        }
    }

}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    // A `&X -> X` read-out copies X out of the borrow with no user-written clone, via a CopyPrim.
    // True for primitives and str (KindT::is_implicitly_cloneable), and also for a bare `share`
    // citizen (an RC bump). The citizen case needs the sharedness query, hence a Compiler method.
    // VCOORD: temporary; once share citizens lower to ShareRef uniformly this collapses.
    // VCOORD: unify this with other helpers perhaps
    pub fn kind_is_implicitly_cloneable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        kind: KindT<'s, 't>,
    ) -> bool {
        if kind.is_implicitly_cloneable() {
            return true;
        }
        match kind {
            KindT::Struct(s) => coutputs.lookup_struct(s.id, self).sharedness == SharednessT::Shared,
            KindT::Interface(i) => coutputs.lookup_interface(*i, self).sharedness == SharednessT::Shared,
            _ => false,
        }
    }

    pub fn is_type_convertible(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_type: KindT<'s, 't>,
        target_type: KindT<'s, 't>,
    ) -> bool {
        // Both borrow refs: convertibility is decided entirely by the referents (convert() row 4 /
        // the &Dog -> &Animal upcast at convert_helper.rs:86). Regions are ignored for now — every
        // borrow is RegionT::Default. Recursing also handles nested upcasts (&&Dog -> &&Animal).
        // VCOORD: when genuine double-borrows land (generics only, decision 3), this must refuse a
        // depth mismatch — `&&X -> &X` should be the row-d error, but recursing here would peel it to
        // the legal `&X -> X` read-out and wrongly accept it. // VCOORD: rewrite this comment
        if let (KindT::BorrowRef(s), KindT::BorrowRef(t)) = (source_type, target_type) {
            return self.is_type_convertible(coutputs, calling_env, parent_ranges, call_location, s.inner, t.inner);
        }
        if let (KindT::ShareRef(s), KindT::ShareRef(t)) = (source_type, target_type) {
            return self.is_type_convertible(coutputs, calling_env, parent_ranges, call_location, s.inner, t.inner);
        }

        // VCOORD: revisit this
        if let KindT::BorrowRef(sb) = source_type {
            if sb.inner == target_type {
                match sb.inner {
                    p if p.is_primitive() => return true,
                    // VCOORD: replace this with an "is implicitly cloneable" check
                    KindT::Str(_) => return true,
                    // A bare `share` citizen reads out of a borrow via an RC bump, like str.
                    _ if self.kind_is_implicitly_cloneable(coutputs, sb.inner) => return true,
                    // A non-cloneable read-out (e.g. a plain struct) isn't convertible; the caller
                    // reports the honest "write .clone()" error rather than crashing here.
                    _ => return false,
                }
            }
        }
        if let KindT::BorrowRef(tb) = target_type {
            if source_type == tb.inner {
                return false;
                // panic!("is_type_convertible: bare-to-borrow {:?} -> {:?} not yet handled (needs convert() unification)", source_type, target_type);
            }
        }
        if let KindT::ShareRef(sb) = source_type {
            // This is saying, if we're trying to send a @str into a str.
            // VCOORD: this is temporary, there should never be a bare mention of str.
            if sb.inner == target_type {
                return true;
            }
        }
        if let KindT::ShareRef(tb) = target_type {
            // This is saying, if we're trying to send a str into a @str.
            // VCOORD: this is temporary, there should never be a bare mention of str.
            if source_type == tb.inner {
                return true;
            }
        }
        if let (KindT::ShareRef(sb), KindT::BorrowRef(tb)) = (source_type, target_type) {
            // Borrowing the pointee out of a share handle, e.g. sending a `@str` into a `&str` param.
            if sb.inner == tb.inner {
                return true;
            }
        }
        // /VCOORD

        match (&source_type, &target_type) {
            (KindT::Never(_), _) => return true,
            (a, b) if a == b => {}
            (KindT::Void(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Str(_) | KindT::Float(_)
                | KindT::RuntimeSizedArray(_) | KindT::StaticSizedArray(_), _) => {
                return false;
            }
            (_, KindT::Void(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Str(_) | KindT::Float(_)
                | KindT::RuntimeSizedArray(_) | KindT::StaticSizedArray(_)) => {
                return false;
            }
            (_, KindT::Struct(_)) => return false,
            (a, b) if ISubKindTT::try_from(*a).is_ok() && ISuperKindTT::try_from(*b).is_ok() => {
                let source_sub_kind = ISubKindTT::try_from(source_type).unwrap();
                let target_super_kind = ISuperKindTT::try_from(target_type).unwrap();
                match self.is_parent(coutputs, calling_env, parent_ranges, call_location, source_sub_kind, target_super_kind) {
                    IsParentResult::IsParent(_) => {}
                    IsParentResult::IsntParent(_) => return false,
                }
            }


            // if source_region != target_region {
            //     return false;
            // }
            _ => {

                panic!("vfail: Dont know if we can convert from {:?} to {:?}", source_type, target_type);
            }
        }

        // match (source_ownership, target_ownership) {
        //     (a, b) if a == b => {}
        //     // VCOORD: revisit
        //     // (Own, Borrow) and (Borrow, Own) permitted uniformly; convert() decides
        //     // target-side:
        //     //   (Own, Borrow) → materialize a hidden local + LetAndLend + deferred drop.
        //     //   (Borrow, Own) → probe `implicit_clone(&kind) kind`. If it resolves → emit
        //     //     the auto-clone call; if missing → emit NoImplicitCloneDefinedT.
        //     // "Does implicit_clone exist for this kind" is what actually matters — no
        //     // is_primitive check. Ambiguity between an exact-Own overload and an
        //     // auto-coerce-permitting overload is resolved by narrow_down_callable_overloads'
        //     // "prefer exact match" tiebreaker.
        //     (OwnershipT::Own, OwnershipT::Borrow) => {}
        //     (OwnershipT::Own, OwnershipT::Weak) => return false,
        //     (OwnershipT::Own, OwnershipT::Share) => return false,
        //     (OwnershipT::Borrow, OwnershipT::Own) => {}
        //     (OwnershipT::Borrow, OwnershipT::Weak) => return false,
        //     // VCOORD: revisit
        //     // `Borrow + share-kind` → Share is the auto-alias coercion; convert() emits
        //     // AliasTE. Ambiguity with an exact-Share candidate is handled by
        //     // narrow_down_callable_overloads' "prefer exact match" tiebreaker.
        //     (OwnershipT::Borrow, OwnershipT::Share) => {}
        //     (OwnershipT::Weak, OwnershipT::Own) => return false,
        //     (OwnershipT::Weak, OwnershipT::Borrow) => return false,
        //     (OwnershipT::Weak, OwnershipT::Share) => return false,
        //     (OwnershipT::Share, OwnershipT::Borrow) => return false,
        //     (OwnershipT::Share, OwnershipT::Weak) => return false,
        //     (OwnershipT::Share, OwnershipT::Own) => return false,
        //     _ => unreachable!(),
        // }

        true
    }

    // Picks an ownership tag from a kind's sharedness — the flat-ownership pattern onion
    // typing dissolves. Its sole caller (evaluate_closure) fed only a tautological assert:
    // make_closure_struct_construct_expression derives the same shape from the same
    // lookup_mutability on the same struct. Under onion, a closure struct's
    // bare-vs-ShareRef shape is settled at construction, so nothing recomputes it.
    // pub fn pointify_kind(
    //     &self,
    //     coutputs: &mut CompilerOutputs<'s, 't>,
    //     kind: KindT<'s, 't>,
    //     region: RegionT,
    //     ownership_if_mutable: OwnershipT,
    // ) -> KindT<'s, 't> {
    //     let ownership = match self.get_sharedness(coutputs, kind) {
    //         SharednessT::Single => ownership_if_mutable,
    //         SharednessT::Shared => OwnershipT::Share,
    //     };
    //     match kind {
    //         KindT::RuntimeSizedArray(_) => {
    //             panic!("Unimplemented: pointify_kind RuntimeSizedArray");
    //             // CoordT(ownership, region, a)
    //         }
    //         KindT::StaticSizedArray(_) => {
    //             panic!("Unimplemented: pointify_kind StaticSizedArray");
    //             // CoordT(ownership, region, a)
    //         }
    //         KindT::Struct(_) => KindT::new(ownership, region, kind),
    //         KindT::Interface(_) => KindT::new(ownership, region, kind),
    //         KindT::Void(_) => KindT::new(ownership, region, kind),
    //         KindT::Int(_) => KindT::new(ownership, region, kind),
    //         KindT::Float(_) => KindT::new(ownership, region, kind),
    //         KindT::Bool(_) => KindT::new(ownership, region, kind),
    //         KindT::Str(_) => KindT::new(ownership, region, kind),
    //         _ => unreachable!("pointify_kind is exhaustive over RSA/SSA/Struct/Interface/Void/Int/Float/Bool/Str — Never/OverloadSet/KindPlaceholder not accepted"),
    //     }
    // }

    pub fn lookup_templata_by_name(
        &self,
        env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        name: INameT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        panic!("Unimplemented: Slab 15");
    }

    pub fn lookup_templata_by_rune(
        &self,
        env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        name: IImpreciseNameS<'s>,
    ) -> Option<ITemplataT<'s, 't>> {
        // Changed this from AnythingLookupContext to TemplataLookupContext
        // because this is called from StructCompiler to figure out its members.
        // We could instead pipe a lookup context through, if this proves problematic.
        let mut lookup_filter = HashSet::default();
        lookup_filter.insert(ILookupContext::TemplataLookupContext);
        let results = env.lookup_nearest_with_imprecise_name(name, lookup_filter, self.typing_interner);
        if results.iter().count() > 1 {
            panic!("vfail");
        }
        results
    }

    // pub fn coerce_kind_to_coord(
    //     &self,
    //     coutputs: &mut CompilerOutputs<'s, 't>,
    //     kind: KindT<'s, 't>,
    //     region: RegionT,
    // ) -> KindT<'s, 't> {
    //     let ownership = match self.get_sharedness(coutputs, kind) {
    //         SharednessT::Single => OwnershipT::Own,
    //         SharednessT::Shared => OwnershipT::Share,
    //     };
    //     KindT::new(ownership, region, kind)
    // }

    // pub fn coerce_to_coord(
    //     &self,
    //     coutputs: &mut CompilerOutputs<'s, 't>,
    //     env: IInDenizenEnvironmentT<'s, 't>,
    //     range: &[RangeS<'s>],
    //     templata: ITemplataT<'s, 't>,
    //     region: RegionT,
    // ) -> ITemplataT<'s, 't> {
    //     match templata {
    //         ITemplataT::Kind(kind_templata) => {
    //             ITemplataT::Kind(self.typing_interner.alloc(
    //                 CoordTemplataT { coord: self.coerce_kind_to_coord(coutputs, kind_templata.kind, region) }
    //             ))
    //         }
    //         ITemplataT::Kind(_) => { panic!("vcurious"); }
    //         ITemplataT::StructDefinition(_) => { panic!("vcurious"); }
    //         ITemplataT::InterfaceDefinition(_) => { panic!("vcurious"); }
    //         _ => {
    //             panic!("Unimplemented: coerce_to_coord for {:?}", templata);
    //             // vfail("Can't coerce a " + templata.tyype + " to be a coord!")
    //         }
    //     }
    // }

    pub fn resolve_struct_template(
        &self,
        coutputs: &CompilerOutputs<'s, 't>,
        struct_templata: &'t StructDefinitionTemplataT<'s, 't>,
    ) -> &'t IdT<'s, 't> {
        let declaring_env = struct_templata.declaring_env;
        let struct_a = coutputs.get_postparsed_struct(struct_templata.struct_template_id);
        let translated = self.translate_struct_name(struct_a.name);
        let local_name = match translated {
            IStructTemplateNameT::StructTemplate(r) => INameT::StructTemplate(r),
            IStructTemplateNameT::AnonymousSubstructTemplate(r) => INameT::AnonymousSubstructTemplate(r),
            IStructTemplateNameT::LambdaCitizenTemplate(r) => INameT::LambdaCitizenTemplate(r),
        };
        declaring_env.id().add_step(self.typing_interner, local_name)
    }

    pub fn resolve_interface_template(
        &self,
        coutputs: &CompilerOutputs<'s, 't>,
        interface_templata: &'t InterfaceDefinitionTemplataT<'s, 't>,
    ) -> &'t IdT<'s, 't> {
        let declaring_env = interface_templata.declaring_env;
        let interface_a = coutputs.get_postparsed_interface(interface_templata.interface_template_id);
        let translated = self.translate_interface_name(*interface_a.name);
        let local_name = match translated {
            IInterfaceTemplateNameT::InterfaceTemplate(r) => INameT::InterfaceTemplate(r),
        };
        declaring_env.id().add_step(self.typing_interner, local_name)
    }

    pub fn resolve_citizen_template(
        &self,
        citizen_templata: &'t CitizenDefinitionTemplataT<'s, 't>,
    ) -> IdT<'s, 't> {
        panic!("Unimplemented: Slab 15");
    }

    pub fn citizen_is_from_template(
        &self,
        coutputs: &CompilerOutputs<'s, 't>,
        actual_citizen_ref: ICitizenTT<'s, 't>,
        expected_citizen_templata: ITemplataT<'s, 't>,
    ) -> bool {
        let citizen_template_id = match expected_citizen_templata {
            ITemplataT::StructDefinition(st) => *self.resolve_struct_template(coutputs, st),
            ITemplataT::InterfaceDefinition(it) => *self.resolve_interface_template(coutputs, it),
            ITemplataT::Kind(kt) => {
                match ISubKindTT::try_from(kt.kind) {
                    // VCOORD: doublecheck. ISubKindTT::try_from accepts a KindPlaceholder, so a
                    // generic T passed as expected_citizen_templata reaches get_citizen_template,
                    // which panics on a placeholder id.
                    Ok(sub) => self.get_citizen_template(sub.id()),
                    Err(_) => return false,
                }
            }
            _ => return false,
        };
        self.get_citizen_template(ISubKindTT::from(actual_citizen_ref).id()) == citizen_template_id
    }

    pub fn create_placeholder(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
        name_prefix: IdT<'s, 't>,
        generic_param: &'s GenericParameterS<'s>,
        index: i32,
        rune_to_type: &IndexMap<IRuneS<'s>, ITemplataType<'s>>,
        current_height: Option<i32>,
        register_with_compiler_outputs: bool,
    ) -> ITemplataT<'s, 't> {
        let rune_type = *rune_to_type.get(&generic_param.rune.rune).unwrap();
        let rune = generic_param.rune.rune;
        match rune_type {
            ITemplataType::KindTemplataType(_) => {
                ITemplataT::Kind(self.typing_interner.alloc(self.create_kind_placeholder_inner(
                    coutputs, env, name_prefix, index, rune, register_with_compiler_outputs)))
            }
            // ITemplataType::KindTemplataType(_) => {
                // let (kind_mutable, region_mutability) = match &generic_param.tyype {
                    // IGenericParameterTypeS::CoordGenericParameterType(CoordGenericParameterTypeS { kind_mutable, region_mutable, .. }) => {
                        // (if *kind_mutable { OwnershipT::Own } else { OwnershipT::Share },
                         // if *region_mutable { IRegionMutabilityS::ReadWriteRegion } else { IRegionMutabilityS::ReadOnlyRegion })
                    // }
                    // _ => (OwnershipT::Own, IRegionMutabilityS::ReadOnlyRegion),
                // };
                // ITemplataT::Kind(self.typing_interner.alloc(self.create_coord_placeholder_inner(
                    // coutputs, env, name_prefix, index, rune, current_height,
                    // region_mutability, kind_mutable, register_with_compiler_outputs)))
            // }
            other_type => {
                self.create_non_kind_non_region_placeholder_inner(name_prefix, index, rune, other_type)
            }
        }
    }

    pub fn create_kind_placeholder_inner(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
        name_prefix: IdT<'s, 't>,
        index: i32,
        rune: IRuneS<'s>,
        register_with_compiler_outputs: bool,
    ) -> KindTemplataT<'s, 't> {
        // val kindPlaceholderId =
        //   namePrefix.addStep(
        //     interner.intern(KindPlaceholderNameT(
        //       interner.intern(KindPlaceholderTemplateNameT(index, rune)))))
        let template_name = self.typing_interner.intern_kind_placeholder_template_name(
            KindPlaceholderTemplateNameT { index, rune});
        let placeholder_name = self.typing_interner.intern_kind_placeholder_name(
            KindPlaceholderNameT { template: template_name });
        let kind_placeholder_id = name_prefix.add_step(
            self.typing_interner, INameT::KindPlaceholder(placeholder_name));

        // val kindPlaceholderTemplateId =
        //   TemplataCompiler.getPlaceholderTemplate(kindPlaceholderId)
        let kind_placeholder_template_id_val = self.get_placeholder_template(*kind_placeholder_id);
        let kind_placeholder_template_id = self.typing_interner.intern_id(IdValT {
            package_coord: kind_placeholder_template_id_val.package_coord,
            init_steps: kind_placeholder_template_id_val.init_steps,
            local_name: kind_placeholder_template_id_val.local_name,
        });

        // if (registerWithCompilerOutputs) {
        if register_with_compiler_outputs {
            // coutputs.declareType(kindPlaceholderTemplateId)
            coutputs.declare_type(kind_placeholder_template_id);

            // Per @BDPFWDZ: the placeholder env stays empty. Bound declarations
            // (IsaTemplataT, FunctionBoundNameT) live in the introducing function's near-env, not
            // here. Lookups walk from the calling env to find them.
            // val placeholderEnv = GeneralEnvironmentT.childOf(interner, env, kindPlaceholderTemplateId, kindPlaceholderTemplateId)
            let placeholder_env = child_of(
                self.typing_interner,
                self.scout_arena,
                env,
                *kind_placeholder_template_id,
                kind_placeholder_template_id,
                vec![],
            );
            let placeholder_env_ref: IInDenizenEnvironmentT<'s, 't> =
                IInDenizenEnvironmentT::General(placeholder_env);
            // coutputs.declareTypeOuterEnv(kindPlaceholderTemplateId, placeholderEnv)
            coutputs.declare_type_outer_env(kind_placeholder_template_id, placeholder_env_ref);
            // coutputs.declareTypeInnerEnv(kindPlaceholderTemplateId, placeholderEnv)
            coutputs.declare_type_inner_env(kind_placeholder_template_id, placeholder_env_ref);
        }

        // KindTemplataT(KindPlaceholderT(kindPlaceholderId))
        let kind_placeholder = self.typing_interner.intern_kind_placeholder(
            KindPlaceholderT { id: *kind_placeholder_id });
        KindTemplataT { kind: KindT::KindPlaceholder(kind_placeholder) }
    }

    pub fn create_non_kind_non_region_placeholder_inner(
        &self,
        name_prefix: IdT<'s, 't>,
        index: i32,
        rune: IRuneS<'s>,
        tyype: ITemplataType<'s>,
    ) -> ITemplataT<'s, 't> {
        // val idT = namePrefix.addStep(interner.intern(NonKindNonRegionPlaceholderNameT(index, rune)))
        let placeholder_name = self.typing_interner.intern_non_kind_non_region_placeholder_name(
            NonKindNonRegionPlaceholderNameT { index, rune}
        );
        let id_t = name_prefix.add_step(
            self.typing_interner,
            INameT::NonKindNonRegionPlaceholder(placeholder_name),
        );
        // PlaceholderTemplataT(idT, tyype)
        ITemplataT::Placeholder(self.typing_interner.alloc(PlaceholderTemplataT {
            id: *id_t,
            tyype,
        }))
    }

}

pub fn translate_sharedness(sharedness_p: SharednessP) -> SharednessT {
    match sharedness_p {
        SharednessP::Single => SharednessT::Single,
        SharednessP::Shared => SharednessT::Shared,
    }
}
