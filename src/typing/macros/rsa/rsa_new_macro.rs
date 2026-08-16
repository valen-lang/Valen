use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;

use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::names::{CodeRuneS, IImpreciseNameValS, IRuneValS, RuneNameValS};
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::{IInDenizenEnvironmentT, ILookupContext};
use crate::typing::env::function_environment_t::*;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::RegionT;
use crate::typing::types::types::*;
use crate::utils::fx::HashSet;
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn generate_function_body_rsa_new(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    life: LocationInFunctionEnvironmentT<'t>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function: Option<&FunctionS<'s>>,
    param_coords: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> (FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>) {
    let header = FunctionHeaderT {
      id: env.id,
      attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
      params: self.typing_interner.alloc_slice_from_vec(param_coords.to_vec()),
      return_type: maybe_ret_coord.expect("vassertSome: maybeRetCoord"),
      maybe_origin_function_templata: Some(env.templata()),
    };
    coutputs.declare_function_return_type(
      self.typing_interner.alloc(header.to_signature()),
      header.return_type,
    );

    let rune_e =
      self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.e }));
    let rune_name_e = self
      .scout_arena
      .intern_imprecise_name(IImpreciseNameValS::RuneName(RuneNameValS { rune: rune_e }));
    let element_type = match IInDenizenEnvironmentT::from(env)
      .lookup_nearest_with_imprecise_name(
        rune_name_e,
        {
          let mut s = HashSet::default();
          s.insert(ILookupContext::TemplataLookupContext);
          s
        },
        self.typing_interner,
      )
      .expect("vassertSome: E rune")
    {
      ITemplataT::Kind(ct) => ct.kind,
      _ => panic!("vwat"),
    };

    let array_tt = self.resolve_runtime_sized_array(element_type, RegionT::Default);

    let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(ExpressionTE::Return(
      self.typing_interner.alloc(ReturnTE::new(ExpressionTE::NewRuntimeSizedArray(
        self.typing_interner.alloc(NewRuntimeSizedArrayTE::new(
          self.typing_interner.alloc(array_tt),
          RegionT::Default,
          ExpressionTE::ArgLookup(
            self.typing_interner.alloc(ArgLookupTE::new(0, param_coords[0].tyype)),
          ),
        )),
      ))),
    ))));
    (header, body)
  }
}
