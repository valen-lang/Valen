# Migration Checking

We're in the process of doublechecking every single function and class in FrontendRust to make sure that the Rust version matches the Scala version (which should be in a comment just below it), so we can consider together any differences and whether they're okay.

To be clear, for every Rust function, there should be a /* ... */ comment underneath it with Scala code.

Your goal is to find all logic differences between the Rust code and the Scala code.

Read the *entire* file, not just pieces of it. Even if it's large, read the entire file.

Please assemble me a report, that lists all the functions/structs in the file, with a list of checks performed plus possible differences between them.

For example, if I gave you these four warnings:

```
Check for:

1. There shouldn't be places where Rust uses an iterator where Scala used an index.

2. There shouldn't be any "placeholder" comments or "ignore for now" comments. There should instead be panic!("unimplemented") calls.

3. Seemingly dead code in Scala should still have corresponding Rust, which either has similar dead Rust code, or at least a `panic!("seemingly dead code ...` in the Rust version instead.

4. There shouldn't be any Rust code that doesn't have a /* ... */ underneath it with Scala code.
```

If those were the four rules, and this was the code:

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

If those were the four rules, and the above was the code, then your report for each function would be like this:

```
lex_angled:
1 ok
2 difference: There's a placeholder instead of a report_advance call!
3 ok
4 ok
Other Differences:
- Rust isn't checking for the ending '>'
- Rust has an extra iter.consume_comments_and_whitespace() call
```

Of course, there are more rules in reality, so your report will be longer.

Your file should be a bunch of these, one after the other, one for each function/struct.

Notes on format:

 * Don't mention any similarities. They're not important.
 * Just "ok" for the ok cases. No description for those necessary.
 * If there are no Other Differences then feel free to omit that section. Don't include a useless "perfect match" Other Differences section.
 * Do not summarize at the end, and do not give an overall assessment.

## Rules

Be very careful when looking at these rules. Don't assume a difference is fine unless it's explicitly mentioned in here.

## Watch out for

Watch out for:

1. Where the Scala version has an assertion or an exception, the Rust version should not just silently fail and it shouldnt just log and continue.

2. There shouldn't be any "placeholder" comments or "ignore for now" comments. There should instead be panic!("unimplemented") calls.

3. Seemingly dead code in Scala should still have corresponding Rust, which either has similar dead Rust code, or at least a `panic!("seemingly dead code ...` in the Rust version instead.

4. There shouldn't be any Rust code that doesn't have a /* ... */ underneath it with Scala code.

5. There shouldn't be places where Rust uses an iterator where Scala used an index, or vice versa.

6. There shouldn't be any missing comments from the Rust code that wasn't in the Scala code. Comments are important.

7. If Rust handled errors gracefully where Scala just vimpl()'d or vfail()'d, that's bad. Rust should do what Scala did even if Scala looks worse.

8. Potentially large data structures shouldn't be `.clone()`d.

## MIGALLOW

If there's a comment *in the Scala* code that says MIGALLOW, it will describe an allowed
difference between the Scala and Rust versions. Only humans can add those, and they can only
add them to the Scala code. AI isn't allowed to modify the reference Scala code.

## Allowed differences

No need to mention any of these.

### GC vs Arc

The Scala version used GC, because it's Scala. Rust doesn't have GC.

Because of this, anything that we need shared should have an Arc around it. This includes:

 * Interner, Keywords.
 * Interned values, like StrI, FileCoordinate, PackageCoordinate.

However, Arc<Mutex<T>> is a code smell. It's invalid, unless there is a comment above the T describing why it's okay to be Arc-Mutex'd. AI isn't allowed to write those, only users.

### Clone

Cloning is sometimes needed in Rust when it's not needed in Scala.

However, only do this on value types, nothing that might ever be mutated.

### Scala traits vs Rust enums

It's fine if Rust uses enums for Scala sealed traits.

Though, they should contain the same things.

### Box

Box is sometimes needed in Rust, that's fine.

### Minor allowed differences

These aren't worth mentioning:

 * Ignore Scala Profiler.frame() calls.
 * Scala StringBuilder is equivalent to Rust String::new()/push().
 * Scala vimpl() is equivalent to Rust panic!().
 * Scala vassert() is equivalent to Rust assert!()
 * Scala's iter.code() is equivalent to Rust iter.code.chars().nth()
 * Scala's iter.code.slice(begin, end) is equivalent to Rust's &iter.code[begin as usize..end as usize]
 * Scala's vassertSome is equivalent to Rust .expect()
 * If there's an unused Scala variable, and Rust also has it but underscored, that's fine.
 * Scala's Accumulator is equivalent to Rust Vec
 * Scala's Either vs equivalent custom Rust enums


