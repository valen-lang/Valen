---
name: placehold-tests
description: Add Rust test placeholders corresponding to commented out Scala tests
---

You were pointed at some commented out Scala code in a Rust test file.

I want you to do these things:

 1. First, please look at migration_process.md, migration_checks.md, and testing.md.
 2. Then, say to me a list of all the Scala function definitions and type definitions in the file, in order. Don't include any Rust things.
 3. Then, say to me the "target" list: a list like #2, except we're interleaving in the names of the Rust things, to get an ordered interleaved list of all the names of the Rust and Scala things, in the order they should be in. Rust name, then old Scala name, then the next ones.
 4. Then, can you please add some test stubs for any Scala functions/types that don't yet have a corresponding Rust function/type right above them? Each commented out Scala thing should have a corresponding Rust thing right above it.
 5. Ensure that no two Scala functions/types are next to each other. There should be a Rust function/type above every Scala function/type.
 6. Ensure that you didn't change the order of the Scala function/type definitions. Those comments must not move relative to each other, they must be in the same order they were in.

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

Then you should give me this list for step 2:

 * class PostParserVariableTests
 * def compileForError
 * def compile
 * test("Regular variable")
 * test("Type-less local has no coord rune")

Then you should give me this target list for step 3:

 * (no Rust equivalent needed for Scala class PostParserVariableTests)
 * class PostParserVariableTests
 * fn compile_for_error
 * def compileForError
 * fn compile
 * def compile
 * fn regular_variable
 * test("Regular variable")
 * fn type_less_local_has_no_coord_rune
 * test("Type-less local has no coord rune")

Then you would make it look like this:

```rs
/*
class PostParserVariableTests extends FunSuite with Matchers {
*/
fn compile_for_error<'a, 'ctx, 'p>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  arena: &'p Bump,
  code: &str,
) -> ICompileErrorS<'a>
where
  'a: 'ctx,
  'a: 'p,
{
  panic!("Unimplemented: compile_for_error");
}
/*
  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Successfully compiled!\n" + t.toString)
    }
  }
*/
fn compile<'a, 'ctx, 'p>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  arena: &'p Bump,
  code: &str,
) -> ProgramS<'a, 'p>
where
  'a: 'ctx,
  'a: 'p,
{
  panic!("Unimplemented: compile");
}
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
#[test]
fn regular_variable() {
  panic!("Unmigrated test: typeless_local_has_no_coord_rune");
}
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
#[test]
fn typeless_local_has_no_coord_rune() {
  panic!("Unmigrated test: typeless_local_has_no_coord_rune");
}
/*
  test("Type-less local has no coord rune") {
    val program1 = compile("exported func main() int { x = 4; }")
    val main = program1.lookupFunction("main")
    val local = Collector.only(main, { case let @ LetSE(_, rules, pattern, _) => let })
    local.pattern.coordRune shouldEqual None
  }
*/
```

Note how the Rust placeholder tests are interleaved with the old Scala tests, and we inserted `*/` and `/*` to make that happen. That is good.

# Example 2: Splitting an impl

Here we have an example of a file that already has some things migrated to Rust, but they're in the wrong place:

```rs
#[derive(Clone)]
pub struct FileCoordinateMap<'a, Contents> {
  pub package_coord_to_file_coords: HashMap<&'a PackageCoordinate<'a>, Vec<&'a FileCoordinate<'a>>>,
  pub file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
}
impl<'a, Contents: Clone> FileCoordinateMap<'a, Contents> {
  pub fn put_package(
    &mut self,
    package_coord: &'a PackageCoordinate<'a>,
    new_file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
  ) {
    panic!("Unimplemented: FileCoordinateMap::put_package");
  }

  pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<'a, T>
  where
    F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
    T: Clone,
  {
    panic!("Unimplemented: FileCoordinateMap::map");
  }
}
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

Then you should give me this list for step 2:

 * class FileCoordinateMap
 * def putPackage
 * def map

Then you should give me this target list for step 3:

 * struct FileCoordianateMap
 * impl FileCoordinateMap
 * class FileCoordinateMap
 * fn put_package
 * def putPackage
 * fn map
 * def map

It doesn't matter that they're in an `impl` block. Put the `impl` block around the scala comment so that we can interleave the functions. Like this:

```rs
#[derive(Clone)]
pub struct FileCoordinateMap<'a, Contents> {
  pub package_coord_to_file_coords: HashMap<&'a PackageCoordinate<'a>, Vec<&'a FileCoordinate<'a>>>,
  pub file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
}
/*
class FileCoordinateMap[Contents](
  val packageCoordToFileCoords: mutable.Map[PackageCoordinate, Vector[FileCoordinate]] =
    mutable.HashMap[PackageCoordinate, Vector[FileCoordinate]](),
  val fileCoordToContents: mutable.Map[FileCoordinate, Contents] =
    mutable.HashMap[FileCoordinate, Contents]()
) {
*/
impl<'a, Contents: Clone> FileCoordinateMap<'a, Contents> {
/*
*/
  pub fn put_package(
    &mut self,
    package_coord: &'a PackageCoordinate<'a>,
    new_file_coord_to_contents: HashMap<&'a FileCoordinate<'a>, Contents>,
  ) {
    panic!("Unimplemented: FileCoordinateMap::put_package");
  }
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
  pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<'a, T>
  where
    F: Fn(&'a FileCoordinate<'a>, &Contents) -> T,
    T: Clone,
  {
    panic!("Unimplemented: FileCoordinateMap::map");
  }
/*
  def map[T](func: (FileCoordinate, Contents) => T): FileCoordinateMap[T] = {
    val resultFileCoordToContents = mutable.HashMap[FileCoordinate, T]()
    fileCoordToContents.foreach({ case (fileCoord, contents) =>
      resultFileCoordToContents.put(fileCoord, func(fileCoord, contents))
    })
    new FileCoordinateMap(packageCoordToFileCoords, resultFileCoordToContents)
  }
  def flatMap[T](func: (FileCoordinate, Contents) => T): Iterable[T] = {
    fileCoordToContents.map({ case (fileCoord, contents) =>
      func(fileCoord, contents)
    })
  }
*/
}
/*
}
*/
```

This is good, because now every Rust function/type is directly above the corresponding Scala function/type.

# Example of What Not To Do

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

Then this would be WRONG:

```rs

