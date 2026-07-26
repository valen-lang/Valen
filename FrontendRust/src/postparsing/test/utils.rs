use crate::postparsing::ast::IBodyS;
use crate::postparsing::expressions::IExpressionSE;
use crate::postparsing::names::{CodeNameS, IImpreciseNameS, IRuneS};
use crate::postparsing::rules::rules::{CallSR, IRulexSR, LookupSR};
use crate::postparsing::test::traverse::NodeRefS;
use crate::{collect_only_snodes, collect_where_snodes};


// The head expression of a normal (code) function body. Panics with the actual body if it isn't a
// code body, since a wrong body variant (extern/abstract) means the test's setup is off.
pub fn expect_code_body_expr<'s>(body: &'s IBodyS<'s>) -> &'s IExpressionSE<'s> {
  match body {
    IBodyS::CodeBody(code_body) => code_body.body.block.expr,
    other => panic!("expected a code body, got {:?}", other),
  }
}

// Asserts `rune` is where the named type resolved to, following the whole lowering.
//
// A bare type name lowers to `Lookup(template)` plus a zero-arg `Call`, so a rune that "is" a named
// type holds the Call's result, not the Lookup's. Asserting against the Lookup is the natural
// mistake and passes nothing.
pub fn assert_rune_resolves_to<'s>(rules: &'s [IRulexSR<'s>], rune: IRuneS<'s>, name: &str) {
  let nodes: Vec<NodeRefS<'s>> = rules.iter().map(NodeRefS::Rulex).collect();
  let lookup_rune = collect_only_snodes!(
    &nodes,
    NodeRefS::LookupRule(LookupSR {
      rune, name: IImpreciseNameS::CodeName(CodeNameS { name: looked_up, .. }), ..
    }) if looked_up.as_str() == name => Some(rune.rune));
  let call_result_rune = collect_only_snodes!(
    &nodes,
    NodeRefS::Rulex(IRulexSR::Call(CallSR { result_rune, template_rune, args: [], .. }))
      if template_rune.rune == lookup_rune => Some(result_rune.rune));
  assert_eq!(rune, call_result_rune, "expected the rune to be {}'s zero-arg Call result", name);
}

// Asserts no rule mentions `rune` — that nothing constrains it.
//
// Worth checking for any rune whose value arrives from outside the solve: the completeness check
// demands a conclusion for every rune a rule mentions, so one filled in afterward must appear in
// none of them. Pass the rule list rather than a whole denizen, since a denizen's traversal can
// reach a rune through fields that aren't rules.
pub fn assert_rune_absent_from_rules<'s>(rules: &'s [IRulexSR<'s>], rune: IRuneS<'s>) {
  let nodes: Vec<NodeRefS<'s>> = rules.iter().map(NodeRefS::Rulex).collect();
  let usages: Vec<()> = collect_where_snodes!(
    &nodes, NodeRefS::RuneUsage(usage) if usage.rune == rune => Some(()));
  assert!(
    usages.is_empty(),
    "expected no rule to mention the rune; found {} usage(s)", usages.len());
}
