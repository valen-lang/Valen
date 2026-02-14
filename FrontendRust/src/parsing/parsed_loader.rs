// From Frontend/ParsingPass/src/dev/vale/parsing/ParsedLoader.scala
// Loads .vpst files from JSON
//
// MIGRATION GUIDELINES FOR THIS FILE
// 1) Keep Rust structure/order close to the Scala reference below.
// 2) Use serde_json types directly (`Value`, `Map<String, Value>`), not lift-json names.
// 3) Prefer borrowed JSON inputs (`&Value`, `&Map<String, Value>`) to avoid cloning.
// 4) Match "__type" tags exactly and map each case to the corresponding Rust AST variant.
// 5) Keep helper error messages explicit so bad VPST shapes fail at the exact field/type.
// 6) Keep new Rust code above the equivalent Scala comment block.
// 7) If a Scala case is not ported yet, leave an explicit `panic!`/`vimpl` marker.

use crate::interner::Interner;
use crate::lexing::{ParseError, RangeL};
use crate::parsing::ast::*;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use serde_json::{Map, Value, from_str};
use std::sync::Arc;

/*
package dev.vale.parsing

import dev.vale.lexing.{BadVPSTError, BadVPSTException, IParseError, RangeL}
import dev.vale.{Err, FileCoordinate, Interner, Ok, PackageCoordinate, Profiler, Result, StrI, vimpl, vwat}
import dev.vale.parsing.ast._
import net.liftweb.json._
import dev.vale.parsing.ast._

class ParsedLoader(interner: Interner) {
*/
fn expect_object<'a>(obj: &'a Value) -> &'a Map<String, Value> {
  obj
    .as_object()
    .unwrap_or_else(|| panic!("BadVPSTError: Expected JSON object, got: {:?}", obj))
}
/*
  def expectObject(obj: Object): JObject = {
    if (!obj.isInstanceOf[JObject]) {
      throw BadVPSTException(BadVPSTError("Expected JSON object, got: " + obj.getClass.getSimpleName))
    }
    obj.asInstanceOf[JObject]
  }
*/
fn expect_string<'a>(obj: &'a Value) -> &'a str {
  obj
    .as_str()
    .unwrap_or_else(|| panic!("BadVPSTError: Expected JSON string, got: {:?}", obj))
}
/*
  def expectString(obj: Object): JString = {
    if (!obj.isInstanceOf[JString]) {
      throw BadVPSTException(BadVPSTError("Expected JSON string, got: " + obj.getClass.getSimpleName))
    }
    obj.asInstanceOf[JString]
  }
*/
fn expect_number(obj: &Value) -> i64 {
  obj
    .as_i64()
    .unwrap_or_else(|| panic!("BadVPSTError: Expected JSON number, got: {:?}", obj))
}
/*
  def expectNumber(obj: Object): BigInt = {
    if (!obj.isInstanceOf[JInt]) {
      throw BadVPSTException(BadVPSTError("Expected JSON number, got: " + obj.getClass.getSimpleName))
    }
    obj.asInstanceOf[JInt].num
  }
*/
fn expect_object_typed<'a>(obj: &'a Value, expected_type: &str) -> &'a Map<String, Value> {
  let jobj = expect_object(obj);
  let actual_type = get_string_field(jobj, "__type");
  if actual_type != expected_type {
    panic!(
      "BadVPSTError: Expected {} but got a {}",
      expected_type, actual_type
    );
  }
  jobj
}
/*
  def expectObjectTyped(obj: JValue, expectedType: String): JObject = {
    val jobj = expectObject(obj)
    val actualType = getStringField(jobj, "__type")
    if (!actualType.equals(expectedType)) {
      throw BadVPSTException(BadVPSTError("Expected " + expectedType + " but got a " + actualType))
    }
    jobj
  }
*/
fn get_field<'a>(jobj: &'a Map<String, Value>, field_name: &str) -> &'a Value {
  jobj
    .get(field_name)
    .unwrap_or_else(|| panic!("BadVPSTError: Object had no field named {}", field_name))
}
/*
  def getField(jobj: JValue, fieldName: String): JValue = {
    (jobj \ fieldName) match {
      case JNothing => throw BadVPSTException(BadVPSTError("Object had no field named " + fieldName))
      case other => other
    }
  }
*/
fn get_object_field<'a>(
  container_jobj: &'a Map<String, Value>,
  field_name: &str,
) -> &'a Map<String, Value> {
  expect_object(get_field(container_jobj, field_name))
}
/*
  def getObjectField(containerJobj: JObject, fieldName: String): JObject = {
    expectObject(getField(containerJobj, fieldName))
  }
*/
// fn get_object_field_with_expected_type<'a>(
//   container_jobj: &'a Map<String, Value>,
//   field_name: &str,
//   expected_type: &str,
// ) -> &'a Map<String, Value> {
//   let jobj = expect_object(get_field(container_jobj, field_name));
//   expect_type(jobj, expected_type);
//   jobj
// }
/*
  def getObjectField(containerJobj: JObject, fieldName: String, expectedType: String): JObject = {
    val jobj = expectObject(getField(containerJobj, fieldName))
    expectType(jobj, expectedType)
    jobj
  }
*/
fn get_string_field<'a>(jobj: &'a Map<String, Value>, field_name: &str) -> &'a str {
  expect_string(get_field(jobj, field_name))
}
/*
  def getStringField(jobj: JObject, fieldName: String): String = {
    getField(jobj, fieldName) match {
      case JString(s) => {
        s
      }
      case _ => throw BadVPSTException(BadVPSTError("Field " + fieldName + " wasn't a string!"))
    }
  }
*/
fn get_int_field(jobj: &Map<String, Value>, field_name: &str) -> i32 {
  get_field(jobj, field_name)
    .as_i64()
    .unwrap_or_else(|| panic!("BadVPSTError: Field {} wasn't a number!", field_name))
    as i32
}
/*
  def getIntField(jobj: JObject, fieldName: String): Int = {
    getField(jobj, fieldName) match {
      case JInt(s) => s.toInt
      case _ => throw BadVPSTException(BadVPSTError("Field " + fieldName + " wasn't a number!"))
    }
  }
*/
fn get_long_field(jobj: &Map<String, Value>, field_name: &str) -> i64 {
  let field = get_field(jobj, field_name);
  if let Some(n) = field.as_i64() {
    return n;
  }
  if let Some(s) = field.as_str() {
    let parsed = s
      .parse::<i64>()
      .unwrap_or_else(|_| panic!("BadVPSTError: Field {} wasn't a number!", field_name));
    if parsed.to_string() == s {
      return parsed;
    }
  }
  panic!("BadVPSTError: Field {} wasn't a number!", field_name);
}
/*
  def getLongField(jobj: JObject, fieldName: String): Long = {
    getField(jobj, fieldName) match {
      case JInt(s) => s.toLong
      case JString(s) if s.toLong.toString == s => s.toLong
      case _ => throw BadVPSTException(BadVPSTError("Field " + fieldName + " wasn't a number!"))
    }
  }
*/
fn get_float_field(jobj: &Map<String, Value>, field_name: &str) -> f64 {
  get_field(jobj, field_name)
    .as_f64()
    .unwrap_or_else(|| panic!("BadVPSTError: Field {} wasn't a double!", field_name))
}
/*
  def getFloatField(jobj: JObject, fieldName: String): Double = {
    getField(jobj, fieldName) match {
      case JDouble(s) => s
      case _ => throw BadVPSTException(BadVPSTError("Field " + fieldName + " wasn't a double!"))
    }
  }
*/
fn get_boolean_field(jobj: &Map<String, Value>, field_name: &str) -> bool {
  get_field(jobj, field_name)
    .as_bool()
    .unwrap_or_else(|| panic!("BadVPSTError: Field {} wasn't a boolean!", field_name))
}
/*
  def getBooleanField(jobj: JObject, fieldName: String): Boolean = {
    getField(jobj, fieldName) match {
      case JBool(b) => b
      case _ => throw BadVPSTException(BadVPSTError("Field " + fieldName + " wasn't a boolean!"))
    }
  }
*/
fn get_array_field<'a>(jobj: &'a Map<String, Value>, field_name: &str) -> &'a [Value] {
  get_field(jobj, field_name)
    .as_array()
    .map(|v| v.as_slice())
    .unwrap_or_else(|| panic!("BadVPSTError: Field {} wasn't an array!", field_name))
}
/*
  def getArrayField(jobj: JObject, fieldName: String): Vector[JValue] = {
    getField(jobj, fieldName) match {
      case JArray(arr) => arr.toVector
      case _ => throw BadVPSTException(BadVPSTError("Field " + fieldName + " wasn't an array!"))
    }
  }
*/
fn expect_type(jobj: &Map<String, Value>, expected_type: &str) -> () {
  let actual_type = get_type(jobj);
  if actual_type != expected_type {
    panic!(
      "BadVPSTError: Expected {} but got a {}",
      expected_type, actual_type
    );
  }
}
/*
  def expectType(jobj: JObject, expectedType: String): Unit = {
    val actualType = getType(jobj)
    if (!actualType.equals(expectedType)) {
      throw BadVPSTException(BadVPSTError("Expected " + expectedType + " but got a " + actualType))
    }
  }
*/
fn get_type<'a>(jobj: &'a Map<String, Value>) -> &'a str {
  get_string_field(jobj, "__type")
}
/*
  def getType(jobj: JObject): String = {
    getStringField(jobj, "__type")
  }
*/
fn load_range(jobj: &Map<String, Value>) -> RangeL {
  expect_type(jobj, "Range");
  RangeL {
    begin: get_int_field(jobj, "begin"),
    end: get_int_field(jobj, "end"),
  }
}
/*
  def loadRange(jobj: JObject): RangeL = {
    expectType(jobj, "Range")
    RangeL(
      getIntField(jobj, "begin"),
      getIntField(jobj, "end"))
  }
*/
fn load_name<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> NameP<'a> {
  expect_type(jobj, "Name");
  NameP {
    range: load_range(get_object_field(jobj, "range")),
    str: interner.intern(get_string_field(jobj, "name")),
  }
}
/*
  def loadName(jobj: JObject): NameP = {
    expectType(jobj, "Name")
    NameP(
      loadRange(getObjectField(jobj, "range")),
      interner.intern(StrI(getStringField(jobj, "name"))))
  }

*/
pub fn load<'a>(interner: &Interner<'a>, source: &str) -> Result<FileP<'a>, ParseError> {
  let parsed: Value = from_str(source).map_err(|err| ParseError::BadVPSTError {
    message: format!("Failed to parse VPST JSON: {}", err),
  })?;
  let jfile = expect_object_typed(&parsed, "File");
  Ok(FileP {
    file_coord: Arc::new(load_file_coord(interner, get_object_field(jfile, "fileCoord"))),
    comments_ranges: get_array_field(jfile, "commentsRanges")
      .iter()
      .map(expect_object)
      .map(load_range)
      .collect(),
    denizens: get_array_field(jfile, "denizens")
      .iter()
      .map(expect_object)
      .map(|denizen| match get_type(denizen) {
        "Struct" => IDenizenP::TopLevelStruct(load_struct(interner, denizen)),
        "Interface" => IDenizenP::TopLevelInterface(load_interface(interner, denizen)),
        "Function" => IDenizenP::TopLevelFunction(load_function(interner, denizen)),
        "Impl" => IDenizenP::TopLevelImpl(load_impl(interner, denizen)),
        "Import" => IDenizenP::TopLevelImport(load_import(interner, denizen)),
        "ExportAs" => IDenizenP::TopLevelExportAs(load_export_as(interner, denizen)),
        other => panic!("Not implemented: unknown denizen type {}", other),
      })
      .collect(),
  })
}
/*
  def load(source: String): Result[FileP, IParseError] = {
    Profiler.frame(() => {
      try {
        val jfile = expectObjectTyped(parse(source), "File")
        Ok(
          FileP(
            loadFileCoord(getObjectField(jfile, "fileCoord")),
            getArrayField(jfile, "commentsRanges").map(expectObject).map(x => loadRange(x)).toVector,
            getArrayField(jfile, "denizens").map(expectObject).map(denizen => {
              getType(denizen) match {
                case "Struct" => TopLevelStructP(loadStruct(denizen))
                case "Interface" => TopLevelInterfaceP(loadInterface(denizen))
                case "Function" => TopLevelFunctionP(loadFunction(denizen))
                case "Impl" => TopLevelImplP(loadImpl(denizen))
                case "Import" => TopLevelImportP(loadImport(denizen))
                case "ExportAs" => TopLevelExportAsP(loadExportAs(denizen))
                case x => vimpl(x.toString)
              }
            }).toVector))
      } catch {
        case BadVPSTException(err) => Err(err)
      }
    })
  }

*/
fn load_function<'a>(interner: &Interner<'a>, denizen: &Map<String, Value>) -> FunctionP<'a> {
  FunctionP {
    range: load_range(get_object_field(denizen, "range")),
    header: load_function_header(interner, get_object_field(denizen, "header")),
    body: load_optional_object(get_object_field(denizen, "body"), |x| load_block(interner, x))
      .map(Box::new),
  }
}
/*
  def loadFunction(denizen: JObject) = {
    FunctionP(
      loadRange(getObjectField(denizen, "range")),
      loadFunctionHeader(getObjectField(denizen, "header")),
      loadOptionalObject(getObjectField(denizen, "body"), loadBlock))
  }

*/
fn load_impl<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> ImplP<'a> {
  ImplP {
    range: load_range(get_object_field(jobj, "range")),
    generic_params: load_optional_object(get_object_field(jobj, "identifyingRunes"), |x| {
      load_identifying_runes(interner, x)
    }),
    template_rules: load_optional_object(get_object_field(jobj, "templateRules"), |x| {
      load_template_rules(interner, x)
    }),
    struct_: load_optional_object(get_object_field(jobj, "struct"), |x| load_templex(interner, x)),
    interface: load_templex(interner, get_object_field(jobj, "interface")),
    attributes: get_array_field(jobj, "attributes")
      .iter()
      .map(expect_object)
      .map(|x| load_attribute(interner, x))
      .collect(),
  }
}
/*
  private def loadImpl(jobj: JObject) = {
    ImplP(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "identifyingRunes"), loadIdentifyingRunes),
      loadOptionalObject(getObjectField(jobj, "templateRules"), loadTemplateRules),
      loadOptionalObject(getObjectField(jobj, "struct"), loadTemplex),
      loadTemplex(getObjectField(jobj, "interface")),
      getArrayField(jobj, "attributes").map(expectObject).map(loadAttribute))
  }
*/
fn load_export_as<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> ExportAsP<'a> {
  ExportAsP {
    range: load_range(get_object_field(jobj, "range")),
    struct_: load_templex(interner, get_object_field(jobj, "struct")),
    exported_name: load_name(interner, get_object_field(jobj, "exportedName")),
  }
}
/*

  private def loadExportAs(jobj: JObject) = {
    ExportAsP(
      loadRange(getObjectField(jobj, "range")),
      loadTemplex(getObjectField(jobj, "struct")),
      loadName(getObjectField(jobj, "exportedName")))
  }
*/
fn load_import<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> ImportP<'a> {
  ImportP {
    range: load_range(get_object_field(jobj, "range")),
    module_name: load_name(interner, get_object_field(jobj, "moduleName")),
    package_steps: get_array_field(jobj, "packageSteps")
      .iter()
      .map(expect_object)
      .map(|x| load_name(interner, x))
      .collect(),
    importee_name: load_name(interner, get_object_field(jobj, "importeeName")),
  }
}
/*
  private def loadImport(jobj: JObject) = {
    ImportP(
      loadRange(getObjectField(jobj, "range")),
      loadName(getObjectField(jobj, "moduleName")),
      getArrayField(jobj, "packageSteps").map(expectObject).map(loadName),
      loadName(getObjectField(jobj, "importeeName")))
  }
*/
fn load_struct<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> StructP<'a> {
  StructP {
    range: load_range(get_object_field(jobj, "range")),
    name: load_name(interner, get_object_field(jobj, "name")),
    attributes: get_array_field(jobj, "attributes")
      .iter()
      .map(expect_object)
      .map(|x| load_attribute(interner, x))
      .collect(),
    mutability: load_optional_object(get_object_field(jobj, "mutability"), |x| load_templex(interner, x)),
    identifying_runes: load_optional_object(
      get_object_field(jobj, "identifyingRunes"),
      |x| load_identifying_runes(interner, x),
    ),
    template_rules: load_optional_object(get_object_field(jobj, "templateRules"), |x| {
      load_template_rules(interner, x)
    }),
    maybe_default_region_rune: load_optional_object(
      get_object_field(jobj, "maybeDefaultRegion"),
      |x| load_region_rune(interner, x),
    ),
    body_range: load_range(get_object_field(jobj, "bodyRange")),
    members: load_struct_members(interner, get_object_field(jobj, "members")),
  }
}
/*
  private def loadStruct(jobj: JObject) = {
    StructP(
      loadRange(getObjectField(jobj, "range")),
      loadName(getObjectField(jobj, "name")),
      getArrayField(jobj, "attributes").map(expectObject).map(loadAttribute),
      loadOptionalObject(getObjectField(jobj, "mutability"), loadTemplex),
      loadOptionalObject(getObjectField(jobj, "identifyingRunes"), loadIdentifyingRunes),
      loadOptionalObject(getObjectField(jobj, "templateRules"), loadTemplateRules),
      loadOptionalObject(getObjectField(jobj, "maybeDefaultRegion"), loadRegionRune),
      loadRange(getObjectField(jobj, "bodyRange")),
      loadStructMembers(getObjectField(jobj, "members")))
  }
*/
fn load_interface<'a>(interner: &Interner<'a>, denizen: &Map<String, Value>) -> InterfaceP<'a> {
  InterfaceP {
    range: load_range(get_object_field(denizen, "range")),
    name: load_name(interner, get_object_field(denizen, "name")),
    attributes: get_array_field(denizen, "attributes")
      .iter()
      .map(expect_object)
      .map(|x| load_attribute(interner, x))
      .collect(),
    mutability: load_optional_object(get_object_field(denizen, "mutability"), |x| load_templex(interner, x)),
    maybe_identifying_runes: load_optional_object(
      get_object_field(denizen, "maybeIdentifyingRunes"),
      |x| load_identifying_runes(interner, x),
    ),
    template_rules: load_optional_object(get_object_field(denizen, "templateRules"), |x| {
      load_template_rules(interner, x)
    }),
    maybe_default_region_rune: load_optional_object(
      get_object_field(denizen, "maybeDefaultRegion"),
      |x| load_region_rune(interner, x),
    ),
    body_range: load_range(get_object_field(denizen, "bodyRange")),
    members: get_array_field(denizen, "members")
      .iter()
      .map(expect_object)
      .map(|x| load_function(interner, x))
      .collect(),
  }
}
/*
  private def loadInterface(denizen: JObject) = {
    InterfaceP(
      loadRange(getObjectField(denizen, "range")),
      loadName(getObjectField(denizen, "name")),
      getArrayField(denizen, "attributes").map(expectObject).map(loadAttribute),
//      getArrayField(denizen, "attributes").map(expectObject).map(loadCitizenAttribute),
      loadOptionalObject(getObjectField(denizen, "mutability"), loadTemplex),
      loadOptionalObject(getObjectField(denizen, "maybeIdentifyingRunes"), loadIdentifyingRunes),
      loadOptionalObject(getObjectField(denizen, "templateRules"), loadTemplateRules),
      loadOptionalObject(getObjectField(denizen, "maybeDefaultRegion"), loadRegionRune),
      loadRange(getObjectField(denizen, "bodyRange")),
      getArrayField(denizen, "members").map(expectObject).map(loadFunction))
  }
*/
fn load_function_header<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> FunctionHeaderP<'a> {
  FunctionHeaderP {
    range: load_range(get_object_field(jobj, "range")),
    name: load_optional_object(get_object_field(jobj, "name"), |x| load_name(interner, x)),
    attributes: get_array_field(jobj, "attributes")
      .iter()
      .map(expect_object)
      .map(|x| load_attribute(interner, x))
      .collect(),
    generic_parameters: load_optional_object(
      get_object_field(jobj, "maybeUserSpecifiedIdentifyingRunes"),
      |x| load_identifying_runes(interner, x),
    ),
    template_rules: load_optional_object(get_object_field(jobj, "templateRules"), |x| {
      load_template_rules(interner, x)
    }),
    params: load_optional_object(get_object_field(jobj, "params"), |x| load_params(interner, x)),
    ret: load_function_return(interner, get_object_field(jobj, "return")),
  }
}
/*
  def loadFunctionHeader(jobj: JObject): FunctionHeaderP = {
    FunctionHeaderP(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "name"), loadName),
      getArrayField(jobj, "attributes").map(expectObject).map(loadAttribute),
      loadOptionalObject(getObjectField(jobj, "maybeUserSpecifiedIdentifyingRunes"), loadIdentifyingRunes),
      loadOptionalObject(getObjectField(jobj, "templateRules"), loadTemplateRules),
      loadOptionalObject(getObjectField(jobj, "params"), loadParams),
      loadFunctionReturn(getObjectField(jobj, "return")))
//      loadOptionalObject(getObjectField(jobj, "maybeDefaultRegion"), loadName))
  }
*/
fn load_file_coord<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> FileCoordinate<'a> {
  let package_coord = load_package_coord(interner, get_object_field(jobj, "packageCoord"));
  (*interner.intern_file_coordinate(FileCoordinate {
    package_coord: Arc::new((*package_coord).clone()),
    filepath: get_string_field(jobj, "filepath").to_string(),
  }))
  .clone()
}
/*
  def loadFileCoord(jobj: JObject): FileCoordinate = {
    FileCoordinate(
      loadPackageCoord(getObjectField(jobj, "packageCoord")),
      getStringField(jobj, "filepath"))
  }
*/
fn load_package_coord<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> &'a PackageCoordinate<'a> {
  interner.intern_package_coordinate(PackageCoordinate {
    module: interner.intern(get_string_field(jobj, "module")),
    packages: get_array_field(jobj, "packages")
      .iter()
      .map(expect_string)
      .map(|s| interner.intern(s))
      .collect(),
  })
}
/*
  def loadPackageCoord(jobj: JObject): PackageCoordinate = {
    interner.intern(PackageCoordinate(
      interner.intern(StrI(getStringField(jobj, "module"))),
      getArrayField(jobj, "packages").map(expectString).map(s => interner.intern(StrI(s.s)))))
  }
*/
fn load_params<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> ParamsP<'a> {
  ParamsP {
    range: load_range(get_object_field(jobj, "range")),
    params: get_array_field(jobj, "params")
      .iter()
      .map(expect_object)
      .map(|x| load_parameter(interner, x))
      .collect(),
  }
}
/*
  def loadParams(jobj: JObject): ParamsP = {
    ast.ParamsP(
      loadRange(getObjectField(jobj, "range")),
      getArrayField(jobj, "params").map(expectObject).map(loadParameter))
  }
*/
fn load_parameter<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> ParameterP<'a> {
  ParameterP {
    range: load_range(get_object_field(jobj, "range")),
    virtuality: load_optional_object(get_object_field(jobj, "virtuality"), |x| {
      load_virtuality(interner, x)
    }),
    maybe_pre_checked: load_optional_object(get_object_field(jobj, "maybePreChecked"), load_range),
    self_borrow: load_optional_object(get_object_field(jobj, "selfBorrow"), load_range),
    pattern: load_optional_object(get_object_field(jobj, "pattern"), |x| load_pattern(interner, x)),
  }
}
/*
  def loadParameter(jobj: JObject): ParameterP = {
    ast.ParameterP(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "virtuality"), loadVirtuality),
      loadOptionalObject(getObjectField(jobj, "maybePreChecked"), loadRange),
      loadOptionalObject(getObjectField(jobj, "selfBorrow"), loadRange),
      loadOptionalObject(getObjectField(jobj, "pattern"), loadPattern))
  }
*/
fn load_pattern<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> PatternPP<'a> {
  PatternPP {
    range: load_range(get_object_field(jobj, "range")),
    destination: load_optional_object(get_object_field(jobj, "capture"), |x| {
      load_destination_local(interner, x)
    }),
    templex: load_optional_object(get_object_field(jobj, "templex"), |x| load_templex(interner, x)),
    destructure: load_optional_object(get_object_field(jobj, "destructure"), |x| {
      load_destructure(interner, x)
    }),
  }
}
/*

  def loadPattern(jobj: JObject): PatternPP = {
    ast.PatternPP(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "capture"), loadDestinationLocal),
      loadOptionalObject(getObjectField(jobj, "templex"), loadTemplex),
      loadOptionalObject(getObjectField(jobj, "destructure"), loadDestructure))
//      loadOptionalObject(getObjectField(jobj, "virtuality"), loadVirtuality))
  }
*/
fn load_destructure<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> DestructureP<'a> {
  DestructureP {
    range: load_range(get_object_field(jobj, "range")),
    patterns: get_array_field(jobj, "patterns")
      .iter()
      .map(expect_object)
      .map(|x| load_pattern(interner, x))
      .collect(),
  }
}
/*
  def loadDestructure(jobj: JObject): DestructureP = {
    DestructureP(
      loadRange(getObjectField(jobj, "range")),
      getArrayField(jobj, "patterns").map(expectObject).map(loadPattern))
  }
*/
fn load_destination_local<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> DestinationLocalP<'a> {
  DestinationLocalP {
    decl: load_name_declaration(interner, get_object_field(jobj, "name")),
    mutate: load_optional_object(get_object_field(jobj, "mutate"), load_range),
  }
}
/*
  def loadDestinationLocal(jobj: JObject): DestinationLocalP = {
    DestinationLocalP(
      loadNameDeclaration(getObjectField(jobj, "name")),
      loadOptionalObject(getObjectField(jobj, "mutate"), loadRange))
  }
*/

