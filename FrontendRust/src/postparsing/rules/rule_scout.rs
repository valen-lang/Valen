// Per @DSAUIMZ, all borrow_val() calls in this file borrow from a stack-local
// LocationInDenizenBuilder instead of arena-allocating. The slice is promoted
// to permanent arena storage only inside intern_rune on a miss.

use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::parsing::ast::{BuiltinCallPR, EqualsPR, IRulexPR, ITypePR};
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::itemplatatype::{
  BooleanTemplataType, ITemplataType, IntegerTemplataType, KindTemplataType,
  PackTemplataType, RegionTemplataType,
};
use crate::postparsing::names::{CodeRuneS, IRuneS, IRuneValS, ImplicitRuneValS};
use crate::postparsing::post_parser::{IEnvironmentS, PostParser};
use crate::postparsing::rules::rules::{EqualsSR, ImplBoundS, IRulexSR, RuneUsage};
use crate::postparsing::rules::templex_scout::translate_templex;


/// Returns the translated versions of the given rules. Two things exit through out-params instead:
/// `builder` collects rules produced on the side, and `impl_bounds` collects `implements(..)`
/// clauses, which are declared bounds rather than rules — see ImplBoundS.
pub fn translate_rulexes<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  impl_bounds: &mut Vec<ImplBoundS<'s>>,
  context_region: IRuneS<'s>,
  rules_p: &[IRulexPR<'p>],
) -> Vec<RuneUsage<'s>> {
  rules_p
    .iter()
    .map(|rule_p| {
      let mut child_lidb = lidb.child();
      translate_rulex(
        scout_arena,
        keywords,
        env.clone(),
        &mut child_lidb,
        builder,
        impl_bounds,
        context_region.clone(),
        rule_p,
      )
    })
    .collect()
}

fn translate_rulex<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  impl_bounds: &mut Vec<ImplBoundS<'s>>,
  context_region: IRuneS<'s>,
  rulex: &IRulexPR<'p>,
) -> RuneUsage<'s> {
  let file = match &env {
    IEnvironmentS::Environment(environment) => environment.file,
    IEnvironmentS::FunctionEnvironment(function_environment) => function_environment.file,
  };
  match rulex {
    IRulexPR::Typed(typed_rule) => {
      let rune = match &typed_rule.rune {
        Some(rune_name) => scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: scout_arena.intern_str(rune_name.str().as_str()) })),
        None => {
          let mut child_lidb = lidb.child();
          scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val())))
        }
      };
      RuneUsage {
        range: PostParser::eval_range(file, typed_rule.range),
        rune,
      }
    }
    IRulexPR::Templex(templex) => {
      let mut child_lidb = lidb.child();
      translate_templex(
        scout_arena,
        keywords,
        env,
        &mut child_lidb,
        builder,
        context_region,
        templex,
      )
    }
    IRulexPR::Equals(EqualsPR { range, left, right }) => {
      let mut child_lidb = lidb.child();
      let rune = scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(child_lidb.borrow_val())));
      let left_usage = {
        let mut child_lidb = lidb.child();
        translate_rulex(scout_arena,
          keywords,
          env.clone(),
          &mut child_lidb,
          builder,
          impl_bounds,
          context_region.clone(),
          left,
        )
      };
      let right_usage = {
        let mut child_lidb = lidb.child();
        translate_rulex(scout_arena,
          keywords,
          env.clone(),
          &mut child_lidb,
          builder,
          impl_bounds,
          context_region.clone(),
          right,
        )
      };
      builder.push(IRulexSR::Equals(EqualsSR {
        range: PostParser::eval_range(file, *range),
        left: left_usage,
        right: right_usage,
      }));
      RuneUsage {
        range: PostParser::eval_range(file, *range),
        rune,
      }
    }
    IRulexPR::Or(r) => panic!("POSTPARSER_TRANSLATE_RULEX_NOT_YET_IMPLEMENTED: Or at {:?}", r.range),
    IRulexPR::Dot(r) => panic!("POSTPARSER_TRANSLATE_RULEX_NOT_YET_IMPLEMENTED: Dot at {:?}", r.range),
    IRulexPR::BuiltinCall(BuiltinCallPR { range, name, args }) => {
      // Compare on content, not identity: `name` is interned in the parse arena while
      // `keywords` here is the scout one, so a StrI comparison across them never matches.
      if name.str().as_str() != "implements" {
        panic!(
          "POSTPARSER_TRANSLATE_RULEX_BUILTINCALL_NOT_YET_IMPLEMENTED: {} at {:?}",
          name.str().as_str(), range);
      }
      assert_eq!(args.len(), 2, "POSTPARSER_IMPLEMENTS_ARGS_LEN");
      let sub_rune = translate_rulex(
        scout_arena, keywords, env.clone(), &mut lidb.child(), builder, impl_bounds,
        context_region.clone(), &args[0]);
      let super_rune = translate_rulex(
        scout_arena, keywords, env, &mut lidb.child(), builder, impl_bounds,
        context_region, &args[1]);

      // The result rune joins this declared bound to the impl a caller supplies; the instantiator
      // zips the two maps by it. Nothing solves it — the post-solve pass fills it in, which is why
      // it is deliberately absent from every rune-usage list.
      let mut result_child_lidb = lidb.child();
      let result_rune = RuneUsage {
        range: PostParser::eval_range(file, *range),
        rune: scout_arena.intern_rune(
          IRuneValS::ImplicitRune(ImplicitRuneValS::new(result_child_lidb.borrow_val()))),
      };

      impl_bounds.push(ImplBoundS {
        range: PostParser::eval_range(file, *range),
        sub_rune,
        super_rune,
        result_rune,
      });

      sub_rune
    }
    IRulexPR::Pack(r) => panic!("POSTPARSER_TRANSLATE_RULEX_NOT_YET_IMPLEMENTED: Pack at {:?}", r.range),
  }
}


pub fn translate_type<'s>(scout_arena: &ScoutArena<'s>, tyype: ITypePR) -> ITemplataType<'s> {
  match tyype {
    ITypePR::IntType => ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
    ITypePR::BoolType => ITemplataType::BooleanTemplataType(BooleanTemplataType {}),
    ITypePR::CoordListType => ITemplataType::PackTemplataType(PackTemplataType {
      element_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    }),
    ITypePR::RegionType => ITemplataType::RegionTemplataType(RegionTemplataType {}),
    ITypePR::CitizenTemplateType => {
      panic!("POSTPARSER_TRANSLATE_TYPE_CITIZEN_TEMPLATE_NOT_YET_IMPLEMENTED")
    }
  }
}