/*
object PackageCoordinate {// extends Ordering[PackageCoordinate] {
  def TEST_TLD(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(interner.intern(StrI("test")), Vector.empty))

  def BUILTIN(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))

  def internal(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
*/
  // WRONG PLACEMENT:
  pub fn test_tld<'ctx>(
    interner: &'ctx crate::Interner<'a>,
    keywords: &'ctx crate::Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    panic!("Unimplemented: PackageCoordinate::test_tld");
  }

  // WRONG PLACEMENT:
  pub fn builtin<'ctx>(
    interner: &'ctx crate::Interner<'a>,
    keywords: &'ctx crate::Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    panic("Unimplemented: PackageCoordinate::builtin");
  }

  // WRONG PLACEMENT:
  pub fn internal<'ctx>(
    interner: &'ctx crate::Interner<'a>,
    keywords: &'ctx crate::Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    panic!("Unimplemented: PackageCoordinate::internal");
  }
/*
}
*/
```

The above is wrong. The Rust functions should be directly above their Scala counterparts, like this correct version:

```rs
/*
object PackageCoordinate {// extends Ordering[PackageCoordinate] {
*/
  pub fn test_tld<'ctx>(
    interner: &'ctx crate::Interner<'a>,
    keywords: &'ctx crate::Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    panic!("Unimplemented: PackageCoordinate::test_tld");
  }
/*
  def TEST_TLD(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(interner.intern(StrI("test")), Vector.empty))
*/
  pub fn builtin<'ctx>(
    interner: &'ctx crate::Interner<'a>,
    keywords: &'ctx crate::Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    interner.intern_package_coordinate(keywords.empty_string, &[])
  }
/*
  def BUILTIN(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
*/
  pub fn internal<'ctx>(
    interner: &'ctx crate::Interner<'a>,
    keywords: &'ctx crate::Keywords<'a>,
  ) -> &'a PackageCoordinate<'a>
  where
    'a: 'ctx,
  {
    panic!("Unimplemented: PackageCoordinate::internal");
  }
/*
  def internal(interner: Interner, keywords: Keywords): PackageCoordinate = interner.intern(PackageCoordinate(keywords.emptyString, Vector.empty))
*/
/*
}
*/
```

Rule of thumb: No two Scala functions/types should be next to each other; No Scala function/type should be next to another Scala function/type. The file needs to alternate Rust->Scala->Rust->Scala->Rust->Scala->etc. (except for Rust `impl`, which I consider to be part of the Rust `struct` so they can be next to each other)

# Step 3 List Format

Use the format above for the step 3 target list.

Do not combine them onto one line; this is WRONG:

 * struct FileCoordinate → case class FileCoordinate
 * fn is_internal → def isInternal
 * fn is_test → def isTest

This is correct:

 * struct FileCoordinate
 * case class FileCoordinate
 * fn is_internal
 * def isInternal
 * fn is_test
 * def isTest

# Guidelines

 * Slice apart scala comments with `*/` and `/*` so you can put the Rust code where it belongs.
 * No two Scala functions/types should be next to each other; No Scala function/type should be next to another Scala function/type. The file needs to alternate Rust->Scala->Rust->Scala->Rust->Scala->etc. (except for Rust `impl`, which I consider to be part of the Rust `struct` so they can be next to each other)

# Restrictions

 * Your job is *not* to make it build, so please do not build it. Do not run `cargo build`, do not run `cargo run`, do not run `cargo test`.
 * Every Rust function/type should be directly above the corresponding Scala function/type.
 * You cannot reorder the old scala comments relative to each other. They must be in the same order as they were before.

# Questions

Q: "What if we find a matching Rust function/type already in the file, and it's in the wrong place?"

A: Please move that Rust thing to the right place.

Q: "What if we find a matching Rust function/type already in the file, and it's in the right place?"

A: Just leave it there.

Q: "What if it's in an impl block?"

A: Ignore the impl block. Move it.

# When done

Say "done" when you're done modifying the code.
