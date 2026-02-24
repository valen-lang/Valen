---
name: slice-start
model: composer-1.5
description: Insert mig slice comments above every Scala definition in commented-out Scala code
---

You were pointed at some commented out Scala code in a Rust test file.

I want you to put a "mig slice comment" above every Scala definition in this file's comments.

A "mig slice comment" for e.g. def compileForError would look like this:

```rs
*/
// mig: def compileForError
/*
```

For a `test("...")` block, include the full test name string in the comment:

```rs
*/
// mig: test("Regular variable")
/*
```

You should put one of these above every Scala function definition, type definition, and test.

# Example 1

For example, if the file currently contains this:

```rs
/*
class PostParserVariableTests extends FunSuite with Matchers {
  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Successfully compiled!\n" + t.toString)
    }
  }
  private def compile(code: String): ProgramS = {
    val interner = new Interner()
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => {
        val codeMap = FileCoordinateMap.test(interner, code)
        vfail(
          PostParserErrorHumanizer.humanize(
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            e))
      }
      case Ok(t) => t.expectOne()
    }
  }
  test("Regular variable") {
    val program1 = compile("exported func main() int { x = 4; }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    vassert(body.block.locals.size == 1)
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
      NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
  test("Type-less local has no coord rune") {
    val program1 = compile("exported func main() int { x = 4; }")
    val main = program1.lookupFunction("main")
    val local = Collector.only(main, { case let @ LetSE(_, rules, pattern, _) => let })
    local.pattern.coordRune shouldEqual None
  }
*/
```

Then you would make it look like this:

```rs
// mig: class PostParserVariableTests
/*
class PostParserVariableTests extends FunSuite with Matchers {
*/
// mig: def compileForError
/*
  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Successfully compiled!\n" + t.toString)
    }
  }
*/
// mig: def compile
/*
  private def compile(code: String): ProgramS = {
    val interner = new Interner()
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => {
        val codeMap = FileCoordinateMap.test(interner, code)
        vfail(
          PostParserErrorHumanizer.humanize(
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            e))
      }
      case Ok(t) => t.expectOne()
    }
  }
*/
// mig: test("Regular variable")
/*
  test("Regular variable") {
    val program1 = compile("exported func main() int { x = 4; }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    vassert(body.block.locals.size == 1)
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
      NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
// mig: test("Type-less local has no coord rune")
/*
  test("Type-less local has no coord rune") {
    val program1 = compile("exported func main() int { x = 4; }")
    val main = program1.lookupFunction("main")
    val local = Collector.only(main, { case let @ LetSE(_, rules, pattern, _) => let })
    local.pattern.coordRune shouldEqual None
  }
*/
```

Note how the mig slice comments are interleaved with the old Scala tests, and we inserted `*/` and `/*` to make that happen. That is good.

# Example 2: Splitting an impl

If you were given this:

```rs
/*
class FileCoordinateMap[Contents](
  val packageCoordToFileCoords: mutable.Map[PackageCoordinate, Vector[FileCoordinate]] =
    mutable.HashMap[PackageCoordinate, Vector[FileCoordinate]](),
  val fileCoordToContents: mutable.Map[FileCoordinate, Contents] =
    mutable.HashMap[FileCoordinate, Contents]()
) {
  // This is different from put in that we can hand in an empty map here.
  // It's the only way to have an empty package in the FileCoordinateMap.
  def putPackage(
    interner: Interner,
    packageCoord: PackageCoordinate,
    newFileCoordToContents: Map[FileCoordinate, Contents]):
  Unit = {
    packageCoordToFileCoords.put(packageCoord, newFileCoordToContents.keys.toVector)
    newFileCoordToContents.foreach({ case (fileCoord, contents) =>
      fileCoordToContents.put(fileCoord, contents)
    })
  }
  def map[T](func: (FileCoordinate, Contents) => T): FileCoordinateMap[T] = {
    val resultFileCoordToContents = mutable.HashMap[FileCoordinate, T]()
    fileCoordToContents.foreach({ case (fileCoord, contents) =>
      resultFileCoordToContents.put(fileCoord, func(fileCoord, contents))
    })
    new FileCoordinateMap(packageCoordToFileCoords, resultFileCoordToContents)
  }
}
*/
```

THen you should make it look like this:

```rs
/*
*/
// mig: class FileCoordinateMap
/*
class FileCoordinateMap[Contents](
  val packageCoordToFileCoords: mutable.Map[PackageCoordinate, Vector[FileCoordinate]] =
    mutable.HashMap[PackageCoordinate, Vector[FileCoordinate]](),
  val fileCoordToContents: mutable.Map[FileCoordinate, Contents] =
    mutable.HashMap[FileCoordinate, Contents]()
) {
*/
// mig: def putPackage
/*
  // This is different from put in that we can hand in an empty map here.
  // It's the only way to have an empty package in the FileCoordinateMap.
  def putPackage(
    interner: Interner,
    packageCoord: PackageCoordinate,
    newFileCoordToContents: Map[FileCoordinate, Contents]):
  Unit = {
    packageCoordToFileCoords.put(packageCoord, newFileCoordToContents.keys.toVector)
    newFileCoordToContents.foreach({ case (fileCoord, contents) =>
      fileCoordToContents.put(fileCoord, contents)
    })
  }
*/
// mig: def map
/*
  def map[T](func: (FileCoordinate, Contents) => T): FileCoordinateMap[T] = {
    val resultFileCoordToContents = mutable.HashMap[FileCoordinate, T]()
    fileCoordToContents.foreach({ case (fileCoord, contents) =>
      resultFileCoordToContents.put(fileCoord, func(fileCoord, contents))
    })
    new FileCoordinateMap(packageCoordToFileCoords, resultFileCoordToContents)
  }
*/
// mig: def flatMap
/*
  def flatMap[T](func: (FileCoordinate, Contents) => T): Iterable[T] = {
    fileCoordToContents.map({ case (fileCoord, contents) =>
      func(fileCoord, contents)
    })
  }
}
*/
```

# Example 3

If you're given this:

```rs
/*
object PackageCoordinate {// extends Ordering[PackageCoordinate] {
  def TEST_TLD(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(interner.intern(StrI("test")), Vector.empty))

  def BUILTIN(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))

  def internal(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
}
*/
```

You should give me this:

```rs
/*
object PackageCoordinate {// extends Ordering[PackageCoordinate] {
*/
// mig: def TEST_TLD
/*
  def TEST_TLD(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(interner.intern(StrI("test")), Vector.empty))
*/
// mig: def BUILTIN
/*
  def BUILTIN(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
*/
// mig: def internal
/*
  def internal(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
*/
/*
}
*/
```

Rule of thumb: No two Scala functions/types should be next to each other; No Scala function/type should be next to another Scala function/type. The file needs to alternate mig->Scala->mig->Scala->mig->Scala->etc.

# Guidelines

 * Slice apart scala comments with `*/` and `/*` so you can put the mig comment where it belongs.
 * No two Scala functions/types should be next to each other; No Scala function/type should be next to another Scala function/type. The file needs to alternate mig->Scala->mig->Scala->mig->Scala->etc.
 * You can ignore Scala `object` statements, but pay attention to the things inside them.

# Restrictions

 * Your job is *not* to make it build, so please do not build it. Do not run `cargo build`, do not run `cargo run`, do not run `cargo test`.
 * You cannot reorder the old scala comments relative to each other. They must be in the same order as they were before.

# When done

Say "done" when you're done modifying the code.
