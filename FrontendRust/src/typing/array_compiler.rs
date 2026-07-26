use crate::utils::fx::IndexMap;
use crate::utils::fx::HashMap;
use std::iter::once;

use crate::utils::range::RangeS;

use crate::postparsing::names::*;

use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::postparsing::ast::{LocationInDenizen};
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::postparsing::itemplatatype::{IntegerTemplataType, ITemplataType, KindTemplataType};
use crate::postparsing::rules::rules::*;
use crate::typing::compiler::Compiler;
use crate::typing::names::names::*;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::typing::rune_typing::rune_type_solver::solve_rune_types;
use crate::typing::infer_compiler::{CompleteResolveSolve, InferEnv, InitialKnown};
use crate::typing::templata::templata::expect_integer;
use crate::utils::fx::HashSet;
use crate::typing::types::types::KindT;
use crate::typing::ast::expressions::DestroyStaticSizedArrayIntoFunctionTE;
use crate::typing::templata::templata::expect_kind_templata;
use crate::postparsing::names::CodeNameS;
use crate::postparsing::names::CodeRuneS;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::names::IRuneValS;
use crate::postparsing::rules::rules::RuneParentEnvLookupSR;
use crate::postparsing::rules::rules::RuneUsage;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::names::names::RuneNameT;
use std::marker::PhantomData;


impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_static_sized_array_from_callable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        region: RegionT,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        rules_s: &[IRulexSR<'s>],
        maybe_element_type_rune_a: Option<IRuneS<'s>>,
        size_rune_a: IRuneS<'s>,
        callable_te: ExpressionTE<'s, 't>,
    ) -> Result<StaticArrayFromCallableTE<'s, 't>, ICompileErrorT<'s, 't>> {

        let rune_typing_env = self.create_rune_type_solver_env(calling_env);

        let mut initially_known_runes: IndexMap<IRuneS<'s>, ITemplataType<'s>> = IndexMap::default();
        initially_known_runes.insert(size_rune_a, ITemplataType::IntegerTemplataType(IntegerTemplataType {}));
        if let Some(rune) = maybe_element_type_rune_a {
            initially_known_runes.insert(rune, ITemplataType::KindTemplataType(KindTemplataType {}));
        }
        let rune_a_to_type_with_implicitly_coercing_lookups_s =
            solve_rune_types(
                self.scout_arena,
                self.opts.global_options.sanity_check,
                &rune_typing_env,
                parent_ranges.to_vec(),
                &rules_s,
                &[],
                true,
                initially_known_runes,
            ).map_err(|e| ICompileErrorT::HigherTypingInferError {
                range: self.typing_interner.alloc_slice_copy(parent_ranges),
                err: e,
            })?;

        let rune_a_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> =
            IndexMap::from_iter(rune_a_to_type_with_implicitly_coercing_lookups_s.iter().map(|(k, v)| (*k, *v)));
        // The rules are already explicit Lookup/Call, so nothing rewrites them here (explicify_lookups is retired).
        // Name-resolution failures (CouldntFindType/TooManyMatchingTypes) still surface from solve_rune_types above.
        let rules_a = rules_s.to_vec();
        // We preprocess out the rune parent env lookups, see MKRFA.
        let (initial_knowns, rules_without_rune_parent_env_lookups): (Vec<InitialKnown>, Vec<IRulexSR<'s>>) =
            rules_a.iter().fold(
                (Vec::new(), Vec::new()),
                |(mut previous_conclusions, mut remaining_rules), rule| {
                    match rule {
                        IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR { rune, .. }) => {
                            let name = self.scout_arena.intern_imprecise_name(
                                IImpreciseNameValS::RuneName(RuneNameValS { rune: rune.rune }));
                            let mut filter = HashSet::default();
                            filter.insert(ILookupContext::TemplataLookupContext);
                            let templata = calling_env.lookup_nearest_with_imprecise_name(
                                name, filter, self.typing_interner).unwrap();
                            previous_conclusions.push(InitialKnown { rune: *rune, templata });
                            (previous_conclusions, remaining_rules)
                        }
                        rule => {
                            remaining_rules.push(*rule);
                            (previous_conclusions, remaining_rules)
                        }
                    }
                },
            );

        let parent_ranges_t = self.typing_interner.alloc_slice_copy(parent_ranges);
        let CompleteResolveSolve { conclusions: templatas, .. } =
            self.solve_for_resolving(
                InferEnv {
                    original_calling_env: calling_env,
                    parent_ranges: parent_ranges_t,
                    call_location,
                    self_env: IEnvironmentT::from(calling_env),
                    context_region: region,
                },
                coutputs,
                &rules_without_rune_parent_env_lookups,
                &[], // An array is not a denizen and declares no bounds.
                &rune_a_to_type,
                parent_ranges,
                call_location,
                &[],
                &initial_knowns,
            )
            .unwrap_or_else(|_e| panic!("Unimplemented: ICompileErrorT from solve_for_resolving in evaluate_static_sized_array_from_callable"))
            .unwrap_or_else(|_e| panic!("Unimplemented: evaluate_static_sized_array_from_callable — TypingPassResolvingError"));

        let size = expect_integer(templatas.get(&size_rune_a).copied().expect("vassertSome: sizeRuneA not in templatas"));
        let prototype = self.get_array_generator_prototype(
            coutputs, calling_env, parent_ranges, call_location, callable_te, region)?;
        let ssa_mt = self.resolve_static_sized_array(
            size, prototype.return_type, region);

        if let Some(element_type_rune_a) = maybe_element_type_rune_a {
            let expected_element_type = self.get_array_element_type(&templatas, element_type_rune_a);
            if prototype.return_type != expected_element_type {
                return Err(ICompileErrorT::UnexpectedArrayElementType {
                    range: self.typing_interner.alloc_slice_copy(parent_ranges),
                    expected_type: expected_element_type,
                    actual_type: prototype.return_type,
                });
            }
        }

        Ok(StaticArrayFromCallableTE::new(
            self.typing_interner.alloc(ssa_mt),
            region,
            callable_te,
            prototype,
        ))
    }

    pub fn evaluate_runtime_sized_array_from_callable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: &'t NodeEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        rules_s: &[IRulexSR<'s>],
        maybe_element_type_rune: Option<IRuneS<'s>>,
        size_te: ExpressionTE<'s, 't>,
        maybe_callable_te: Option<ExpressionTE<'s, 't>>,
    ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let rune_typing_env = self.create_rune_type_solver_env(IInDenizenEnvironmentT::Node(calling_env));
        let mut initially_known_runes: IndexMap<IRuneS<'s>, ITemplataType<'s>> = IndexMap::default();
        if let Some(rune) = maybe_element_type_rune {
            initially_known_runes.insert(rune, ITemplataType::KindTemplataType(KindTemplataType {}));
        }
        let rune_a_to_type_with_implicitly_coercing_lookups_s =
            solve_rune_types(
                self.scout_arena,
                self.opts.global_options.sanity_check,
                &rune_typing_env,
                parent_ranges.to_vec(),
                &rules_s,
                &[],
                true,
                initially_known_runes,
            ).map_err(|e| ICompileErrorT::HigherTypingInferError {
                range: self.typing_interner.alloc_slice_copy(parent_ranges),
                err: e,
            })?;
        let rune_a_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> =
            IndexMap::from_iter(rune_a_to_type_with_implicitly_coercing_lookups_s.iter().map(|(k, v)| (*k, *v)));
        // The rules are already explicit Lookup/Call, so nothing rewrites them here (explicify_lookups is retired).
        // Name-resolution failures (CouldntFindType/TooManyMatchingTypes) still surface from solve_rune_types above.
        let rules_a = rules_s.to_vec();
        // We preprocess out the rune parent env lookups, see MKRFA.
        let (initial_knowns, rules_without_rune_parent_env_lookups): (Vec<InitialKnown>, Vec<IRulexSR<'s>>) =
            rules_a.iter().fold(
                (Vec::new(), Vec::new()),
                |(mut previous_conclusions, mut remaining_rules), rule| {
                    match rule {
                        IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR { rune, .. }) => {
                            let name = self.scout_arena.intern_imprecise_name(
                                IImpreciseNameValS::RuneName(RuneNameValS { rune: rune.rune }));
                            let mut filter = HashSet::default();
                            filter.insert(ILookupContext::TemplataLookupContext);
                            let templata = IInDenizenEnvironmentT::Node(calling_env).lookup_nearest_with_imprecise_name(
                                name, filter, self.typing_interner).unwrap();
                            previous_conclusions.push(InitialKnown { rune: *rune, templata });
                            (previous_conclusions, remaining_rules)
                        }
                        rule => {
                            remaining_rules.push(*rule);
                            (previous_conclusions, remaining_rules)
                        }
                    }
                },
            );

        let parent_ranges_t = self.typing_interner.alloc_slice_copy(parent_ranges);
        let envs = InferEnv {
            original_calling_env: IInDenizenEnvironmentT::Node(calling_env),
            parent_ranges: parent_ranges_t,
            call_location,
            self_env: IEnvironmentT::from(IInDenizenEnvironmentT::Node(calling_env)),
            context_region: region,
        };
        let mut solver_state = self.make_solver_state(
            envs, coutputs, &rules_without_rune_parent_env_lookups, &rune_a_to_type, parent_ranges, &initial_knowns);
        match self.incrementally_solve(envs, coutputs, &mut solver_state, |_coutputs, _solver| false) {
            Err(_f) => {
                panic!("implement: evaluate_runtime_sized_array_from_callable — TypingPassSolverError");
                // throw CompileErrorExceptionT(TypingPassSolverError(invocationRange, f))
            }
            Ok(true) => {}
            Ok(false) => {}
        }
        let CompleteResolveSolve { conclusions: templatas, .. } =
            self.check_resolving_conclusions_and_resolve(
                envs, coutputs, parent_ranges, call_location, &rune_a_to_type, &rules_without_rune_parent_env_lookups,
                &[], // An array is not a denizen and declares no bounds.
                &[], &mut solver_state)
            .unwrap_or_else(|_e| panic!("Unimplemented: ICompileErrorT from check_resolving_conclusions_and_resolve in evaluate_runtime_sized_array_from_callable"))
            .unwrap_or_else(|_e| panic!("Unimplemented: evaluate_runtime_sized_array_from_callable — TypingPassResolvingError"));
        let mut entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();
        if let Some(e) = maybe_element_type_rune {
            let e_rune_name_t = INameT::Rune(self.typing_interner.intern_rune_name(RuneNameT { rune: e }));
            let element_type = self.get_array_element_type(&templatas, e);
            entries.push((e_rune_name_t, IEnvEntryT::Templata(ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: element_type })))));
        }
        let extended_env = calling_env.add_entries(self.typing_interner, self.scout_arena, &entries);
        let head_range = parent_ranges[0];
        let mut explicit_rules: Vec<IRulexSR<'s>> = Vec::new();
        if let Some(e) = maybe_element_type_rune {
            explicit_rules.push(IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR {
                range: head_range,
                rune: RuneUsage { range: head_range, rune: e },
            }));
        }
        let mut positional_runes: Vec<IRuneS<'s>> = Vec::new();
        if let Some(e) = maybe_element_type_rune {
            positional_runes.push(e);
        }
        let mut args: Vec<KindT<'s, 't>> = Vec::new();
        args.push(size_te.result());
        if let Some(c) = maybe_callable_te {
            args.push(c.result());
        }
        let array_imprecise_name = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.array }));
        let stamp = self.find_function(
            IInDenizenEnvironmentT::Node(extended_env),
            coutputs,
            parent_ranges,
            call_location,
            array_imprecise_name,
            &explicit_rules,
            &positional_runes,
            &[],
            region,
            &args,
            &[],
            true,
        )?
            .map_err(|e| ICompileErrorT::CouldntFindFunctionToCallT {
                range: self.typing_interner.alloc_slice_copy(parent_ranges),
                fff: e,
            })?;
        let prototype = stamp.prototype;
        let element_type = match prototype.return_type {
            KindT::RuntimeSizedArray(rsa) => match rsa.name.local_name {
                INameT::RuntimeSizedArray(name) => {
                    name.arr.element_type
                }
                _ => panic!("Array function returned wrong type!"),
            },
            _ => panic!("Array function returned wrong type!"),
        };
        if let Some(e) = maybe_element_type_rune {
            let expected_element_type = self.get_array_element_type(&templatas, e);
            if element_type != expected_element_type {
                panic!("UnexpectedArrayElementType");
            }
        }
        assert!(coutputs.get_instantiation_bounds(self.typing_interner, prototype.id).is_some());
        let result_te = prototype.return_type;
        let mut args_te: Vec<ExpressionTE<'s, 't>> = Vec::new();
        args_te.push(size_te);
        if let Some(c) = maybe_callable_te {
            args_te.push(c);
        }
        let call_te = ExpressionTE::FunctionCall(self.typing_interner.alloc(FunctionCallTE::new(
            prototype,
            self.typing_interner.alloc_slice_from_vec(args_te),
            result_te,
        )));
        Ok(call_te)
    }

    pub fn evaluate_static_sized_array_from_values(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        rules_s: &[IRulexSR<'s>],
        maybe_element_type_rune_a: Option<IRuneS<'s>>,
        size_rune_a: IRuneS<'s>,
        exprs_2: Vec<ExpressionTE<'s, 't>>,
        region: RegionT,
    ) -> Result<StaticArrayFromValuesTE<'s, 't>, ICompileErrorT<'s, 't>> {

        let rune_typing_env = self.create_rune_type_solver_env(calling_env);

        let mut initially_known_runes: IndexMap<IRuneS<'s>, ITemplataType<'s>> = IndexMap::default();
        initially_known_runes.insert(size_rune_a, ITemplataType::IntegerTemplataType(IntegerTemplataType {}));
        if let Some(rune) = maybe_element_type_rune_a {
            initially_known_runes.insert(rune, ITemplataType::KindTemplataType(KindTemplataType {}));
        }
        // Note: Rust solve_rune_types doesn't accept useOptimizedSolver (pre-existing API difference)
        let rune_a_to_type_with_implicitly_coercing_lookups_s =
            solve_rune_types(
                self.scout_arena,
                self.opts.global_options.sanity_check,
                &rune_typing_env,
                parent_ranges.to_vec(),
                &rules_s,
                &[],
                true,
                initially_known_runes,
            ).map_err(|e| ICompileErrorT::HigherTypingInferError {
                range: self.typing_interner.alloc_slice_copy(parent_ranges),
                err: e,
            })?;

        let member_types: crate::utils::fx::IndexSet<KindT<'s, 't>> =
            exprs_2.iter().map(|e| e.result()).collect();
        if member_types.len() > 1 {
            let parent_ranges_t = self.typing_interner.alloc_slice_copy(parent_ranges);
            let types_t = self.typing_interner.alloc_slice_copy(&member_types.iter().copied().collect::<Vec<_>>());
            return Err(ICompileErrorT::ArrayElementsHaveDifferentTypes { range: parent_ranges_t, types: types_t });
        }
        let member_type = *member_types.iter().next().expect("vassert: memberTypes is empty");

        let rune_a_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> =
            IndexMap::from_iter(rune_a_to_type_with_implicitly_coercing_lookups_s.iter().map(|(k, v)| (*k, *v)));
        // The rules are already explicit Lookup/Call, so nothing rewrites them here (explicify_lookups is retired).
        // Name-resolution failures (CouldntFindType/TooManyMatchingTypes) still surface from solve_rune_types above.
        let rules_a = rules_s.to_vec();
        // We preprocess out the rune parent env lookups, see MKRFA.
        let (initial_knowns, rules_without_rune_parent_env_lookups): (Vec<InitialKnown>, Vec<IRulexSR<'s>>) =
            rules_a.iter().fold(
                (Vec::new(), Vec::new()),
                |(mut previous_conclusions, mut remaining_rules), rule| {
                    match rule {
                        IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR { rune, .. }) => {
                            let name = self.scout_arena.intern_imprecise_name(
                                IImpreciseNameValS::RuneName(RuneNameValS { rune: rune.rune }));
                            let mut filter = HashSet::default();
                            filter.insert(ILookupContext::TemplataLookupContext);
                            let templata = calling_env.lookup_nearest_with_imprecise_name(
                                name, filter, self.typing_interner).unwrap();
                            previous_conclusions.push(InitialKnown { rune: *rune, templata });
                            (previous_conclusions, remaining_rules)
                        }
                        rule => {
                            remaining_rules.push(*rule);
                            (previous_conclusions, remaining_rules)
                        }
                    }
                },
            );

        let parent_ranges_t = self.typing_interner.alloc_slice_copy(parent_ranges);
        let envs = InferEnv {
            original_calling_env: calling_env,
            parent_ranges: parent_ranges_t,
            call_location,
            self_env: IEnvironmentT::from(calling_env),
            context_region: region,
        };
        let mut solver_state = self.make_solver_state(
            envs, coutputs, &rules_without_rune_parent_env_lookups, &rune_a_to_type, parent_ranges, &initial_knowns);
        match self.incrementally_solve(envs, coutputs, &mut solver_state, |_coutputs, _solver| false) {
            Err(_f) => {
                panic!("implement: evaluate_static_sized_array_from_values — TypingPassSolverError");
                // throw CompileErrorExceptionT(TypingPassSolverError(invocationRange, f))
            }
            Ok(true) => {}
            Ok(false) => {}
        }
        let CompleteResolveSolve { conclusions: templatas, .. } =
            self.check_resolving_conclusions_and_resolve(
                envs, coutputs, parent_ranges, call_location, &rune_a_to_type, &rules_without_rune_parent_env_lookups,
                &[], // An array is not a denizen and declares no bounds.
                &[], &mut solver_state)
            .unwrap_or_else(|_e| panic!("Unimplemented: ICompileErrorT from check_resolving_conclusions_and_resolve in evaluate_static_sized_array_from_values"))
            .unwrap_or_else(|_e| panic!("Unimplemented: evaluate_static_sized_array_from_values — TypingPassResolvingError"));

        if let Some(element_type_rune_a) = maybe_element_type_rune_a {
            let expected_element_type = self.get_array_element_type(&templatas, element_type_rune_a);
            if member_type != expected_element_type {
                return Err(ICompileErrorT::UnexpectedArrayElementType {
                    range: self.typing_interner.alloc_slice_copy(parent_ranges),
                    expected_type: expected_element_type,
                    actual_type: member_type,
                });
            }
        }

        let static_sized_array_type = self.resolve_static_sized_array(
            ITemplataT::Integer(exprs_2.len() as i64), member_type, region);
        let ssa_ref = self.typing_interner.alloc(static_sized_array_type);
        let ssa_coord = KindT::StaticSizedArray(ssa_ref);
        Ok(StaticArrayFromValuesTE::new(
            self.typing_interner.alloc_slice_from_vec(exprs_2),
            ssa_coord,
            ssa_ref,
        ))
    }

    pub fn evaluate_destroy_static_sized_array_into_callable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        fate: &'t FunctionEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        arr_te: ExpressionTE<'s, 't>,
        callable_te: ExpressionTE<'s, 't>,
        context_region: RegionT,
    ) -> Result<DestroyStaticSizedArrayIntoFunctionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let array_tt = match arr_te.result() {
            KindT::StaticSizedArray(s) => s,
            other => panic!("Destroying a non-array with a callable! Destroying: {:?}", other),
        };
        let prototype = self.get_array_consumer_prototype(
            coutputs, fate, range, call_location, callable_te, array_tt.element_type(), context_region)?;
        Ok(DestroyStaticSizedArrayIntoFunctionTE::new(
            arr_te,
            array_tt,
            callable_te,
            prototype,
        ))
    }

    pub fn compile_static_sized_array(&self, global_env: &'t GlobalEnvironmentT<'s, 't>, coutputs: &mut CompilerOutputs<'s, 't>) {
        // val builtinPackage = PackageCoordinate.BUILTIN(interner, keywords)
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        // val templateId =
        //   IdT(builtinPackage, Vector.empty, interner.intern(StaticSizedArrayTemplateNameT()))
        let template_name = self.typing_interner.intern_static_sized_array_template_name(
            StaticSizedArrayTemplateNameT { }
        );
        let template_id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::StaticSizedArrayTemplate(template_name),
        });

        // See CSFMSEO and SAFHE.
        // val arrayOuterEnv =
        //   CitizenEnvironmentT(
        //     globalEnv,
        //     PackageEnvironmentT(globalEnv, templateId, globalEnv.nameToTopLevelEnvironment.values.toVector),
        //     templateId,
        //     templateId,
        //     TemplatasStore(templateId, Map(), Map()))
        let global_namespaces: Vec<&TemplatasStoreT<'s, 't>> =
            global_env.name_to_top_level_environment.iter().map(|(_, ts)| *ts).collect();
        let global_namespaces = self.typing_interner.alloc_slice_from_vec(global_namespaces);
        let parent_env = self.typing_interner.alloc(PackageEnvironmentT {
            global_env,
            id: *template_id,
            global_namespaces,
        });
        let empty_templatas = TemplatasStoreBuilder::new(template_id).build_in(self.typing_interner);
        let array_outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: IEnvironmentT::Package(parent_env),
            template_id: *template_id,
            id: *template_id,
            templatas: empty_templatas,
        });
        // coutputs.declareType(templateId)
        coutputs.declare_type(template_id);
        // coutputs.declareTypeOuterEnv(templateId, arrayOuterEnv)
        let array_outer_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_outer_env);
        coutputs.declare_type_outer_env(template_id, array_outer_env_ref);

        // val TemplateTemplataType(types, _) = StaticSizedArrayTemplateTemplataT().tyype
        // val Vector(IntegerTemplataType(), SharednessTemplataType(), VariabilityTemplataType(), KindTemplataType()) = types
        // (assertion only — types are verified by the placeholder calls below)

        // val sizePlaceholder =
        //   templataCompiler.createNonKindNonRegionPlaceholderInner(
        //     templateId, 0, CodeRuneS(interner.intern(StrI("N"))), IntegerTemplataType())
        let rune_n = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("N"),
        }));
        let size_placeholder = self.create_non_kind_non_region_placeholder_inner(
            *template_id, 0, rune_n, ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
        );
        let rune_e = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("E"),
        }));
        let element_placeholder = self.create_kind_placeholder_inner(
            coutputs,
            array_outer_env_ref,
            *template_id, 1, rune_e, true,
        );

        let element_placeholder_templata = ITemplataT::Kind(
            self.typing_interner.alloc(element_placeholder));
        let placeholders = [
            size_placeholder, element_placeholder_templata,
        ];
        // val id = templateId.copy(localName = templateId.localName.makeCitizenName(interner, placeholders))
        let local_name = template_name.make_citizen_name(self.typing_interner, &placeholders);
        let id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name,
        });
        // vassert(TemplataCompiler.getTemplate(id) == templateId)
        assert!(*Compiler::get_template(self.typing_interner, *id) == *template_id);

        // val arrayInnerEnv =
        //   arrayOuterEnv.copy(
        //     id = id,
        //     templatas = arrayOuterEnv.templatas.copy(templatasStoreName = id))
        let inner_templatas = TemplatasStoreBuilder::new(id).build_in(self.typing_interner);
        let array_inner_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: array_outer_env.parent_env,
            template_id: array_outer_env.template_id,
            id: *id,
            templatas: inner_templatas,
        });
        let array_inner_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_inner_env);
        // coutputs.declareTypeInnerEnv(templateId, arrayInnerEnv)
        coutputs.declare_type_inner_env(template_id, array_inner_env_ref);
    }

    pub fn resolve_static_sized_array(
        &self,
        size: ITemplataT<'s, 't>,
        type_2: KindT<'s, 't>,
        region: RegionT,
    ) -> StaticSizedArrayTT<'s, 't> {
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        let template_name = self.typing_interner.intern_static_sized_array_template_name(
            StaticSizedArrayTemplateNameT { }
        );
        let arr_name = self.typing_interner.intern_raw_array_name(
            RawArrayNameT { element_type: type_2, self_region: region }
        );
        let ssa_name = self.typing_interner.intern_static_sized_array_name(
            StaticSizedArrayNameT { template: template_name, size, arr: arr_name }
        );
        let id = *self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::StaticSizedArray(ssa_name),
        });
        *self.typing_interner.intern_static_sized_array_tt(StaticSizedArrayTTValT { name: id })
    }

    pub fn compile_runtime_sized_array(&self, global_env: &'t GlobalEnvironmentT<'s, 't>, coutputs: &mut CompilerOutputs<'s, 't>) {
        // val builtinPackage = PackageCoordinate.BUILTIN(interner, keywords)
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        // val templateId =
        //   IdT(builtinPackage, Vector.empty, interner.intern(RuntimeSizedArrayTemplateNameT()))
        let template_name = self.typing_interner.intern_runtime_sized_array_template_name(
            RuntimeSizedArrayTemplateNameT { }
        );
        let template_id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::RuntimeSizedArrayTemplate(template_name),
        });

        // See CSFMSEO and SAFHE.
        // val arrayOuterEnv =
        //   CitizenEnvironmentT(
        //     globalEnv,
        //     PackageEnvironmentT(globalEnv, templateId, globalEnv.nameToTopLevelEnvironment.values.toVector),
        //     templateId,
        //     templateId,
        //     TemplatasStore(templateId, Map(), Map()))
        let global_namespaces: Vec<&TemplatasStoreT<'s, 't>> =
            global_env.name_to_top_level_environment.iter().map(|(_, ts)| *ts).collect();
        let global_namespaces = self.typing_interner.alloc_slice_from_vec(global_namespaces);
        let parent_env = self.typing_interner.alloc(PackageEnvironmentT {
            global_env,
            id: *template_id,
            global_namespaces,
        });
        let empty_templatas = TemplatasStoreBuilder::new(template_id).build_in(self.typing_interner);
        let array_outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: IEnvironmentT::Package(parent_env),
            template_id: *template_id,
            id: *template_id,
            templatas: empty_templatas,
        });
        // coutputs.declareType(templateId)
        coutputs.declare_type(template_id);
        // coutputs.declareTypeOuterEnv(templateId, arrayOuterEnv)
        let array_outer_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_outer_env);
        coutputs.declare_type_outer_env(template_id, array_outer_env_ref);

        // val TemplateTemplataType(types, _) = RuntimeSizedArrayTemplateTemplataT().tyype
        // val Vector(SharednessTemplataType(), KindTemplataType()) = types
        // (assertion only — types are verified by the placeholder calls below)

        // val elementPlaceholder =
        //   templataCompiler.createCoordPlaceholderInner(
        //     coutputs, arrayOuterEnv, templateId, 0, CodeRuneS(interner.intern(StrI("E"))), None, ReadOnlyRegionS, OwnT, true)
        let rune_e = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("E"),
        }));
        let element_placeholder = self.create_kind_placeholder_inner(
            coutputs,
            array_outer_env_ref,
            *template_id, 0, rune_e, true,
        );

        // val placeholders = Vector(elementPlaceholder)
        let element_placeholder_templata = ITemplataT::Kind(
            self.typing_interner.alloc(element_placeholder));
        let placeholders = [element_placeholder_templata];
        // val id = templateId.copy(localName = templateId.localName.makeCitizenName(interner, placeholders))
        let local_name = template_name.make_citizen_name(self.typing_interner, &placeholders);
        let id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name,
        });

        // val arrayInnerEnv =
        //   arrayOuterEnv.copy(
        //     id = id,
        //     templatas = arrayOuterEnv.templatas.copy(templatasStoreName = id))
        let inner_templatas = TemplatasStoreBuilder::new(id).build_in(self.typing_interner);
        let array_inner_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: array_outer_env.parent_env,
            template_id: array_outer_env.template_id,
            id: *id,
            templatas: inner_templatas,
        });
        let array_inner_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_inner_env);
        // coutputs.declareTypeInnerEnv(templateId, arrayInnerEnv)
        coutputs.declare_type_inner_env(template_id, array_inner_env_ref);
    }

    pub fn resolve_runtime_sized_array(
        &self,
        type_2: KindT<'s, 't>,
        region: RegionT,
    ) -> RuntimeSizedArrayTT<'s, 't> {
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        let template_name = self.typing_interner.intern_runtime_sized_array_template_name(
            RuntimeSizedArrayTemplateNameT { }
        );
        let arr_name = self.typing_interner.intern_raw_array_name(
            RawArrayNameT { element_type: type_2, self_region: region }
        );
        let rsa_name = self.typing_interner.intern_runtime_sized_array_name(
            RuntimeSizedArrayNameT { template: template_name, arr: arr_name }
        );
        let id = *self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::RuntimeSizedArray(rsa_name),
        });
        *self.typing_interner.intern_runtime_sized_array_tt(RuntimeSizedArrayTTValT { name: id })
    }

    fn get_array_size(&self, templatas: &IndexMap<IRuneS<'s>, ITemplataT<'s, 't>>, size_rune_a: IRuneS<'s>) -> i32 {
        panic!("Unimplemented: get_array_size");
        // val IntegerTemplataT(m) = vassertSome(templatas.get(sizeRuneA))
        // m.toInt
    }

    fn get_array_element_type(&self, templatas: &IndexMap<IRuneS<'s>, ITemplataT<'s, 't>>, type_rune_a: IRuneS<'s>) -> KindT<'s, 't> {
        let kind_templata = expect_kind_templata(*templatas.get(&type_rune_a).expect("vassertSome: typeRuneA not in templatas"));
        kind_templata.kind
    }

    pub fn lookup_in_static_sized_array(
        &self,
        range: RangeS<'s>,
        container_expr_2: ExpressionTE<'s, 't>,
        index_expr_2: ExpressionTE<'s, 't>,
        at: StaticSizedArrayTT<'s, 't>,
    ) -> StaticSizedArrayLookupTE<'s, 't> {
        StaticSizedArrayLookupTE::new(
            self.typing_interner,
            range,
            container_expr_2,
            self.typing_interner.alloc(at),
            index_expr_2,
        )
    }

    pub fn lookup_in_unknown_sized_array(
        &self,
        parent_ranges: &[RangeS<'s>],
        range: RangeS<'s>,
        container_expr_2: ExpressionTE<'s, 't>,
        index_expr_2: ExpressionTE<'s, 't>,
        rsa: &'t RuntimeSizedArrayTT<'s, 't>,
    ) -> Result<RuntimeSizedArrayLookupTE<'s, 't>, ICompileErrorT<'s, 't>> {
        if index_expr_2.result() != KindT::Int(IntT::I32) {
            let range_with_parent: Vec<RangeS<'s>> =
                once(range).chain(parent_ranges.iter().copied()).collect();
            return Err(ICompileErrorT::IndexedArrayWithNonInteger {
                range: self.typing_interner.alloc_slice_from_vec(range_with_parent),
                types: index_expr_2.result(),
            });
        }
        Ok(RuntimeSizedArrayLookupTE::new(self.typing_interner, range, container_expr_2, rsa, index_expr_2))
    }

}
