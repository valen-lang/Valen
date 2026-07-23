use crate::typing::compiler::Compiler;
use crate::postparsing::ast::LocationInDenizen;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::patterns::patterns::AtomSP;
use crate::postparsing::rules::rules::{IRulexSR, RuneParentEnvLookupSR};
use crate::postparsing::itemplatatype::ITemplataType;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::compiler_outputs::*;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::postparsing::rules::RuneUsage;
use crate::typing::infer_compiler::{InferEnv, InitialKnown, InitialSend};
use crate::typing::templata::templata::{ITemplataT, KindTemplataT};
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::postparsing::expressions::IExpressionSE;
use crate::utils::fx::IndexMap;
use crate::utils::fx::HashMap;
use crate::utils::fx::HashSet;
use crate::postparsing::names::IRuneValS;
use std::iter::once;
use std::marker::PhantomData;


impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't, 't: 'ctx, 's: 'ctx,
{
    pub fn infer_and_translate_pattern(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        rules_s: &[IRulexSR<'s>],
        rune_a_to_type_with_implicitly_coercing_lookups_s: &IndexMap<IRuneS<'s>, ITemplataType<'s>>,
        pattern: &'s AtomSP<'s>,
        unconverted_input_expr: ExpressionTE<'s, 't>,
        region: RegionT,
        after_patterns_success_continuation: impl FnOnce(
            &Compiler<'s, 'ctx, 't>,
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBox<'s, 't>,
            LocationInFunctionEnvironmentT<'t>,
            &[LocalVariable<'s, 't>],
        ) -> ExpressionTE<'s, 't> + 'ctx,
    ) -> ExpressionTE<'s, 't> {
        // The rules are different depending on the incoming type.
        // See Impl Rule For Upcasts (IRFU).
        let converted_input_expr = match &pattern.kind_rune {
            None => {
                unconverted_input_expr
            }
            Some(receiver_rune) => {
                let rune_a_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> =
                    rune_a_to_type_with_implicitly_coercing_lookups_s.clone();
                let snapshot = nenv.snapshot(self.typing_interner);
                let snapshot_env = IInDenizenEnvironmentT::Node(snapshot);
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
                                    let templata = snapshot_env.lookup_nearest_with_imprecise_name(
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

                let invocation_range: Vec<RangeS<'s>> =
                    once(pattern.range).chain(parent_ranges.iter().copied()).collect();
                let complete_define_solve =
                    // We could probably just solveForResolving (see DBDAR) but seems right to solveForDefining since we're
                    // declaring a bunch of things.
                    self.solve_for_defining(
                        InferEnv {
                            original_calling_env: snapshot_env,
                            parent_ranges: self.typing_interner.alloc_slice_copy(parent_ranges),
                            call_location,
                            self_env: IEnvironmentT::from(IInDenizenEnvironmentT::Node(snapshot)),
                            context_region: nenv.default_region(),
                        },
                        coutputs,
                        &rules_without_rune_parent_env_lookups,
                        &rune_a_to_type,
                        &invocation_range,
                        call_location,
                        &initial_knowns,
                        &[],
                    ).unwrap_or_else(|_f| {
                        panic!("implement: infer_and_translate_pattern — TypingPassDefiningError");
                        // throw CompileErrorExceptionT(TypingPassDefiningError(pattern.range :: parentRanges, f))
                    });

                nenv.add_entries(
                    self.scout_arena,
                    self.typing_interner,
                    &complete_define_solve.conclusions.iter()
                        .map(|(key, value)| {
                            let name: INameT<'s, 't> = self.typing_interner.intern_rune_name(RuneNameT { rune: *key}).into();
                            let entry = IEnvEntryT::Templata(*value);
                            (name, entry)
                        })
                        .collect::<Vec<_>>());
                let expected_coord = match complete_define_solve.conclusions.get(&receiver_rune.rune) {
                    Some(ITemplataT::Kind(kind_templata)) => kind_templata.kind,
                    _ => panic!("Expected kind templata for receiver rune"),
                };

                let range_list: Vec<RangeS<'s>> =
                    once(pattern.range).chain(parent_ranges.iter().copied()).collect();
                self.convert(
                    nenv, life, coutputs, &range_list, call_location,
                    region, unconverted_input_expr, expected_coord)
                    // VCOORD: revisit
                    .expect("convert() in pattern position returned NoImplicitCloneDefinedT — thread Result if this fires")
            }
        };

        self.inner_translate_sub_pattern_and_maybe_continue(
            coutputs, nenv, life, parent_ranges, call_location,
            pattern, self.typing_interner.alloc_slice_copy(&[]), converted_input_expr, region,
            move |compiler, coutputs, nenv, life, live_capture_locals| {
                after_patterns_success_continuation(compiler, coutputs, nenv, life, live_capture_locals)
            })
    }

    pub fn inner_translate_sub_pattern_and_maybe_continue(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        pattern: &'s AtomSP<'s>,
        previous_live_capture_locals: &'t [LocalVariable<'s, 't>],
        input_expr: ExpressionTE<'s, 't>,
        region: RegionT,
        after_sub_pattern_success_continuation: impl FnOnce(
            &Compiler<'s, 'ctx, 't>,
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBox<'s, 't>,
            LocationInFunctionEnvironmentT<'t>,
            &[LocalVariable<'s, 't>],
        ) -> ExpressionTE<'s, 't> + 'ctx,
    ) -> ExpressionTE<'s, 't> {
        {
            let names: Vec<_> = previous_live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names {
                    if !seen.contains(n) { seen.push(*n); }
                }
                seen
            };
            assert!(names == distinct);
        }

        // TODO(CRASTBU): make test that we have the right type in there, cuz the coordRuneA seems to be unused

        let mut current_instructions: Vec<ExpressionTE<'s, 't>> = Vec::new();

        let (maybe_capture_local_var_t, expr_to_destructure_or_drop_or_pass_te) =
            match &pattern.name {
                None => (None, input_expr),
                Some(capture_s) => {
                    let local_name_t = self.translate_var_name_step(capture_s.name);
                    let range_list: Vec<RangeS<'s>> =
                        once(pattern.range).chain(parent_ranges.iter().copied()).collect();
                    let local_t = if capture_s.mutate {
                        let local_t = match nenv.declared_locals().iter().find(|v| v.name() == local_name_t) {
                            Some(IVariableT::Local(rlv)) => *rlv,
                            _ => panic!("expected ReferenceLocalVariableT in declared_locals"),
                        };
                        nenv.mark_local_restackified(local_name_t);
                        current_instructions.push(
                            ExpressionTE::Restackify(self.typing_interner.alloc(
                                RestackifyTE::new(local_t, input_expr))));
                        local_t
                    } else {
                        let (_block_env, block_expr) = nenv.nearest_block_env(self.typing_interner)
                            .expect("Expected nearest block env");
                        let block_se = match block_expr {
                            IExpressionSE::Block(b) => b,
                            _ => panic!("Expected BlockSE from nearestBlockEnv"),
                        };
                        let local_s = block_se.locals.iter()
                            .find(|l| l.var_name == capture_s.name)
                            .expect("Expected local");
                        let local_t = self.make_user_local_variable(
                            coutputs, nenv, &range_list, local_s.var_name, input_expr.result());
                        current_instructions.push(
                            ExpressionTE::LetNormal(self.typing_interner.alloc(
                                LetNormalTE::new(local_t, input_expr))));
                        local_t
                    };
                    // A local lookup is already a borrow reference to the local's value.
                    let captured_local_alias_te =
                        ExpressionTE::LocalLookup(self.typing_interner.alloc(
                            LocalLookupTE::new(self.typing_interner, pattern.range, local_t)));
                    (Some(local_t), captured_local_alias_te)
                }
            };

        if maybe_capture_local_var_t.is_some() {
            // Capturing moved the input into the local, so what we pass on is a borrow of it.
            assert!(matches!(
                expr_to_destructure_or_drop_or_pass_te.result(), KindT::BorrowRef(_)));
        }

        let mut live_capture_locals: Vec<LocalVariable<'s, 't>> = previous_live_capture_locals.to_vec();
        if let Some(local_t) = maybe_capture_local_var_t {
            live_capture_locals.push(local_t);
        }
        {
            let names: Vec<_> = live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names {
                    if !seen.contains(n) { seen.push(*n); }
                }
                seen
            };
            assert!(names == distinct);
        }

        let destructure_exprs: Vec<ExpressionTE<'s, 't>> = match pattern.destructure {
            None => {
                let mut result: Vec<ExpressionTE<'s, 't>> = Vec::new();
                match &pattern.name {
                    None => {
                        // If we didn't store it, and we aren't destructuring it, then we're just ignoring it. Let's drop it.
                        let snap = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
                        let ranges: Vec<RangeS<'s>> =
                            once(pattern.range).chain(parent_ranges.iter().copied()).collect();
                        // Until a test path forces Result conversion through this pattern_compiler site.
                        result.push(self.drop(snap, coutputs, &ranges, call_location, region, expr_to_destructure_or_drop_or_pass_te)
                            .unwrap_or_else(|_| panic!("Unimplemented: Result propagation through pattern_compiler drop")));
                    }
                    Some(_) => {
                        // We aren't destructuring it, but we stored it, so just do nothing.
                    }
                }
                result.push(after_sub_pattern_success_continuation(
                    self, coutputs, nenv, life.add(self.typing_interner, 0), &live_capture_locals));
                result
            }
            Some(list_of_maybe_destructure_member_patterns) => {
                let ranges: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
                    &once(pattern.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>());
                let list_refs: &'t [&'s AtomSP<'s>] = self.typing_interner.alloc_slice_copy(
                    &list_of_maybe_destructure_member_patterns.iter().collect::<Vec<_>>());
                let live_capture_locals_t: &'t [LocalVariable<'s, 't>] = self.typing_interner.alloc_slice_copy(&live_capture_locals);
                match expr_to_destructure_or_drop_or_pass_te.result() {
                    KindT::BorrowRef(_) | KindT::ShareRef(_) => {
                        vec![self.destructure_non_owning_and_maybe_continue(
                            coutputs, nenv, life.add(self.typing_interner, 2),
                            ranges, call_location, live_capture_locals_t,
                            expr_to_destructure_or_drop_or_pass_te,
                            list_refs,
                            region,
                            after_sub_pattern_success_continuation)]
                    }
                    KindT::OwnRef(_) => {
                        panic!("implement: destructure a heap-owned value");
                    }
                    KindT::WeakRef(_) => {
                        unreachable!("a weak reference is never destructured; the pattern compiler only sees it via lock");
                    }
                    // A bare value is owned, so destructuring it destroys it.
                    _ => {
                        vec![self.destructure_owning(
                            coutputs, nenv, life.add(self.typing_interner, 1),
                            ranges, call_location, live_capture_locals_t,
                            expr_to_destructure_or_drop_or_pass_te,
                            list_refs,
                            region,
                            after_sub_pattern_success_continuation)]
                    }
                }
            }
        };

        let mut all_exprs = current_instructions;
        all_exprs.extend(destructure_exprs);
        self.consecutive(&all_exprs)
    }

    pub fn destructure_owning(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        initial_live_capture_locals: &'t [LocalVariable<'s, 't>],
        input_expr: ExpressionTE<'s, 't>,
        list_of_maybe_destructure_member_patterns: &'t [&'s AtomSP<'s>],
        region: RegionT,
        after_destructure_success_continuation: impl FnOnce(
            &Compiler<'s, 'ctx, 't>,
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBox<'s, 't>,
            LocationInFunctionEnvironmentT<'t>,
            &[LocalVariable<'s, 't>],
        ) -> ExpressionTE<'s, 't> + 'ctx,
    ) -> ExpressionTE<'s, 't> {
        {
            let names: Vec<_> = initial_live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                seen
            };
            assert!(names == distinct);
        }
        let expected_container_kind = match input_expr.result() {
            // Only a bare value is owned, and destructure_owning destroys what it's given.
            KindT::BorrowRef(_) | KindT::OwnRef(_) | KindT::ShareRef(_) | KindT::WeakRef(_) =>
                panic!("destructure_owning: expected a bare value"),
            bare_kind => bare_kind,
        };
        match expected_container_kind {
            KindT::Struct(_) => {
                // Example:
                //   struct Marine { bork: Bork; }
                //   Marine(b) = m;
                // In this case, expectedStructType1 = TypeName1("Marine") and
                // destructureMemberPatterns = Vector(CaptureSP("b", FinalP, None)).
                // Since we're receiving an owning reference, and we're *not* capturing
                // it in a variable, it will be destroyed and we will harvest its parts.
                self.translate_destroy_struct_inner_and_maybe_continue(
                    coutputs, nenv, life.add(self.typing_interner, 0),
                    parent_ranges, call_location, initial_live_capture_locals,
                    list_of_maybe_destructure_member_patterns, input_expr, region,
                    after_destructure_success_continuation)
            }
            KindT::StaticSizedArray(static_sized_array_t) => {
                let size_templata = static_sized_array_t.size();
                let size = match size_templata {
                    ITemplataT::Placeholder(_) => {
                        panic!("implement: destructureOwning StaticSizedArray — RangedInternalErrorT: Can't create static sized array by values, can't guarantee size is correct!");
                        // throw CompileErrorExceptionT(RangedInternalErrorT(parentRanges, "Can't create static sized array by values, can't guarantee size is correct!"))
                    }
                    ITemplataT::Integer(size) => {
                        if size != list_of_maybe_destructure_member_patterns.len() as i64 {
                            panic!("implement: destructureOwning StaticSizedArray — RangedInternalErrorT: Wrong num exprs!");
                            // throw CompileErrorExceptionT(RangedInternalErrorT(parentRanges, "Wrong num exprs!"))
                        }
                        size
                    }
                    _ => panic!("vwat"),
                };
                let element_type = static_sized_array_t.element_type();
                let element_locals: Vec<LocalVariable<'s, 't>> = (0..size as usize).map(|i| {
                    self.make_temporary_local(nenv, life.add(self.typing_interner, (3 + i) as i32), element_type)
                }).collect();
                let destroy_te = ExpressionTE::DestroyStaticSizedArrayIntoLocals(self.typing_interner.alloc(
                    DestroyStaticSizedArrayIntoLocalsTE::new(
                        input_expr,
                        self.typing_interner.alloc(*static_sized_array_t),
                        self.typing_interner.alloc_slice_from_vec(element_locals.clone()))));
                let live_capture_locals: Vec<LocalVariable<'s, 't>> = initial_live_capture_locals.iter().copied()
                    .chain(element_locals.iter().copied())
                    .collect();
                {
                    let names: Vec<_> = live_capture_locals.iter().map(|l: &LocalVariable<'s, 't>| l.name).collect();
                    let distinct: Vec<_> = { let mut seen = Vec::new(); for n in &names { if !seen.contains(n) { seen.push(*n); } } seen };
                    assert!(names == distinct);
                }
                if element_locals.len() != list_of_maybe_destructure_member_patterns.len() {
                    panic!("implement: destructureOwning StaticSizedArray — WrongNumberOfDestructuresError");
                    // throw CompileErrorExceptionT(WrongNumberOfDestructuresError(parentRanges, ...))
                }
                let live_capture_locals_slice = self.typing_interner.alloc_slice_from_vec(live_capture_locals);
                let element_locals_slice = self.typing_interner.alloc_slice_from_vec(element_locals);
                let lets = self.make_lets_for_own_and_maybe_continue(
                    coutputs, nenv, life.add(self.typing_interner, 4), parent_ranges, call_location,
                    live_capture_locals_slice, element_locals_slice, list_of_maybe_destructure_member_patterns, region,
                    Box::new(after_destructure_success_continuation));
                self.consecutive(&[destroy_te, lets])
            }
            KindT::RuntimeSizedArray(_) => {
                if !list_of_maybe_destructure_member_patterns.is_empty() {
                    panic!("implement: destructureOwning RuntimeSizedArray — RangedInternalErrorT: Can only destruct RSA with zero destructure targets.");
                    // throw CompileErrorExceptionT(RangedInternalErrorT(parentRanges, "Can only destruct RSA with zero destructure targets."))
                }
                ExpressionTE::DestroyRuntimeSizedArray(self.typing_interner.alloc(
                    DestroyRuntimeSizedArrayTE::new(input_expr)))
            }
            _ => {
                panic!("implement: destructureOwning — non-struct kind");
                // vfail("impl!")
            }
        }
    }

    pub fn destructure_non_owning_and_maybe_continue(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        range: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        live_capture_locals: &'t [LocalVariable<'s, 't>],
        container_te: ExpressionTE<'s, 't>,
        list_of_maybe_destructure_member_patterns: &'t [&'s AtomSP<'s>],
        region: RegionT,
        after_destructure_success_continuation: impl FnOnce(
            &Compiler<'s, 'ctx, 't>,
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBox<'s, 't>,
            LocationInFunctionEnvironmentT<'t>,
            &[LocalVariable<'s, 't>],
        ) -> ExpressionTE<'s, 't> + 'ctx,
    ) -> ExpressionTE<'s, 't> {
        {
            let names: Vec<_> = live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                seen
            };
            assert!(names == distinct);
        }

        let local_t = self.make_temporary_local(nenv, life.add(self.typing_interner, 0), container_te.result());
        let let_te = ExpressionTE::LetNormal(self.typing_interner.alloc(
            LetNormalTE::new(local_t, container_te)));
        // A local lookup is already a borrow reference to the local's value.
        let container_aliasing_expr_te: ExpressionTE<'s, 't> =
            ExpressionTE::LocalLookup(self.typing_interner.alloc(
                LocalLookupTE::new(self.typing_interner, range[0], local_t)));
        let iterate_expr = self.iterate_destructure_non_owning_and_maybe_continue(
            coutputs, nenv, life.add(self.typing_interner, 1), range, call_location, live_capture_locals,
            container_te.result(), container_aliasing_expr_te, 0,
            list_of_maybe_destructure_member_patterns, region, Box::new(after_destructure_success_continuation));
        self.consecutive(&[let_te, iterate_expr])
    }

    pub fn iterate_destructure_non_owning_and_maybe_continue(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        live_capture_locals: &'t [LocalVariable<'s, 't>],
        expected_container_coord: KindT<'s, 't>,
        container_aliasing_expr_te: ExpressionTE<'s, 't>,
        member_index: i32,
        list_of_maybe_destructure_member_patterns: &'t [&'s AtomSP<'s>],
        region: RegionT,
        after_destructure_success_continuation: Box<dyn FnOnce(
            &Compiler<'s, 'ctx, 't>,
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBox<'s, 't>,
            LocationInFunctionEnvironmentT<'t>,
            &[LocalVariable<'s, 't>],
        ) -> ExpressionTE<'s, 't> + 'ctx>,
    ) -> ExpressionTE<'s, 't> {
        {
            let names: Vec<_> = live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                seen
            };
            assert!(names == distinct);
        }

        let expected_container_kind = expected_container_coord;

        match list_of_maybe_destructure_member_patterns {
            [] => after_destructure_success_continuation(self, coutputs, nenv, life.add(self.typing_interner, 0), live_capture_locals),
            [head_maybe_destructure_member_pattern, tail_destructure_member_pattern_maybes @ ..] => {
                let head_maybe_destructure_member_pattern = *head_maybe_destructure_member_pattern;
                let tail_destructure_member_pattern_maybes: &'t [&'s AtomSP<'s>] = self.typing_interner.alloc_slice_copy(tail_destructure_member_pattern_maybes);
                let env = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
                let member_addr_expr_te = match expected_container_kind {
                    KindT::Struct(struct_tt) => {
                        self.load_from_struct(coutputs, env, head_maybe_destructure_member_pattern.range, region, container_aliasing_expr_te, *struct_tt, member_index)
                    }
                    KindT::StaticSizedArray(static_sized_array_t) => {
                        self.load_from_static_sized_array(head_maybe_destructure_member_pattern.range, *static_sized_array_t, container_aliasing_expr_te, member_index)
                    }
                    _ => {
                        panic!("implement: iterate_destructure_non_owning_and_maybe_continue — unknown container kind");
                        // throw CompileErrorExceptionT(RangedInternalErrorT(parentRanges, "Unknown type to destructure: " + other))
                    }
                };
                // A member lookup is already a borrow reference to the member. Reading it as a
                // value is the sub-pattern's business: its type annotation drives convert(),
                // which probes implicit_clone.
                let load_expr = member_addr_expr_te;
                let next_member_index = member_index + 1;
                self.inner_translate_sub_pattern_and_maybe_continue(
                    coutputs, nenv, life.add(self.typing_interner, 1), parent_ranges, call_location,
                    head_maybe_destructure_member_pattern, live_capture_locals, load_expr, region,
                    Box::new(move |compiler: &Compiler<'s, 'ctx, 't>, coutputs: &mut CompilerOutputs<'s, 't>, nenv: &mut NodeEnvironmentBox<'s, 't>, life: LocationInFunctionEnvironmentT<'t>, live_capture_locals: &[LocalVariable<'s, 't>]| {
                        let live_capture_locals: &'t [LocalVariable<'s, 't>] = compiler.typing_interner.alloc_slice_copy(live_capture_locals);
                        {
                            let names: Vec<_> = live_capture_locals.iter().map(|l| l.name).collect();
                            let distinct: Vec<_> = {
                                let mut seen = Vec::new();
                                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                                seen
                            };
                            assert!(names == distinct);
                        }
                        compiler.iterate_destructure_non_owning_and_maybe_continue(
                            coutputs, nenv, life,
                            parent_ranges, call_location, live_capture_locals,
                            expected_container_coord, container_aliasing_expr_te, next_member_index,
                            tail_destructure_member_pattern_maybes, region, after_destructure_success_continuation)
                    }))
            }
        }
    }

    pub fn translate_destroy_struct_inner_and_maybe_continue(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        initial_live_capture_locals: &'t [LocalVariable<'s, 't>],
        inner_patterns: &'t [&'s AtomSP<'s>],
        input_struct_expr: ExpressionTE<'s, 't>,
        region: RegionT,
        after_destroy_success_continuation: impl FnOnce(
            &Compiler<'s, 'ctx, 't>,
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBox<'s, 't>,
            LocationInFunctionEnvironmentT<'t>,
            &[LocalVariable<'s, 't>],
        ) -> ExpressionTE<'s, 't> + 'ctx,
    ) -> ExpressionTE<'s, 't> {
        {
            let names: Vec<_> = initial_live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                seen
            };
            assert!(names == distinct);
        }
        let struct_tt = match &input_struct_expr.result() {
            KindT::Struct(s) => *s,
            _ => panic!("translateDestroyStructInnerAndMaybeContinue: expected Struct kind"),
        };
        // We don't pattern match against closure structs.
        let struct_def_t = coutputs.lookup_struct(struct_tt.id, self);
        let substituter = self.get_placeholder_substituter(
            self.opts.global_options.sanity_check,
            nenv.function_environment().template_id,
            struct_tt.id,
            IBoundArgumentsSource::InheritBoundsFromTypeItself,
        );
        let member_locals: Vec<LocalVariable<'s, 't>> = struct_def_t.members.iter()
            .enumerate()
            .map(|(i, member)| {
                let member_type = substituter.substitute_for_kind(coutputs, member.tyype);
                self.make_temporary_local(nenv, life.add(self.typing_interner, 1 + i as i32), member_type)
            })
            .collect();
        let struct_tt_ref = self.typing_interner.alloc(struct_tt);
        let member_locals_ref = self.typing_interner.alloc_slice_copy(&member_locals);
        let destroy_te = ExpressionTE::Destroy(self.typing_interner.alloc(
            DestroyTE::new(input_struct_expr, struct_tt_ref, member_locals_ref)));
        let live_capture_locals: Vec<LocalVariable<'s, 't>> = initial_live_capture_locals.iter().copied()
            .chain(member_locals.iter().copied())
            .collect();
        {
            let names: Vec<_> = live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                seen
            };
            assert!(names == distinct);
        }
        if member_locals.len() != inner_patterns.len() {
            panic!("WrongNumberOfDestructuresError: expected {} got {}", inner_patterns.len(), member_locals.len());
        }
        let live_capture_locals: &'t [LocalVariable<'s, 't>] = self.typing_interner.alloc_slice_copy(&live_capture_locals);
        let member_locals_as_local: &'t [LocalVariable<'s, 't>] = self.typing_interner.alloc_slice_copy(&member_locals);
        let rest_te = self.make_lets_for_own_and_maybe_continue(
            coutputs, nenv, life.add(self.typing_interner, 0),
            parent_ranges, call_location, live_capture_locals,
            member_locals_as_local, inner_patterns, region,
            Box::new(after_destroy_success_continuation));
        self.consecutive(&[destroy_te, rest_te])
    }

    pub fn make_lets_for_own_and_maybe_continue(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        life: LocationInFunctionEnvironmentT<'t>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        initial_live_capture_locals: &'t [LocalVariable<'s, 't>],
        member_local_variables: &'t [LocalVariable<'s, 't>],
        inner_patterns: &'t [&'s AtomSP<'s>],
        region: RegionT,
        after_lets_success_continuation: Box<dyn FnOnce(
            &Compiler<'s, 'ctx, 't>,
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBox<'s, 't>,
            LocationInFunctionEnvironmentT<'t>,
            &[LocalVariable<'s, 't>],
        ) -> ExpressionTE<'s, 't> + 'ctx>,
    ) -> ExpressionTE<'s, 't> {
        {
            let names: Vec<_> = initial_live_capture_locals.iter().map(|l| l.name).collect();
            let distinct: Vec<_> = {
                let mut seen = Vec::new();
                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                seen
            };
            assert!(names == distinct);
        }
        assert!(member_local_variables.len() == inner_patterns.len());
        match (member_local_variables, inner_patterns) {
            ([], []) => {
                after_lets_success_continuation(self, coutputs, nenv, life.add(self.typing_interner, 0), initial_live_capture_locals)
            }
            ([head_member_local_variable, tail_member_local_variables @ ..], [head_inner_pattern, tail_inner_pattern_maybes @ ..]) => {
                let unlet_expr = self.unlet_local_without_dropping(nenv, head_member_local_variable);
                let unlet_expr_te = ExpressionTE::Unlet(self.typing_interner.alloc(unlet_expr));
                let live_capture_locals: &'t [LocalVariable<'s, 't>] = self.typing_interner.alloc_slice_copy(
                    &initial_live_capture_locals.iter().copied()
                        .filter(|l| l.name != head_member_local_variable.name)
                        .collect::<Vec<_>>());
                assert!(live_capture_locals.len() == initial_live_capture_locals.len() - 1);
                let head_inner_pattern_range = head_inner_pattern.range;
                let ranges: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
                    &once(head_inner_pattern_range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>());
                let tail_member_local_variables: &'t [LocalVariable<'s, 't>] = self.typing_interner.alloc_slice_copy(tail_member_local_variables);
                let tail_inner_pattern_maybes: &'t [&'s AtomSP<'s>] = self.typing_interner.alloc_slice_copy(tail_inner_pattern_maybes);
                self.inner_translate_sub_pattern_and_maybe_continue(
                    coutputs, nenv, life.add(self.typing_interner, 1),
                    ranges, call_location, head_inner_pattern,
                    live_capture_locals, unlet_expr_te, region,
                    move |compiler, coutputs, nenv, life, live_capture_locals_raw| {
                        let live_capture_locals: &'t [LocalVariable<'s, 't>] = compiler.typing_interner.alloc_slice_copy(live_capture_locals_raw);
                        {
                            let names: Vec<_> = initial_live_capture_locals.iter().map(|l| l.name).collect();
                            let distinct: Vec<_> = {
                                let mut seen = Vec::new();
                                for n in &names { if !seen.contains(n) { seen.push(*n); } }
                                seen
                            };
                            assert!(names == distinct);
                        }
                        compiler.make_lets_for_own_and_maybe_continue(
                            coutputs, nenv, life, parent_ranges, call_location,
                            live_capture_locals, tail_member_local_variables,
                            tail_inner_pattern_maybes, region,
                            after_lets_success_continuation)
                    })
            }
            _ => panic!("make_lets_for_own_and_maybe_continue: mismatched lengths"),
        }
    }

    pub fn load_from_struct(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
        load_range: RangeS<'s>,
        region: RegionT,
        container_alias: ExpressionTE<'s, 't>,
        struct_tt: StructTT<'s, 't>,
        index: i32,
    ) -> ExpressionTE<'s, 't> {
        let struct_def_t = coutputs.lookup_struct(struct_tt.id, self);
        let member = &struct_def_t.members[index as usize];
        let instantiation_bounds = coutputs.get_instantiation_bounds(self.typing_interner, struct_tt.id).unwrap();
        let member_type = self.get_placeholder_substituter(
            self.opts.global_options.sanity_check,
            env.denizen_template_id(),
            struct_tt.id,
            IBoundArgumentsSource::UseBoundsFromContainer {
                instantiation_bound_params: struct_def_t.instantiation_bound_params,
                instantiation_bound_arguments: instantiation_bounds,
            },
        ).substitute_for_kind(coutputs, member.tyype);
        ExpressionTE::ReferenceMemberLookup(self.typing_interner.alloc(
            ReferenceMemberLookupTE::new(
                self.typing_interner, load_range, container_alias, member.name, member_type)))
    }

    pub fn load_from_static_sized_array(
        &self,
        range: RangeS<'s>,
        static_sized_array_t: StaticSizedArrayTT<'s, 't>,
        container_alias: ExpressionTE<'s, 't>,
        index: i32,
    ) -> ExpressionTE<'s, 't> {
        let index_expr = ExpressionTE::ConstantInt(self.typing_interner.alloc(
            ConstantIntTE::new(ITemplataT::Integer(index as i64), 32, RegionT::Default)));
        let lookup = self.lookup_in_static_sized_array(range, container_alias, index_expr, static_sized_array_t);
        ExpressionTE::StaticSizedArrayLookup(self.typing_interner.alloc(lookup))
    }

}