use crate::utils::code_hierarchy::FileCoordinate;
use crate::Interner;
use std::sync::Arc;

/*
package dev.vale

object CodeLocationS {
  // Keep in sync with CodeLocation2
  def testZero(interner: Interner): CodeLocationS = {
    CodeLocationS.internal(interner, -1)
  }
  def internal(interner: Interner, internalNum: Int): CodeLocationS = {
    vassert(internalNum < 0)
    CodeLocationS(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(
          interner.intern(StrI("")),
          Vector.empty)),
        "internal")),
      internalNum)
  }
}

sealed trait ICrumb
case class RangeCrumb(range: RangeS) extends ICrumb
case class InternalCrumb(num: Int) extends ICrumb

object RangeS {
  // Should only be used in tests.
  def testZero(interner: Interner): RangeS = {
    RangeS(CodeLocationS.testZero(interner), CodeLocationS.testZero(interner))
  }

  def internal(interner: Interner, internalNum: Int): RangeS = {
    vassert(internalNum < 0)
    RangeS(CodeLocationS.internal(interner, internalNum), CodeLocationS.internal(interner, internalNum))
  }
}
*/

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeLocationS<'a> {
  pub file: Arc<FileCoordinate<'a>>,
  pub offset: i32,
}
/*
case class CodeLocationS(
  // The index in the original source code files list.
  // If negative, it means it came from some internal non-file code.
  file: FileCoordinate,
  offset: Int) {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;

  // Just for debug purposes
  override def toString: String = {
    if (file.isTest()) {
      "tvl" + ":" + offset
    } else {
      file.toString + ":" + offset
    }
  }
}
Guardian: disable: NECX
*/

impl<'a> CodeLocationS<'a> {
  // Keep in sync with CodeLocation2
  pub fn test_zero(interner: &Interner<'a>) -> CodeLocationS<'a> {
    Self::internal(interner, -1)
  }

  // SPORK
  pub fn internal(interner: &Interner<'a>, internal_num: i32) -> CodeLocationS<'a> {
    assert!(internal_num < 0, "CodeLocationS::internal - internal_num must be negative");
    let package_coord =
      interner.intern_package_coordinate(interner.intern(""), &[]);
    let file = interner.intern_file_coordinate(package_coord, "internal");
    CodeLocationS {
      file: Arc::new(file.clone()),
      offset: internal_num,
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeS<'a> {
  pub begin: CodeLocationS<'a>,
  pub end: CodeLocationS<'a>,
}

// Scala's toString was just for debug purposes (covered by #[derive(Debug)])
impl<'a> RangeS<'a> {
  pub fn new(begin: CodeLocationS<'a>, end: CodeLocationS<'a>) -> RangeS<'a> {
    assert!(begin.file == end.file, "RangeS: begin.file != end.file");
    assert!(begin.offset <= end.offset, "RangeS: begin.offset > end.offset");
    RangeS { begin, end }
  }

  // Should only be used in tests.
  pub fn test_zero(interner: &Interner<'a>) -> RangeS<'a> {
    let tz = CodeLocationS::test_zero(interner);
    RangeS::new(tz.clone(), tz)
  }

  // SPORK
  pub fn file(&self) -> &Arc<FileCoordinate<'_>> {
    &self.begin.file
  }
}

/*
case class RangeS(begin: CodeLocationS, end: CodeLocationS) {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
  vassert(begin.file == end.file)
  vassert(begin.offset <= end.offset)
  def file: FileCoordinate = begin.file

  // Just for debug purposes
  override def toString: String = {
    if (file.isTest()) {
      "tvr" + ":" + begin.offset + "-" + end.offset
    } else {
      "RangeS(" + begin.toString + ", " + end.toString + ")"
    }
  }
}

Guardian: disable: NECX
*/
