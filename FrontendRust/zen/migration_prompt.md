======== PLACEHOLDERS

You're going to help me migrate some code.

First, please look at these files for guidelines to follow:
- migration_process.md
- migration_principles.md

Then say "ready"

==

I'd like you to add a rust function header for every unmigrated Scala function you see.

Feel free to use the `check_scala_rust_mapping.py` script to know what to add.

It should have no instructions inside, just a `panic!("Unimplemented {function name here}");`. For example if we're trying to migrate this:

    /*
    def flattenExpressions(expr: IExpressionSE): Vector[IExpressionSE] = {
      expr match {
        case ConsecutorSE(exprs) => exprs.flatMap(flattenExpressions)
        case other => Vector(other)
      }
    }
    */

then you should insert this rust function right above it:

    fn flatten_expressions<'a, 's>(
      expr: &IExpressionSE<'a, 's>,
    ) -> Vec<&'s IExpressionSE<'a, 's>> {
      panic!("Unimplemented flatten_expressions");
    }
    /*
    def flattenExpressions(expr: IExpressionSE): Vector[IExpressionSE] = {
      expr match {
        case ConsecutorSE(exprs) => exprs.flatMap(flattenExpressions)
        case other => Vector(other)
      }
    }
    */

DO NOT attempt to build, run, or test. Ignore the placement of impl blocks, and you don't have to add any impl blocks. Just add `fn` statements where they should be, and I'll add the necessarily impl blocks myself later.

Feel free to leave off namespace qualifiers, for example `crate::postparsing::rules::rules::IRulexSR<'a>` can just be `IRulexSR<'a>`. I'll add the imports later.

======== MIGRATE

You're going to help me migrate some code.

First, please look at these files for guidelines to follow:
- migration_process.md
- migration_principles.md
- testing.md

Then say "ready"

== (tests only)

Please help me replace these unimplemented stubs with good Rust code that matches the old Scala code, in 

== (tests and impl)

I'm about to tell you to replace a certain test with good Rust code that matches the old scala code.

Implement anything in src/postparsing that is directly and immediately needed to make the next test. The unimplemented parts will only be in src/postparsing.

CRITICAL RULES:

 * DO NOT ADD ANY novel logic! All the functions you need should already exist as Scala code in a comment. NO adding new functions. You will only be modifying existing functions.
 * Anything you add should be *directly immediately above* the Scala comment. NOT below the comment. NOT in a different file. Feel free to slice scala comments apart so the new rust code can be directly above the old scala code.
 * Only *iteratively* implement the bare minimum that you need to make it compile. Add panic!/assert! placeholders until it compiles, then implement only the panic!s/assert!s your test runs into.

If any of these sound like a problem, then stop and ask me for help.

The test we'll be working to enable is 

== (followup)

/migration-correct

======== CHECKS

Look again at these files for guidelines to follow (they might have changed):
- migration_process.md
- migration_principles.md
- testing.md

then say "ready"

==

/migration-diff-review

== (followup impl)

/migration-drive


======== NOTES

AI is really bad at putting code where it should go, and understanding that it should only be adding functions that already existed in scala. even when i call it out, it doesnt understand it, and puts it somewhere else. even when i ask it to doublecheck, it's wrong: "You’re right. Right now in expression_scout.rs, the structs are below their Scala case-class comments, not directly above them:" AND even when I asked it to fix them, it still couldn't put them in the right place!

is this similar to how it didnt know when to do resolution order things? when it had a choice of what to do and it had to use its best judgement, it couldnt and basically rolled the dice. possible moral: when there are multiple degrees of freedom, instead of a straight linear path, it tends to fall over?

possible fixes:
- a pre-pass creating the rust equivalents of all the signatures
- plan mode, with a specific step afterward calling it out for putting things in the wrong places?
- never have it implement multiple changes at once. and for each thing it needs to add, tell it to first look for where the scala equivalent is.


Its also bad at knowing when to call out to a helper that doesnt yet exist... it was mimicking scoutElementsAsExpressions inline, instead of making that function.



Also it didnt follow the "no novel logic" rule, because instead of doing a call out to the pattern functions like 
      val patternS =
        patternScout.translatePattern(
          stackFrame1, lidb.child(), ruleBuilder, runeToExplicitType, patternP)
it just put in a temporary hack like:
      let declared_name = match &destination.decl {
        INameDeclarationP::LocalNameDeclaration(local_name) => IVarNameS::CodeVarName(local_name.str.clone()),
        INameDeclarationP::ConstructingMemberNameDeclaration(member_name) => {
          IVarNameS::ConstructingMemberName(member_name.str.clone())
        }
        _ => panic!("POSTPARSER_SCOUT_LET_DECL_NOT_YET_IMPLEMENTED"),
      };
so perhaps:
 * we should just say that it can only add panics in branches? but then it would create default values which isnt right
 * we should just do the pre-pass that creates rust stubs.




wow, it sucks at dealing with lifetimes.