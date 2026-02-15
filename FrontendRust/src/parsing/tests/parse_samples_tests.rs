use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::tests::parser_test_compilation;
use std::fs;
use std::path::PathBuf;
use crate::utils::code_hierarchy::{FileCoordinateMap, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use crate::utils::code_hierarchy::FileCoordinate;

/*
package dev.vale.parsing

import dev.vale.{Collector, Err, Interner, Keywords, Ok, Tests, vfail}
import dev.vale.options.GlobalOptions
import org.scalatest._



class ParseSamplesTests extends FunSuite with Collector with TestParseUtils {
  def parse(path: String): Unit = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val code = Tests.loadExpected(path)
    val compilation = ParserTestCompilation.test(interner, keywords, code)
    compilation.getParseds() match {
      case Ok(x) => x
      case Err(e) => vfail(ParseErrorHumanizer.humanize(path, code, e.error))
    }
  }
*/
fn load_expected(path: &str) -> String {
  let full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("../Frontend/Tests/test/main/resources")
    .join(path);
  fs::read_to_string(&full_path)
    .unwrap_or_else(|e| panic!("Failed to load sample '{}': {} ({:?})", path, e, full_path))
}

fn parse<'a, 'i, 'k, 'b>(
  path: &str,
  interner: &'i Interner<'a>,
  keywords: &'k Keywords<'a>,
  resolver: &'b dyn IPackageResolver<'a, HashMap<String, String>>,
  test_package_coord: &'a PackageCoordinate<'a>,
)
where
  'i: 'a,
  'k: 'a,
  'b: 'a,
{
  let mut compilation = parser_test_compilation::test(interner, keywords, resolver, test_package_coord);
  compilation
    .get_parseds()
    .unwrap_or_else(|e| panic!("Failed to parse sample '{}': {:?}", path, e));
}

struct ParserTestResolver<'arena> {
  code_map: FileCoordinateMap<'arena, String>,
}
impl<'arena> IPackageResolver<'arena, HashMap<String, String>> for ParserTestResolver<'arena> {
  fn resolve(&self, package_coord: &'arena PackageCoordinate<'arena>) -> Option<HashMap<String, String>> {
    // For testing the parser, we dont want it to fetch things with import statements.
    Some(
      self
        .code_map
        .resolve(package_coord)
        .unwrap_or_else(|| HashMap::from([("".to_string(), "".to_string())])),
    )
  }
}

macro_rules! parse_sample_test {
  ($name:ident, $path:literal) => {
    #[test]
    fn $name() {
      let interner = Interner::new();
      let keywords = Keywords::new(&interner);

      let test_module = interner.intern("test");
      let test_package_coord = interner.intern_package_coordinate(PackageCoordinate {
        module: test_module,
        packages: vec![],
      });

      let code: &[String] = &[load_expected($path)];

      let mut code_map = FileCoordinateMap::new();
      for (index, contents) in code.iter().enumerate() {
        let filepath = if code.len() == 1 {
          "test.vale".to_string()
        } else {
          format!("{}.vale", index)
        };
        let file_coord = interner.intern_file_coordinate(FileCoordinate {
          package_coord: &test_package_coord,
          filepath,
        });
        code_map.put(&file_coord, contents.clone());
      }
    
      let resolver = ParserTestResolver { code_map };

      parse($path, &interner, &keywords, &resolver, &test_package_coord);
    }
  };
}