fn load_name_declaration<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> INameDeclarationP<'a> {
  match get_type(jobj) {
    "IgnoredLocalNameDeclaration" => {
      INameDeclarationP::IgnoredLocalNameDeclaration(load_range(get_object_field(jobj, "range")))
    }
    "LocalNameDeclaration" => {
      INameDeclarationP::LocalNameDeclaration(load_name(interner, get_object_field(jobj, "name")))
    }
    "IterableNameDeclaration" => {
      INameDeclarationP::IterableNameDeclaration(load_range(get_object_field(jobj, "range")))
    }
    "IteratorNameDeclaration" => {
      INameDeclarationP::IteratorNameDeclaration(load_range(get_object_field(jobj, "range")))
    }
    "IterationOptionNameDeclaration" => INameDeclarationP::IterationOptionNameDeclaration(
      load_range(get_object_field(jobj, "range")),
    ),
    "ConstructingMemberNameDeclaration" => {
      INameDeclarationP::ConstructingMemberNameDeclaration(load_name(
        interner,
        get_object_field(jobj, "name"),
      ))
    }
    other => panic!("Not implemented: load_name_declaration {}", other),
  }
}
/*
  def loadNameDeclaration(jobj: JObject): INameDeclarationP = {
    getType(jobj) match {
      case "IgnoredLocalNameDeclaration" => IgnoredLocalNameDeclarationP(loadRange(getObjectField(jobj, "range")))
      case "LocalNameDeclaration" => LocalNameDeclarationP(loadName(getObjectField(jobj, "name")))
      case "IterableNameDeclaration" => IterableNameDeclarationP(loadRange(getObjectField(jobj, "range")))
      case "IteratorNameDeclaration" => IteratorNameDeclarationP(loadRange(getObjectField(jobj, "range")))
      case "IterationOptionNameDeclaration" => IterationOptionNameDeclarationP(loadRange(getObjectField(jobj, "range")))
      case "ConstructingMemberNameDeclaration" => ConstructingMemberNameDeclarationP(loadName(getObjectField(jobj, "name")))
    }
  }
*/
fn load_imprecise_name<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> IImpreciseNameP<'a> {
  match get_type(jobj) {
    "LookupName" => IImpreciseNameP::LookupName(load_name(interner, get_object_field(jobj, "name"))),
    "IterableName" => IImpreciseNameP::IterableName(load_range(get_object_field(jobj, "range"))),
    "IteratorName" => IImpreciseNameP::IteratorName(load_range(get_object_field(jobj, "range"))),
    "IterationOptionName" => {
      IImpreciseNameP::IterationOptionName(load_range(get_object_field(jobj, "range")))
    }
    other => panic!("Not implemented: load_imprecise_name {}", other),
  }
}
/*
  def loadImpreciseName(jobj: JObject): IImpreciseNameP = {
    getType(jobj) match {
      case "LookupName" => LookupNameP(loadName(getObjectField(jobj, "name")))
      case "IterableName" => IterableNameP(loadRange(getObjectField(jobj, "range")))
      case "IteratorName" => IteratorNameP(loadRange(getObjectField(jobj, "range")))
      case "IterationOptionName" => IterationOptionNameP(loadRange(getObjectField(jobj, "range")))
    }
  }
*/
// fn load_capture_name(jobj: &Map<String, Value>) -> INameDeclarationP {
//   panic!("Not implemented");
// }
/*
  def loadCaptureName(jobj: JObject): INameDeclarationP = {
    getType(jobj) match {
      case "LocalName" => {
        LocalNameDeclarationP(
          loadName(getObjectField(jobj, "name")))
      }
      case "ConstructingMemberName" => {
        ConstructingMemberNameDeclarationP(
          loadName(getObjectField(jobj, "name")))
      }
    }
  }
*/
fn load_block<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> BlockPE<'a> {
  BlockPE {
    range: load_range(get_object_field(jobj, "range")),
    maybe_pure: load_optional_object(get_object_field(jobj, "maybePure"), load_range),
    maybe_default_region: load_optional_object(
      get_object_field(jobj, "maybeDefaultRegion"),
      |x| load_region_rune(interner, x),
    ),
    inner: Box::new(load_expression(interner, get_object_field(jobj, "inner"))),
  }
}
/*
  def loadBlock(jobj: JObject): BlockPE = {
    BlockPE(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "maybePure"), loadRange),
      loadOptionalObject(getObjectField(jobj, "maybeDefaultRegion"), loadRegionRune),
      loadExpression(getObjectField(jobj, "inner")))
  }
*/
fn load_consecutor<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> ConsecutorPE<'a> {
  ConsecutorPE {
    inners: get_array_field(jobj, "inners")
      .iter()
      .map(expect_object)
      .map(|x| load_expression(interner, x))
      .collect(),
  }
}
/*
  def loadConsecutor(jobj: JObject): ConsecutorPE = {
    ConsecutorPE(
      getArrayField(jobj, "inners").map(expectObject).map(loadExpression))
  }
*/
fn load_function_return<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> FunctionReturnP<'a> {
  FunctionReturnP {
    range: load_range(get_object_field(jobj, "range")),
    ret_type: load_optional_object(get_object_field(jobj, "retType"), |x| load_templex(interner, x)),
  }
}
/*
  def loadFunctionReturn(jobj: JObject): FunctionReturnP = {
    FunctionReturnP(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "retType"), loadTemplex))
  }
*/
fn load_unit(_jobj: &Map<String, Value>) -> UnitP {
  panic!("Not implemented");
}
/*
  def loadUnit(jobj: JObject): UnitP = {
    UnitP(
      loadRange(getObjectField(jobj, "range")))
  }
*/
fn load_struct_members<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> StructMembersP<'a> {
  StructMembersP {
    range: load_range(get_object_field(jobj, "range")),
    contents: get_array_field(jobj, "members")
      .iter()
      .map(expect_object)
      .map(|x| load_struct_content(interner, x))
      .collect(),
  }
}
/*
  def loadStructMembers(jobj: JObject): StructMembersP = {
    StructMembersP(
      loadRange(getObjectField(jobj, "range")),
      getArrayField(jobj, "members").map(expectObject).map(loadStructContent))
  }
*/
fn load_expression<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> IExpressionPE<'a> {
  match get_type(jobj) {
    "Return" => IExpressionPE::Return(ReturnPE {
      range: load_range(get_object_field(jobj, "range")),
      expr: Box::new(load_expression(interner, get_object_field(jobj, "expr"))),
    }),
    "Void" => IExpressionPE::Void(VoidPE {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "ConstantInt" => IExpressionPE::ConstantInt(ConstantIntPE {
      range: load_range(get_object_field(jobj, "range")),
      value: get_long_field(jobj, "value"),
      bits: {
        let bits = get_object_field(jobj, "bits");
        match get_type(bits) {
          "None" => None,
          "Some" => Some(expect_number(get_field(bits, "value"))),
          other => panic!("BadVPSTError: Expected None/Some for bits but got {}", other),
        }
      },
    }),
    "ConstantStr" => IExpressionPE::ConstantStr(ConstantStrPE {
      range: load_range(get_object_field(jobj, "range")),
      value: get_string_field(jobj, "value").to_string(),
    }),
    "ConstantFloat" => IExpressionPE::ConstantFloat(ConstantFloatPE {
      range: load_range(get_object_field(jobj, "range")),
      value: get_float_field(jobj, "value"),
    }),
    "ConstantBool" => IExpressionPE::ConstantBool(ConstantBoolPE {
      range: load_range(get_object_field(jobj, "range")),
      value: get_boolean_field(jobj, "value"),
    }),
    "StrInterpolate" => IExpressionPE::StrInterpolate(StrInterpolatePE {
      range: load_range(get_object_field(jobj, "range")),
      parts: get_array_field(jobj, "parts")
        .iter()
        .map(expect_object)
        .map(|x| load_expression(interner, x))
        .collect(),
    }),
    "Dot" => IExpressionPE::Dot(DotPE {
      range: load_range(get_object_field(jobj, "range")),
      left: Box::new(load_expression(interner, get_object_field(jobj, "left"))),
      operator_range: load_range(get_object_field(jobj, "operatorRange")),
      member: load_name(interner, get_object_field(jobj, "member")),
    }),
    "FunctionCall" => IExpressionPE::FunctionCall(FunctionCallPE {
      range: load_range(get_object_field(jobj, "range")),
      operator_range: load_range(get_object_field(jobj, "operatorRange")),
      callable_expr: Box::new(load_expression(interner, get_object_field(jobj, "callableExpr"))),
      arg_exprs: get_array_field(jobj, "argExprs")
        .iter()
        .map(expect_object)
        .map(|x| load_expression(interner, x))
        .collect(),
    }),
    "BinaryCall" => IExpressionPE::BinaryCall(BinaryCallPE {
      range: load_range(get_object_field(jobj, "range")),
      function_name: load_name(interner, get_object_field(jobj, "functionName")),
      left_expr: Box::new(load_expression(interner, get_object_field(jobj, "leftExpr"))),
      right_expr: Box::new(load_expression(interner, get_object_field(jobj, "rightExpr"))),
    }),
    "Lambda" => IExpressionPE::Lambda(LambdaPE {
      captures: load_optional_object(get_object_field(jobj, "captures"), load_unit),
      function: load_function(interner, get_object_field(jobj, "function")),
    }),
    "MagicParamLookup" => IExpressionPE::MagicParamLookup(MagicParamLookupPE {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "If" => IExpressionPE::If(IfPE {
      range: load_range(get_object_field(jobj, "range")),
      condition: Box::new(load_expression(interner, get_object_field(jobj, "condition"))),
      then_body: Box::new(load_block(interner, get_object_field(jobj, "thenBody"))),
      else_body: Box::new(load_block(interner, get_object_field(jobj, "elseBody"))),
    }),
    "SubExpression" => IExpressionPE::SubExpression(SubExpressionPE {
      range: load_range(get_object_field(jobj, "range")),
      inner: Box::new(load_expression(interner, get_object_field(jobj, "innerExpr"))),
    }),
    "Let" => IExpressionPE::Let(LetPE {
      range: load_range(get_object_field(jobj, "range")),
      pattern: load_pattern(interner, get_object_field(jobj, "pattern")),
      source: Box::new(load_expression(interner, get_object_field(jobj, "source"))),
    }),
    "While" => IExpressionPE::While(WhilePE {
      range: load_range(get_object_field(jobj, "range")),
      condition: Box::new(load_expression(interner, get_object_field(jobj, "condition"))),
      body: Box::new(load_block(interner, get_object_field(jobj, "body"))),
    }),
    "Mutate" => IExpressionPE::Mutate(MutatePE {
      range: load_range(get_object_field(jobj, "range")),
      mutatee: Box::new(load_expression(interner, get_object_field(jobj, "mutatee"))),
      source: Box::new(load_expression(interner, get_object_field(jobj, "source"))),
    }),
    "MethodCall" => IExpressionPE::MethodCall(MethodCallPE {
      range: load_range(get_object_field(jobj, "range")),
      subject_expr: Box::new(load_expression(interner, get_object_field(jobj, "subjectExpr"))),
      operator_range: load_range(get_object_field(jobj, "operatorRange")),
      method_lookup: Box::new(load_lookup(interner, get_object_field(jobj, "method"))),
      arg_exprs: get_array_field(jobj, "argExprs")
        .iter()
        .map(expect_object)
        .map(|x| load_expression(interner, x))
        .collect(),
    }),
    "Tuple" => IExpressionPE::Tuple(TuplePE {
      range: load_range(get_object_field(jobj, "range")),
      elements: get_array_field(jobj, "elements")
        .iter()
        .map(expect_object)
        .map(|x| load_expression(interner, x))
        .collect(),
    }),
    "Augment" => IExpressionPE::Augment(AugmentPE {
      range: load_range(get_object_field(jobj, "range")),
      target_ownership: load_ownership(get_object_field(jobj, "targetOwnership")),
      inner: Box::new(load_expression(interner, get_object_field(jobj, "inner"))),
    }),
    "Each" => IExpressionPE::Each(EachPE {
      range: load_range(get_object_field(jobj, "range")),
      maybe_pure: load_optional_object(get_object_field(jobj, "maybePure"), load_range),
      entry_pattern: load_pattern(interner, get_object_field(jobj, "entryPattern")),
      in_keyword_range: load_range(get_object_field(jobj, "inRange")),
      iterable_expr: Box::new(load_expression(interner, get_object_field(jobj, "iterableExpr"))),
      body: Box::new(load_block(interner, get_object_field(jobj, "body"))),
    }),
    "Destruct" => IExpressionPE::Destruct(DestructPE {
      range: load_range(get_object_field(jobj, "range")),
      inner: Box::new(load_expression(interner, get_object_field(jobj, "inner"))),
    }),
    "And" => IExpressionPE::And(AndPE {
      range: load_range(get_object_field(jobj, "range")),
      left: Box::new(load_expression(interner, get_object_field(jobj, "left"))),
      right: Box::new(load_block(interner, get_object_field(jobj, "right"))),
    }),
    "Range" => IExpressionPE::Range(RangePE {
      range: load_range(get_object_field(jobj, "range")),
      from_expr: Box::new(load_expression(interner, get_object_field(jobj, "begin"))),
      to_expr: Box::new(load_expression(interner, get_object_field(jobj, "end"))),
    }),
    "BraceCall" => IExpressionPE::BraceCall(BraceCallPE {
      range: load_range(get_object_field(jobj, "range")),
      operator_range: load_range(get_object_field(jobj, "operatorRange")),
      subject_expr: Box::new(load_expression(interner, get_object_field(jobj, "callableExpr"))),
      arg_exprs: get_array_field(jobj, "argExprs")
        .iter()
        .map(expect_object)
        .map(|x| load_expression(interner, x))
        .collect(),
      callable_readwrite: get_boolean_field(jobj, "callableReadwrite"),
    }),
    "Not" => IExpressionPE::Not(NotPE {
      range: load_range(get_object_field(jobj, "range")),
      inner: Box::new(load_expression(interner, get_object_field(jobj, "innerExpr"))),
    }),
    "ConstructArray" => IExpressionPE::ConstructArray(load_construct_array(interner, jobj)),
    "Lookup" => IExpressionPE::Lookup(load_lookup(interner, jobj)),
    "Consecutor" => IExpressionPE::Consecutor(load_consecutor(interner, jobj)),
    "Block" => IExpressionPE::Block(load_block(interner, jobj)),
    other => panic!("Not implemented: load_expression {}", other),
  }
}
/*
  def loadExpression(jobj: JObject): IExpressionPE = {
    getType(jobj) match {
      case "Pack" => {
        PackPE(
          loadRange(getObjectField(jobj, "range")),
          getArrayField(jobj, "innerExprs").map(expectObject).map(loadExpression))
      }
      case "FunctionCall" => {
        FunctionCallPE(
          loadRange(getObjectField(jobj, "range")),
          loadRange(getObjectField(jobj, "operatorRange")),
          loadExpression(getObjectField(jobj, "callableExpr")),
          getArrayField(jobj, "argExprs").map(expectObject).map(loadExpression))
      }
      case "BraceCall" => {
        BraceCallPE(
          loadRange(getObjectField(jobj, "range")),
          loadRange(getObjectField(jobj, "operatorRange")),
          loadExpression(getObjectField(jobj, "callableExpr")),
          getArrayField(jobj, "argExprs").map(expectObject).map(loadExpression),
          getBooleanField(jobj, "callableReadwrite"))
      }
      case "BinaryCall" => {
        BinaryCallPE(
          loadRange(getObjectField(jobj, "range")),
          loadName(getObjectField(jobj, "functionName")),
          loadExpression(getObjectField(jobj, "leftExpr")),
          loadExpression(getObjectField(jobj, "rightExpr")))
      }
      case "MethodCall" => {
        MethodCallPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "subjectExpr")),
          loadRange(getObjectField(jobj, "operatorRange")),
          loadLookup(getObjectField(jobj, "method")),
          getArrayField(jobj, "argExprs").map(expectObject).map(loadExpression))
      }
      case "Shortcall" => {
        ShortcallPE(
          loadRange(getObjectField(jobj, "range")),
          getArrayField(jobj, "argExprs").map(expectObject).map(loadExpression))
      }
      case "Lookup" => {
        loadLookup(jobj)
      }
      case "MagicParamLookup" => {
        MagicParamLookupPE(
          loadRange(getObjectField(jobj, "range")))
      }
      case "ConstantInt" => {
        ConstantIntPE(
          loadRange(getObjectField(jobj, "range")),
          getLongField(jobj, "value"),
          loadOptional(getObjectField(jobj, "bits"), expectNumber).map(_.toInt))
      }
      case "ConstantFloat" => {
        ConstantFloatPE(
          loadRange(getObjectField(jobj, "range")),
          getFloatField(jobj, "value"))
      }
      case "ConstantStr" => {
        ConstantStrPE(
          loadRange(getObjectField(jobj, "range")),
          getStringField(jobj, "value"))
      }
      case "ConstantBool" => {
        ConstantBoolPE(
          loadRange(getObjectField(jobj, "range")),
          getBooleanField(jobj, "value"))
      }
      case "Void" => {
        VoidPE(
          loadRange(getObjectField(jobj, "range")))
      }
      case "Dot" => {
        DotPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "left")),
          loadRange(getObjectField(jobj, "operatorRange")),
          loadName(getObjectField(jobj, "member")))
      }
      case "Lambda" => {
        LambdaPE(
          loadOptionalObject(getObjectField(jobj, "captures"), loadUnit),
          loadFunction(getObjectField(jobj, "function")))
      }
      case "Let" => {
        LetPE(
          loadRange(getObjectField(jobj, "range")),
//          getArrayField(jobj, "attributes").map(expectObject).map(loadAttribute),
//          loadOptionalObject(getObjectField(jobj, "templateRules"), loadTemplateRules),
          loadPattern(getObjectField(jobj, "pattern")),
          loadExpression(getObjectField(jobj, "source")))
      }
      case "Augment" => {
        AugmentPE(
          loadRange(getObjectField(jobj, "range")),
          loadOwnership(getObjectField(jobj, "targetOwnership")),
          loadExpression(getObjectField(jobj, "inner")))
      }
      case "Transmigrate" => {
        TransmigratePE(
          loadRange(getObjectField(jobj, "range")),
          loadName(getObjectField(jobj, "targetRegion")),
          loadExpression(getObjectField(jobj, "inner")))
      }
      case "Mutate" => {
        MutatePE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "mutatee")),
          loadExpression(getObjectField(jobj, "source")))
      }
      case "Return" => {
        ReturnPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "expr")))
      }
      case "Break" => {
        BreakPE(
          loadRange(getObjectField(jobj, "range")))
      }
      case "Consecutor" => {
        loadConsecutor(jobj)
      }
      case "Block" => {
        loadBlock(jobj)
//        BlockPE(
//          loadRange(getObjectField(jobj, "range")),
//          getArrayField(jobj, "elements").map(expectObject).map(loadExpression))
      }
      case "If" => {
        IfPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "condition")),
          loadBlock(getObjectField(jobj, "thenBody")),
          loadBlock(getObjectField(jobj, "elseBody")))
      }
      case "While" => {
        WhilePE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "condition")),
          loadBlock(getObjectField(jobj, "body")))
      }
      case "Index" => {
        IndexPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "left")),
          getArrayField(jobj, "args").map(expectObject).map(loadExpression))
      }
      case "Tuple" => {
        TuplePE(
          loadRange(getObjectField(jobj, "range")),
          getArrayField(jobj, "elements").map(expectObject).map(loadExpression))
      }
      case "ConstructArray" => {
        loadConstructArray(jobj)
      }
      case "Destruct" => {
        DestructPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "inner")))
      }
      case "Unlet" => {
        UnletPE(
          loadRange(getObjectField(jobj, "range")),
          loadImpreciseName(getObjectField(jobj, "localName")))
      }
      case "Each" => {
        EachPE(
          loadRange(getObjectField(jobj, "range")),
          loadOptionalObject(getObjectField(jobj, "maybePure"), loadRange),
          loadPattern(getObjectField(jobj, "entryPattern")),
          loadRange(getObjectField(jobj, "inRange")),
          loadExpression(getObjectField(jobj, "iterableExpr")),
          loadBlock(getObjectField(jobj, "body")))
      }
      case "Or" => {
        OrPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "left")),
          loadBlock(getObjectField(jobj, "right")))
      }
//      case "Result" => {
//        ResultPE(
//          loadRange(getObjectField(jobj, "range")),
//          loadExpression(getObjectField(jobj, "source")))
//      }
      case "And" => {
        AndPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "left")),
          loadBlock(getObjectField(jobj, "right")))
      }
      case "SubExpression" => {
        SubExpressionPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "innerExpr")))
      }
      case "Not" => {
        NotPE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "innerExpr")))
      }
      case "Range" => {
        RangePE(
          loadRange(getObjectField(jobj, "range")),
          loadExpression(getObjectField(jobj, "begin")),
          loadExpression(getObjectField(jobj, "end")))
      }
      case "StrInterpolate" => {
        StrInterpolatePE(
          loadRange(getObjectField(jobj, "range")),
          getArrayField(jobj, "parts").map(expectObject).map(loadExpression))
      }
      case x => vimpl(x.toString)
    }
  }
*/
fn load_array_size<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> IArraySizeP<'a> {
  match get_type(jobj) {
    "RuntimeSized" => IArraySizeP::RuntimeSized,
    "StaticSized" => IArraySizeP::StaticSized(StaticSizedArraySizeP {
      size_pt: load_optional_object(get_object_field(jobj, "size"), |x| load_templex(interner, x)),
    }),
    other => panic!("Not implemented: load_array_size {}", other),
  }
}
/*
  private def loadArraySize(jobj: JObject): IArraySizeP = {
    getType(jobj) match {
      case "RuntimeSized" => RuntimeSizedP
      case "StaticSized" => {
        StaticSizedP(loadOptionalObject(getObjectField(jobj, "size"), loadTemplex))
      }
    }
  }
*/
fn load_construct_array<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> ConstructArrayPE<'a> {
  ConstructArrayPE {
    range: load_range(get_object_field(jobj, "range")),
    type_pt: load_optional_object(get_object_field(jobj, "type"), |x| load_templex(interner, x)),
    mutability_pt: load_optional_object(get_object_field(jobj, "mutability"), |x| {
      load_templex(interner, x)
    }),
    variability_pt: load_optional_object(get_object_field(jobj, "variability"), |x| {
      load_templex(interner, x)
    }),
    size: load_array_size(interner, get_object_field(jobj, "size")),
    initializing_individual_elements: get_boolean_field(jobj, "initializingIndividualElements"),
    args: get_array_field(jobj, "args")
      .iter()
      .map(expect_object)
      .map(|x| load_expression(interner, x))
      .collect(),
  }
}
/*
  private def loadConstructArray(jobj: JObject) = {
    ConstructArrayPE(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "type"), loadTemplex),
      loadOptionalObject(getObjectField(jobj, "mutability"), loadTemplex),
      loadOptionalObject(getObjectField(jobj, "variability"), loadTemplex),
      loadArraySize(getObjectField(jobj, "size")),
      getBooleanField(jobj, "initializingIndividualElements"),
      getArrayField(jobj, "args").map(expectObject).map(loadExpression))
  }
*/
fn load_lookup<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> LookupPE<'a> {
  LookupPE {
    name: load_imprecise_name(interner, get_object_field(jobj, "name")),
    template_args: load_optional_object(get_object_field(jobj, "templateArgs"), |x| {
      load_template_args(interner, x)
    }),
  }
}
/*
  private def loadLookup(jobj: JObject) = {
    LookupPE(
      loadImpreciseName(getObjectField(jobj, "name")),
      loadOptionalObject(getObjectField(jobj, "templateArgs"), loadTemplateArgs))
  }
*/
fn load_template_args<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> TemplateArgsP<'a> {
  TemplateArgsP {
    range: load_range(get_object_field(jobj, "range")),
    args: get_array_field(jobj, "args")
      .iter()
      .map(expect_object)
      .map(|x| load_templex(interner, x))
      .collect(),
  }
}
/*
  def loadTemplateArgs(jobj: JObject): TemplateArgsP = {
    ast.TemplateArgsP(
      loadRange(getObjectField(jobj, "range")),
      getArrayField(jobj, "args").map(expectObject).map(loadTemplex))
  }
*/
// fn load_load_as(jobj: &Map<String, Value>) -> LoadAsP {
//   match get_type(jobj) {
//     "Move" => LoadAsP::Move,
//     "Use" => LoadAsP::Use,
//     "LoadAsBorrow" => LoadAsP::LoadAsBorrow,
//     "LoadAsWeak" => LoadAsP::LoadAsWeak,
//     other => panic!("Not implemented: load_load_as {}", other),
//   }
// }
/*
  def loadLoadAs(jobj: JObject): LoadAsP = {
    getType(jobj) match {
      case "Move" => MoveP
      case "Use" => UseP
      case "LoadAsBorrow" => LoadAsBorrowP
      case "LoadAsWeak" => LoadAsWeakP
      case other => vwat(other)
    }
  }
*/
fn load_virtuality<'a>(_interner: &Interner<'a>, jobj: &Map<String, Value>) -> AbstractP {
  AbstractP {
    range: load_range(get_object_field(jobj, "range")),
  }
}
/*
  def loadVirtuality(jobj: JObject): AbstractP = {
//    getType(jobj) match {
//      case "Override" => {
//        OverrideP(
//          loadRange(getObjectField(jobj, "range")),
//          loadTemplex(getObjectField(jobj, "type")))
//      }
//      case "Abstract" => {
        AbstractP(
          loadRange(getObjectField(jobj, "range")))
//      }
//    }
  }
*/
fn load_struct_content<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> IStructContent<'a> {
  match get_type(jobj) {
    "NormalStructMember" => IStructContent::NormalStructMember(NormalStructMemberP {
      range: load_range(get_object_field(jobj, "range")),
      name: load_name(interner, get_object_field(jobj, "name")),
      variability: load_variability(get_object_field(jobj, "variability")),
      tyype: load_templex(interner, get_object_field(jobj, "type")),
    }),
    "VariadicStructMember" => IStructContent::VariadicStructMember(VariadicStructMemberP {
      range: load_range(get_object_field(jobj, "range")),
      variability: load_variability(get_object_field(jobj, "variability")),
      tyype: load_templex(interner, get_object_field(jobj, "type")),
    }),
    "StructMethod" => IStructContent::StructMethod(load_function(interner, get_object_field(jobj, "function"))),
    other => panic!("Not implemented: load_struct_content {}", other),
  }
}
/*
  def loadStructContent(jobj: JObject): IStructContent = {
    getType(jobj) match {
      case "NormalStructMember" => {
        NormalStructMemberP(
          loadRange(getObjectField(jobj, "range")),
          loadName(getObjectField(jobj, "name")),
          loadVariability(getObjectField(jobj, "variability")),
          loadTemplex(getObjectField(jobj, "type")))
      }
      case "VariadicStructMember" => {
        VariadicStructMemberP(
          loadRange(getObjectField(jobj, "range")),
          loadVariability(getObjectField(jobj, "variability")),
          loadTemplex(getObjectField(jobj, "type")))
      }
      case "StructMethod" => {
        StructMethodP(
          loadFunction(getObjectField(jobj, "function")))
      }
    }
  }
*/
fn load_optional_object<T, F>(jobj: &Map<String, Value>, load_contents: F) -> Option<T>
where
  F: Fn(&Map<String, Value>) -> T,
{
  match get_type(jobj) {
    "None" => None,
    "Some" => Some(load_contents(get_object_field(jobj, "value"))),
    other => panic!("BadVPSTError: Expected None/Some but got {}", other),
  }
}
/*
  def loadOptionalObject[T](jobj: JObject, loadContents: JObject => T): Option[T] = {
    getType(jobj) match {
      case "None" => None
      case "Some" => Some(loadContents(getObjectField(jobj, "value")))
    }
  }
*/
// fn load_optional<T, F>(jobj: &Map<String, Value>, load_contents: F) -> Option<T>
// where
//   F: Fn(&Value) -> T,
// {
//   panic!("Not implemented");
// }
/*
  def loadOptional[T](jobj: JObject, loadContents: JValue => T): Option[T] = {
    getType(jobj) match {
      case "None" => None
      case "Some" => Some(loadContents(getField(jobj, "value")))
    }
  }
*/
fn load_template_rules<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> TemplateRulesP<'a> {
  TemplateRulesP {
    range: load_range(get_object_field(jobj, "range")),
    rules: get_array_field(jobj, "rules")
      .iter()
      .map(expect_object)
      .map(|x| load_rulex(interner, x))
      .collect(),
  }
}
/*
  def loadTemplateRules(jobj: JObject): TemplateRulesP = {
    ast.TemplateRulesP(
      loadRange(getObjectField(jobj, "range")),
      getArrayField(jobj, "rules").map(expectObject).map(loadRulex))
  }
*/
fn load_rulex<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> IRulexPR<'a> {
  match get_type(jobj) {
    "TypedPR" => IRulexPR::Typed(load_typed_pr(interner, jobj)),
    "ComponentsPR" => IRulexPR::Components(ComponentsPR {
      range: load_range(get_object_field(jobj, "range")),
      container: load_rulex_type(get_object_field(jobj, "container")),
      components: get_array_field(jobj, "components")
        .iter()
        .map(expect_object)
        .map(|x| load_rulex(interner, x))
        .collect(),
    }),
    "OrPR" => IRulexPR::Or(OrPR {
      range: load_range(get_object_field(jobj, "range")),
      possibilities: get_array_field(jobj, "possibilities")
        .iter()
        .map(expect_object)
        .map(|x| load_rulex(interner, x))
        .collect(),
    }),
    "DotPR" => IRulexPR::Dot(DotPR {
      range: load_range(get_object_field(jobj, "range")),
      container: Box::new(load_rulex(interner, get_object_field(jobj, "container"))),
      member_name: load_name(interner, get_object_field(jobj, "memberName")),
    }),
    "TemplexPR" => IRulexPR::Templex(load_templex(interner, get_object_field(jobj, "templex"))),
    "EqualsPR" => IRulexPR::Equals(EqualsPR {
      range: load_range(get_object_field(jobj, "range")),
      left: Box::new(load_rulex(interner, get_object_field(jobj, "left"))),
      right: Box::new(load_rulex(interner, get_object_field(jobj, "right"))),
    }),
    "BuiltinCallPR" => IRulexPR::BuiltinCall(BuiltinCallPR {
      range: load_range(get_object_field(jobj, "range")),
      name: load_name(interner, get_object_field(jobj, "name")),
      args: get_array_field(jobj, "args")
        .iter()
        .map(expect_object)
        .map(|x| load_rulex(interner, x))
        .collect(),
    }),
    other => panic!("Not implemented: load_rulex {}", other),
  }
}
/*
  def loadRulex(jobj: JObject): IRulexPR = {
    getType(jobj) match {
      case "TypedPR" => {
        loadTypedPR(jobj)
      }
      case "ComponentsPR" => {
        ComponentsPR(
          loadRange(getObjectField(jobj, "range")),
          loadRulexType(getObjectField(jobj, "container")),
          getArrayField(jobj, "components").map(expectObject).map(loadRulex))
      }
      case "OrPR" => {
        OrPR(
          loadRange(getObjectField(jobj, "range")),
          getArrayField(jobj, "possibilities").map(expectObject).map(loadRulex))
      }
      case "TemplexPR" => {
        TemplexPR(
          loadTemplex(getObjectField(jobj, "templex")))
      }
      case "EqualsPR" => {
        EqualsPR(
          loadRange(getObjectField(jobj, "range")),
          loadRulex(getObjectField(jobj, "left")),
          loadRulex(getObjectField(jobj, "right")))
      }
      case "BuiltinCallPR" => {
        BuiltinCallPR(
          loadRange(getObjectField(jobj, "range")),
          loadName(getObjectField(jobj, "name")),
          getArrayField(jobj, "args").map(expectObject).map(loadRulex))
      }
      case x => vimpl(x.toString)
    }
  }
*/
fn load_typed_pr<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> TypedPR<'a> {
  TypedPR {
    range: load_range(get_object_field(jobj, "range")),
    rune: load_optional_object(get_object_field(jobj, "rune"), |x| load_name(interner, x)),
    tyype: load_rulex_type(get_object_field(jobj, "type")),
  }
}
/*
  private def loadTypedPR(jobj: JObject) = {
    TypedPR(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "rune"), loadName),
      loadRulexType(getObjectField(jobj, "type")))
  }
*/
fn load_rulex_type(jobj: &Map<String, Value>) -> ITypePR {
  match get_type(jobj) {
    "IntTypePR" => ITypePR::IntType,
    "BoolTypePR" => ITypePR::BoolType,
    "OwnershipTypePR" => ITypePR::OwnershipType,
    "MutabilityTypePR" => ITypePR::MutabilityType,
    "VariabilityTypePR" => ITypePR::VariabilityType,
    "LocationTypePR" => ITypePR::LocationType,
    "CoordTypePR" => ITypePR::CoordType,
    "CoordListTypePR" => ITypePR::CoordListType,
    "PrototypeTypePR" => ITypePR::PrototypeType,
    "KindTypePR" => ITypePR::KindType,
    "RegionTypePR" => ITypePR::RegionType,
    "CitizenTemplateTypePR" => ITypePR::CitizenTemplateType,
    other => panic!("Not implemented: load_rulex_type {}", other),
  }
}
/*
  def loadRulexType(jobj: JObject): ITypePR = {
    getType(jobj) match {
      case "IntTypePR" => IntTypePR
      case "BoolTypePR" => BoolTypePR
      case "OwnershipTypePR" => OwnershipTypePR
      case "MutabilityTypePR" => MutabilityTypePR
      case "VariabilityTypePR" => VariabilityTypePR
      case "LocationTypePR" => LocationTypePR
      case "CoordTypePR" => CoordTypePR
      case "CoordListTypePR" => CoordListTypePR
      case "PrototypeTypePR" => PrototypeTypePR
      case "KindTypePR" => KindTypePR
      case "RegionTypePR" => RegionTypePR
      case "CitizenTemplateTypePR" => CitizenTemplateTypePR
      case x => vimpl(x.toString)
    }
  }
*/
fn load_generic_parameter_type(jobj: &Map<String, Value>) -> GenericParameterTypeP {
  GenericParameterTypeP {
    range: load_range(get_object_field(jobj, "range")),
    tyype: load_rulex_type(get_object_field(jobj, "type")),
  }
}
/*
  def loadGenericParameterType(jobj: JObject): GenericParameterTypeP = {
    GenericParameterTypeP(
      loadRange(getObjectField(jobj, "range")),
      loadRulexType(getObjectField(jobj, "type")))
  }
*/
fn load_rune_attribute(jobj: &Map<String, Value>) -> IRuneAttributeP {
  match get_type(jobj) {
    "ReadOnlyRuneAttribute" => {
      IRuneAttributeP::ReadOnlyRegionRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "ReadWriteRuneAttribute" => {
      IRuneAttributeP::ReadWriteRegionRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "ImmutableRuneAttribute" => {
      IRuneAttributeP::ImmutableRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "MutableRuneAttribute" => {
      IRuneAttributeP::MutableRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "ImmutableRegionRuneAttribute" => {
      IRuneAttributeP::ImmutableRegionRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "AdditiveRuneAttribute" => {
      IRuneAttributeP::AdditiveRegionRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "PoolRuneAttribute" => {
      IRuneAttributeP::PoolRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "ArenaRuneAttribute" => {
      IRuneAttributeP::ArenaRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    "BumpRuneAttribute" => {
      IRuneAttributeP::BumpRuneAttribute(load_range(get_object_field(jobj, "range")))
    }
    other => panic!("Not implemented: load_rune_attribute {}", other),
  }
}
/*
  def loadRuneAttribute(jobj: JObject): IRuneAttributeP = {
    getType(jobj) match {
      case "ReadOnlyRuneAttribute" => ReadOnlyRegionRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "ReadWriteRuneAttribute" => ReadWriteRegionRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "ImmutableRuneAttribute" => ImmutableRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "MutableRuneAttribute" => MutableRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "ImmutableRegionRuneAttribute" => ImmutableRegionRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "AdditiveRuneAttribute" => AdditiveRegionRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "PoolRuneAttribute" => PoolRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "ArenaRuneAttribute" => ArenaRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case "BumpRuneAttribute" => BumpRuneAttributeP(loadRange(getObjectField(jobj, "range")))
      case x => vimpl(x.toString)
    }
  }
*/
fn load_attribute<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> IAttributeP<'a> {
  match get_type(jobj) {
    "AbstractAttribute" => IAttributeP::AbstractAttribute(AbstractAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "PureAttribute" => IAttributeP::PureAttribute(PureAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "AdditiveAttribute" => IAttributeP::AdditiveAttribute(AdditiveAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "ExportAttribute" => IAttributeP::ExportAttribute(ExportAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "ExternAttribute" => IAttributeP::ExternAttribute(ExternAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "LinearAttribute" => IAttributeP::LinearAttribute(LinearAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "BuiltinAttribute" => IAttributeP::BuiltinAttribute(BuiltinAttributeP {
      range: load_range(get_object_field(jobj, "range")),
      generator_name: load_name(interner, get_object_field(jobj, "generatorName")),
    }),
    "SealedAttribute" => IAttributeP::SealedAttribute(SealedAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "WeakableAttribute" => IAttributeP::WeakableAttribute(WeakableAttributeP {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "MacroCall" => IAttributeP::MacroCall(MacroCallP {
      range: load_range(get_object_field(jobj, "range")),
      inclusion: if get_boolean_field(jobj, "dontCall") {
        IMacroInclusionP::DontCallMacro
      } else {
        IMacroInclusionP::CallMacro
      },
      name: load_name(interner, get_object_field(jobj, "name")),
    }),
    other => panic!("Not implemented: unknown attribute type {}", other),
  }
}
/*
  def loadAttribute(jobj: JObject): IAttributeP = {
    getType(jobj) match {
      case "AbstractAttribute" => AbstractAttributeP(loadRange(getObjectField(jobj, "range")))
      case "PureAttribute" => PureAttributeP(loadRange(getObjectField(jobj, "range")))
      case "AdditiveAttribute" => AdditiveAttributeP(loadRange(getObjectField(jobj, "range")))
      case "ExportAttribute" => ExportAttributeP(loadRange(getObjectField(jobj, "range")))
      case "ExternAttribute" => ExternAttributeP(loadRange(getObjectField(jobj, "range")))
      case "LinearAttribute" => LinearAttributeP(loadRange(getObjectField(jobj, "range")))
      case "BuiltinAttribute" => {
        BuiltinAttributeP(
          loadRange(getObjectField(jobj, "range")),
          loadName(getObjectField(jobj, "generatorName")))
      }
      case "ExportAttribute" => ExportAttributeP(loadRange(getObjectField(jobj, "range")))
      case "SealedAttribute" => SealedAttributeP(loadRange(getObjectField(jobj, "range")))
      case "WeakableAttribute" => WeakableAttributeP(loadRange(getObjectField(jobj, "range")))
      case "MacroCall" => {
        MacroCallP(
          loadRange(getObjectField(jobj, "range")),
          if (getBooleanField(jobj, "dontCall")) DontCallMacroP else CallMacroP,
          loadName(getObjectField(jobj, "name")))
      }
      case x => vimpl(x.toString)
    }
  }
*/
fn load_mutability(jobj: &Map<String, Value>) -> MutabilityP {
  match get_type(jobj) {
    "Mutable" => MutabilityP::Mutable,
    "Immutable" => MutabilityP::Immutable,
    other => panic!("Not implemented: load_mutability {}", other),
  }
}
/*
  def loadMutability(jobj: JObject): MutabilityP = {
    getType(jobj) match {
      case "Mutable" => MutableP
      case "Immutable" => ImmutableP
    }
  }
*/
fn load_variability(jobj: &Map<String, Value>) -> VariabilityP {
  match get_type(jobj) {
    "Varying" => VariabilityP::Varying,
    "Final" => VariabilityP::Final,
    other => panic!("Not implemented: load_variability {}", other),
  }
}
/*
  def loadVariability(jobj: JObject): VariabilityP = {
    getType(jobj) match {
      case "Varying" => VaryingP
      case "Final" => FinalP
    }
  }
*/
fn load_ownership(jobj: &Map<String, Value>) -> OwnershipP {
  match get_type(jobj) {
    "Own" => OwnershipP::Own,
    "Borrow" => OwnershipP::Borrow,
    "Live" => OwnershipP::Live,
    "Weak" => OwnershipP::Weak,
    "Share" => OwnershipP::Share,
    other => panic!("Not implemented: load_ownership {}", other),
  }
}
/*
  def loadOwnership(jobj: JObject): OwnershipP = {
    getType(jobj) match {
      case "Own" => OwnP
      case "Borrow" => BorrowP
      case "Weak" => WeakP
      case "Share" => ShareP
    }
  }
*/
fn load_templex<'a>(interner: &Interner<'a>, jobj: &Map<String, Value>) -> ITemplexPT<'a> {
  match get_type(jobj) {
    "NameOrRuneT" => ITemplexPT::NameOrRune(NameOrRunePT {
      name: load_name(interner, get_object_field(jobj, "rune")),
    }),
    "InterpretedT" => ITemplexPT::Interpreted(InterpretedPT {
      range: load_range(get_object_field(jobj, "range")),
      maybe_ownership: load_optional_object(
        get_object_field(jobj, "maybeOwnership"),
        |x| load_ownership_pt(interner, x),
      )
      .map(Box::new),
      maybe_region: load_optional_object(get_object_field(jobj, "maybeRegion"), |x| {
        load_region_rune(interner, x)
      })
      .map(Box::new),
      inner: Box::new(load_templex(interner, get_object_field(jobj, "inner"))),
    }),
    "CallT" => ITemplexPT::Call(CallPT {
      range: load_range(get_object_field(jobj, "range")),
      template: Box::new(load_templex(interner, get_object_field(jobj, "template"))),
      args: get_array_field(jobj, "args")
        .iter()
        .map(expect_object)
        .map(|x| load_templex(interner, x))
        .collect(),
    }),
    "MutabilityT" => ITemplexPT::Mutability(MutabilityPT {
      range: load_range(get_object_field(jobj, "range")),
      mutability: load_mutability(get_object_field(jobj, "mutability")),
    }),
    "VariabilityT" => ITemplexPT::Variability(VariabilityPT {
      range: load_range(get_object_field(jobj, "range")),
      variability: load_variability(get_object_field(jobj, "variability")),
    }),
    "IntT" => ITemplexPT::Int(IntPT {
      range: load_range(get_object_field(jobj, "range")),
      value: get_long_field(jobj, "inner"),
    }),
    "AnonymousRuneT" => ITemplexPT::AnonymousRune(AnonymousRunePT {
      range: load_range(get_object_field(jobj, "range")),
    }),
    "RuntimeSizedArrayT" => ITemplexPT::RuntimeSizedArray(RuntimeSizedArrayPT {
      range: load_range(get_object_field(jobj, "range")),
      mutability: Box::new(load_templex(interner, get_object_field(jobj, "mutability"))),
      element: Box::new(load_templex(interner, get_object_field(jobj, "element"))),
    }),
    "StaticSizedArrayT" => ITemplexPT::StaticSizedArray(StaticSizedArrayPT {
      range: load_range(get_object_field(jobj, "range")),
      mutability: Box::new(load_templex(interner, get_object_field(jobj, "mutability"))),
      variability: Box::new(load_templex(interner, get_object_field(jobj, "variability"))),
      size: Box::new(load_templex(interner, get_object_field(jobj, "size"))),
      element: Box::new(load_templex(interner, get_object_field(jobj, "element"))),
    }),
    "ManualSequenceT" => ITemplexPT::Tuple(TuplePT {
      range: load_range(get_object_field(jobj, "range")),
      elements: get_array_field(jobj, "members")
        .iter()
        .map(expect_object)
        .map(|x| load_templex(interner, x))
        .collect(),
    }),
    "PrototypeT" => ITemplexPT::Func(FuncPT {
      range: load_range(get_object_field(jobj, "range")),
      name: load_name(interner, get_object_field(jobj, "name")),
      params_range: load_range(get_object_field(jobj, "paramsRange")),
      parameters: get_array_field(jobj, "params")
        .iter()
        .map(expect_object)
        .map(|x| load_templex(interner, x))
        .collect(),
      return_type: Box::new(load_templex(interner, get_object_field(jobj, "returnType"))),
    }),
    other => panic!("Not implemented: load_templex {}", other),
  }
}
/*
  def loadTemplex(jobj: JObject): ITemplexPT = {
    getType(jobj) match {
      case "NameOrRuneT" => {
        NameOrRunePT(
          loadName(getObjectField(jobj, "rune")))
      }
      case "MutabilityT" => {
        MutabilityPT(
          loadRange(getObjectField(jobj, "range")),
          loadMutability(getObjectField(jobj, "mutability")))
      }
      case "VariabilityT" => {
        VariabilityPT(
          loadRange(getObjectField(jobj, "range")),
          loadVariability(getObjectField(jobj, "variability")))
      }
      case "StringT" => {
        StringPT(
          loadRange(getObjectField(jobj, "range")),
          getStringField(jobj, "str"))
      }
      case "IntT" => {
        IntPT(
          loadRange(getObjectField(jobj, "range")),
          getLongField(jobj, "inner"))
      }
      case "AnonymousRuneT" => {
        AnonymousRunePT(
          loadRange(getObjectField(jobj, "range")))
      }
      case "RegionRuneT" => {
        loadRegionRune(jobj)
      }
      case "OwnershipT" => {
        loadOwnershipPT(jobj)
      }
      case "InterpretedT" => {
        InterpretedPT(
          loadRange(getObjectField(jobj, "range")),
          loadOptionalObject(getObjectField(jobj, "maybeOwnership"), loadOwnershipPT),
          loadOptionalObject(getObjectField(jobj, "maybeRegion"), loadRegionRune),
          loadTemplex(getObjectField(jobj, "inner")))
      }
      case "CallT" => {
        CallPT(
          loadRange(getObjectField(jobj, "range")),
          loadTemplex(getObjectField(jobj, "template")),
          getArrayField(jobj, "args").map(expectObject).map(loadTemplex))
      }
      case "PackT" => {
        PackPT(
          loadRange(getObjectField(jobj, "range")),
          getArrayField(jobj, "members").map(expectObject).map(loadTemplex))
      }
      case "ManualSequenceT" => {
        TuplePT(
          loadRange(getObjectField(jobj, "range")),
          getArrayField(jobj, "members").map(expectObject).map(loadTemplex))
      }
      case "StaticSizedArrayT" => {
        StaticSizedArrayPT(
          loadRange(getObjectField(jobj, "range")),
          loadTemplex(getObjectField(jobj, "mutability")),
          loadTemplex(getObjectField(jobj, "variability")),
          loadTemplex(getObjectField(jobj, "size")),
          loadTemplex(getObjectField(jobj, "element")))
      }
      case "RuntimeSizedArrayT" => {
        RuntimeSizedArrayPT(
          loadRange(getObjectField(jobj, "range")),
          loadTemplex(getObjectField(jobj, "mutability")),
          loadTemplex(getObjectField(jobj, "element")))
      }
      case "InlineT" => {
        InlinePT(
          loadRange(getObjectField(jobj, "range")),
          loadTemplex(getObjectField(jobj, "inner")))
      }
      case "PrototypeT" => {
        FuncPT(
          loadRange(getObjectField(jobj, "range")),
          loadName(getObjectField(jobj, "name")),
          loadRange(getObjectField(jobj, "paramsRange")),
          getArrayField(jobj, "params").map(expectObject).map(loadTemplex),
          loadTemplex(getObjectField(jobj, "returnType")))
      }
      case x => vimpl(x.toString)
    }
  }
*/
fn load_ownership_pt<'a>(_interner: &Interner<'a>, jobj: &Map<String, Value>) -> OwnershipPT {
  OwnershipPT {
    range: load_range(get_object_field(jobj, "range")),
    ownership: load_ownership(get_object_field(jobj, "ownership")),
  }
}
/*
  private def loadOwnershipPT(jobj: JObject) = {
    OwnershipPT(
      loadRange(getObjectField(jobj, "range")),
      loadOwnership(getObjectField(jobj, "ownership")))
  }
*/
fn load_region_rune<'a>(_interner: &Interner<'a>, _jobj: &Map<String, Value>) -> RegionRunePT<'a> {
  panic!("Not implemented");
}
/*
  private def loadRegionRune(jobj: JObject): RegionRunePT = {
    RegionRunePT(
      loadRange(getObjectField(jobj, "range")),
      loadOptionalObject(getObjectField(jobj, "name"), loadName))
  }
*/
fn load_identifying_runes<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> GenericParametersP<'a> {
  GenericParametersP {
    range: load_range(get_object_field(jobj, "range")),
    params: get_array_field(jobj, "identifyingRunes")
      .iter()
      .map(expect_object)
      .map(|x| load_identifying_rune(interner, x))
      .collect(),
  }
}
/*
  def loadIdentifyingRunes(jobj: JObject): GenericParametersP = {
    GenericParametersP(
      loadRange(getObjectField(jobj, "range")),
      getArrayField(jobj, "identifyingRunes").map(expectObject).map(loadIdentifyingRune))
  }
*/
fn load_identifying_rune<'a>(
  interner: &Interner<'a>,
  jobj: &Map<String, Value>,
) -> GenericParameterP<'a> {
  GenericParameterP {
    range: load_range(get_object_field(jobj, "range")),
    name: load_name(interner, get_object_field(jobj, "name")),
    maybe_type: load_optional_object(
      get_object_field(jobj, "maybeType"),
      load_generic_parameter_type,
    ),
    coord_region: load_optional_object(get_object_field(jobj, "maybeCoordRegion"), |x| {
      load_region_rune(interner, x)
    }),
    attributes: get_array_field(jobj, "attributes")
      .iter()
      .map(expect_object)
      .map(load_rune_attribute)
      .collect(),
    maybe_default: load_optional_object(get_object_field(jobj, "maybeDefault"), |x| {
      load_templex(interner, x)
    }),
  }
}
/*
  def loadIdentifyingRune(jobj: JObject): GenericParameterP = {
    GenericParameterP(
      loadRange(getObjectField(jobj, "range")),
      loadName(getObjectField(jobj, "name")),
      loadOptionalObject(getObjectField(jobj, "maybeType"), loadGenericParameterType),
      loadOptionalObject(getObjectField(jobj, "maybeCoordRegion"), loadRegionRune),
      getArrayField(jobj, "attributes").map(expectObject).map(loadRuneAttribute),
      loadOptionalObject(getObjectField(jobj, "maybeDefault"), loadTemplex))
  }
}
*/
