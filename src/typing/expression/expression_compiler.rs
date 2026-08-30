use crate::parsing::ast::*;
use crate::postparsing::ast::{FunctionS, IExpressionSE as IExpressionSETrait, LocationInDenizen};
use crate::postparsing::expressions::*;
use crate::postparsing::itemplatatype::{ITemplataType, KindTemplataType};
use crate::postparsing::names::ArbitraryNameS;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::names::IRuneValS;
use crate::postparsing::names::SelfRuneS;
use crate::postparsing::names::*;
use crate::postparsing::names::{CodeNameS, IImpreciseNameValS};
use crate::postparsing::patterns::patterns::AtomSP;
use crate::postparsing::rules::rules::IRulexSR;
use crate::postparsing::rules::rules::RuneParentEnvLookupSR;
use crate::postparsing::rules::rules::RuneUsage;
use crate::scout_arena::ScoutArena;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::StructMemberT;
use crate::typing::ast::expressions::*;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::typing::citizen::struct_compiler::IResolveOutcome;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::IEnvironmentT;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::env::environment::ILookupContext;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::NodeEnvironmentBox;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::function::function_compiler::IResolveFunctionResult;
use crate::typing::names::names::ArbitraryNameT;
use crate::typing::names::names::RuneNameT;
use crate::typing::names::names::*;
use crate::typing::rune_typing::patterns::get_rune_types_from_pattern;
use crate::typing::rune_typing::rune_type_solver::citizen_or_templata_rune_type_lookup;
use crate::typing::rune_typing::rune_type_solver::solve_rune_types;
use crate::typing::rune_typing::rune_type_solver::CitizenRuneTypeSolverLookupResult;
use crate::typing::rune_typing::rune_type_solver::IRuneTypeSolverEnv;
use crate::typing::rune_typing::rune_type_solver::IRuneTypeSolverLookupResult;
use crate::typing::rune_typing::rune_type_solver::IRuneTypingLookupFailedError;
use crate::typing::rune_typing::rune_type_solver::RuneTypeSolver;
use crate::typing::rune_typing::rune_type_solver::RuneTypingCouldntFindType;
use crate::typing::rune_typing::rune_type_solver::TemplataLookupResult;
use crate::typing::templata::templata::*;
use crate::typing::templata::templata::{ITemplataT, KindTemplataT};
use crate::typing::templata_compiler::{
  is_ref, peel_all_references, peel_one_reference, IBoundArgumentsSource,
};
use crate::typing::types::types::*;
use crate::typing::types::types::{ISubKindTT, ISuperKindTT, InterfaceTTValT, KindT};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::fx::IndexMap;
use crate::utils::fx::{HashMap, HashSet};
use crate::utils::range::RangeS;
use std::iter::once;
use std::marker::PhantomData;
use crate::typing::expression::local_helper::PendingTempDrops;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn evaluate_and_coerce_to_reference_expressions(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    loct: LocT<'t>,
    parent_ranges: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    exprs_1: &[&'s IExpressionSE<'s>],
  ) -> Result<
    (Vec<ExpressionTE<'s, 't>>, HashSet<KindT<'s, 't>>, PendingTempDrops<'s, 't>),
    ICompileErrorT<'s, 't>,
  > {
    let mut result_exprs = Vec::new();
    let mut all_returns = HashSet::default();
    let mut all_pending = PendingTempDrops::none();
    for (index, expr) in exprs_1.iter().enumerate() {
      let (ref_expr, returns, pending) = match self.evaluate_expression(
        coutputs,
        nenv,
        loct.add(self.typing_interner, index as i32),
        parent_ranges,
        call_location,
        region,
        expr,
      ) {
        Ok(v) => v,
        Err(e) => {
          all_pending.defuse_on_error();
          return Err(e);
        }
      };
      result_exprs.push(ref_expr);
      all_returns.extend(returns);
      all_pending.absorb(pending);
    }
    Ok((result_exprs, all_returns, all_pending))
  }

  pub fn evaluate_lookup_for_load(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    name_imprecise: IImpreciseNameS<'s>,
  ) -> Result<Option<ExpressionTE<'s, 't>>, ICompileErrorT<'s, 't>> {
    match nenv.get_variable(name_imprecise, self.typing_interner) {
      Some(IVariableT::Local(rlv)) => {
        if nenv.unstackifieds().contains(&rlv.name) {
          return Err(ICompileErrorT::CantUseUnstackifiedLocal {
            range: self.typing_interner.alloc_slice_copy(ranges),
            local_id: rlv.name,
          });
        }
        // "undecayed": We want to decay any &&Ship to &Ship, that happens later.
        let lookup_te_undecayed = ExpressionTE::LocalLookup(
          self.typing_interner.alloc(LocalLookupTE::new(self.typing_interner, ranges[0], rlv)),
        );
        // Now, decay any &&Ship to &Ship.
        let lookup_te_decayed = match lookup_te_undecayed.result() {
          KindT::BorrowRef(BorrowRefT {
            inner: KindT::BorrowRef(BorrowRefT { inner: inner_kind }),
          }) => ExpressionTE::Deref(self.typing_interner.alloc(DerefTE::new(
            self.typing_interner,
            ranges[0],
            lookup_te_undecayed,
          ))),
          _ => lookup_te_undecayed,
        };
        Ok(Some(lookup_te_decayed))
      }
      Some(IVariableT::Capture(rcv)) => {
        // A captured variable lives in the closure struct, reached through the closure `self` param.
        // Borrow `self` (LocalLookup -> `&Closure`), then member-access the capture (MemberLookup ->
        // `&<member kind>`). The member is itself a borrow, so this is a genuine `&&`, decayed back
        // to a single borrow the same way the local-load path above does.
        let closure_param_imprecise = IImpreciseNameS::ClosureParamImpreciseName(
          self.scout_arena.intern_closure_param_imprecise_name(),
        );
        let self_local = match nenv.get_variable(closure_param_imprecise, self.typing_interner) {
          Some(IVariableT::Local(local)) => local,
          _ => panic!("closure self param not found while reading capture {:?}", rcv.name),
        };
        let self_lookup = ExpressionTE::LocalLookup(
          self.typing_interner.alloc(LocalLookupTE::new(self.typing_interner, ranges[0], self_local)),
        );
        let member_lookup = ExpressionTE::MemberLookup(self.typing_interner.alloc(
          MemberLookupTE::new(self.typing_interner, ranges[0], self_lookup, rcv.name, rcv.kind),
        ));
        // VCOORD: do we really want to decay this like this here? i think so, but unsure.
        let member_lookup_decayed = match member_lookup.result() {
          KindT::BorrowRef(BorrowRefT { inner: KindT::BorrowRef(_) }) => ExpressionTE::Deref(
            self.typing_interner.alloc(DerefTE::new(self.typing_interner, ranges[0], member_lookup)),
          ),
          _ => member_lookup,
        };
        Ok(Some(member_lookup_decayed))
      }
      None => {
        let lookup_filter: HashSet<ILookupContext> =
          [ILookupContext::TemplataLookupContext].into_iter().collect();
        match nenv.lookup_nearest_with_imprecise_name(name_imprecise, &lookup_filter, self.typing_interner) {
                    Some(ITemplataT::Integer(num)) => {
                        Ok(Some(ExpressionTE::ConstantInt(self.typing_interner.alloc(
                            ConstantIntTE::new(ITemplataT::Integer(num), 32, region)))))
                    }
                    Some(ITemplataT::Boolean(b)) => {
                        Ok(Some(ExpressionTE::ConstantBool(self.typing_interner.alloc(
                            ConstantBoolTE::new(b, region)))))
                    }
                    None => Ok(None),
                    _ => unreachable!("evaluateLookupForLoad None-branch is exhaustive over IntegerTemplataT/BooleanTemplataT/None"),
                }
      }
      #[allow(unreachable_patterns)]
      _ => panic!("evaluate_addressible_lookup: unexpected variable type"),
    }

    // match self.evaluate_addressible_lookup(coutputs, nenv, range, region, name)? {
    //     Some(x) => {
    //         // VCOORD: revisit
    //         // Bare-use (LoadAsP::Use) of an Own non-primitive local produces a
    //         // Borrow-flavored SoftLoad; auto-coercion (implicit_clone, alias,
    //         // MustExplicitlyMove) lives target-side in convert(). Primitives still
    //         // fire wrap_in_implicit_clone because get_borrow_ownership returns Share
    //         // for Int/Bool/Float/Str/Void — borrow_soft_load would construct an
    //         // illegal Share+primitive CoordT.
    //         let thing = match (target_ownership, x.result().ownership) {
    //             (LoadAsP::Use, OwnershipT::Own) if !self.is_primitive(x.result()) => {
    //                 self.borrow_soft_load(coutputs, x)
    //             }
    //             (LoadAsP::Use, OwnershipT::Own) => {
    //                 // VCOORD: retire — this is the primitive Own bare-use path still
    //                 // going through wrap_in_implicit_clone.
    //                 self.wrap_in_implicit_clone(coutputs, nenv, range, call_location, region, x)?
    //             }
    //             _ => self.soft_load(nenv, range, x, target_ownership, region),
    //         };
    //         Ok(Some(ExpressionTE::Reference(thing)))
    //     }
    //     None => {
    //     }
    // }
  }

  pub fn evaluate_addressible_lookup_for_mutate(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    parent_ranges: &'t [RangeS<'s>],
    region: RegionT,
    load_range: RangeS<'s>,
    name_imprecise: IImpreciseNameS<'s>,
  ) -> Option<ExpressionTE<'s, 't>> {
    match nenv.get_variable(name_imprecise, self.typing_interner) {
      Some(IVariableT::Local(rlv)) => Some(ExpressionTE::LocalLookup(
        self.typing_interner.alloc(LocalLookupTE::new(self.typing_interner, load_range, rlv)),
      )),
      Some(IVariableT::Capture(acv)) => {
        // let closured_vars_struct_ref = *acv.closured_vars_struct_type;
        // let closured_vars_struct_template_id = self.get_struct_template(closured_vars_struct_ref.id);
        // let closured_vars_struct_template_name = match closured_vars_struct_template_id.local_name {
        //     INameT::LambdaCitizenTemplate(n) => n,
        //     _ => panic!("evaluate_addressible_lookup_for_mutate AddressibleClosure: expected LambdaCitizenTemplateNameT"),
        // };
        // // VCOORD: this might need to go away
        // let closured_vars_struct_ref_coord = KindT::BorrowRef(BorrowRefT{ region: RegionT::Default, inner: KindT::Struct(self.typing_interner.alloc(closured_vars_struct_ref)) });
        // let closure_param_var_name_2 = IVarNameT::ClosureParam(self.typing_interner.intern_closure_param_name(ClosureParamNameT { code_location: closured_vars_struct_template_name.code_location}));
        // let borrow_expr = self.borrow_soft_load(coutputs, ExpressionTE::LocalLookup(self.typing_interner.alloc(
        //     LocalLookupTE::new(self.typing_interner, load_range, LocalVariable { name: closure_param_var_name_2, tyype: closured_vars_struct_ref_coord }))));
        // let closured_vars_struct_def = coutputs.lookup_struct(closured_vars_struct_ref.id, self);
        // assert!(closured_vars_struct_def.members.iter().any(|m| m.name == acv.name));
        // Some(ExpressionTE::AddressMemberLookup(self.typing_interner.alloc(
        //     AddressMemberLookupTE::new(self.typing_interner, load_range, borrow_expr, acv.name, acv.coord))))
        panic!("unimplemented!");
      }
      Some(IVariableT::Capture(_)) => {
        panic!("implement: evaluate_addressible_lookup_for_mutate — ReferenceClosureVariableT");
        // val closuredVarsStructId = closuredVarsStructRef.id
        // val closuredVarsStructTemplateId =
        //   TemplataCompiler.getStructTemplate(closuredVarsStructId)
        // val closuredVarsStructTemplateName =
        //   closuredVarsStructTemplateId.localName match {
        //     case n @ LambdaCitizenTemplateNameT(_) => n
        //     case _ => vwat()
        //   }
        //
        // val mutability = Compiler.getMutability(coutputs, closuredVarsStructRef)
        // val ownership =
        //   mutability match {
        //     case MutabilityTemplataT(MutableT) => BorrowT
        //     case MutabilityTemplataT(ImmutableT) => ShareT
        //     case PlaceholderTemplataT(idT, MutabilityTemplataType()) => vimpl()
        //   }
        // val closuredVarsStructRefCoord = CoordT(ownership, RegionT(DefaultRegionT), closuredVarsStructRef)
        // val borrowExpr =
        //   localHelper.borrowSoftLoad(
        //     coutputs,
        //     LocalLookupTE(
        //       loadRange,
        //       ReferenceLocalVariableT(interner.intern(ClosureParamNameT(closuredVarsStructTemplateName.codeLocation)), FinalT, closuredVarsStructRefCoord)))
        //
        // val lookup =
        //   ast.MemberLookupTE(loadRange, borrowExpr, varName, tyype, variability)
        // Some(lookup)
      }
      None => None,
    }
  }

  pub fn make_closure_struct_construct_expression(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    closure_struct_ref: StructTT<'s, 't>,
  ) -> ExpressionTE<'s, 't> {
    let closure_struct_def = coutputs.lookup_struct(closure_struct_ref.id, self);
    let substituter = self.get_placeholder_substituter(
      self.opts.global_options.sanity_check,
      nenv.function_environment().template_id,
      closure_struct_ref.id,
      IBoundArgumentsSource::InheritBoundsFromTypeItself,
    );
    // Note, this is where the unordered closuredNames set becomes ordered.
    let lookup_expressions2: Vec<ExpressionTE<'s, 't>> = closure_struct_def
      .members
      .iter()
      .map(|member| {
        let StructMemberT { name: member_name, tyype, .. } = member;
        let member_imprecise = IImpreciseNameS::CodeName(member_name.imprecise_name);
        let lookup = self
            .evaluate_lookup_for_load(coutputs, nenv, range, call_location, region, member_imprecise)
            .unwrap_or_else(|_| panic!("evaluate_lookup_for_load error"))
            .unwrap_or_else(|| panic!("Couldn't find {:?}", member_name));
        let coord = substituter.substitute_for_kind(coutputs, *tyype);
        assert_eq!(coord, lookup.result());
        // Closures never contain owned objects.
        // If we're capturing an own, then on the inside of the closure
        // it's a borrow or a weak. See "Captured own is borrow" test for more.
        assert!(is_ref(coord));
        // A lookup already yields a borrow of the outer variable (LocalLookup/MemberLookup
        // result is a BorrowRef), so it is directly the borrow we store in the closure member.
        lookup
      })
      .collect();
    let struct_ref = self.typing_interner.alloc(closure_struct_ref);
    let result_type = KindT::Struct(struct_ref);

    let construct_expr2 = ConstructTE::new(
      struct_ref,
      result_type,
      self.typing_interner.alloc_slice_from_vec(lookup_expressions2),
    );
    ExpressionTE::Construct(self.typing_interner.alloc(construct_expr2))
  }

  /// Compiler-inserted `Call(implicit_clone, &addr)`. Looks up `implicit_clone` in the
  /// callsite env via resolve_function; constructs the call IR directly. Failure to find
  /// surfaces as the standard `CouldntFindFunctionToCallT`.
  // VCOORD: retire — three problems:
  //   (a) fires source-side (both call sites are on the source coord); the coercion decision
  //       belongs at the target in convert().
  //   (b) fires for ALL Own kinds; the refined model auto-clones only Own+primitive at Own
  //       target — Own non-primitive → Own is MustExplicitlyMove, Own → Borrow is just borrow.
  //   (c) runs full resolve_function() on every bare-use of an Own local; a primitive-clone
  //       builtin dispatch shouldn't go through overload resolution, and lookup-failure surfaces
  //       as CouldntFindFunctionToCallT — the wrong error class.
  pub fn wrap_in_implicit_clone(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    addr: ExpressionTE<'s, 't>,
  ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
    panic!("implement");
    // let addr_coord = addr.result();
    // let borrow_coord = KindT::BorrowRef(&BorrowRefT{ region: RegionT::Default, inner: addr_coord });
    // let borrow_te = ExpressionTE::SoftLoad(
    //     self.typing_interner.alloc(SoftLoadTE { expr: addr, target_ownership: OwnershipT::Borrow }));
    // let calling_env = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
    // let stamp = self.resolve_function(
    //     calling_env, coutputs, range, call_location,
    //     self.keywords.implicit_clone,
    //     &[borrow_coord],
    //     region, true,
    // )?.map_err(|fff| ICompileErrorT::CouldntFindFunctionToCallT {
    //     range: self.typing_interner.alloc_slice_copy(range),
    //     fff,
    // })?;
    // assert!(coutputs.get_instantiation_bounds(self.typing_interner, stamp.prototype.id).is_some());
    // let args_te = self.typing_interner.alloc_slice_from_vec(vec![borrow_te]);
    // Ok(ExpressionTE::FunctionCall(self.typing_interner.alloc(
    //     FunctionCallTE::new(stamp.prototype, args_te, stamp.prototype.return_type))))
  }

  // VCOORD: think about bringing back AddressibleExpression,
  // so we can have more guarantees about things getting references
  pub fn evaluate_expression(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    loct: LocT<'t>,
    parent_ranges: &'t [RangeS<'s>],
    outer_call_location: LocationInDenizen<'s>,
    region: RegionT,
    expr_1: &'s IExpressionSE<'s>,
  ) -> Result<(ExpressionTE<'s, 't>, HashSet<KindT<'s, 't>>, PendingTempDrops<'s, 't>), ICompileErrorT<'s, 't>> {
    // VTRACE: show
    match expr_1 {
      IExpressionSE::Void(_) => Ok((
        ExpressionTE::VoidLiteral(self.typing_interner.alloc(VoidLiteralTE::new(region))),
        HashSet::default(),
        PendingTempDrops::none(),
      )),
      IExpressionSE::ConstantInt(c) => Ok((
        ExpressionTE::ConstantInt(self.typing_interner.alloc(ConstantIntTE::new(
          ITemplataT::Integer(c.value),
          c.bits,
          region,
        ))),
        HashSet::default(),
        PendingTempDrops::none(),
      )),
      IExpressionSE::Return(ret) => {
        let (uncasted_inner_expr_2_with_pending_drops, returns_from_inner_expr, pending_temp_drops_from_inner) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          region,
          ret.inner,
        )?;

        let uncasted_inner_expr_2 =
            self.drop_since(
              coutputs,
              nenv,
              &parent_ranges,
              outer_call_location,
              loct,
              region,
              uncasted_inner_expr_2_with_pending_drops,
              pending_temp_drops_from_inner.take_vars()
            )?;

        let inner_expr_2 = match nenv.maybe_return_type() {
          None => uncasted_inner_expr_2,
          Some(return_type) => {
            let snapshot = nenv.snapshot(self.typing_interner);
            let snapshot_env = IInDenizenEnvironmentT::Node(snapshot);
            let range_list: Vec<RangeS<'s>> =
              once(ret.range).chain(parent_ranges.iter().copied()).collect();
            match self.is_type_convertible(
              coutputs,
              snapshot_env,
              &range_list,
              outer_call_location,
              uncasted_inner_expr_2.result(),
              return_type,
            ) {
              false => {
                return Err(ICompileErrorT::CouldntConvertForReturnT {
                  range: self.typing_interner.alloc_slice_from_vec(range_list),
                  expected_type: return_type,
                  actual_type: uncasted_inner_expr_2.result(),
                });
              }
              true => self.convert(
                nenv,
                loct,
                coutputs,
                &range_list,
                outer_call_location,
                region,
                uncasted_inner_expr_2,
                return_type,
              )?,
            }
          }
        };

        let all_locals = nenv.get_all_locals();
        let unstackified_locals = nenv.get_all_unstackified_locals();
        // VCOORD: feels like this is redundant with some other code somewhere...
        let variables_to_destruct: Vec<&LocalVariable<'s, 't>> =
          all_locals.iter().filter(|x| !unstackified_locals.contains(&x.name)).copied().collect();
        let reversed_variables_to_destruct: Vec<&LocalVariable<'s, 't>> =
          variables_to_destruct.into_iter().rev().collect();

        let mut returns = returns_from_inner_expr;
        returns.insert(inner_expr_2.result());

        let result_var_name = self
          .typing_interner
          .intern_typing_pass_function_result_var_name(TypingPassFunctionResultVarNameT { loct: loct });
        let result_var_id = IVarNameT::TypingPassFunctionResultVar(result_var_name);
        let result_variable: &'t LocalVariable<'s, 't> = self
          .typing_interner
          .alloc(LocalVariable { name: result_var_id, tyype: inner_expr_2.result() });
        let result_let = ExpressionTE::LetNormal(
          self.typing_interner.alloc(LetNormalTE::new(result_variable, inner_expr_2)),
        );
        nenv.add_variable(IVariableT::Local(result_variable));

        let range_list: Vec<RangeS<'s>> =
          once(ret.range).chain(parent_ranges.iter().copied()).collect();
        let destruct_exprs_refs = self.unlet_and_drop_all(
          coutputs,
          nenv,
          &range_list,
          outer_call_location,
          region,
          &reversed_variables_to_destruct,
        )?;

        let get_result_expr = self.unlet_local_without_dropping(nenv, &result_variable);
        let get_result_expr_ref = ExpressionTE::Unlet(self.typing_interner.alloc(get_result_expr));

        let mut all_exprs: Vec<ExpressionTE<'s, 't>> = Vec::new();
        all_exprs.push(result_let);
        all_exprs.extend(destruct_exprs_refs);
        all_exprs.push(get_result_expr_ref);

        let consecutor = self.consecutive(&all_exprs);

        let return_te = ExpressionTE::Return(self.typing_interner.alloc(ReturnTE::new(consecutor)));

        Ok((return_te, returns, PendingTempDrops::none()))
      }
      IExpressionSE::Let(let_se) => {
        let (source_expr_2, returns_from_source, pending_from_source) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          let_se.expr,
        )?;

        let rune_type_solve_env = LetExprRuneTypeSolverEnv {
          nenv,
          typing_interner: self.typing_interner,
          scout_arena: self.scout_arena,
        };
        let rune_to_initially_known_type: IndexMap<_, _> =
          get_rune_types_from_pattern(&let_se.pattern).into_iter().collect();
        let range_list: Vec<RangeS<'s>> =
          once(let_se.range).chain(parent_ranges.iter().copied()).collect();
        let rune_to_type = solve_rune_types(
          coutputs,
          self.scout_arena,
          self.opts.global_options.sanity_check,
          &rune_type_solve_env,
          range_list,
          let_se.rules,
          &[],
          true,
          rune_to_initially_known_type,
        )
        .unwrap_or_else(|_e| {
          panic!("implement: LetSE — HigherTypingInferError");
          // throw CompileErrorExceptionT(HigherTypingInferError(
          //   range ::
          //       parentRanges, e))
        });

        let result_te = match self.infer_and_translate_pattern(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 1),
          parent_ranges,
          outer_call_location,
          let_se.rules,
          &rune_to_type,
          &let_se.pattern,
          source_expr_2,
          region,
          |compiler, _coutputs, nenv, _life, _live_capture_locals| {
            ExpressionTE::VoidLiteral(
              self.typing_interner.alloc(VoidLiteralTE::new(nenv.default_region())),
            )
          },
        ) {
          Ok(v) => v,
          Err(e) => {
            pending_from_source.defuse_on_error();
            return Err(e);
          }
        };

        Ok((result_te, returns_from_source, pending_from_source))
      }
      IExpressionSE::Consecutor(consecutor_se) => {
        assert!(region == nenv.default_region());
        let region_for_inners = region;

        let mut init_exprs_te: Vec<ExpressionTE<'s, 't>> = Vec::new();
        let mut init_returns: HashSet<KindT<'s, 't>> = HashSet::default();
        for (index, expr_se) in
          consecutor_se.exprs.iter().enumerate().take(consecutor_se.exprs.len() - 1)
        {
          let (undropped_expr_te_with_pending_drops, returns, pending_temp_drops) = self.evaluate_expression(
            coutputs,
            nenv,
            loct.add(self.typing_interner, index as i32),
            parent_ranges,
            outer_call_location,
            region_for_inners,
            expr_se,
          )?;

          let range_with_parent: Vec<RangeS<'s>> =
              once((*expr_se).range()).chain(parent_ranges.iter().copied()).collect();

          let undropped_expr_te =
              self.drop_since(
                coutputs,
                nenv,
                &range_with_parent,
                outer_call_location,
                loct.add(self.typing_interner, (consecutor_se.exprs.len() + index) as i32),
                region,
                undropped_expr_te_with_pending_drops,
                pending_temp_drops.take_vars()
              )?;

          let expr_te = match undropped_expr_te.result() {
            KindT::Void(_) => undropped_expr_te,
            _ => {
              let snap = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
              self.drop(
                snap,
                coutputs,
                &range_with_parent,
                outer_call_location,
                region,
                undropped_expr_te,
              )?
            }
          };

          init_exprs_te.push(expr_te);
          init_returns.extend(returns);
        }

        let (last_expr_te_with_pending, last_returns, last_expr_pending_temp_drops) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, (consecutor_se.exprs.len() - 1) as i32),
          parent_ranges,
          outer_call_location,
          region_for_inners,
          consecutor_se.exprs.last().unwrap(),
        )?;

        let last_expr_te =
            self.drop_since(
              coutputs,
              nenv,
              &parent_ranges,
              outer_call_location,
              loct,
              region,
              last_expr_te_with_pending,
              last_expr_pending_temp_drops.take_vars()
            )?;

        init_exprs_te.push(last_expr_te);
        init_returns.extend(last_returns);

        let result = self.consecutive(&init_exprs_te);
        Ok((result, init_returns, PendingTempDrops::none()))
      }
      IExpressionSE::LocalLoad(local_load) => {
        let range_list = vec![local_load.range];
        let lookup_expr_1 = self.evaluate_lookup_for_load(
          coutputs,
          nenv,
          &range_list,
          outer_call_location,
          region,
          local_load.name,
        )?;
        match lookup_expr_1 {
          None => unreachable!(
            "scout pass intercepts unknown names with CouldntFindVarToMutateS before typing runs"
          ),
          Some(x) => Ok((x, HashSet::default(), PendingTempDrops::none())),
        }
      }
      IExpressionSE::FunctionCall(fc) => {
        match fc.callable_expr {
          IExpressionSE::OverloadSet(overload_set) => {
            let (args_exprs_2, returns_from_args, pending_from_args) = self
              .evaluate_and_coerce_to_reference_expressions(
                coutputs,
                nenv,
                loct.add(self.typing_interner, 0),
                parent_ranges,
                fc.location,
                // See SRIE
                nenv.default_region(),
                fc.arg_exprs,
              )?;
            let mut range_list = vec![fc.range];
            range_list.extend_from_slice(parent_ranges);
            let initial_container_receiving: Vec<(RuneUsage<'s>, RuneUsage<'s>)> = Vec::new();
            let initial_look_in_env: IInDenizenEnvironmentT<'s, 't> =
              IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
            let parts = overload_set.lookup.parts;
            let (final_look_in_env, container_receiving_rune_to_explicit_template_arg_rune) =
              parts[..parts.len() - 1].iter().try_fold(
                (initial_look_in_env, initial_container_receiving),
                |(previous_look_in_env, previous_container_receiving),
                 part|
                 -> Result<
                  (IInDenizenEnvironmentT<'s, 't>, Vec<(RuneUsage<'s>, RuneUsage<'s>)>),
                  ICompileErrorT<'s, 't>,
                > {
                  let struct_templata = match previous_look_in_env
                    .lookup_nearest_with_imprecise_name(
                      part.name,
                      once(ILookupContext::TemplataLookupContext).collect(),
                      self.typing_interner,
                    ) {
                    Some(ITemplataT::StructDefinition(s)) => s,
                    _ => {
                      return Err(ICompileErrorT::CouldntFindTypeT {
                        range: self.typing_interner.alloc_slice_copy(&range_list),
                        name: part.name,
                      })
                    }
                  };
                  let struct_template_id = self.resolve_struct_template(coutputs, struct_templata);
                  let look_in_env =
                    coutputs.get_outer_env_for_type(*struct_template_id);
                  let part_rune_to_template_arg: Vec<(RuneUsage<'s>, RuneUsage<'s>)> = coutputs
                    .get_postparsed_struct(struct_templata.struct_template_id)
                    .generic_params
                    .iter()
                    .zip(part.explicit_template_args.iter())
                    .map(|(gp, arg_rune)| (gp.rune, *arg_rune))
                    .collect();
                  let mut next_container_receiving = previous_container_receiving;
                  next_container_receiving.extend(part_rune_to_template_arg);
                  Ok((look_in_env, next_container_receiving))
                },
              )?;
            let env_ref = final_look_in_env;
            let last_part =
              overload_set.lookup.parts.last().expect("OverloadSet parts must be non-empty");
            let callable_expr = self.new_global_function_group_expression(
              env_ref,
              coutputs,
              nenv.default_region(),
              last_part.name,
            );
            let template_arg_runes: Vec<IRuneS<'s>> =
              last_part.explicit_template_args.iter().map(|a| a.rune).collect();
            let (call_expr_2, pending_from_call) = match self.evaluate_prefix_call(
              coutputs,
              nenv,
              loct.add(self.typing_interner, 1),
              &range_list,
              fc.location,
              region,
              callable_expr,
              overload_set.lookup.rules,
              &template_arg_runes,
              &container_receiving_rune_to_explicit_template_arg_rune,
              &args_exprs_2,
            ) {
              Ok(v) => v,
              Err(e) => {
                pending_from_args.defuse_on_error();
                return Err(e);
              }
            };
            let mut all_pending = pending_from_args;
            all_pending.absorb(pending_from_call);
            Ok((call_expr_2, returns_from_args, all_pending))
          }
          _ => {
            let (undecayed_callable_expr_2, returns_from_callable, pending_from_callable) = self.evaluate_expression(
              coutputs,
              nenv,
              loct.add(self.typing_interner, 0),
              parent_ranges,
              fc.location,
              region,
              fc.callable_expr,
            )?;
            // let decayed_callable_expr_2_ref =
            //     self.maybe_borrow_soft_load(coutputs, &undecayed_callable_expr_2);
            // let decayed_callable_reference_expr_2 =
            //     self.coerce_to_reference_expression(
            //         coutputs, nenv, life.add(self.typing_interner, 1),
            //         parent_ranges, fc.location,
            //         decayed_callable_expr_2_ref, region)?;
            let decayed_callable_reference_expr_2 = undecayed_callable_expr_2;
            let (args_exprs_2, returns_from_args, pending_from_args) = match self
              .evaluate_and_coerce_to_reference_expressions(
                coutputs,
                nenv,
                loct.add(self.typing_interner, 1),
                parent_ranges,
                fc.location,
                nenv.default_region(),
                fc.arg_exprs,
              ) {
              Ok(v) => v,
              Err(e) => {
                pending_from_callable.defuse_on_error();
                return Err(e);
              }
            };
            let (function_pointer_call_2, pending_from_call) = match self.evaluate_prefix_call(
              coutputs,
              nenv,
              loct.add(self.typing_interner, 2),
              &{
                let mut range_list = vec![fc.range];
                range_list.extend_from_slice(parent_ranges);
                range_list
              },
              fc.location,
              region,
              decayed_callable_reference_expr_2,
              &[],
              &[],
              &[],
              &args_exprs_2,
            ) {
              Ok(v) => v,
              Err(e) => {
                pending_from_callable.defuse_on_error();
                pending_from_args.defuse_on_error();
                return Err(e);
              }
            };
            let mut all_returns = returns_from_callable;
            all_returns.extend(returns_from_args);
            let mut all_pending = pending_from_callable;
            all_pending.absorb(pending_from_args);
            all_pending.absorb(pending_from_call);
            Ok((function_pointer_call_2, all_returns, all_pending))
          }
        }
      }
      IExpressionSE::Function(function_se) => {
        let function_s = function_se.function;
        let range_list: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
          &once(function_s.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>(),
        );
        let call_expr_2 = self.evaluate_closure(
          coutputs,
          nenv.global_env(),
          nenv,
          range_list,
          outer_call_location,
          region,
          function_s.name,
          function_s,
        )?;
        Ok((call_expr_2, HashSet::default(), PendingTempDrops::none()))
      }
      IExpressionSE::CopyPrim(cp) => {
        // TEMP: typing-pass handler for source-level `__copy_prim(x)` syntax.
        // Evaluates the inner expression (any ownership/
        // kind), asserts result kind is Int/Bool/Float, and produces a fresh
        // Own+primitive via CopyPrimTE. Removable when auto-insertion of
        // CopyPrim replaces the source-level syntax; the CopyPrimTE emission
        // moves to convert_helper.rs (for `&int → int` coercions) and the
        // move tracker (for primitive `y = x` assignments) instead.
        let (inner_te, returns_from_inner, pending_from_inner) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          region,
          cp.inner_expr,
        )?;
        let inner_coord = inner_te.result();
        // VCOORD: eventually put here something that checks if it's Copyable.
        // or maybe thisll just go away? we might not even be able to do this from source someday.
        // /VCOORD
        let result_coord = match peel_one_reference(&inner_coord) {
          // &int -> int, &bool -> bool, ...
          Some(inner) if inner.is_primitive() => inner,
          _ => panic!("__copy_prim expects &primitive, got {:?}", inner_coord),
        };
        let copy_prim_te = self.typing_interner.alloc(CopyPrimTE::new(inner_te, result_coord));
        Ok((ExpressionTE::CopyPrim(copy_prim_te), returns_from_inner, pending_from_inner))
      }
      IExpressionSE::Ownershipped(ownershipped) => {
        // VCOORD: clean this all up
        let (inner_expr_2, returns_from_inner, pending_from_inner) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          region,
          ownershipped.inner_expr,
        )?;

        match inner_expr_2.result() {
          KindT::BorrowRef(BorrowRefT { inner }) => {
            // source is borrow
            match ownershipped.target_ownership {
              LoadAsP::Move => {
                // want to move a borrow source
                // ZHERE: the `ExpressionTE::LocalLookup => Unlet` case here is now
                // DEAD — `^<local>` is lowered to IExpressionSE::Unlet at scout and
                // never reaches Ownershipped, and a LocalLookupTE is only ever built
                // from a bare LocalLoad. So the only Moves that reach here are
                // `^<non-local>` (a member/element → CantMoveOutOfMember, or an owned
                // rvalue → no-op). Delete the LocalLookup sub-arm and put the real
                // move-out-of-place error in its place.
                // V: this can happen if ...
                match inner_expr_2 {
                  ExpressionTE::LocalLookup(LocalLookupTE { local_variable, .. }) => {
                    // VCOORD: it's weird that we' previously allocated an LocalLookupTE but now we're discarding it.
                    Ok((
                      ExpressionTE::Unlet(
                        self.typing_interner.alloc(UnletTE::new(*local_variable)),
                      ),
                      returns_from_inner,
                      pending_from_inner,
                    ))
                  }
                  _ => {
                    // V: put an error here
                    unimplemented!()
                  }
                }
              }
              LoadAsP::LoadAsBorrow => {
                // want to borrow a borrow source
                Ok((inner_expr_2, returns_from_inner, pending_from_inner))
              }
              LoadAsP::LoadAsWeak => {
                // want to weak-borrow a borrow source
                let range_with_parent: Vec<RangeS<'s>> =
                  once(ownershipped.range).chain(parent_ranges.iter().copied()).collect();
                Ok((
                  self.weak_alias(
                    coutputs,
                    self.typing_interner.alloc_slice_copy(&range_with_parent),
                    inner_expr_2,
                  )?,
                  returns_from_inner,
                  pending_from_inner,
                ))
              }
              LoadAsP::Use => Ok((inner_expr_2, returns_from_inner, pending_from_inner)),
            }
          }
          KindT::WeakRef(WeakRefT { inner }) => {
            // source is weak
            panic!("implement: Ownershipped WeakT");
            // loadAsP match {
            //   case MoveP => vcurious() // Can we even coerce to an owning reference?
            //   case LoadAsBorrowP => vimpl()
            //   case LoadAsWeakP => sourceTE
            //   case UseP => sourceTE
            // }
          }
          KindT::ShareRef(ShareRefT { inner }) => {
            // source is share
            match ownershipped.target_ownership {
              LoadAsP::Move => {
                // want to move a share source
                // Allow this, we can do ^ on a share ref, itll just give us a share ref.
                Ok((inner_expr_2, returns_from_inner, pending_from_inner))
              }
              LoadAsP::LoadAsBorrow => {
                // want to borrow a share source
                // This is basically the same case as borrowing an own source.
                // Allow this, store it in a local for the duration of the borrow.
                // This happens if the user wants to do e.g.:
                //   func foo_string() str { "foo" }
                //   func bar_string() str { "bar" }
                //   func main() { foo_string() + bar_string() }
                // + borrows its inputs, they want to borrow it for
                // the duration of the +.
                let range_with_parent = [&[ownershipped.range][..], parent_ranges].concat();
                let (let_and_lend_te, pending_temp_drops) = self.make_temporary_local_borrow(
                  coutputs,
                  nenv,
                  &range_with_parent,
                  outer_call_location,
                  loct.add(self.typing_interner, 1),
                  region,
                  inner_expr_2,
                )?;
                let mut pending = pending_from_inner;
                pending.absorb(pending_temp_drops);
                Ok((ExpressionTE::LetAndLend(let_and_lend_te), returns_from_inner, pending))
              }
              LoadAsP::LoadAsWeak => {
                // want to weak-borrow a share source
                // ZHERE: implement `weak x` (LoadAsWeak) — WeakRef of the source.
                panic!("implement: Ownershipped ShareT LoadAsWeakP");
                // vfail()
              }
              LoadAsP::Use => {
                // want to use a share source, no mention of how
                Ok((inner_expr_2, returns_from_inner, pending_from_inner))
              }
            }
          }
          _ => {
            // Not a ref
            match ownershipped.target_ownership {
              LoadAsP::Move => {
                // want to move an owning source
                // this can happen if we put a ^ on an owning reference. No harm, let it go.
                panic!("vcurious");
                // source_te
              }
              LoadAsP::LoadAsBorrow => {
                // want to borrow an owning source
                let range_with_parent: Vec<RangeS<'s>> =
                  once(ownershipped.range).chain(parent_ranges.iter().copied()).collect();
                let (let_and_lend_te, pending_temp_drops) = self.make_temporary_local_borrow(
                  coutputs,
                  nenv,
                  &range_with_parent,
                  outer_call_location,
                  loct.add(self.typing_interner, 1),
                  region,
                  inner_expr_2,
                )?;
                let mut pending = pending_from_inner;
                pending.absorb(pending_temp_drops);
                Ok((ExpressionTE::LetAndLend(let_and_lend_te), returns_from_inner, pending))
              }
              LoadAsP::LoadAsWeak => {
                // want to weak-borrow a owning source
                let range_with_parent: Vec<RangeS<'s>> =
                  once(ownershipped.range).chain(parent_ranges.iter().copied()).collect();
                let (let_and_lend_te, pending_temp_drops) = self.make_temporary_local_borrow(
                  coutputs,
                  nenv,
                  &range_with_parent,
                  outer_call_location,
                  loct.add(self.typing_interner, 3),
                  region,
                  inner_expr_2,
                )?;
                let expr = ExpressionTE::LetAndLend(let_and_lend_te);
                panic!("unimplemented");
                // self.weak_alias(coutputs, self.typing_interner.alloc_slice_copy(&range_with_parent), expr)?
              }
              LoadAsP::Use => {
                panic!("vcurious");
              }
            }
          }
        }
      }
      IExpressionSE::Dot(dot) => {
        let needle = self.scout_arena.intern_imprecise_name(
          IImpreciseNameValS::CodeName(CodeNameValS { name: dot.member }),
        );
        let (unborrowed_container_expr_2, returns_from_container_expr, pending_from_container) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          region,
          dot.left,
        )?;
        let container_expr_2 = match unborrowed_container_expr_2.result() {
          KindT::BorrowRef(_) => unborrowed_container_expr_2,
          KindT::WeakRef(_) => panic!("implement: dot on a weak is a compile error"),
          KindT::KindPlaceholder(_) => panic!("implement: dot on a placeholder is a compile error"),
          // Anything else is a value rather than a place, so it wants materializing into a
          // temporary and lending, with its drop deferred, probably with
          // make_temporary_local_borrow at life.add(1).
          _ => panic!("implement: materialize an rvalue container"),
        };
        let expr_2 = match peel_all_references(container_expr_2.result()) {
          KindT::Struct(struct_tt) => {
            let struct_def = coutputs.lookup_struct(struct_tt.id, self);
            let (struct_member, _member_index) = struct_def
              .get_member_and_index(needle)
              .unwrap_or_else(|| panic!("CouldntFindMemberT"));
            let unsubstituted_member_type = struct_member.tyype;
            let instantiation_bounds = coutputs
              .get_instantiation_bounds(self.typing_interner, struct_tt.id)
              .unwrap_or_else(|| panic!("vassertSome: getInstantiationBounds"));
            let member_type = self
              .get_placeholder_substituter(
                self.opts.global_options.sanity_check,
                nenv.function_environment().template_id,
                struct_tt.id,
                IBoundArgumentsSource::UseBoundsFromContainer {
                  instantiation_bound_params: struct_def.instantiation_bound_params,
                  instantiation_bound_arguments: instantiation_bounds,
                },
              )
              .substitute_for_kind(coutputs, unsubstituted_member_type);
            assert!(struct_def.members.iter().any(|m| IImpreciseNameS::CodeName(m.name.imprecise_name) == needle));
            ExpressionTE::MemberLookup(self.typing_interner.alloc(
              MemberLookupTE::new(
                self.typing_interner,
                dot.range,
                container_expr_2,
                IVarNameT::Member(struct_member.name),
                member_type,
              ),
            ))
          }
          KindT::StaticSizedArray(ssa) => {
            if dot.member.0.chars().all(|c| c.is_ascii_digit()) {
              let index = dot.member.0.parse::<i64>().expect("vassert: member is digit string");
              let index_expr_2 =
                ExpressionTE::ConstantInt(self.typing_interner.alloc(ConstantIntTE::new(
                  ITemplataT::Integer(index),
                  32,
                  region,
                )));
              ExpressionTE::StaticSizedArrayLookup(self.typing_interner.alloc(
                self.lookup_in_static_sized_array(dot.range, container_expr_2, index_expr_2, *ssa),
              ))
            } else {
              let range_with_parent: Vec<RangeS<'s>> =
                once(dot.range).chain(parent_ranges.iter().copied()).collect();
              pending_from_container.defuse_on_error();
              return Err(ICompileErrorT::RangedInternalErrorT {
                range: self.typing_interner.alloc_slice_from_vec(range_with_parent),
                message: self
                  .scout_arena
                  .intern_str(&format!("Sequence has no member named {}", dot.member.0))
                  .0,
              });
            }
          }
          KindT::RuntimeSizedArray(rsa) => {
            if dot.member.0.chars().all(|c| c.is_ascii_digit()) {
              let index = dot.member.0.parse::<i64>().expect("vassert: member is digit string");
              let index_expr_2 =
                ExpressionTE::ConstantInt(self.typing_interner.alloc(ConstantIntTE::new(
                  ITemplataT::Integer(index),
                  32,
                  region,
                )));
              let range_with_parent: Vec<RangeS<'s>> =
                once(dot.range).chain(parent_ranges.iter().copied()).collect();
              ExpressionTE::RuntimeSizedArrayLookup(self.typing_interner.alloc(
                match self.lookup_in_unknown_sized_array(
                  &range_with_parent,
                  dot.range,
                  container_expr_2,
                  index_expr_2,
                  rsa,
                ) {
                  Ok(lookup) => lookup,
                  Err(e) => {
                    pending_from_container.defuse_on_error();
                    return Err(e);
                  }
                },
              ))
            } else {
              let range_with_parent: Vec<RangeS<'s>> =
                once(dot.range).chain(parent_ranges.iter().copied()).collect();
              pending_from_container.defuse_on_error();
              return Err(ICompileErrorT::RangedInternalErrorT {
                range: self.typing_interner.alloc_slice_from_vec(range_with_parent),
                message: self
                  .scout_arena
                  .intern_str(&format!("Array has no member named {}", dot.member.0))
                  .0,
              });
            }
          }
          other => {
            let range_with_parent: Vec<RangeS<'s>> =
              once(dot.range).chain(parent_ranges.iter().copied()).collect();
            pending_from_container.defuse_on_error();
            return Err(ICompileErrorT::RangedInternalErrorT {
              range: self.typing_interner.alloc_slice_from_vec(range_with_parent),
              message: self
                .scout_arena
                .intern_str(&format!("Can't apply .{} to {:?}", dot.member.0, other))
                .0,
            });
          }
        };
        match expr_2.result() {
          KindT::Struct(s) => {
            assert!(coutputs.get_instantiation_bounds(self.typing_interner, s.id).is_some());
          }
          KindT::Interface(i) => {
            assert!(coutputs.get_instantiation_bounds(self.typing_interner, i.id).is_some());
          }
          _ => {}
        }
        Ok((expr_2, returns_from_container_expr, pending_from_container))
      }
      IExpressionSE::If(if_se) => {
        // We make a block for the if-statement which contains its condition (the "if block"),
        // and then two child blocks under that for the then and else blocks.
        // The then and else blocks are children of the block which contains the condition
        // so they can access any locals declared by the condition.

        let (uncoerced_condition_expr_with_pending_drops, returns_from_condition, pending_temp_drops_from_condition) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 1),
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          if_se.condition,
        )?;

        let uncoerced_condition_expr =
            self.drop_since(
              coutputs,
              nenv,
              &parent_ranges,
              outer_call_location,
              loct,
              region,
              uncoerced_condition_expr_with_pending_drops,
              pending_temp_drops_from_condition.take_vars()
            )?;

        let condition_expr = match uncoerced_condition_expr.result() {
          // VCOORD: is there some unified thing we can do to coerce like this? or maybe we can make if-statements take references? thatd be easy to coerce...
          KindT::Bool(_) => uncoerced_condition_expr,
          KindT::BorrowRef(BorrowRefT { inner: KindT::Bool(_), .. }) => ExpressionTE::CopyPrim(
            self
              .typing_interner
              .alloc(CopyPrimTE::new(uncoerced_condition_expr, KindT::Bool(BoolT {}))),
          ),
          actual_type => {
            let range_with_parent: Vec<RangeS<'s>> =
              once(if_se.condition.range()).chain(parent_ranges.iter().copied()).collect();
            return Err(ICompileErrorT::ConditionIsntBoolean {
              range: self.typing_interner.alloc_slice_from_vec(range_with_parent),
              actual_type,
            });
          }
        };

        let then_body_se_as_expr: &'s IExpressionSE<'s> =
          self.scout_arena.alloc(IExpressionSE::Block(if_se.then_body));
        let mut then_fate = NodeEnvironmentBox::new(nenv.make_child(
          self.typing_interner,
          then_body_se_as_expr,
          None,
        ));
        let then_fate_starting = then_fate.snapshot(self.typing_interner);
        let (then_expressions_with_result, then_returns_from_exprs) = self
          .evaluate_block_statements(
            coutputs,
            then_fate_starting,
            &mut then_fate,
            loct.add(self.typing_interner, 2),
            parent_ranges,
            outer_call_location,
            nenv.default_region(),
            if_se.then_body,
          )?;
        let uncoerced_then_block_2 = BlockTE::new(then_expressions_with_result);
        let (then_unstackified_ancestor_locals, then_restackified_ancestor_locals) = then_fate
          .snapshot(self.typing_interner)
          .get_effects_since(nenv.snapshot(self.typing_interner));
        let then_continues = match uncoerced_then_block_2.result {
          KindT::Never(_) => false,
          _ => true,
        };

        let else_body_se_as_expr: &'s IExpressionSE<'s> =
          self.scout_arena.alloc(IExpressionSE::Block(if_se.else_body));
        let mut else_fate = NodeEnvironmentBox::new(nenv.make_child(
          self.typing_interner,
          else_body_se_as_expr,
          None,
        ));
        let else_fate_starting = else_fate.snapshot(self.typing_interner);
        let (else_expressions_with_result, else_returns_from_exprs) = self
          .evaluate_block_statements(
            coutputs,
            else_fate_starting,
            &mut else_fate,
            loct.add(self.typing_interner, 3),
            parent_ranges,
            outer_call_location,
            nenv.default_region(),
            if_se.else_body,
          )?;
        let uncoerced_else_block_2 = BlockTE::new(else_expressions_with_result);
        let (else_unstackified_ancestor_locals, else_restackified_ancestor_locals) = else_fate
          .snapshot(self.typing_interner)
          .get_effects_since(nenv.snapshot(self.typing_interner));
        let else_continues = match uncoerced_else_block_2.result {
          KindT::Never(_) => false,
          _ => true,
        };

        let common_type = match (uncoerced_then_block_2.result, uncoerced_else_block_2.result) {
          // If one side has a return-never, use the other side.
          (KindT::Never(NeverT { from_break: false }), _) => uncoerced_else_block_2.result,
          (_, KindT::Never(NeverT { from_break: false })) => uncoerced_then_block_2.result,
          // If we get here, theres no return-nevers in play.
          // If one side has a break-never, use the other side.
          (KindT::Never(NeverT { from_break: true }), _) => uncoerced_else_block_2.result,
          (_, KindT::Never(NeverT { from_break: true })) => uncoerced_then_block_2.result,
          (a, b) if a == b => uncoerced_then_block_2.result,
          (a, b) => {
            let a_citizen = ICitizenTT::try_from(a);
            let b_citizen = ICitizenTT::try_from(b);
            match (a_citizen, b_citizen) {
              (Ok(a_c), Ok(b_c)) => {
                let nenv_snap = IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner));
                let a_ancestors: HashSet<ISuperKindTT<'s, 't>> = self
                  .get_parents(
                    coutputs,
                    parent_ranges,
                    outer_call_location,
                    nenv_snap,
                    ISubKindTT::try_from(a).unwrap(),
                  )
                  .into_iter()
                  .collect();
                let b_ancestors: HashSet<ISuperKindTT<'s, 't>> = self
                  .get_parents(
                    coutputs,
                    parent_ranges,
                    outer_call_location,
                    nenv_snap,
                    ISubKindTT::try_from(b).unwrap(),
                  )
                  .into_iter()
                  .collect();
                let common_ancestors: Vec<ISuperKindTT<'s, 't>> =
                  a_ancestors.intersection(&b_ancestors).copied().collect();

                if common_ancestors.is_empty() {
                  let range_with_parent: Vec<RangeS<'s>> =
                    once(if_se.range).chain(parent_ranges.iter().copied()).collect();
                  let _ = range_with_parent;
                  panic!("CompileErrorExceptionT RangedInternalErrorT: No common ancestors of two branches of if:\n{:?}\n{:?}", a_c, b_c);
                } else if common_ancestors.len() > 1 {
                  let range_with_parent: Vec<RangeS<'s>> =
                    once(if_se.range).chain(parent_ranges.iter().copied()).collect();
                  let _ = range_with_parent;
                  panic!("CompileErrorExceptionT RangedInternalErrorT: More than one common ancestor of two branches of if:\n{:?}\n{:?}", a_c, b_c);
                } else {
                  KindT::from(common_ancestors[0])
                }
              }
              _ => {
                let range_with_parent: Vec<RangeS<'s>> =
                  once(if_se.range).chain(parent_ranges.iter().copied()).collect();
                return Err(ICompileErrorT::CantReconcileBranchesResults {
                  range: self.typing_interner.alloc_slice_from_vec(range_with_parent),
                  then_result: uncoerced_then_block_2.result,
                  else_result: uncoerced_else_block_2.result,
                });
              }
            }
          }
        };

        let _ = then_fate;
        let _ = else_fate;
        let range_with_parent: Vec<RangeS<'s>> =
          once(if_se.range).chain(parent_ranges.iter().copied()).collect();
        let then_expr_2 = self.convert(
          nenv,
          loct,
          coutputs,
          &range_with_parent,
          outer_call_location,
          region,
          ExpressionTE::Block(self.typing_interner.alloc(uncoerced_then_block_2)),
          common_type,
        )?;
        let else_expr_2 = self.convert(
          nenv,
          loct,
          coutputs,
          &range_with_parent,
          outer_call_location,
          region,
          ExpressionTE::Block(self.typing_interner.alloc(uncoerced_else_block_2)),
          common_type,
        )?;

        let if_expr_2 = ExpressionTE::If(self.typing_interner.alloc(IfTE::new(
          LocT::from_lid(self.typing_interner, if_se.loc),
          condition_expr,
          then_expr_2,
          else_expr_2,
        )));

        if then_continues == else_continues {
          // Both continue, or both don't
          // Each branch might have moved some things. Make sure they moved the same things.
          if then_unstackified_ancestor_locals != else_unstackified_ancestor_locals {
            return Err(ICompileErrorT::RangedInternalErrorT {
                            range: self.typing_interner.alloc_slice_copy(&range_with_parent),
                            message: self.scout_arena.intern_str(&format!(
                                "Must move same variables from inside branches!\nFrom then branch: {:?}\nFrom else branch: {:?}",
                                then_unstackified_ancestor_locals, else_unstackified_ancestor_locals)).0,
                        });
          }
          if then_restackified_ancestor_locals != else_restackified_ancestor_locals {
            unreachable!(
              "Vale's flow analysis swallows restackify-mismatches before reaching this point"
            );
          }
          for local in &then_unstackified_ancestor_locals {
            nenv.mark_local_unstackified(*local);
          }
          for local in &then_restackified_ancestor_locals {
            nenv.mark_local_restackified(*local);
          }
        } else {
          // One of them continues and the other does not.
          if then_continues {
            for local in &then_unstackified_ancestor_locals {
              nenv.mark_local_unstackified(*local);
            }
            for local in &then_restackified_ancestor_locals {
              nenv.mark_local_restackified(*local);
            }
          } else if else_continues {
            for local in &else_unstackified_ancestor_locals {
              nenv.mark_local_unstackified(*local);
            }
            for local in &else_restackified_ancestor_locals {
              nenv.mark_local_restackified(*local);
            }
          } else {
            panic!("implement: evaluate_expression If — vfail branch");
            // vfail()
          }
        }

        let (if_block_unstackified_ancestor_locals, if_block_restackified_ancestor_locals) = nenv
          .snapshot(self.typing_interner)
          .get_effects_since(nenv.snapshot(self.typing_interner));
        for local in if_block_unstackified_ancestor_locals {
          nenv.mark_local_unstackified(local);
        }
        for local in if_block_restackified_ancestor_locals {
          nenv.mark_local_restackified(local);
        }

        let mut all_returns = returns_from_condition;
        all_returns.extend(then_returns_from_exprs);
        all_returns.extend(else_returns_from_exprs);
        Ok((if_expr_2, all_returns, PendingTempDrops::none()))
      }
      IExpressionSE::Break(b) => {
        // See BEAFB, we need to find the nearest while to see local since then.
        let range_with_parent: Vec<RangeS<'s>> =
          once(b.range).chain(parent_ranges.iter().copied()).collect();
        match nenv.nearest_loop_env(self.typing_interner) {
          None => {
            panic!("RangedInternalErrorT: Using break while not inside loop!");
          }
          Some((while_nenv, _)) => {
            assert!(region == nenv.default_region()); // vcurious
            let void_literal =
              ExpressionTE::VoidLiteral(self.typing_interner.alloc(VoidLiteralTE::new(region)));

            let drops_te = self.drop_since(
              coutputs,
              nenv,
              &range_with_parent,
              outer_call_location,
              loct,
              region,
              void_literal,
              nenv.snapshot(self.typing_interner).get_live_variables_introduced_since(while_nenv),
            )?;
            let break_te = ExpressionTE::Break(self.typing_interner.alloc(BreakTE::new(region)));
            let drops_and_break_te = self.consecutive(&[drops_te, break_te]);
            Ok((drops_and_break_te, HashSet::default(), PendingTempDrops::none()))
          }
        }
      }
      IExpressionSE::While(w) => {
        // We make a block for the while-statement which contains its condition (the "if block"),
        // and the body block, so they can access any locals declared by the condition.

        // See BEAFB for why we make a new environment for the While
        let loop_nenv = nenv.make_child(self.typing_interner, expr_1, None);

        let body_se_as_expr: &'s IExpressionSE<'s> =
          self.scout_arena.alloc(IExpressionSE::Block(w.body));
        let mut loop_block_fate = NodeEnvironmentBox::new(loop_nenv.make_child(
          self.typing_interner,
          body_se_as_expr,
          None,
        ));
        let loop_block_fate_starting = loop_block_fate.snapshot(self.typing_interner);
        let (body_expressions_with_result, body_returns_from_exprs) = self
          .evaluate_block_statements(
            coutputs,
            loop_block_fate_starting,
            &mut loop_block_fate,
            loct.add(self.typing_interner, 1),
            parent_ranges,
            outer_call_location,
            nenv.default_region(),
            w.body,
          )?;
        let uncoerced_body_block_2 = BlockTE::new(body_expressions_with_result);

        match uncoerced_body_block_2.result {
          KindT::Never(_) => {}
          _ => {
            let (body_unstackified_ancestor_locals, body_restackified_ancestor_locals) =
              loop_block_fate
                .snapshot(self.typing_interner)
                .get_effects_since(nenv.snapshot(self.typing_interner));

            if !body_unstackified_ancestor_locals.is_empty() {
              let range_with_parent: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
                &once(w.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>(),
              );
              return Err(ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile {
                range: range_with_parent,
                local_id: *body_unstackified_ancestor_locals.iter().next().unwrap(),
              });
            }
            if !body_restackified_ancestor_locals.is_empty() {
              let range_with_parent: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
                &once(w.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>(),
              );
              return Err(ICompileErrorT::CantRestackifyOutsideLocalFromInsideWhile {
                range: range_with_parent,
                local_id: *body_unstackified_ancestor_locals.iter().next().unwrap(),
              });
            }
            if !body_restackified_ancestor_locals.is_empty() {
              let range_with_parent: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
                &once(w.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>(),
              );
              return Err(ICompileErrorT::CantRestackifyOutsideLocalFromInsideWhile {
                range: range_with_parent,
                local_id: *body_unstackified_ancestor_locals.iter().next().unwrap(),
              });
            }
          }
        }

        let loop_expr_2 = ExpressionTE::While(self.typing_interner.alloc(WhileTE::new(
          LocT::from_lid(self.typing_interner, w.loc),
          uncoerced_body_block_2,
        )));
        Ok((loop_expr_2, body_returns_from_exprs, PendingTempDrops::none()))
      }
      IExpressionSE::Map(m) => {
        // // Preprocess the entire loop once, to predict what its result type
        // // will be.
        // // We can't just use this, because any returns inside won't drop
        // // the temporary list.
        // let element_ref_t = {
        //     // See BEAFB for why we make a new environment for the While
        //     let loop_nenv = nenv.make_child(self.typing_interner, expr_1, None);
        //     let body_se_as_expr: &'s IExpressionSE<'s> =
        //         self.scout_arena.alloc(IExpressionSE::Block(m.body));
        //     let mut loop_block_fate = NodeEnvironmentBox::new(loop_nenv.make_child(self.typing_interner, body_se_as_expr, None));
        //     let loop_block_fate_starting = loop_block_fate.snapshot(self.typing_interner);
        //     let (body_expressions_with_result, _) =
        //         self.evaluate_block_statements(
        //             coutputs,
        //             loop_block_fate_starting,
        //             &mut loop_block_fate,
        //             life.add(self.typing_interner, 1),
        //             parent_ranges,
        //             outer_call_location,
        //             nenv.default_region(),
        //             m.body)?;
        //     body_expressions_with_result.result()
        // };
        //
        // // Now that we know the result type, let's make a temporary list.
        //
        // let self_rune_irune = self.scout_arena.intern_rune(IRuneValS::SelfRune(SelfRuneS {}));
        // let self_rune_name_t = INameT::Rune(self.typing_interner.intern_rune_name(RuneNameT { rune: self_rune_irune}));
        // let element_coord_templata: &'t KindTemplataT<'s, 't> = self.typing_interner.alloc(KindTemplataT { kind: element_ref_t });
        // let snap = nenv.snapshot(self.typing_interner);
        // let call_env_node = snap.add_entries(
        //     self.typing_interner,
        //     self.scout_arena,
        //     &[(self_rune_name_t, IEnvEntryT::Templata(ITemplataT::Kind(element_coord_templata)))]);
        // let call_env = IInDenizenEnvironmentT::Node(call_env_node);
        // let make_list_callable = self.new_global_function_group_expression(
        //     call_env, coutputs, RegionT::Default,
        //     self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.keywords.list })));
        // let range_with_parent_t: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
        //     &once(m.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>());
        // let rune_parent_env_lookup_rule = IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR {
        //     range: m.range,
        //     rune: RuneUsage { range: m.range, rune: self_rune_irune },
        // });
        // let make_list_te = self.evaluate_prefix_call(
        //     coutputs,
        //     nenv,
        //     life.add(self.typing_interner, 1),
        //     range_with_parent_t,
        //     outer_call_location,
        //     region,
        //     make_list_callable,
        //     &[rune_parent_env_lookup_rule],
        //     &[self_rune_irune],
        //     &[],
        //     &[])?;
        //
        // let list_local = self.make_temporary_local(
        //     nenv, life.add(self.typing_interner, 2), make_list_te.result());
        // let let_list_te = ExpressionTE::LetNormal(self.typing_interner.alloc(
        //     LetNormalTE::new(LocalVariable::Reference(list_local), make_list_te)));
        //
        // let (loop_te, returns_from_loop) = {
        //     // See BEAFB for why we make a new environment for the While
        //     let loop_nenv = nenv.make_child(self.typing_interner, expr_1, None);
        //     let body_se_as_expr: &'s IExpressionSE<'s> =
        //         self.scout_arena.alloc(IExpressionSE::Block(m.body));
        //     let mut loop_block_fate = NodeEnvironmentBox::new(loop_nenv.make_child(self.typing_interner, body_se_as_expr, None));
        //     let loop_block_fate_starting = loop_block_fate.snapshot(self.typing_interner);
        //     let (user_body_te, body_returns_from_exprs) =
        //         self.evaluate_block_statements(
        //             coutputs,
        //             loop_block_fate_starting,
        //             &mut loop_block_fate,
        //             life.add(self.typing_interner, 1),
        //             parent_ranges,
        //             outer_call_location,
        //             nenv.default_region(),
        //             m.body)?;
        //
        //     // We store the iteration result in a local because the loop body will have
        //     // breaks, and we can't have a BreakTE inside a FunctionCallTE, see BRCOBS.
        //     let iteration_result_local = self.make_temporary_local(
        //         nenv, life.add(self.typing_interner, 3), user_body_te.result());
        //     let let_iteration_result_te = ExpressionTE::LetNormal(self.typing_interner.alloc(
        //         LetNormalTE::new(LocalVariable::Reference(iteration_result_local), user_body_te)));
        //
        //     let add_callable = self.new_global_function_group_expression(
        //         call_env, coutputs, RegionT::Default,
        //         self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.keywords.add })));
        //     let local_lookup_te = ExpressionTE::LocalLookup(self.typing_interner.alloc(
        //         LocalLookupTE::new(self.typing_interner, m.range, LocalVariable::Reference(list_local))));
        //     let borrow_load = self.borrow_soft_load(coutputs, local_lookup_te);
        //     let unlet_iter = ExpressionTE::Unlet(self.typing_interner.alloc(self.unlet_local_without_dropping(nenv, &LocalVariable::Reference(iteration_result_local))));
        //     let add_call = self.evaluate_prefix_call(
        //         coutputs,
        //         nenv,
        //         life.add(self.typing_interner, 4),
        //         range_with_parent_t,
        //         outer_call_location,
        //         region,
        //         add_callable,
        //         &[],
        //         &[],
        //         &[],
        //         &[borrow_load, unlet_iter])?;
        //     let body_te = BlockTE::new(self.consecutive(&[let_iteration_result_te, add_call]));
        //
        //     let (body_unstackified_ancestor_locals, body_restackified_ancestor_locals) =
        //         loop_block_fate.snapshot(self.typing_interner).get_effects_since(nenv.snapshot(self.typing_interner));
        //     if !body_unstackified_ancestor_locals.is_empty() {
        //         return Err(ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile {
        //             range: range_with_parent_t,
        //             local_id: *body_unstackified_ancestor_locals.iter().next().unwrap(),
        //         });
        //     }
        //     if !body_restackified_ancestor_locals.is_empty() {
        //         return Err(ICompileErrorT::CantRestackifyOutsideLocalFromInsideWhile {
        //             range: range_with_parent_t,
        //             local_id: *body_unstackified_ancestor_locals.iter().next().unwrap(),
        //         });
        //     }
        //
        //     let while_te = ExpressionTE::While(self.typing_interner.alloc(WhileTE::new(body_te)));
        //     (while_te, body_returns_from_exprs)
        // };
        //
        // let unlet_list_te = ExpressionTE::Unlet(self.typing_interner.alloc(self.unlet_local_without_dropping(nenv, &LocalVariable::Reference(list_local))));
        //
        // let combined_te = self.consecutive(&[let_list_te, loop_te, unlet_list_te]);
        //
        // Ok((ExpressionTE::Reference(combined_te), returns_from_loop))
        unimplemented!();
      }
      IExpressionSE::ExprMutate(em) => {
        let (unconverted_source_expr_2, returns_from_source, pending_from_source) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          em.expr,
        )?;
        let (destination_expr_2, returns_from_destination, pending_from_destination) = match self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 1),
          parent_ranges,
          outer_call_location,
          region,
          em.mutatee,
        ) {
          Ok(v) => v,
          Err(e) => {
            pending_from_source.defuse_on_error();
            return Err(e);
          }
        };
        assert!(is_ref(destination_expr_2.result()));

        let range_with_parent: Vec<RangeS<'s>> =
          once(em.range).chain(parent_ranges.iter().copied()).collect();
        let destination_value_type = match destination_expr_2.result() {
          KindT::BorrowRef(BorrowRefT { inner, .. }) => inner,
          _ => panic!("evaluate_addressible_lookup_for_mutate returned non-borrow"),
        };
        let is_convertible = self.is_type_convertible(
          coutputs,
          IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner)),
          &range_with_parent,
          outer_call_location,
          unconverted_source_expr_2.result(),
          *destination_value_type,
        );
        if !is_convertible {
          pending_from_source.defuse_on_error();
          pending_from_destination.defuse_on_error();
          return Err(ICompileErrorT::CouldntConvertForMutateT {
            range: self.typing_interner.alloc_slice_copy(&range_with_parent),
            expected_type: *destination_value_type,
            actual_type: unconverted_source_expr_2.result(),
          });
        }
        let converted_source_expr_2 = match self.convert(
          nenv,
          loct,
          coutputs,
          &range_with_parent,
          outer_call_location,
          region,
          unconverted_source_expr_2,
          *destination_value_type,
        ) {
          Ok(v) => v,
          Err(e) => {
            pending_from_source.defuse_on_error();
            pending_from_destination.defuse_on_error();
            return Err(e);
          }
        };
        // VCOORD: lets rename all the _2 to _te etc.
        let mutate_2 = ExpressionTE::Mutate(
          self.typing_interner.alloc(MutateTE::new(destination_expr_2, converted_source_expr_2)),
        );
        let mut returns = returns_from_source;
        returns.extend(returns_from_destination);
        let mut all_pending = pending_from_source;
        all_pending.absorb(pending_from_destination);
        Ok((mutate_2, returns, all_pending))
      }
      IExpressionSE::LocalMutate(lm) => {
        let (unconverted_source_expr_2, returns_from_source, pending_from_source) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          region,
          lm.expr,
        )?;
        // We do this after the source because of statements like these:
        //   set ship = foo(ship);
        // which move the thing on the right and then restackify it on the left.
        let range_with_parent: Vec<RangeS<'s>> =
          once(lm.range).chain(parent_ranges.iter().copied()).collect();
        let destination_expr_2 = self
          .evaluate_addressible_lookup_for_mutate(
            coutputs,
            nenv,
            parent_ranges,
            region,
            lm.range,
            lm.name,
          )
          .unwrap_or_else(|| panic!("Couldnt find {:?}", lm.name));
        let destination_value_type = match destination_expr_2.result() {
          KindT::BorrowRef(BorrowRefT { inner, .. }) => inner,
          _ => panic!("evaluate_addressible_lookup_for_mutate returned non-borrow"),
        };
        let is_convertible = self.is_type_convertible(
          coutputs,
          IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner)),
          &range_with_parent,
          outer_call_location,
          unconverted_source_expr_2.result(),
          *destination_value_type,
        );
        if !is_convertible {
          pending_from_source.defuse_on_error();
          return Err(ICompileErrorT::CouldntConvertForMutateT {
            range: self.typing_interner.alloc_slice_copy(&range_with_parent),
            expected_type: *destination_value_type,
            actual_type: unconverted_source_expr_2.result(),
          });
        }
        assert!(is_convertible);
        let converted_source_expr_2 = match self.convert(
          nenv,
          loct,
          coutputs,
          &range_with_parent,
          outer_call_location,
          region,
          unconverted_source_expr_2,
          *destination_value_type,
        ) {
          Ok(v) => v,
          Err(e) => {
            pending_from_source.defuse_on_error();
            return Err(e);
          }
        };
        let expr_te = match destination_expr_2 {
          ExpressionTE::LocalLookup(local_lookup)
            if nenv.unstackifieds().contains(&local_lookup.local_variable.name) =>
          {
            nenv.mark_local_restackified(local_lookup.local_variable.name);
            ExpressionTE::Restackify(
              self
                .typing_interner
                .alloc(RestackifyTE::new(local_lookup.local_variable, converted_source_expr_2)),
            )
          }
          _ => ExpressionTE::Mutate(
            self.typing_interner.alloc(MutateTE::new(destination_expr_2, converted_source_expr_2)),
          ),
        };
        Ok((expr_te, returns_from_source, pending_from_source))
      }
      IExpressionSE::Tuple(t) => {
        let (exprs_2, returns_from_elements, pending_from_elements) = self.evaluate_and_coerce_to_reference_expressions(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          t.elements,
        )?;
        let expr_2 = self.resolve_tuple(
          IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner)),
          coutputs,
          parent_ranges,
          outer_call_location,
          exprs_2,
        );
        Ok((expr_2, returns_from_elements, pending_from_elements))
      }
      IExpressionSE::StaticArrayFromValues(sav) => {
        let (exprs_2, returns_from_elements, pending_from_elements) = self.evaluate_and_coerce_to_reference_expressions(
          coutputs,
          nenv,
          loct,
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          sav.elements,
        )?;
        let new_parent_ranges: Vec<RangeS<'s>> =
          once(sav.range).chain(parent_ranges.iter().copied()).collect();
        let expr_2 = match self.evaluate_static_sized_array_from_values(
          coutputs,
          IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner)),
          &new_parent_ranges,
          outer_call_location,
          sav.rules,
          sav.maybe_element_type_st.map(|r| r.rune),
          sav.size_st.rune,
          exprs_2,
          region,
        ) {
          Ok(v) => v,
          Err(e) => {
            pending_from_elements.defuse_on_error();
            return Err(e);
          }
        };
        Ok((
          ExpressionTE::StaticArrayFromValues(self.typing_interner.alloc(expr_2)),
          returns_from_elements,
          pending_from_elements,
        ))
      }
      IExpressionSE::StaticArrayFromCallable(sa) => {
        let (callable_te, returns_from_callable, pending_from_callable) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          sa.callable,
        )?;
        let range_with_parent: Vec<RangeS<'s>> =
          once(sa.range).chain(parent_ranges.iter().copied()).collect();
        let expr_2 = match self.evaluate_static_sized_array_from_callable(
          coutputs,
          IInDenizenEnvironmentT::Node(nenv.snapshot(self.typing_interner)),
          region,
          &range_with_parent,
          outer_call_location,
          sa.rules,
          sa.maybe_element_type_st.map(|r| r.rune),
          sa.size_st.rune,
          callable_te,
        ) {
          Ok(v) => v,
          Err(e) => {
            pending_from_callable.defuse_on_error();
            return Err(e);
          }
        };
        Ok((
          ExpressionTE::StaticArrayFromCallable(self.typing_interner.alloc(expr_2)),
          returns_from_callable,
          pending_from_callable,
        ))
      }
      IExpressionSE::NewRuntimeSizedArray(nrsa) => {
        let (size_te, returns_from_size, pending_from_size) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          region,
          nrsa.size,
        )?;
        let (maybe_callable_te, returns_from_callable, pending_from_callable) = match nrsa.callable {
          None => (None, HashSet::default(), PendingTempDrops::none()),
          Some(callable_ae) => {
            let (callable_te, rets, cb_pending) = self.evaluate_expression(
              coutputs,
              nenv,
              loct.add(self.typing_interner, 1),
              parent_ranges,
              outer_call_location,
              nenv.default_region(),
              callable_ae,
            )?;
            (Some(callable_te), rets, cb_pending)
          }
        };
        let range_with_parent: Vec<RangeS<'s>> =
          once(nrsa.range).chain(parent_ranges.iter().copied()).collect();
        let expr_2 = match self.evaluate_runtime_sized_array_from_callable(
          coutputs,
          nenv.snapshot(self.typing_interner),
          &range_with_parent,
          outer_call_location,
          region,
          nrsa.rules,
          nrsa.maybe_element_type_st.map(|r| r.rune),
          size_te,
          maybe_callable_te,
        ) {
          Ok(v) => v,
          Err(e) => {
            pending_from_size.defuse_on_error();
            pending_from_callable.defuse_on_error();
            return Err(e);
          }
        };
        let mut returns = returns_from_size;
        returns.extend(returns_from_callable);
        let mut all_pending = pending_from_size;
        all_pending.absorb(pending_from_callable);
        Ok((expr_2, returns, all_pending))
      }
      IExpressionSE::Block(b) => {
        let mut child_environment =
          NodeEnvironmentBox::new(nenv.make_child(self.typing_interner, expr_1, None));
        let child_starting = child_environment.snapshot(self.typing_interner);
        let (expressions_with_result, returns_from_exprs) = self.evaluate_block_statements(
          coutputs,
          child_starting,
          &mut child_environment,
          loct,
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          b,
        )?;
        let block_2 =
          ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(expressions_with_result)));
        let (unstackified_ancestor_locals, restackified_ancestor_locals) = child_environment
          .snapshot(self.typing_interner)
          .get_effects_since(nenv.snapshot(self.typing_interner));
        for local in unstackified_ancestor_locals {
          nenv.mark_local_unstackified(local);
        }
        for local in restackified_ancestor_locals {
          nenv.mark_local_restackified(local);
        }
        Ok((block_2, returns_from_exprs, PendingTempDrops::none()))
      }
      // IExpressionSE::Pure(_) => {
      // panic!("implement: evaluate_expression — Pure");
      // evaluateAndCoerceToReferenceExpression(
      //   coutputs, nenv, life + 0, parentRanges, outerCallLocation, region, inner)
      // }
      IExpressionSE::ConstantStr(c) => {
        let result = ExpressionTE::ConstantStr(self.typing_interner.alloc(ConstantStrTE::new(
          self.typing_interner,
          c.value,
          region,
        )));
        Ok((result, HashSet::default(), PendingTempDrops::none()))
      }
      IExpressionSE::ConstantFloat(c) => {
        let result = ExpressionTE::ConstantFloat(
          self.typing_interner.alloc(ConstantFloatTE::new(c.value, region)),
        );
        Ok((result, HashSet::default(), PendingTempDrops::none()))
      }
      IExpressionSE::Destruct(destruct_se) => {
        let (inner_expr_2, returns_from_array_expr, pending_from_inner) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          region,
          destruct_se.inner,
        )?;
        if is_ref(inner_expr_2.result()) {
          // V: implement this
          // return Err();
          panic!("unimplemented");
        }

        let destroy_2 = match inner_expr_2.result() {
          KindT::Struct(struct_tt) => {
            let struct_def = coutputs.lookup_struct(struct_tt.id, self);
            let substituter = self.get_placeholder_substituter(
              self.opts.global_options.sanity_check,
              nenv.function_environment().template_id,
              struct_tt.id,
              IBoundArgumentsSource::InheritBoundsFromTypeItself,
            );
            let destination_locals: Vec<&'t LocalVariable<'s, 't>> = struct_def
              .members
              .iter()
              .enumerate()
              .map(|(index, m)| {
                let unsubstituted_coord = m.tyype;
                let reference = substituter.substitute_for_kind(coutputs, unsubstituted_coord);
                self.make_temporary_local(
                  nenv,
                  loct.add(self.typing_interner, 1 + index as i32),
                  reference,
                )
              })
              .collect();
            ExpressionTE::Destroy(self.typing_interner.alloc(DestroyTE::new(
              inner_expr_2,
              struct_tt,
              self.typing_interner.alloc_slice_from_vec(destination_locals),
            )))
          }
          KindT::Interface(_) => {
            panic!("implement: evaluate_expression Destruct — Interface");
            // destructorCompiler.drop(nenv.snapshot, coutputs, range :: parentRanges, outerCallLocation, region, innerExpr2)
          }
          _ => panic!("vfail: Can't destruct type: {:?}", inner_expr_2.result()),
        };
        Ok((destroy_2, returns_from_array_expr, pending_from_inner))
      }
      IExpressionSE::Unlet(unlet_se) => {
        let name_imprecise = unlet_se.name;
        let local = match nenv.get_variable(name_imprecise, self.typing_interner) {
          Some(IVariableT::Local(rlv)) => rlv,
          Some(IVariableT::Capture(_)) => {
            panic!("implement: Unlet — AddressibleClosure (not a local)");
            // throw CompileErrorExceptionT(RangedInternalErrorT(
            //   range ::
            //     parentRanges, "Can't unlet local: " + name))
          }
          None => {
            panic!("implement: Unlet — No local with name");
            // throw CompileErrorExceptionT(RangedInternalErrorT(
            //   range :: parentRanges,
            //   "No local with name: " + name))
          }
        };
        let result_expr = self.unlet_local_without_dropping(nenv, &local);
        // This will likely be dropped, as theyre probably not doing anything with it.
        // But who knows, maybe they'll do something with it, like pass it as a parameter
        // to something.
        Ok((ExpressionTE::Unlet(self.typing_interner.alloc(result_expr)), HashSet::default(), PendingTempDrops::none()))
      }
      IExpressionSE::Index(index_se) => {
        let (unborrowed_container_expr_2, returns_from_container_expr, pending_from_container) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 0),
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          index_se.left,
        )?;
        let range_with_parent: Vec<RangeS<'s>> =
          once(index_se.range).chain(parent_ranges.iter().copied()).collect();
        let container_expr_2 = match unborrowed_container_expr_2.result() {
          KindT::BorrowRef(_) => unborrowed_container_expr_2,
          KindT::WeakRef(_) => panic!("implement: indexing a weak is a compile error"),
          KindT::KindPlaceholder(_) => {
            panic!("implement: indexing a placeholder is a compile error")
          }
          // Anything else is a value rather than a place, so it wants materializing into a
          // temporary and lending, with its drop deferred, with
          // make_temporary_local_borrow at life.add(1).
          _ => panic!("implement: materialize an rvalue container"),
        };
        let (index_expr_2, returns_from_index_expr, pending_from_index) = self.evaluate_expression(
          coutputs,
          nenv,
          loct.add(self.typing_interner, 2),
          parent_ranges,
          outer_call_location,
          nenv.default_region(),
          index_se.index_expr,
        )?;
        let expr_templata = match peel_all_references(container_expr_2.result()) {
          KindT::RuntimeSizedArray(rsa) => {
            let lookup = match self.lookup_in_unknown_sized_array(
              &range_with_parent,
              index_se.range,
              container_expr_2,
              index_expr_2,
              rsa,
            ) {
              Ok(v) => v,
              Err(e) => {
                pending_from_container.defuse_on_error();
                pending_from_index.defuse_on_error();
                return Err(e);
              }
            };
            ExpressionTE::RuntimeSizedArrayLookup(self.typing_interner.alloc(lookup))
          }
          KindT::StaticSizedArray(at) => {
            let lookup = self.lookup_in_static_sized_array(
              index_se.range,
              container_expr_2,
              index_expr_2,
              *at,
            );
            ExpressionTE::StaticSizedArrayLookup(self.typing_interner.alloc(lookup))
          }
          _ => {
            pending_from_container.defuse_on_error();
            pending_from_index.defuse_on_error();
            return Err(ICompileErrorT::CannotSubscriptT {
              range: self.typing_interner.alloc_slice_copy(&range_with_parent),
              tyype: container_expr_2.result(),
            });
          }
        };
        let mut returns = returns_from_container_expr;
        returns.extend(returns_from_index_expr);
        let mut all_pending = pending_from_container;
        all_pending.absorb(pending_from_index);
        Ok((expr_templata, returns, all_pending))
      }
      IExpressionSE::RuneLookup(r) => {
        let rune_name_s = self
          .scout_arena
          .intern_imprecise_name(IImpreciseNameValS::RuneName(RuneNameValS { rune: r.rune }));
        let templata = nenv
          .lookup_nearest_with_imprecise_name(
            rune_name_s,
            &{
              let mut s = HashSet::default();
              s.insert(ILookupContext::TemplataLookupContext);
              s
            },
            self.typing_interner,
          )
          .unwrap();
        match templata {
          ITemplataT::Integer(value) => {
            let result = ExpressionTE::ConstantInt(self.typing_interner.alloc(ConstantIntTE::new(
              ITemplataT::Integer(value),
              32,
              region,
            )));
            Ok((result, HashSet::default(), PendingTempDrops::none()))
          }
          ITemplataT::Placeholder(p)
            if matches!(p.tyype, ITemplataType::IntegerTemplataType(_)) =>
          {
            let result = ExpressionTE::ConstantInt(self.typing_interner.alloc(ConstantIntTE::new(
              ITemplataT::Placeholder(p),
              32,
              region,
            )));
            Ok((result, HashSet::default(), PendingTempDrops::none()))
          }
          ITemplataT::Prototype(_pt) => {
            let mut tiny_env =
              nenv.function_environment().make_child_node_environment(expr_1, loct);
            let arbitrary_name_t =
              INameT::Arbitrary(self.typing_interner.intern_arbitrary_name(ArbitraryNameT {}));
            tiny_env.add_entries(
              self.scout_arena,
              self.typing_interner,
              &[(arbitrary_name_t, IEnvEntryT::Templata(templata))],
            );
            let arbitrary_imprecise = self
              .scout_arena
              .intern_imprecise_name(IImpreciseNameValS::ArbitraryName(ArbitraryNameValS {}));
            let tiny_env_snapshot = tiny_env.snapshot(self.typing_interner);
            let expr = self.new_global_function_group_expression(
              IInDenizenEnvironmentT::Node(tiny_env_snapshot),
              coutputs,
              RegionT::Default,
              arbitrary_imprecise,
            );
            Ok((expr, HashSet::default(), PendingTempDrops::none()))
          }
          _ => {
            let mut ranges: Vec<RangeS<'s>> = vec![r.range.clone()];
            ranges.extend_from_slice(parent_ranges);
            return Err(ICompileErrorT::CantUseRuneValueAsExpression {
              range: self.typing_interner.alloc_slice_copy(&ranges),
              rune: r.rune,
            });
          }
        }
      }
      IExpressionSE::ConstantBool(c) => {
        let result = ExpressionTE::ConstantBool(
          self.typing_interner.alloc(ConstantBoolTE::new(c.value, region)),
        );
        Ok((result, HashSet::default(), PendingTempDrops::none()))
      }
      IExpressionSE::OverloadSet(overload_set) => {
        // Per canonical: vassert(rules.isEmpty); val name = parts.head.name
        assert!(overload_set.lookup.rules.is_empty()); // implement
        let name =
          overload_set.lookup.parts.first().expect("OverloadSet parts must be non-empty").name;
        let mut lookup_filter = HashSet::default();
        lookup_filter.insert(ILookupContext::ExpressionLookupContext);
        let templatas_from_env =
          nenv.lookup_all_with_imprecise_name(name, &lookup_filter, self.typing_interner);
        let range_list: Vec<RangeS<'s>> =
          once(overload_set.lookup.range).chain(parent_ranges.iter().copied()).collect();
        let range_list_t: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_from_vec(range_list);
        let templata_from_env = match templatas_from_env.as_slice() {
          [ITemplataT::Boolean(_value)] => {
            panic!("implement: evaluate_expression OverloadSet — BooleanTemplataT")
            // ConstantBoolTE(value, region)
          }
          [ITemplataT::Integer(_value)] => {
            panic!("implement: evaluate_expression OverloadSet — IntegerTemplataT")
            // ConstantIntTE(
            //   IntegerTemplataT(value),
            //   32,
            //   region)
          }
          [ITemplataT::Placeholder(_t)] => {
            panic!("implement: evaluate_expression OverloadSet — PlaceholderTemplataT IntegerTemplataType")
            // ConstantIntTE(PlaceholderTemplataT(name, IntegerTemplataType()), 32, region)
          }
          _ if !templatas_from_env.is_empty()
            && templatas_from_env.iter().all(|t| matches!(t, ITemplataT::Function(_))) =>
          {
            panic!("implement: evaluate_expression OverloadSet — all functions")
            // newGlobalFunctionGroupExpression(nenv.snapshot, coutputs, region, name)
          }
          _ if templatas_from_env.len() > 1 => {
            panic!("implement: evaluate_expression OverloadSet — too many results");
            // throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Found too many different things named \"" + name + "\" in env:\n" + things.map("\n" + _)))
          }
          [] => {
            return Err(ICompileErrorT::CouldntFindIdentifierToLoadT { range: range_list_t, name });
          }
          _ => unreachable!(
            "OverloadSet match is exhaustive; over-matched for slice-pattern exhaustiveness"
          ),
        };
        Ok((templata_from_env, HashSet::default(), PendingTempDrops::none()))
      }
    }
  }

  pub fn check_array(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    range: &[RangeS<'s>],
    array_mutability: SharednessT,
    element_coord: KindT<'s, 't>,
    generator_prototype: PrototypeT<'s, 't>,
    generator_type: KindT<'s, 't>,
  ) {
    panic!("Unimplemented: Slab 15");
    // if (generatorPrototype.returnType != elementCoord) {
    //   throw CompileErrorExceptionT(RangedInternalErrorT(range, "Generator return type doesn't agree with array element type!"))
    // }
    // if (generatorPrototype.paramTypes.size != 2) {
    //   throw CompileErrorExceptionT(RangedInternalErrorT(range, "Generator must take in 2 args!"))
    // }
    // if (generatorPrototype.paramTypes(0) != generatorType) {
    //   throw CompileErrorExceptionT(RangedInternalErrorT(range, "Generator first param doesn't agree with generator expression's result!"))
    // }
    // generatorPrototype.paramTypes(1) match {
    //   case CoordT(ShareT, _, IntT.i32) =>
    //   case _ => {
    //     throw CompileErrorExceptionT(
    //       RangedInternalErrorT(range, "Generator must take in an integer as its second param!"))
    //   }
    // }
    // if (arrayMutability == ImmutableT &&
    //   Compiler.getMutability(coutputs, elementCoord.kind) == MutabilityTemplataT(MutableT)) {
    //   throw CompileErrorExceptionT(RangedInternalErrorT(range, "Can't have an immutable array of mutable elements!"))
    // }
  }

  pub fn get_option(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &'t FunctionEnvironmentT<'s, 't>,
    range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    context_region: RegionT,
    contained_coord: KindT<'s, 't>,
  ) -> Result<
    (KindT<'s, 't>, PrototypeT<'s, 't>, PrototypeT<'s, 't>, IdT<'s, 't>, IdT<'s, 't>),
    ICompileErrorT<'s, 't>,
  > {
    let opt_name = self
      .scout_arena
      .intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.keywords.opt }));
    let interface_templata = match IEnvironmentT::from(nenv).lookup_nearest_with_imprecise_name(
      opt_name,
      [ILookupContext::TemplataLookupContext].into_iter().collect(),
      self.typing_interner,
    ) {
      Some(ITemplataT::InterfaceDefinition(it)) => *it,
      _ => panic!("vfail"),
    };

    let call_range_t = self.typing_interner.alloc_slice_copy(range);
    let opt_interface_val = match self.resolve_interface(
      coutputs,
      IInDenizenEnvironmentT::from(nenv),
      call_range_t,
      call_location,
      interface_templata,
      &[ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: contained_coord }))],
    ) {
      IResolveOutcome::ResolveSuccess(s) => s.kind,
      _ => panic!("vfail"),
    };
    let opt_interface_ref =
      self.typing_interner.intern_interface_tt(InterfaceTTValT { id: opt_interface_val.id });
    let own_opt_coord = KindT::Interface(opt_interface_ref);

    let some_name = self
      .scout_arena
      .intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.keywords.some }));
    let some_constructor_templata = match IEnvironmentT::from(nenv)
      .lookup_nearest_with_imprecise_name(
        some_name,
        [ILookupContext::ExpressionLookupContext].into_iter().collect(),
        self.typing_interner,
      ) {
      Some(ITemplataT::Function(ft)) => *ft,
      _ => panic!("vwat"),
    };
    let some_constructor = match self.evaluate_generic_light_function_from_call_for_prototype(
      coutputs,
      range,
      call_location,
      IInDenizenEnvironmentT::from(nenv),
      some_constructor_templata,
      &[ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: contained_coord }))],
      context_region,
      &[contained_coord],
      &[],
    )? {
      IResolveFunctionResult::ResolveFunctionFailure(_fff) => {
        panic!("CompileErrorExceptionT: RangedInternalErrorT")
      }
      IResolveFunctionResult::ResolveFunctionSuccess(p) => p.prototype.prototype,
    };

    let none_name = self
      .scout_arena
      .intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.keywords.none }));
    let none_constructor_templata = match IEnvironmentT::from(nenv)
      .lookup_nearest_with_imprecise_name(
        none_name,
        [ILookupContext::ExpressionLookupContext].into_iter().collect(),
        self.typing_interner,
      ) {
      Some(ITemplataT::Function(ft)) => *ft,
      _ => panic!("vwat"),
    };
    let none_constructor = match self.evaluate_generic_light_function_from_call_for_prototype(
      coutputs,
      range,
      call_location,
      IInDenizenEnvironmentT::from(nenv),
      none_constructor_templata,
      &[ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: contained_coord }))],
      context_region,
      &[],
      &[],
    )? {
      IResolveFunctionResult::ResolveFunctionFailure(_fff) => {
        panic!("CompileErrorExceptionT: RangedInternalErrorT")
      }
      IResolveFunctionResult::ResolveFunctionSuccess(p) => p.prototype.prototype,
    };

    let some_impl_id = match self.is_parent(
      coutputs,
      IInDenizenEnvironmentT::from(nenv),
      range,
      call_location,
      ISubKindTT::from(some_constructor.return_type.expect_citizen()),
      ISuperKindTT::Interface(opt_interface_ref),
    ) {
      IsParentResult::IsParent(p) => p.impl_id,
      IsParentResult::IsntParent(_) => panic!("vwat"),
    };

    let none_impl_id = match self.is_parent(
      coutputs,
      IInDenizenEnvironmentT::from(nenv),
      range,
      call_location,
      ISubKindTT::from(none_constructor.return_type.expect_citizen()),
      ISuperKindTT::Interface(opt_interface_ref),
    ) {
      IsParentResult::IsParent(p) => p.impl_id,
      IsParentResult::IsntParent(_) => panic!("vwat"),
    };

    Ok((own_opt_coord, *some_constructor, *none_constructor, some_impl_id, none_impl_id))
  }

  pub fn get_result(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &'t FunctionEnvironmentT<'s, 't>,
    range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    contained_success_coord: KindT<'s, 't>,
    contained_fail_coord: KindT<'s, 't>,
  ) -> Result<
    (KindT<'s, 't>, PrototypeT<'s, 't>, IdT<'s, 't>, PrototypeT<'s, 't>, IdT<'s, 't>),
    ICompileErrorT<'s, 't>,
  > {
    let result_name =
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS {
        name: self.keywords.result,
      }));
    let interface_templata = match IEnvironmentT::from(nenv).lookup_nearest_with_imprecise_name(
      result_name,
      [ILookupContext::TemplataLookupContext].into_iter().collect(),
      self.typing_interner,
    ) {
      Some(ITemplataT::InterfaceDefinition(it)) => *it,
      _ => panic!("vfail"),
    };

    let call_range_t = self.typing_interner.alloc_slice_copy(range);
    let result_interface_val = match self.resolve_interface(
      coutputs,
      IInDenizenEnvironmentT::from(nenv),
      call_range_t,
      call_location,
      interface_templata,
      &[
        ITemplataT::Kind(
          self.typing_interner.alloc(KindTemplataT { kind: contained_success_coord }),
        ),
        ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: contained_fail_coord })),
      ],
    ) {
      IResolveOutcome::ResolveSuccess(s) => s.kind,
      _ => panic!("vfail"),
    };
    let result_interface_ref =
      self.typing_interner.intern_interface_tt(InterfaceTTValT { id: result_interface_val.id });
    let own_result_coord = KindT::Interface(result_interface_ref);

    let ok_name = self
      .scout_arena
      .intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.keywords.ok }));
    let ok_constructor_templata = match IEnvironmentT::from(nenv)
      .lookup_nearest_with_imprecise_name(
        ok_name,
        [ILookupContext::ExpressionLookupContext].into_iter().collect(),
        self.typing_interner,
      ) {
      Some(ITemplataT::Function(ft)) => *ft,
      _ => panic!("vwat"),
    };
    let ok_constructor = match self.evaluate_generic_light_function_from_call_for_prototype(
      coutputs,
      range,
      call_location,
      IInDenizenEnvironmentT::from(nenv),
      ok_constructor_templata,
      &[
        ITemplataT::Kind(
          self.typing_interner.alloc(KindTemplataT { kind: contained_success_coord }),
        ),
        ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: contained_fail_coord })),
      ],
      region,
      &[contained_success_coord],
      &[],
    )? {
      IResolveFunctionResult::ResolveFunctionFailure(fff) => {
        return Err(ICompileErrorT::TypingPassResolvingError {
          range: self.typing_interner.alloc_slice_copy(range),
          inner: fff.reason,
        });
      }
      IResolveFunctionResult::ResolveFunctionSuccess(p) => p.prototype.prototype,
    };
    let ok_kind = ok_constructor.return_type;
    let ok_result_impl = match self.is_parent(
      coutputs,
      IInDenizenEnvironmentT::from(nenv),
      range,
      call_location,
      ISubKindTT::from(ok_kind.expect_struct()),
      ISuperKindTT::Interface(result_interface_ref),
    ) {
      IsParentResult::IsParent(p) => p.impl_id,
      IsParentResult::IsntParent(_) => panic!("vfail"),
    };

    let err_name = self
      .scout_arena
      .intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.keywords.err }));
    let err_constructor_templata = match IEnvironmentT::from(nenv)
      .lookup_nearest_with_imprecise_name(
        err_name,
        [ILookupContext::ExpressionLookupContext].into_iter().collect(),
        self.typing_interner,
      ) {
      Some(ITemplataT::Function(ft)) => *ft,
      _ => panic!("vwat"),
    };
    let err_constructor = match self.evaluate_generic_light_function_from_call_for_prototype(
      coutputs,
      range,
      call_location,
      IInDenizenEnvironmentT::from(nenv),
      err_constructor_templata,
      &[
        ITemplataT::Kind(
          self.typing_interner.alloc(KindTemplataT { kind: contained_success_coord }),
        ),
        ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: contained_fail_coord })),
      ],
      region,
      &[contained_fail_coord],
      &[],
    )? {
      IResolveFunctionResult::ResolveFunctionFailure(fff) => {
        return Err(ICompileErrorT::TypingPassResolvingError {
          range: self.typing_interner.alloc_slice_copy(range),
          inner: fff.reason,
        });
      }
      IResolveFunctionResult::ResolveFunctionSuccess(p) => p.prototype.prototype,
    };
    let err_kind = err_constructor.return_type;
    let err_result_impl = match self.is_parent(
      coutputs,
      IInDenizenEnvironmentT::from(nenv),
      range,
      call_location,
      ISubKindTT::from(err_kind.expect_struct()),
      ISuperKindTT::Interface(result_interface_ref),
    ) {
      IsParentResult::IsParent(p) => p.impl_id,
      IsParentResult::IsntParent(_) => panic!("vfail"),
    };

    Ok((own_result_coord, *ok_constructor, ok_result_impl, *err_constructor, err_result_impl))
  }

  pub fn weak_alias(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &'t [RangeS<'s>],
    expr: ExpressionTE<'s, 't>,
  ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
    match expr.result() {
      KindT::Struct(sr) => {
        let struct_def = coutputs.lookup_struct(sr.id, self);
        if !struct_def.weakable {
          return Err(ICompileErrorT::TookWeakRefOfNonWeakableError { range: parent_ranges });
        }
      }
      KindT::Interface(ir) => {
        let interface_def = coutputs.lookup_interface(ir.id, self);
        if !interface_def.weakable {
          return Err(ICompileErrorT::TookWeakRefOfNonWeakableError { range: parent_ranges });
        }
      }
      _ => panic!("vfail"),
    }

    match expr.result() {
      KindT::BorrowRef(_) => Ok(ExpressionTE::BorrowToWeak(
        self.typing_interner.alloc(BorrowToWeakTE::new(self.typing_interner, expr)),
      )),
      other => panic!("vwat: {:?}", other),
    }
  }

  pub fn evaluate_closure(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    global_env: &'t GlobalEnvironmentT<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    parent_ranges: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    name: IFunctionDeclarationNameS<'s>,
    function_s: &'s FunctionS<'s>,
  ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
    let function_a = self.astronomize_lambda(coutputs, nenv, parent_ranges, function_s);

    let snapshot_env = nenv.snapshot(self.typing_interner);
    let closure_struct_tt = self.evaluate_closure_struct(
      coutputs,
      global_env,
      snapshot_env,
      parent_ranges,
      call_location,
      name,
      function_a,
      true,
    )?;
    let closure_kind = KindT::Struct(self.typing_interner.alloc(closure_struct_tt));

    let mut range_list = vec![function_a.range];
    range_list.extend_from_slice(parent_ranges);
    let construct_expr_2 = self.make_closure_struct_construct_expression(
      coutputs,
      nenv,
      &range_list,
      call_location,
      region,
      closure_struct_tt,
    );
    assert_eq!(construct_expr_2.result(), closure_kind);

    Ok(construct_expr_2)
  }

  pub fn new_global_function_group_expression(
    &self,
    env: IInDenizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    region: RegionT,
    name: IImpreciseNameS<'s>,
  ) -> ExpressionTE<'s, 't> {
    let name_ref: &'s IImpreciseNameS<'s> = self.scout_arena.alloc(name);
    let overload_set =
      self.typing_interner.intern_overload_set(OverloadSetTValT { env, name: name_ref });
    let void_expr: ExpressionTE<'s, 't> =
      ExpressionTE::VoidLiteral(self.typing_interner.alloc(VoidLiteralTE::new(region)));
    ExpressionTE::Reinterpret(
      self.typing_interner.alloc(ReinterpretTE::new(void_expr, KindT::OverloadSet(overload_set))),
    )
  }

  // VTRACE: hide
  pub fn evaluate_block_statements(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    starting_nenv: &'t NodeEnvironmentT<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    loct: LocT<'t>,
    parent_ranges: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    block: &'s BlockSE<'s>,
  ) -> Result<(ExpressionTE<'s, 't>, HashSet<KindT<'s, 't>>), ICompileErrorT<'s, 't>> {
    self.evaluate_block_statements_block(
      coutputs,
      starting_nenv,
      nenv,
      parent_ranges,
      call_location,
      loct,
      region,
      block,
    )
  }

  pub fn astronomize_lambda(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    parent_ranges: &'t [RangeS<'s>],
    function_s: &'s FunctionS<'s>,
  ) -> &'s FunctionS<'s> {
    let range_s = function_s.range;
    let name_s = function_s.name;
    let attributes_s = function_s.attributes;
    let identifying_runes_s = function_s.generic_params;
    let tyype = &function_s.tyype;
    let params_s = function_s.params;
    let maybe_ret_coord_rune = &function_s.maybe_ret_kind_rune;
    let body_s = function_s.body;

    let mut rune_s_to_pre_known_type_a: IndexMap<IRuneS<'s>, ITemplataType<'s>> =
      identifying_runes_s.iter().map(|gp| (gp.rune.rune, gp.tyype.tyype())).collect();
    for param in params_s {
      rune_s_to_pre_known_type_a
        .insert(param.full_type_rune.rune, ITemplataType::KindTemplataType(KindTemplataType {}));
    }

    let snapshot = nenv.snapshot(self.typing_interner);
    let env_ref: IInDenizenEnvironmentT<'s, 't> = IInDenizenEnvironmentT::Node(snapshot);
    let rune_type_solve_env = self.create_rune_type_solver_env(env_ref);

    let rune_type_solver = RuneTypeSolver { scout_arena: self.scout_arena };
    let mut range_list = vec![range_s];
    range_list.extend_from_slice(parent_ranges);
    // Run the solve only to check the rune types are solvable. Its result isn't used here.
    // explicify_lookups was the only consumer of that map, and it is retired.
    // The rules are already explicit Lookup/Call. The lambda's runes get solved again when it is typed.
    match rune_type_solver.solve_rune_types(
      coutputs,
      self.opts.global_options.sanity_check,
      &rune_type_solve_env,
      range_list.clone(),
      function_s.header_rules,
      &identifying_runes_s.iter().map(|gp| gp.rune.rune).collect::<Vec<_>>(),
      true,
      rune_s_to_pre_known_type_a,
    ) {
      Ok(_) => {}
      Err(_e) => panic!("CouldntSolveRuneTypesT"),
    }

    // The UserFunction attribute is stamped in postparse (function_scout), so attributes_s
    // already carries it — pass it through unchanged.
    self.scout_arena.alloc(FunctionS::new(
      range_s,
      name_s,
      attributes_s,
      identifying_runes_s,
      tyype.clone(),
      params_s,
      maybe_ret_coord_rune.clone(),
      function_s.effects,
      function_s.header_rules,
      function_s.impl_bounds,
      body_s,
    ))
  }

  pub fn drop_since(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    loct: LocT<'t>,
    region: RegionT,
    expr_te: ExpressionTE<'s, 't>,
    unreversed_variables_to_destruct: Vec<&'t LocalVariable<'s, 't>>
  ) -> Result<ExpressionTE<'s, 't>, ICompileErrorT<'s, 't>> {
    if unreversed_variables_to_destruct.is_empty() {
      Ok(expr_te)
    } else {
      match expr_te.result() {
        KindT::Void(_) => {
          let reversed_variables_to_destruct: Vec<_> =
            unreversed_variables_to_destruct.iter().rev().copied().collect();
          let destroy_expressions = self.unlet_and_drop_all(
            coutputs,
            nenv,
            range,
            call_location,
            region,
            &reversed_variables_to_destruct,
          )?;
          let mut exprs: Vec<ExpressionTE<'s, 't>> = Vec::new();
          exprs.push(expr_te);
          exprs.extend(destroy_expressions);
          exprs.push(ExpressionTE::VoidLiteral(
            self.typing_interner.alloc(VoidLiteralTE::new(region)),
          ));
          Ok(self.consecutive(&exprs))
        }
        KindT::Never(_) => {
          // In this case, we want to not drop them, so we can support things like:
          //   func drop(self Server) { panic("unreachable"); }
          // and not drop Server.
          let reversed_variables_to_destruct: Vec<_> =
            unreversed_variables_to_destruct.iter().rev().copied().collect();
          let _destroy_expressions =
            self.unlet_all_without_dropping(coutputs, nenv, range, &reversed_variables_to_destruct);
          // Just dont add in the destroyExpressions, let em go.
          // We did the above simply to mark them as unstackified.
          Ok(expr_te)
        }
        _ => {
          let (resultified_expr, result_local_variable) =
            self.resultify_expressions(nenv, loct.add(self.typing_interner, 1), expr_te);
          let reversed_variables_to_destruct: Vec<_> =
            unreversed_variables_to_destruct.iter().rev().copied().collect();
          let destroy_expressions = self.unlet_and_drop_all(
            coutputs,
            nenv,
            range,
            call_location,
            region,
            &reversed_variables_to_destruct,
          )?;
          let mut exprs: Vec<ExpressionTE<'s, 't>> = Vec::new();
          exprs.push(resultified_expr);
          exprs.extend(destroy_expressions);
          let result_ilocal_variable = result_local_variable;
          let unlet_te = self.unlet_local_without_dropping(nenv, result_ilocal_variable);
          exprs.push(ExpressionTE::Unlet(self.typing_interner.alloc(unlet_te)));
          Ok(self.consecutive(&exprs))
        }
      }
    }
  }

  pub fn resultify_expressions(
    &self,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    loct: LocT<'t>,
    expr: ExpressionTE<'s, 't>,
  ) -> (ExpressionTE<'s, 't>, &'t LocalVariable<'s, 't>) {
    let result_var_ref = self
      .typing_interner
      .intern_typing_pass_block_result_var_name(TypingPassBlockResultVarNameT { loct: loct });
    let result_var_name: IVarNameT<'s, 't> = result_var_ref.into();
    let result_variable: &'t LocalVariable<'s, 't> =
      self.typing_interner.alloc(LocalVariable { name: result_var_name, tyype: expr.result() });
    let result_let = LetNormalTE::new(result_variable, expr);
    nenv.add_variable(IVariableT::Local(result_variable));
    (ExpressionTE::LetNormal(self.typing_interner.alloc(result_let)), result_variable)
  }
}

struct LetExprRuneTypeSolverEnv<'a, 's, 't>
where
  's: 't,
{
  nenv: &'a NodeEnvironmentBox<'s, 't>,
  typing_interner: &'a TypingInterner<'s, 't>,
  scout_arena: &'a ScoutArena<'s>,
}

impl<'a, 's, 't> IRuneTypeSolverEnv<'s, 't> for LetExprRuneTypeSolverEnv<'a, 's, 't>
where
  's: 't,
{
  fn lookup(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    range: RangeS<'s>,
    parts: &[IImpreciseNameS<'s>],
  ) -> Result<IRuneTypeSolverLookupResult<'s>, IRuneTypingLookupFailedError<'s>> {
    // The last segment names the item; only diagnostics need it separately.
    let name_s = *parts.last().expect("vwat: an empty lookup path");
    let mut filter = HashSet::default();
    filter.insert(ILookupContext::TemplataLookupContext);
    let found = lookup_nearest_with_path(
      IEnvironmentT::from(self.nenv.snapshot(self.typing_interner)),
      parts,
      filter,
      self.typing_interner,
    );
    citizen_or_templata_rune_type_lookup(coutputs, self.scout_arena, found, range, name_s)
  }
}

// VCOORD: change to 2 spaces
