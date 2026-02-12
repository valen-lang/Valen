You're going to help me migrate some code.

First, please look at these files for guidelines to follow:
- migration_process.md
- migration_checks.md
- testing.md

Then look at expression_tests.rs for an example.

Then say "ready"

-- (tests only)

Please help me replace these unimplemented stubs with good Rust code that matches the old Scala code, in 

-- (tests and impl)

Please help me replace these unimplemented stubs with good Rust code that matches the old Scala code, in
@FrontendRust/src/postparsing/test/post_parser_tests.rs:96-115 .
implement anything in the implementation that you see missing. the unimplemented parts will only be in src/postparsing. anything that's missing will definitely have some old scala code in comments. your new rust code should be in the same exact place as the old scala code comments. each function/struct/whatever should be right above its corresponding old scala comment.

--------

Look again at these files for guidelines to follow (they might have changed):
- migration_process.md
- migration_checks.md
- testing.md

then say "ready"

--

Someone just migrated this file, but im not sure they did it well. can you fix what they missed or got wrong?