// Per @DSAUIMZ, all borrow_val() calls in this file borrow from a stack-local
// LocationInDenizenBuilder instead of arena-allocating. The slice is promoted
// to permanent arena storage only inside intern_rune on a miss.

use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::parsing::ast::{ComponentsPR, EqualsPR, IRulexPR, ITypePR};
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::itemplatatype::{
  BooleanTemplataType, ITemplataType, IntegerTemplataType, KindTemplataType,
  PackTemplataType, RegionTemplataType,
};
use crate::postparsing::names::{CodeRuneS, IRuneS, IRuneValS, ImplicitRuneValS};
use crate::postparsing::post_parser::{IEnvironmentS, PostParser};
use crate::postparsing::rules::rules::{EqualsSR, IRulexSR, RuneUsage};
use crate::postparsing::rules::templex_scout::translate_templex;


// Returns:
// - new rules produced on the side while translating the given rules
// - the translated versions of the given rules
pub fn translate_rulexes<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  keywords: &Keywords<'s>,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  builder: &mut Vec<IRulexSR<'s>>,
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType<'s>)>,
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
        rune_to_explicit_type,
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
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType<'s>)>,
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
      let tyype = translate_type(scout_arena, typed_rule.tyype);
      rune_to_explicit_type.push((rune.clone(), tyype));
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
          rune_to_explicit_type,
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
          rune_to_explicit_type,
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
    IRulexPR::Components(ComponentsPR {
      range,
      container: tyype,
      components,
    }) => {
      let mut rune_child_lidb = lidb.child();
      let rune = RuneUsage {
        range: PostParser::eval_range(file, *range),
        rune: scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(rune_child_lidb.borrow_val()))),
      };
      rune_to_explicit_type.push((rune.rune.clone(), translate_type(scout_arena, *tyype)));
      match tyype {
        ITypePR::KindType => {
          if components.len() != 1 {
            panic!("Kind rule should have one component! Found: {}", components.len())
          }
          let mut translate_child_lidb = lidb.child();
          let _component_usages = translate_rulexes(
            scout_arena,
            keywords,
            env,
            &mut translate_child_lidb,
            builder,
            rune_to_explicit_type,
            context_region,
            components,
          );
        }
        _ => panic!("POSTPARSER_COMPONENTS_INVALID_TYPE_FOR_COMPONENTS_RULE"),
      }
      rune
    }
    _ => panic!("POSTPARSER_TRANSLATE_RULEX_NOT_YET_IMPLEMENTED"),
  }
}


pub fn translate_type<'s>(scout_arena: &ScoutArena<'s>, tyype: ITypePR) -> ITemplataType<'s> {
  match tyype {
    ITypePR::IntType => ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
    ITypePR::BoolType => ITemplataType::BooleanTemplataType(BooleanTemplataType {}),
    ITypePR::CoordListType => ITemplataType::PackTemplataType(PackTemplataType {
      element_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    }),
    ITypePR::KindType => ITemplataType::KindTemplataType(KindTemplataType {}),
    ITypePR::RegionType => ITemplataType::RegionTemplataType(RegionTemplataType {}),
    ITypePR::CitizenTemplateType => {
      panic!("POSTPARSER_TRANSLATE_TYPE_CITIZEN_TEMPLATE_NOT_YET_IMPLEMENTED")
    }
  }
}