parse_sample_test!(parse_sample_000, "optutils/optutils.vale");
parse_sample_test!(parse_sample_001, "printutils/printutils.vale");
parse_sample_test!(parse_sample_002, "ioutils/ioutils.vale");
parse_sample_test!(parse_sample_003, "array/indices/indices.vale");
parse_sample_test!(parse_sample_004, "array/iter/iter.vale");
parse_sample_test!(parse_sample_005, "array/each/each.vale");
parse_sample_test!(parse_sample_006, "array/drop_into/drop_into.vale");
parse_sample_test!(parse_sample_007, "array/make/make.vale");
parse_sample_test!(parse_sample_008, "array/has/has.vale");
parse_sample_test!(parse_sample_009, "listprintutils/listprintutils.vale");
parse_sample_test!(parse_sample_010, "logic/logic.vale");
parse_sample_test!(parse_sample_011, "math/math.vale");
parse_sample_test!(parse_sample_012, "hashmap/hashmap.vale");
parse_sample_test!(parse_sample_013, "intrange/intrange.vale");
parse_sample_test!(parse_sample_014, "programs/lambdas/doubleclosure.vale");
parse_sample_test!(parse_sample_015, "programs/lambdas/mutate.vale");
parse_sample_test!(parse_sample_016, "programs/lambdas/lambda.vale");
parse_sample_test!(parse_sample_017, "programs/lambdas/lambdamut.vale");
parse_sample_test!(parse_sample_018, "programs/comparei64.vale");
parse_sample_test!(parse_sample_019, "programs/unreachablemoot.vale");
parse_sample_test!(parse_sample_020, "programs/constraintRef.vale");
parse_sample_test!(parse_sample_021, "programs/add64ret.vale");
parse_sample_test!(parse_sample_022, "programs/concatstrfloat.vale");
parse_sample_test!(parse_sample_023, "programs/strings/smallstr.vale");
parse_sample_test!(parse_sample_024, "programs/strings/complex/main.vale");
parse_sample_test!(parse_sample_025, "programs/strings/stradd.vale");
parse_sample_test!(parse_sample_026, "programs/strings/strprint.vale");
parse_sample_test!(parse_sample_027, "programs/strings/strneq.vale");
parse_sample_test!(parse_sample_028, "programs/strings/i64tostr.vale");
parse_sample_test!(parse_sample_029, "programs/strings/inttostr.vale");
parse_sample_test!(parse_sample_030, "programs/strings/strlen.vale");
parse_sample_test!(parse_sample_031, "programs/externs/structimmparamextern/test.vale");
parse_sample_test!(parse_sample_032, "programs/externs/structimmparamexport/test.vale");
parse_sample_test!(parse_sample_033, "programs/externs/strreturnexport/test.vale");
parse_sample_test!(parse_sample_034, "programs/externs/voidreturnextern/test.vale");
parse_sample_test!(parse_sample_035, "programs/externs/voidreturnexport/test.vale");
parse_sample_test!(parse_sample_036, "programs/externs/structmutreturnexport/test.vale");
parse_sample_test!(parse_sample_037, "programs/externs/structmutparamexport/test.vale");
parse_sample_test!(parse_sample_038, "programs/externs/structimmparamdeepextern/test.vale");
parse_sample_test!(parse_sample_039, "programs/externs/ssaimmparamdeepextern/test.vale");
parse_sample_test!(parse_sample_040, "programs/externs/rsaimmparamdeepexport/test.vale");
parse_sample_test!(parse_sample_041, "programs/externs/tupleparamextern/test.vale");
parse_sample_test!(parse_sample_042, "programs/externs/ssamutreturnexport/test.vale");
parse_sample_test!(parse_sample_043, "programs/externs/structimmparamdeepexport/test.vale");
parse_sample_test!(parse_sample_044, "programs/externs/ssaimmparamdeepexport/test.vale");
parse_sample_test!(parse_sample_045, "programs/externs/rsaimmparamdeepextern/test.vale");
parse_sample_test!(parse_sample_046, "programs/externs/rsaimmparamextern/test.vale");
parse_sample_test!(parse_sample_047, "programs/externs/rsaimmreturnexport/test.vale");
parse_sample_test!(parse_sample_048, "programs/externs/export.vale");
parse_sample_test!(parse_sample_049, "programs/externs/rsaimmparamexport/test.vale");
parse_sample_test!(parse_sample_050, "programs/externs/rsaimmreturnextern/test.vale");
parse_sample_test!(parse_sample_051, "programs/externs/interfaceimmreturnexport/test.vale");
parse_sample_test!(parse_sample_052, "programs/externs/interfacemutparamexport/test.vale");
parse_sample_test!(parse_sample_053, "programs/externs/interfaceimmreturnextern/test.vale");
parse_sample_test!(parse_sample_054, "programs/externs/strlenextern/test.vale");
parse_sample_test!(parse_sample_055, "programs/externs/ssamutparamexport/test.vale");
parse_sample_test!(parse_sample_056, "programs/externs/rsamutreturnexport/test.vale");
parse_sample_test!(parse_sample_057, "programs/externs/ssaimmreturnextern/test.vale");
parse_sample_test!(parse_sample_058, "programs/externs/interfaceimmparamdeepextern/test.vale");
parse_sample_test!(parse_sample_059, "programs/externs/ssaimmreturnexport/test.vale");
parse_sample_test!(parse_sample_060, "programs/externs/rsamutparamexport/test.vale");
parse_sample_test!(parse_sample_061, "programs/externs/interfaceimmparamdeepexport/test.vale");
parse_sample_test!(parse_sample_062, "programs/externs/extern.vale");
parse_sample_test!(parse_sample_063, "programs/externs/interfaceimmparamextern/test.vale");
parse_sample_test!(parse_sample_064, "programs/externs/tupleretextern/test.vale");
parse_sample_test!(parse_sample_065, "programs/externs/interfaceimmparamexport/test.vale");
parse_sample_test!(parse_sample_066, "programs/externs/structmutparamdeepexport/test.vale");
parse_sample_test!(parse_sample_067, "programs/externs/ssaimmparamextern/test.vale");
parse_sample_test!(parse_sample_068, "programs/externs/interfacemutreturnexport/test.vale");
parse_sample_test!(parse_sample_069, "programs/externs/ssaimmparamexport/test.vale");
parse_sample_test!(parse_sample_070, "programs/weaks/callWeakSelfMethodAfterDrop.vale");
parse_sample_test!(parse_sample_071, "programs/weaks/callWeakSelfMethodWhileLive.vale");
parse_sample_test!(parse_sample_072, "programs/weaks/dropThenLockInterface.vale");
parse_sample_test!(parse_sample_073, "programs/weaks/lockWhileLiveInterface.vale");
parse_sample_test!(parse_sample_074, "programs/weaks/dropWhileLockedStruct.vale");
parse_sample_test!(parse_sample_075, "programs/weaks/weakFromCRefInterface.vale");
parse_sample_test!(parse_sample_076, "programs/weaks/weakFromCRefStruct.vale");
parse_sample_test!(parse_sample_077, "programs/weaks/lockWhileLiveStruct.vale");
parse_sample_test!(parse_sample_078, "programs/weaks/loadFromWeakable.vale");
parse_sample_test!(parse_sample_079, "programs/weaks/dropWhileLockedInterface.vale");
parse_sample_test!(parse_sample_080, "programs/weaks/weakFromLocalCRefStruct.vale");
parse_sample_test!(parse_sample_081, "programs/weaks/weakFromLocalCRefInterface.vale");
parse_sample_test!(parse_sample_082, "programs/weaks/dropThenLockStruct.vale");
parse_sample_test!(parse_sample_083, "programs/readwriteufcs.vale");
parse_sample_test!(parse_sample_084, "programs/tuples/immtupleaccess.vale");
parse_sample_test!(parse_sample_085, "programs/downcast/downcastPointerFailed.vale");
parse_sample_test!(parse_sample_086, "programs/downcast/downcastPointerSuccess.vale");
parse_sample_test!(parse_sample_087, "programs/downcast/downcastBorrowFailed.vale");
parse_sample_test!(parse_sample_088, "programs/downcast/downcastOwningSuccessful.vale");
parse_sample_test!(parse_sample_089, "programs/downcast/downcastBorrowSuccessful.vale");
parse_sample_test!(parse_sample_090, "programs/downcast/downcastOwningFailed.vale");
parse_sample_test!(parse_sample_091, "programs/if/ifnevers.vale");
parse_sample_test!(parse_sample_092, "programs/if/nestedif.vale");
parse_sample_test!(parse_sample_093, "programs/if/if.vale");
parse_sample_test!(parse_sample_094, "programs/if/upcastif.vale");
parse_sample_test!(parse_sample_095, "programs/if/neverif.vale");
parse_sample_test!(parse_sample_096, "programs/virtuals/callingThroughBorrow.vale");
parse_sample_test!(parse_sample_097, "programs/virtuals/calling.vale");
parse_sample_test!(parse_sample_098, "programs/virtuals/round.vale");
parse_sample_test!(parse_sample_099, "programs/virtuals/upcasting.vale");
parse_sample_test!(parse_sample_100, "programs/virtuals/interfaceimm.vale");
parse_sample_test!(parse_sample_101, "programs/virtuals/retUpcast.vale");
parse_sample_test!(parse_sample_102, "programs/virtuals/ordinarylinkedlist.vale");
parse_sample_test!(parse_sample_103, "programs/virtuals/interfacemut.vale");
parse_sample_test!(parse_sample_104, "programs/genericvirtuals/getOr.vale");
parse_sample_test!(parse_sample_105, "programs/genericvirtuals/templatedinterface.vale");
parse_sample_test!(parse_sample_106, "programs/genericvirtuals/templatedlinkedlist.vale");
parse_sample_test!(parse_sample_107, "programs/genericvirtuals/stampMultipleAncestors.vale");
parse_sample_test!(parse_sample_108, "programs/genericvirtuals/foreachlinkedlist.vale");
parse_sample_test!(parse_sample_109, "programs/genericvirtuals/mapFunc.vale");
parse_sample_test!(parse_sample_110, "programs/genericvirtuals/callingAbstract.vale");
parse_sample_test!(parse_sample_111, "programs/genericvirtuals/templatedoption.vale");
parse_sample_test!(parse_sample_112, "programs/addret.vale");
parse_sample_test!(parse_sample_113, "programs/nestedblocks.vale");
parse_sample_test!(parse_sample_114, "programs/panicnot.vale");
parse_sample_test!(parse_sample_115, "programs/floateq.vale");
parse_sample_test!(parse_sample_116, "programs/roguelike.vale");
parse_sample_test!(parse_sample_117, "programs/truncate.vale");
parse_sample_test!(parse_sample_118, "programs/multiUnstackify.vale");
parse_sample_test!(parse_sample_119, "programs/borrowRef.vale");
parse_sample_test!(parse_sample_120, "programs/structs/constructor.vale");
parse_sample_test!(parse_sample_121, "programs/structs/structmutstore.vale");
parse_sample_test!(parse_sample_122, "programs/structs/getMember.vale");
parse_sample_test!(parse_sample_123, "programs/structs/structs.vale");
parse_sample_test!(parse_sample_124, "programs/structs/mutate.vale");
parse_sample_test!(parse_sample_125, "programs/structs/deadmutstruct.vale");
parse_sample_test!(parse_sample_126, "programs/structs/structmutstoreinner.vale");
parse_sample_test!(parse_sample_127, "programs/structs/bigstructimm.vale");
parse_sample_test!(parse_sample_128, "programs/structs/structmut.vale");
parse_sample_test!(parse_sample_129, "programs/structs/structimm.vale");
parse_sample_test!(parse_sample_130, "programs/structs/memberrefcount.vale");
parse_sample_test!(parse_sample_131, "programs/arrays/ssaimmfromvalues.vale");
parse_sample_test!(parse_sample_132, "programs/arrays/rsamutdestroyintocallable.vale");
parse_sample_test!(parse_sample_133, "programs/arrays/ssamutfromcallable.vale");
parse_sample_test!(parse_sample_134, "programs/arrays/ssaimmfromcallable.vale");
parse_sample_test!(parse_sample_135, "programs/arrays/rsamutlen.vale");
parse_sample_test!(parse_sample_136, "programs/arrays/ssamutfromvalues.vale");
parse_sample_test!(parse_sample_137, "programs/arrays/rsaimmlen.vale");
parse_sample_test!(parse_sample_138, "programs/arrays/rsamut.vale");
parse_sample_test!(parse_sample_139, "programs/arrays/swaprsamutdestroy.vale");
parse_sample_test!(parse_sample_140, "programs/arrays/rsamutfromcallable.vale");
parse_sample_test!(parse_sample_141, "programs/arrays/rsaimm.vale");
parse_sample_test!(parse_sample_142, "programs/arrays/rsaimmfromcallable.vale");
parse_sample_test!(parse_sample_143, "programs/arrays/rsamutcapacity.vale");
parse_sample_test!(parse_sample_144, "programs/arrays/ssamutdestroyintocallable.vale");
parse_sample_test!(parse_sample_145, "programs/arrays/inlssaimm.vale");
parse_sample_test!(parse_sample_146, "programs/ufcs.vale");
parse_sample_test!(parse_sample_147, "programs/functions/overloads.vale");
parse_sample_test!(parse_sample_148, "programs/functions/recursion.vale");
parse_sample_test!(parse_sample_149, "programs/printfloat.vale");
parse_sample_test!(parse_sample_150, "programs/mutswaplocals.vale");
parse_sample_test!(parse_sample_151, "programs/mutlocal.vale");
parse_sample_test!(parse_sample_152, "programs/unstackifyret.vale");
parse_sample_test!(parse_sample_153, "programs/while/while.vale");
parse_sample_test!(parse_sample_154, "programs/while/foreach.vale");
parse_sample_test!(parse_sample_155, "programs/invalidaccess.vale");
parse_sample_test!(parse_sample_156, "programs/floatarithmetic.vale");
parse_sample_test!(parse_sample_157, "programs/panic.vale");
parse_sample_test!(parse_sample_158, "list/list.vale");
parse_sample_test!(parse_sample_159, "ifunction/ifunction1/ifunction1.vale");
parse_sample_test!(parse_sample_160, "string/string.vale");
parse_sample_test!(parse_sample_161, "panicutils/panicutils.vale");
parse_sample_test!(parse_sample_162, "castutils/castutils.vale");

