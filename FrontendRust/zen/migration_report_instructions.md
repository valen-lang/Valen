# Migration Checking

We're in the process of doublechecking every single function and class in FrontendRust to make sure that the Rust version matches the Scala version (which should be in a comment just below it), so we can consider together any differences and whether they're okay.

To be clear, for every Rust function, there should be a /* ... */ comment underneath it with Scala code.

Your goal is to find all logic differences between the Rust code and the Scala code.

Read the *entire* file, not just pieces of it. Even if it's large, read the entire file.

Please assemble me a report, that lists all the checks performed for a specific function/struct in the file.

For example, if I gave you these four checks:

```
Check for:

M1: There shouldn't be places where Rust uses an iterator where Scala used an index.

M2: There shouldn't be any "placeholder" comments or "ignore for now" comments. There should instead be panic!("unimplemented") calls.

DCSSHCR: Seemingly dead code in Scala should still have corresponding Rust, which either has similar dead Rust code, or at least a `panic!("seemingly dead code ...` in the Rust version instead.

NNRC: There shouldn't be any Rust code that doesn't have a /* ... */ underneath it with Scala code.
```

If those were the four checks, and this was the code:

```
fn lex_angled(&mut self, iter: &mut LexingIterator) -> Result<Option<AngledLE>> {
    let begin = iter.get_pos();

    if !(iter.peek() == '<' && self.angle_is_open_or_close(iter)) {
        return Ok(None);
    }
    iter.advance();

    // Placeholder for calling report_advance

    iter.consume_comments_and_whitespace();

    let innards = self.lex_scramble(iter, false, false, false)?;

    iter.consume_comments_and_whitespace();

    let end = iter.get_pos();

    iter.consume_comments_and_whitespace();

    Ok(Some(AngledLE {
        range: RangeL::new(begin, end),
        contents: innards,
    }))
}
/*
def lexAngled(iter: LexingIterator): Result[Option[AngledLE], IParseError] = {
    val begin = iter.getPos()

    if (!(iter.peek() == '<' && angleIsOpenOrClose(iter))) {
        return Ok(None)
    }
    iter.advance()

    report_advance()

    iter.consumeCommentsAndWhitespace()

    val innards =
    lexScramble(iter, false, false, false) match {
        case Err(e) => return Err(e)
        case Ok(x) => x
    }

    iter.consumeCommentsAndWhitespace()

    if (!iter.trySkip('>')) {
        vfail()
    }

    val end = iter.getPos()

    Ok(Some(AngledLE(RangeL(begin, end), innards)))
}
*/
```

If those were the four checks, and the above was the code, then your report for the function would be like this:

```
lex_angled:
M1 ok
M2 difference: There's a placeholder instead of a report_advance call!
DCSSHCR ok
NNRC ok
Other Differences:
- Rust isn't checking for the ending '>'
- Rust has an extra iter.consume_comments_and_whitespace() call
```

Of course, there are more than four checks in reality, so your report will be longer.

Notes on format:

 * Don't mention any similarities. They're not important.
 * Just "ok" for the ok cases. No description for those necessary.
 * If there are no Other Differences then feel free to omit that section. Don't include a useless "perfect match" Other Differences section.
 * Do not summarize at the end, and do not give an overall assessment.

## MIGALLOW

If there's a comment *in the Scala* code that says MIGALLOW, it will describe an allowed
difference between the Scala and Rust versions. Only humans can add those, and they can only
add them to the Scala code. AI isn't allowed to modify the reference Scala code.

## Ignorable differences

You can find allowed differences in migration_differences.md. No need to mention any of these.

ONLY MIGALLOW'D THINGS AND THINGS SPECIFICALLY LISTED IN migration_differences.md MAY BE IGNORED. NO OTHER DIFFERENCES MAY BE IGNORED.
