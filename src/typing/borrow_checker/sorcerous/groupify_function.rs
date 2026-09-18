use bumpalo::Bump;
use crate::postparsing::ast::{FunctionS, IBodyS};
use crate::postparsing::names::IVarDeclarationNameS;
use crate::postparsing::rules::RuneUsage;
use crate::postparsing::rules::types::{BorrowRefST, GroupS, ITypeST, RegionS};
use crate::StrI;
use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::ast::expressions::*;
use crate::typing::borrow_checker::access_event::AccessEventG;
use crate::typing::borrow_checker::ast_g::*;
use crate::typing::borrow_checker::group_expr::GroupExprG;
use crate::typing::borrow_checker::kind_g::*;
use crate::typing::borrow_checker::templata_g::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::ICitizenNameT;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::*;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  pub fn groupify_function<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    bump_g: &'g Bump,
  ) -> Result<(ExpressionGE<'s, 't, 'g>, Vec<&'g AccessEventG<'s, 't>>), ICompileErrorT<'s, 't>> {
    // TODO: use function_s to compare to any expression_t that we find, that will let us manually
    // specify things' groups, for example in let statements.
    // For now, just use the typed expressions.
    let mut access_log = Vec::new();
    let expr_ge = self.groupify_expression(coutputs, function_s, function_t, bump_g, &mut access_log, function_t.body)?;
    Ok((expr_ge, access_log))
  }

  fn groupify_expression<'g>(
    &self,
    coutputs: &CompilerOutputs,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    bump_g: &'g Bump,
    access_log: &mut Vec<&'g AccessEventG<'s, 't>>,
    expression_te: ExpressionTE<'s, 't>,
  ) -> Result<ExpressionGE<'s, 't, 'g>, ICompileErrorT<'s, 't>> {
    match expression_te {
      ExpressionTE::LetAndLend(LetAndLendTE { .. }) => unimplemented!(),
      ExpressionTE::LockWeak(LockWeakTE { .. }) => unimplemented!(),
      ExpressionTE::BorrowToWeak(BorrowToWeakTE { .. }) => unimplemented!(),
      ExpressionTE::LetNormal(LetNormalTE { range, variable, expr, result, .. }) => {
        let result_gt = KindGT::Bool(BoolGT { });
        Ok(ExpressionGE::ConstantBool(bump_g.alloc(ConstantBoolGE { range: *range, value: *value, result: result_gt, })))
      }
      ExpressionTE::Unlet(UnletTE { .. }) => unimplemented!(),
      ExpressionTE::Discard(DiscardTE { .. }) => unimplemented!(),
      ExpressionTE::If(IfTE { .. }) => unimplemented!(),
      ExpressionTE::While(WhileTE { .. }) => unimplemented!(),
      ExpressionTE::Mutate(MutateTE { .. }) => unimplemented!(),
      ExpressionTE::Restackify(RestackifyTE { .. }) => unimplemented!(),
      ExpressionTE::Return(ReturnTE { range, source_expr, result, .. }) => {
        let result_gt = KindGT::Never(NeverGT { from_break: false });
        let source_expr_g = self.groupify_expression(coutputs, function_s, function_t, bump_g, access_log, *source_expr)?;
        Ok(ExpressionGE::Return(bump_g.alloc(ReturnGE { range: *range, source_expr: source_expr_g, result: source_expr_g.result(), })))
      }
      ExpressionTE::Break(BreakTE { .. }) => unimplemented!(),
      ExpressionTE::Block(BlockTE { range, inner, result, .. }) => {
        let inner_g = self.groupify_expression(coutputs, function_s, function_t, bump_g, access_log, *inner)?;
        Ok(ExpressionGE::Block(bump_g.alloc(BlockGE { range: *range, inner: inner_g, result: inner_g.result(), })))
      }
      ExpressionTE::Consecutor(ConsecutorTE { range, exprs: exprs_te, result: result_tt, .. }) => {
        let mut exprs_ge = Vec::new();
        for expr_te in exprs_te.iter() {
          exprs_ge.push(
            self.groupify_expression(coutputs, function_s, function_t, bump_g, access_log, *expr_te)?);
        }
        let exprs_ge_slice = bump_g.alloc_slice_copy(exprs_ge.as_slice());
        let result_gt = self.groupify_type(bump_g, *result_tt, None, None);
        Ok(ExpressionGE::Consecutor(bump_g.alloc(ConsecutorGE { range: *range, exprs: exprs_ge_slice, result: result_gt, })))
      }
      ExpressionTE::StaticArrayFromValues(StaticArrayFromValuesTE { .. }) => unimplemented!(),
      ExpressionTE::ArraySize(ArraySizeTE { .. }) => unimplemented!(),
      ExpressionTE::IsSameInstance(IsSameInstanceTE { .. }) => unimplemented!(),
      ExpressionTE::AsSubtype(AsSubtypeTE { .. }) => unimplemented!(),
      ExpressionTE::VoidLiteral(VoidLiteralTE { .. }) => unimplemented!(),
      ExpressionTE::ConstantInt(ConstantIntTE { range, value, bits, .. }) => {
        let value_ge = self.groupify_templata(bump_g, *value);
        let result_gt = KindGT::Int(IntGT { bits: *bits });
        Ok(ExpressionGE::ConstantInt(bump_g.alloc(ConstantIntGE { range: *range, value: value_ge, bits: *bits, result: result_gt, })))
      }
      ExpressionTE::ConstantBool(ConstantBoolTE { range, value, .. }) => {
        let result_gt = KindGT::Bool(BoolGT { });
        Ok(ExpressionGE::ConstantBool(bump_g.alloc(ConstantBoolGE { range: *range, value: *value, result: result_gt, })))
      }
      ExpressionTE::ConstantStr(ConstantStrTE { .. }) => unimplemented!(),
      ExpressionTE::ConstantFloat(ConstantFloatTE { range, value, .. }) => {
        let result_gt = KindGT::Float(FloatGT { });
        Ok(ExpressionGE::ConstantFloat(bump_g.alloc(ConstantFloatGE { range: *range, value: *value, result: result_gt, })))
      }
      ExpressionTE::ArgLookup(ArgLookupTE { range, param_index, result, .. }) => {
        let param_type_t = function_t.header.params[*param_index as usize].tyype;
        let param_type_s = function_s.params[*param_index as usize].tyype;
        let param_name = function_s.params[*param_index as usize].name;
        let result_gt = self.groupify_type(bump_g, param_type_t, Some(param_type_s), Some(param_name));
        Ok(ExpressionGE::ArgLookup(bump_g.alloc(ArgLookupGE { range: *range, param_index: *param_index, result: result_gt, })))
      }
      ExpressionTE::ArrayLength(ArrayLengthTE { range, array_expr, result, .. }) => {
        let array_expr_ge = self.groupify_expression(coutputs, function_s, function_t, bump_g, access_log, *array_expr)?;
        let result_gt = KindGT::Int(IntGT { bits: 32 });
        Ok(ExpressionGE::ArrayLength(bump_g.alloc(ArrayLengthGE { range: *range, array_expr: array_expr_ge, result: result_gt, })))
      }
      ExpressionTE::InterfaceFunctionCall(InterfaceFunctionCallTE { .. }) => unimplemented!(),
      ExpressionTE::ExternFunctionCall(ExternFunctionCallTE { .. }) => unimplemented!(),
      ExpressionTE::FunctionCall(FunctionCallTE { .. }) => unimplemented!(),
      ExpressionTE::BoundFunctionCall(BoundFunctionCallTE { .. }) => unimplemented!(),
      ExpressionTE::Reinterpret(ReinterpretTE { .. }) => unimplemented!(),
      ExpressionTE::Construct(ConstructTE { .. }) => unimplemented!(),
      ExpressionTE::NewRuntimeSizedArray(NewRuntimeSizedArrayTE { .. }) => unimplemented!(),
      ExpressionTE::StaticArrayFromCallable(StaticArrayFromCallableTE { .. }) => unimplemented!(),
      ExpressionTE::DestroyStaticSizedArrayIntoFunction(DestroyStaticSizedArrayIntoFunctionTE { .. }) => unimplemented!(),
      ExpressionTE::DestroyStaticSizedArrayIntoLocals(DestroyStaticSizedArrayIntoLocalsTE { .. }) => unimplemented!(),
      ExpressionTE::DestroyRuntimeSizedArray(DestroyRuntimeSizedArrayTE { .. }) => unimplemented!(),
      ExpressionTE::RuntimeSizedArrayCapacity(RuntimeSizedArrayCapacityTE { .. }) => unimplemented!(),
      ExpressionTE::PushRuntimeSizedArray(PushRuntimeSizedArrayTE { .. }) => unimplemented!(),
      ExpressionTE::PopRuntimeSizedArray(PopRuntimeSizedArrayTE { .. }) => unimplemented!(),
      ExpressionTE::InterfaceToInterfaceUpcast(InterfaceToInterfaceUpcastTE { .. }) => unimplemented!(),
      ExpressionTE::UpcastInterface(UpcastInterfaceTE { .. }) => unimplemented!(),
      ExpressionTE::UpcastGeneric(UpcastGenericTE { .. }) => unimplemented!(),
      ExpressionTE::Destroy(DestroyTE { .. }) => unimplemented!(),
      ExpressionTE::CopyPrim(CopyPrimTE { .. }) => unimplemented!(),
      ExpressionTE::LocalLookup(LocalLookupTE { .. }) => unimplemented!(),
      ExpressionTE::StaticSizedArrayLookup(StaticSizedArrayLookupTE { .. }) => unimplemented!(),
      ExpressionTE::RuntimeSizedArrayLookup(RuntimeSizedArrayLookupTE { .. }) => unimplemented!(),
      ExpressionTE::MemberLookup(MemberLookupTE { .. }) => unimplemented!(),
      ExpressionTE::Deref(DerefTE { .. }) => unimplemented!(),
    }
  }

  fn groupify_type<'g>(
    &self,
    bump_g: &'g Bump,
    type_t: KindT<'s, 't>,
    maybe_type_s: Option<ITypeST<'s>>,
    name: Option<IVarDeclarationNameS<'s>>,
  ) -> KindGT<'s, 't, 'g> {
    match type_t {
      KindT::BorrowRef(BorrowRefT { inner }) => {
        let inner_gt = self.groupify_type(bump_g, type_t, None, None);
        let type_s = maybe_type_s.expect("Encountered a borrow ref with no written group");
        let BorrowRefST { range: bst_range, inner: bst_inner, region: bst_group_s } =
            match type_s {
              ITypeST::BorrowRef(bst) => *bst,
              _ => panic!("Encountered a borrow ref not matching up with a written borrow ref"),
            };
        let bst_specified_group_s =
          match bst_group_s {
            RegionS::Unspecified => panic!("Encountered a borrow ref with unspecified region"),
            RegionS::Held => panic!("Encountered a borrow ref with held region"),
            RegionS::Group(b) => *b,
          };
        let group_expr_g: GroupExprG<'s, 't, 'g> =
              match bst_specified_group_s {
                GroupS::Rune(RuneUsage { range: ru_range, rune: group_expr_g_rune }) => {
                  GroupExprG::Rune(group_expr_g_rune)
                }
                GroupS::Local(_) => unimplemented!(),
                GroupS::Member { .. } => unimplemented!(),
                GroupS::Elements { .. } => unimplemented!(),
                GroupS::Ellipsis { .. } => unimplemented!(),
                GroupS::Union { .. } => unimplemented!(),
              };
        KindGT::BorrowRef(bump_g.alloc(BorrowRefGT {
          inner: inner_gt,
          group: group_expr_g,
        }))
      },
      KindT::Never(NeverT { from_break }) => KindGT::Never(NeverGT { from_break }),
      KindT::Void(VoidT { }) => KindGT::Void(VoidGT { }),
      KindT::Int(IntT { bits }) => KindGT::Int(IntGT { bits }),
      KindT::Bool(BoolT { }) => KindGT::Bool(BoolGT { }),
      KindT::Str(StrT { }) => KindGT::Str(StrGT { }),
      KindT::Float(FloatT { }) => KindGT::Float(FloatGT { }),
      KindT::USize(USizeT { }) => KindGT::USize(USizeGT { }),
      KindT::Struct(StructTT { id, .. }) => {
        let template_args_t: &'t [ITemplataT<'s, 't>] =
            ICitizenNameT::try_from(id.local_name)
                .expect("Struct without ICitizenNameT")
                .template_args();
        let template_args_g =
            bump_g.alloc_slice_copy(
                template_args_t
                    .iter()
                    .map(|x| self.groupify_templata(bump_g, *x))
                    .collect::<Vec<_>>()
                    .as_slice());
        KindGT::Struct(bump_g.alloc(StructGT { id, template_args: template_args_g }))
      }
      KindT::KindPlaceholder(KindPlaceholderT { .. }) => unimplemented!(), // KindGT::KindPlaceholder(KindPlaceholderGT { }),
      KindT::Interface(InterfaceTT { .. }) => unimplemented!(), // KindGT::Interface(InterfaceGT { }),
      KindT::StaticSizedArray(StaticSizedArrayTT { .. }) => unimplemented!(), // KindGT::StaticSizedArray(StaticSizedArrayGT { }),
      KindT::RuntimeSizedArray(RuntimeSizedArrayTT { .. }) => unimplemented!(), // KindGT::RuntimeSizedArray(RuntimeSizedArrayGT { }),
      KindT::OverloadSet(OverloadSetT { .. }) => unimplemented!(), // KindGT::OverloadSet(OverloadSetGT { }),
      KindT::OwnRef(OwnRefT { .. }) => unimplemented!(), // KindGT::OwnRef(OwnRefGT { }),
      KindT::ShareRef(ShareRefT { .. }) => unimplemented!(), // KindGT::ShareRef(ShareRefGT { }),
      KindT::WeakRef(WeakRefT { .. }) => unimplemented!(), // KindGT::WeakRef(WeakRefGT { }),
    }
  }

  fn groupify_templata<'g>(
    &self,
    bump_g: &'g Bump,
    templata_t: ITemplataT<'s, 't>,
  ) -> ITemplataG<'s, 't> {
    match templata_t {
      ITemplataT::Kind(_) => unimplemented!(),
      ITemplataT::Placeholder(_) => unimplemented!(),
      ITemplataT::Integer(_) => unimplemented!(),
      ITemplataT::Boolean(_) => unimplemented!(),
      ITemplataT::String(_) => unimplemented!(),
      ITemplataT::Prototype(_) => unimplemented!(),
      ITemplataT::Isa(_) => unimplemented!(),
      ITemplataT::CoordList(_) => unimplemented!(),
      ITemplataT::RuntimeSizedArrayTemplate(_) => unimplemented!(),
      ITemplataT::StaticSizedArrayTemplate(_) => unimplemented!(),
      ITemplataT::Group(_) => unimplemented!(),
      ITemplataT::Function(_) => unimplemented!(),
      ITemplataT::StructDefinition(_) => unimplemented!(),
      ITemplataT::InterfaceDefinition(_) => unimplemented!(),
      ITemplataT::ImplDefinition(_) => unimplemented!(),
      ITemplataT::ExternFunction(_) => unimplemented!(),
    }
  }
}