/*

  test("optutils/optutils.vale") { parse("optutils/optutils.vale") }
  test("printutils/printutils.vale") { parse("printutils/printutils.vale") }
  test("ioutils/ioutils.vale") { parse("ioutils/ioutils.vale") }
  test("array/indices/indices.vale") { parse("array/indices/indices.vale") }
  test("array/iter/iter.vale") { parse("array/iter/iter.vale") }
  test("array/each/each.vale") { parse("array/each/each.vale") }
  test("array/drop_into/drop_into.vale") { parse("array/drop_into/drop_into.vale") }
  test("array/make/make.vale") { parse("array/make/make.vale") }
  test("array/has/has.vale") { parse("array/has/has.vale") }
  test("listprintutils/listprintutils.vale") { parse("listprintutils/listprintutils.vale") }
  test("logic/logic.vale") { parse("logic/logic.vale") }
  test("math/math.vale") { parse("math/math.vale") }
  test("hashmap/hashmap.vale") { parse("hashmap/hashmap.vale") }
  test("intrange/intrange.vale") { parse("intrange/intrange.vale") }
  test("programs/lambdas/doubleclosure.vale") { parse("programs/lambdas/doubleclosure.vale") }
  test("programs/lambdas/mutate.vale") { parse("programs/lambdas/mutate.vale") }
  test("programs/lambdas/lambda.vale") { parse("programs/lambdas/lambda.vale") }
  test("programs/lambdas/lambdamut.vale") { parse("programs/lambdas/lambdamut.vale") }
  test("programs/comparei64.vale") { parse("programs/comparei64.vale") }
  test("programs/unreachablemoot.vale") { parse("programs/unreachablemoot.vale") }
  test("programs/constraintRef.vale") { parse("programs/constraintRef.vale") }
  test("programs/add64ret.vale") { parse("programs/add64ret.vale") }
  test("programs/concatstrfloat.vale") { parse("programs/concatstrfloat.vale") }
  test("programs/strings/smallstr.vale") { parse("programs/strings/smallstr.vale") }
  test("programs/strings/complex/main.vale") { parse("programs/strings/complex/main.vale") }
  test("programs/strings/stradd.vale") { parse("programs/strings/stradd.vale") }
  test("programs/strings/strprint.vale") { parse("programs/strings/strprint.vale") }
  test("programs/strings/strneq.vale") { parse("programs/strings/strneq.vale") }
  test("programs/strings/i64tostr.vale") { parse("programs/strings/i64tostr.vale") }
  test("programs/strings/inttostr.vale") { parse("programs/strings/inttostr.vale") }
  test("programs/strings/strlen.vale") { parse("programs/strings/strlen.vale") }
  test("programs/externs/structimmparamextern/test.vale") { parse("programs/externs/structimmparamextern/test.vale") }
  test("programs/externs/structimmparamexport/test.vale") { parse("programs/externs/structimmparamexport/test.vale") }
  test("programs/externs/strreturnexport/test.vale") { parse("programs/externs/strreturnexport/test.vale") }
  test("programs/externs/voidreturnextern/test.vale") { parse("programs/externs/voidreturnextern/test.vale") }
  test("programs/externs/voidreturnexport/test.vale") { parse("programs/externs/voidreturnexport/test.vale") }
  test("programs/externs/structmutreturnexport/test.vale") { parse("programs/externs/structmutreturnexport/test.vale") }
  test("programs/externs/structmutparamexport/test.vale") { parse("programs/externs/structmutparamexport/test.vale") }
  test("programs/externs/structimmparamdeepextern/test.vale") { parse("programs/externs/structimmparamdeepextern/test.vale") }
  test("programs/externs/ssaimmparamdeepextern/test.vale") { parse("programs/externs/ssaimmparamdeepextern/test.vale") }
  test("programs/externs/rsaimmparamdeepexport/test.vale") { parse("programs/externs/rsaimmparamdeepexport/test.vale") }
  test("programs/externs/tupleparamextern/test.vale") { parse("programs/externs/tupleparamextern/test.vale") }
  test("programs/externs/ssamutreturnexport/test.vale") { parse("programs/externs/ssamutreturnexport/test.vale") }
  test("programs/externs/structimmparamdeepexport/test.vale") { parse("programs/externs/structimmparamdeepexport/test.vale") }
  test("programs/externs/ssaimmparamdeepexport/test.vale") { parse("programs/externs/ssaimmparamdeepexport/test.vale") }
  test("programs/externs/rsaimmparamdeepextern/test.vale") { parse("programs/externs/rsaimmparamdeepextern/test.vale") }
  test("programs/externs/rsaimmparamextern/test.vale") { parse("programs/externs/rsaimmparamextern/test.vale") }
  test("programs/externs/rsaimmreturnexport/test.vale") { parse("programs/externs/rsaimmreturnexport/test.vale") }
  test("programs/externs/export.vale") { parse("programs/externs/export.vale") }
  test("programs/externs/rsaimmparamexport/test.vale") { parse("programs/externs/rsaimmparamexport/test.vale") }
  test("programs/externs/rsaimmreturnextern/test.vale") { parse("programs/externs/rsaimmreturnextern/test.vale") }
  test("programs/externs/interfaceimmreturnexport/test.vale") { parse("programs/externs/interfaceimmreturnexport/test.vale") }
  test("programs/externs/interfacemutparamexport/test.vale") { parse("programs/externs/interfacemutparamexport/test.vale") }
  test("programs/externs/interfaceimmreturnextern/test.vale") { parse("programs/externs/interfaceimmreturnextern/test.vale") }
  test("programs/externs/strlenextern/test.vale") { parse("programs/externs/strlenextern/test.vale") }
  test("programs/externs/ssamutparamexport/test.vale") { parse("programs/externs/ssamutparamexport/test.vale") }
  test("programs/externs/rsamutreturnexport/test.vale") { parse("programs/externs/rsamutreturnexport/test.vale") }
  test("programs/externs/ssaimmreturnextern/test.vale") { parse("programs/externs/ssaimmreturnextern/test.vale") }
  test("programs/externs/interfaceimmparamdeepextern/test.vale") { parse("programs/externs/interfaceimmparamdeepextern/test.vale") }
  test("programs/externs/ssaimmreturnexport/test.vale") { parse("programs/externs/ssaimmreturnexport/test.vale") }
  test("programs/externs/rsamutparamexport/test.vale") { parse("programs/externs/rsamutparamexport/test.vale") }
  test("programs/externs/interfaceimmparamdeepexport/test.vale") { parse("programs/externs/interfaceimmparamdeepexport/test.vale") }
  test("programs/externs/extern.vale") { parse("programs/externs/extern.vale") }
  test("programs/externs/interfaceimmparamextern/test.vale") { parse("programs/externs/interfaceimmparamextern/test.vale") }
  test("programs/externs/tupleretextern/test.vale") { parse("programs/externs/tupleretextern/test.vale") }
  test("programs/externs/interfaceimmparamexport/test.vale") { parse("programs/externs/interfaceimmparamexport/test.vale") }
  test("programs/externs/structmutparamdeepexport/test.vale") { parse("programs/externs/structmutparamdeepexport/test.vale") }
  test("programs/externs/ssaimmparamextern/test.vale") { parse("programs/externs/ssaimmparamextern/test.vale") }
  test("programs/externs/interfacemutreturnexport/test.vale") { parse("programs/externs/interfacemutreturnexport/test.vale") }
  test("programs/externs/ssaimmparamexport/test.vale") { parse("programs/externs/ssaimmparamexport/test.vale") }
  test("programs/weaks/callWeakSelfMethodAfterDrop.vale") { parse("programs/weaks/callWeakSelfMethodAfterDrop.vale") }
  test("programs/weaks/callWeakSelfMethodWhileLive.vale") { parse("programs/weaks/callWeakSelfMethodWhileLive.vale") }
  test("programs/weaks/dropThenLockInterface.vale") { parse("programs/weaks/dropThenLockInterface.vale") }
  test("programs/weaks/lockWhileLiveInterface.vale") { parse("programs/weaks/lockWhileLiveInterface.vale") }
  test("programs/weaks/dropWhileLockedStruct.vale") { parse("programs/weaks/dropWhileLockedStruct.vale") }
  test("programs/weaks/weakFromCRefInterface.vale") { parse("programs/weaks/weakFromCRefInterface.vale") }
  test("programs/weaks/weakFromCRefStruct.vale") { parse("programs/weaks/weakFromCRefStruct.vale") }
  test("programs/weaks/lockWhileLiveStruct.vale") { parse("programs/weaks/lockWhileLiveStruct.vale") }
  test("programs/weaks/loadFromWeakable.vale") { parse("programs/weaks/loadFromWeakable.vale") }
  test("programs/weaks/dropWhileLockedInterface.vale") { parse("programs/weaks/dropWhileLockedInterface.vale") }
  test("programs/weaks/weakFromLocalCRefStruct.vale") { parse("programs/weaks/weakFromLocalCRefStruct.vale") }
  test("programs/weaks/weakFromLocalCRefInterface.vale") { parse("programs/weaks/weakFromLocalCRefInterface.vale") }
  test("programs/weaks/dropThenLockStruct.vale") { parse("programs/weaks/dropThenLockStruct.vale") }
  test("programs/readwriteufcs.vale") { parse("programs/readwriteufcs.vale") }
  test("programs/tuples/immtupleaccess.vale") { parse("programs/tuples/immtupleaccess.vale") }
  test("programs/downcast/downcastPointerFailed.vale") { parse("programs/downcast/downcastPointerFailed.vale") }
  test("programs/downcast/downcastPointerSuccess.vale") { parse("programs/downcast/downcastPointerSuccess.vale") }
  test("programs/downcast/downcastBorrowFailed.vale") { parse("programs/downcast/downcastBorrowFailed.vale") }
  test("programs/downcast/downcastOwningSuccessful.vale") { parse("programs/downcast/downcastOwningSuccessful.vale") }
  test("programs/downcast/downcastBorrowSuccessful.vale") { parse("programs/downcast/downcastBorrowSuccessful.vale") }
  test("programs/downcast/downcastOwningFailed.vale") { parse("programs/downcast/downcastOwningFailed.vale") }
  test("programs/if/ifnevers.vale") { parse("programs/if/ifnevers.vale") }
  test("programs/if/nestedif.vale") { parse("programs/if/nestedif.vale") }
  test("programs/if/if.vale") { parse("programs/if/if.vale") }
  test("programs/if/upcastif.vale") { parse("programs/if/upcastif.vale") }
  test("programs/if/neverif.vale") { parse("programs/if/neverif.vale") }
  test("programs/virtuals/callingThroughBorrow.vale") { parse("programs/virtuals/callingThroughBorrow.vale") }
  test("programs/virtuals/calling.vale") { parse("programs/virtuals/calling.vale") }
  test("programs/virtuals/round.vale") { parse("programs/virtuals/round.vale") }
  test("programs/virtuals/upcasting.vale") { parse("programs/virtuals/upcasting.vale") }
  test("programs/virtuals/interfaceimm.vale") { parse("programs/virtuals/interfaceimm.vale") }
  test("programs/virtuals/retUpcast.vale") { parse("programs/virtuals/retUpcast.vale") }
  test("programs/virtuals/ordinarylinkedlist.vale") { parse("programs/virtuals/ordinarylinkedlist.vale") }
  test("programs/virtuals/interfacemut.vale") { parse("programs/virtuals/interfacemut.vale") }
  test("programs/genericvirtuals/getOr.vale") { parse("programs/genericvirtuals/getOr.vale") }
  test("programs/genericvirtuals/templatedinterface.vale") { parse("programs/genericvirtuals/templatedinterface.vale") }
  test("programs/genericvirtuals/templatedlinkedlist.vale") { parse("programs/genericvirtuals/templatedlinkedlist.vale") }
  test("programs/genericvirtuals/stampMultipleAncestors.vale") { parse("programs/genericvirtuals/stampMultipleAncestors.vale") }
  test("programs/genericvirtuals/foreachlinkedlist.vale") { parse("programs/genericvirtuals/foreachlinkedlist.vale") }
  test("programs/genericvirtuals/mapFunc.vale") { parse("programs/genericvirtuals/mapFunc.vale") }
  test("programs/genericvirtuals/callingAbstract.vale") { parse("programs/genericvirtuals/callingAbstract.vale") }
  test("programs/genericvirtuals/templatedoption.vale") { parse("programs/genericvirtuals/templatedoption.vale") }
  test("programs/addret.vale") { parse("programs/addret.vale") }
  test("programs/nestedblocks.vale") { parse("programs/nestedblocks.vale") }
  test("programs/panicnot.vale") { parse("programs/panicnot.vale") }
  test("programs/floateq.vale") { parse("programs/floateq.vale") }
  test("programs/roguelike.vale") { parse("programs/roguelike.vale") }
  test("programs/truncate.vale") { parse("programs/truncate.vale") }
  test("programs/multiUnstackify.vale") { parse("programs/multiUnstackify.vale") }
  test("programs/borrowRef.vale") { parse("programs/borrowRef.vale") }
  test("programs/structs/constructor.vale") { parse("programs/structs/constructor.vale") }
  test("programs/structs/structmutstore.vale") { parse("programs/structs/structmutstore.vale") }
  test("programs/structs/getMember.vale") { parse("programs/structs/getMember.vale") }
  test("programs/structs/structs.vale") { parse("programs/structs/structs.vale") }
  test("programs/structs/mutate.vale") { parse("programs/structs/mutate.vale") }
  test("programs/structs/deadmutstruct.vale") { parse("programs/structs/deadmutstruct.vale") }
  test("programs/structs/structmutstoreinner.vale") { parse("programs/structs/structmutstoreinner.vale") }
  test("programs/structs/bigstructimm.vale") { parse("programs/structs/bigstructimm.vale") }
  test("programs/structs/structmut.vale") { parse("programs/structs/structmut.vale") }
  test("programs/structs/structimm.vale") { parse("programs/structs/structimm.vale") }
  test("programs/structs/memberrefcount.vale") { parse("programs/structs/memberrefcount.vale") }
  test("programs/arrays/ssaimmfromvalues.vale") { parse("programs/arrays/ssaimmfromvalues.vale") }
  test("programs/arrays/rsamutdestroyintocallable.vale") { parse("programs/arrays/rsamutdestroyintocallable.vale") }
  test("programs/arrays/ssamutfromcallable.vale") { parse("programs/arrays/ssamutfromcallable.vale") }
  test("programs/arrays/ssaimmfromcallable.vale") { parse("programs/arrays/ssaimmfromcallable.vale") }
  test("programs/arrays/rsamutlen.vale") { parse("programs/arrays/rsamutlen.vale") }
  test("programs/arrays/ssamutfromvalues.vale") { parse("programs/arrays/ssamutfromvalues.vale") }
  test("programs/arrays/rsaimmlen.vale") { parse("programs/arrays/rsaimmlen.vale") }
  test("programs/arrays/rsamut.vale") { parse("programs/arrays/rsamut.vale") }
  test("programs/arrays/swaprsamutdestroy.vale") { parse("programs/arrays/swaprsamutdestroy.vale") }
  test("programs/arrays/rsamutfromcallable.vale") { parse("programs/arrays/rsamutfromcallable.vale") }
  test("programs/arrays/rsaimm.vale") { parse("programs/arrays/rsaimm.vale") }
  test("programs/arrays/rsaimmfromcallable.vale") { parse("programs/arrays/rsaimmfromcallable.vale") }
  test("programs/arrays/rsamutcapacity.vale") { parse("programs/arrays/rsamutcapacity.vale") }
  test("programs/arrays/ssamutdestroyintocallable.vale") { parse("programs/arrays/ssamutdestroyintocallable.vale") }
  test("programs/arrays/inlssaimm.vale") { parse("programs/arrays/inlssaimm.vale") }
  test("programs/ufcs.vale") { parse("programs/ufcs.vale") }
  test("programs/functions/overloads.vale") { parse("programs/functions/overloads.vale") }
  test("programs/functions/recursion.vale") { parse("programs/functions/recursion.vale") }
  test("programs/printfloat.vale") { parse("programs/printfloat.vale") }
  test("programs/mutswaplocals.vale") { parse("programs/mutswaplocals.vale") }
  test("programs/mutlocal.vale") { parse("programs/mutlocal.vale") }
  test("programs/unstackifyret.vale") { parse("programs/unstackifyret.vale") }
  test("programs/while/while.vale") { parse("programs/while/while.vale") }
  test("programs/while/foreach.vale") { parse("programs/while/foreach.vale") }
  test("programs/invalidaccess.vale") { parse("programs/invalidaccess.vale") }
  test("programs/floatarithmetic.vale") { parse("programs/floatarithmetic.vale") }
  test("programs/panic.vale") { parse("programs/panic.vale") }
  test("list/list.vale") { parse("list/list.vale") }
  test("ifunction/ifunction1/ifunction1.vale") { parse("ifunction/ifunction1/ifunction1.vale") }
  test("string/string.vale") { parse("string/string.vale") }
  test("panicutils/panicutils.vale") { parse("panicutils/panicutils.vale") }
  test("castutils/castutils.vale") { parse("castutils/castutils.vale") }
}
*/
