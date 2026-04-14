// Run with: cargo test 

use super::ast::*;

pub struct VonPrinter {
  #[allow(dead_code)]
  line_width: usize,
}
/*
package dev.vale.von

import dev.vale.{Profiler, vcurious, vimpl}
import org.apache.commons.lang3.StringEscapeUtils

sealed trait ISyntax
case class VonSyntax(
  includeFieldNames: Boolean = true,
  squareBracesForArrays: Boolean = true,
  includeEmptyParams: Boolean = true,
  includeEmptyArrayMembersAtEnd: Boolean = true,
) extends ISyntax {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
case object JsonSyntax extends ISyntax

*/

impl VonPrinter {
  pub fn new() -> Self {
    VonPrinter { line_width: 120 }
  }
  /*
  class VonPrinter(
      syntax: ISyntax,
      lineWidth: Int,
      typeNameMap: Map[String, String] = Map(),
      includeSpaceAfterComma: Boolean = true,
  ) {
    val memberSeparator = if (includeSpaceAfterComma) ", " else ","
    */

  pub fn print(&self, data: &IVonData) -> String {
    let mut result = String::new();
    self.print_multiline(&mut result, data, 0);
    result
  }
  /*
  def print(data: IVonData): String = {
    val builder = new StringBuilder()
    printSingleLine(data, lineWidth) match {
      case Some(str) => builder ++= str
      case None => printMultiline(builder, data, 0)
    }
    builder.toString()
  }
  */

  fn escape(&self, value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
      match ch {
        '\\' => escaped.push_str("\\\\"),
        '"' => escaped.push_str("\\\""),
        '\u{0008}' => escaped.push_str("\\b"),
        '\n' => escaped.push_str("\\n"),
        '\u{000C}' => escaped.push_str("\\f"),
        '\r' => escaped.push_str("\\r"),
        '\t' => escaped.push_str("\\t"),
        c if c <= '\u{001F}' || c > '\u{007F}' => {
          let mut units = [0u16; 2];
          for unit in c.encode_utf16(&mut units).iter() {
            escaped.push_str(&format!("\\u{:04X}", unit));
          }
        }
        c => escaped.push(c),
      }
    }
    escaped
  }

  /*
  def escape(value: String): String = {
    syntax match {
      case VonSyntax(_, _, _, _) => StringEscapeUtils.escapeJava(value)
      case JsonSyntax => {
        // We couldn't use escapeJavaScript because it escapes single quotes, and lift-json
        // has a bug where \' turns into \
        StringEscapeUtils.escapeJava(value)
      }
    }
  }
  */

  fn print_multiline(&self, builder: &mut String, data: &IVonData, indentation: usize) {
    match data {
      IVonData::Int(VonInt { value }) => {
        builder.push_str(&value.to_string());
      }
      IVonData::Float(VonFloat { value }) => {
        builder.push_str(&value.to_string());
      }
      IVonData::Bool(VonBool { value }) => {
        builder.push_str(&value.to_string());
      }
      IVonData::Str(VonStr { value }) => {
        builder.push('"');
        builder.push_str(&self.escape(value));
        builder.push('"');
      }
      IVonData::Object(obj) => {
        self.print_object_multiline(builder, obj, indentation);
      }
      IVonData::Array(arr) => {
        self.print_array_multiline(builder, arr, indentation);
      }
    }
  }

  /*
  def printMultiline(
    builder: StringBuilder,
    data: IVonData,
    indentation: Int):
  Unit = {
    Profiler.frame(() => {
      data match {
        case VonInt(value) => builder ++= value.toString
        case VonBool(value) => builder ++= value.toString
        case VonStr(value) => {
          builder ++= "\""
          builder ++= escape(value)
          builder ++= "\""
        }
        case VonReference(id) => vimpl()
        case o@VonObject(_, _, _) => printObjectMultiline(builder, o, indentation)
        case a@VonArray(_, _) => printArrayMultiline(builder, a, indentation)
        case VonListMap(id, members) => vimpl()
        case VonMap(id, members) => vimpl()
      }
    })
  }
  */
  fn print_indent(&self, builder: &mut String, indentation: usize) {
    for _ in 0..indentation {
      builder.push_str("  ");
    }
  }

  /*
  def repeatStr(str: String, n: Int): String = {
    val resultBuilder = new StringBuilder()
    (0 until n).foreach(i => {
      resultBuilder ++= str
    })
    resultBuilder.toString()
  }
  */
  fn print_object_multiline(&self, builder: &mut String, obj: &VonObject, indentation: usize) {
    let members = &obj.members;

    // JSON: {"__type": "TypeName", "field1": value1, ...}
    builder.push_str("{\"__type\": \"");
    builder.push_str(&self.escape(&obj.tyype));
    builder.push('"');

    if !members.is_empty() {
      builder.push_str(", ");
      builder.push('\n');

      for (index, member) in members.iter().enumerate() {
        self.print_indent(builder, indentation + 1);
        self.print_member_multiline(builder, member, indentation + 1);
        if index < members.len() - 1 {
          builder.push(',');
        }
        builder.push('\n');
      }

      self.print_indent(builder, indentation);
    }

    builder.push('}');
  }

  /*
  def printObjectMultiline(builder: StringBuilder, obbject: VonObject, indentation: Int): Unit = {
    val VonObject(tyype, None, unfilteredMembers) = obbject

    val members = filterMembers(unfilteredMembers)

    builder ++= printObjectStart(tyype, members.nonEmpty)
    builder ++= "\n"

    members.zipWithIndex.foreach({ case (member, index) =>
      builder ++= repeatStr("  ", indentation + 1)
      printMemberSingleLine(member, lineWidth) match {
        case None => printMemberMultiline(builder, member, indentation + 1)
        case Some(s) => builder ++= s
      }
      builder ++= (if (index == members.size - 1) "" else ",")
      if (index > 0) {
        builder ++= "\n"
      }
    })
    builder ++= printObjectEnd(members.nonEmpty)
  }
  */
  /*
  def printObjectStart(originalType: String, hasMembers: Boolean): String = {
    val mappedType = typeNameMap.getOrElse(originalType, originalType)
    syntax match {
      case VonSyntax(_, _, true, _) => mappedType + "("
      case VonSyntax(_, _, false, _) => mappedType + (if (hasMembers) "(" else "")
      case JsonSyntax => {
        "{\"__type\": " + "\"" + escape(mappedType) + "\"" + (if (hasMembers) memberSeparator else "")
      }
    }
  }
  def printObjectEnd(hasMembers: Boolean): String = {
    syntax match {
      case VonSyntax(_, _, true, _) => ")"
      case VonSyntax(_, _, false, _) => (if (hasMembers) ")" else "")
      case JsonSyntax => "}"
    }
  }
  */
  /*
  def printArrayStart(): String = {
    syntax match {
      case VonSyntax(_, false, _, _) => "Array("
      case VonSyntax(_, true, _, _) => "["
      case JsonSyntax => "["
    }
  }
  def printArrayEnd(): String = {
    syntax match {
      case VonSyntax(_, false, _, _) => ")"
      case VonSyntax(_, true, _, _) => "]"
      case JsonSyntax => "]"
    }
  }
  */
  fn print_array_multiline(&self, builder: &mut String, arr: &VonArray, indentation: usize) {
    let members = &arr.members;

    builder.push('[');

    if !members.is_empty() {
      builder.push('\n');

      for (index, member) in members.iter().enumerate() {
        self.print_indent(builder, indentation + 1);
        self.print_multiline(builder, member, indentation + 1);
        if index < members.len() - 1 {
          builder.push(',');
        }
        builder.push('\n');
      }

      self.print_indent(builder, indentation);
    }

    builder.push(']');
  }

  /*
  def printArrayMultiline(builder: StringBuilder, array: VonArray, indentation: Int): Unit = {
    val VonArray(None, members) = array

    builder ++= printArrayStart()
    builder ++= "\n"
    members.zipWithIndex.foreach({ case (member, index) =>
      builder ++= repeatStr("  ", indentation + 1)
      printSingleLine(member, lineWidth) match {
        case None => printMultiline(builder, member, indentation + 1)
        case Some(s) => builder ++= s
      }
      builder ++= (if (index == members.size - 1) "" else ",")
      if (index > 0) {
        builder ++= "\n"
      }
    })
    builder ++= printArrayEnd()
  }
  */

  fn print_member_multiline(&self, builder: &mut String, member: &VonMember, indentation: usize) {
    builder.push('"');
    builder.push_str(&member.field_name);
    builder.push_str("\": ");
    self.print_multiline(builder, &member.value, indentation);
  }

  /*
  def printMemberMultiline(
    builder: StringBuilder,
    member: VonMember,
    indentation: Int):
  Unit = {
    builder ++= getMemberPrefix(member)
    printMultiline(builder, member.value, indentation)
  }
  */
  /*
  def getMemberPrefix(member: VonMember):
  // None if we failed to put it on the one line.
  String = {
    val VonMember(fieldName, _) = member
    printMemberPrefix(fieldName)
  }
  */
  /*

  def printSingleLine(
    data: IVonData,
    // None means it will get its own line.
    // Some(n) means we should only try to print to n characters then give up.
    lineWidthRemaining: Int):
  // None if we failed to put it on the one line.
  Option[String] = {
    Profiler.frame(() => {
      data match {
        case VonInt(value) => Some(value.toString)
        case VonBool(value) => Some(value.toString)
        case VonFloat(value) => Some(value.toString)
        case VonStr(value) => {
          val escaped = escape(value)
          Some("\"" + escaped + "\"")
        }
        case VonReference(id) => vimpl()
        case o@VonObject(_, _, _) => printObjectSingleLine(o, lineWidthRemaining)
        case a@VonArray(_, _) => printArraySingleLine(a, lineWidthRemaining)
        case VonListMap(id, members) => vimpl()
        case VonMap(id, members) => vimpl()
      }
    })
  }
  */
  /*
  def objectIsEmpty(data: IVonData): Boolean = {
    data match {
      case VonInt(value) => value == 0
      case VonStr(value) => value == ""
      case VonBool(value) => value == false
      case VonFloat(value) => value == 0
      case VonArray(_, members) => members.forall(objectIsEmpty)
      case VonObject(_, _, members) => false
    }
  }
  */
  /*
  def filterMembers(members: Vector[VonMember]): Vector[VonMember] = {
    syntax match {
      case JsonSyntax => members
      case VonSyntax(_, _, _, true) => members
      case VonSyntax(_, _, _, false) => {
        if (members.nonEmpty && objectIsEmpty(members.last.value)) {
          filterMembers(members.init)
        } else {
          members
        }
      }
    }
  }
  */
  /*
  def printObjectSingleLine(
    obbject: VonObject,
    lineWidthRemainingForWholeObject: Int):
    // None if we failed to put it on the one line.
  Option[String] = {
    val VonObject(tyype, None, unfilteredMembers) = obbject

    val members = filterMembers(unfilteredMembers)

    val prefix = printObjectStart(tyype, members.nonEmpty)

    val lineWidthRemainingAfterPrefix = lineWidthRemainingForWholeObject - prefix.length
    if (lineWidthRemainingAfterPrefix <= 0) {
      // identifier took up too much space, bail!
      None
    } else {
      // None means we failed to put it within lineWidthRemaining
      val initialObjectStr: Option[String] = Some(prefix)
      val maybeMembersStr =
        members.zipWithIndex.foldLeft((initialObjectStr))({
          case (None, _) => None
          case (Some(objectStrSoFar), (member, index)) => {
            val lineWidthRemaining = lineWidthRemainingForWholeObject - objectStrSoFar.length
            // If we get here, we're trying to fit things on one line.
            printMemberSingleLine(member, lineWidthRemaining) match {
              case None => None
              case Some(memberStr) => {
                val memberStrMaybeWithComma = memberStr + (if (index == members.size - 1) "" else memberSeparator)
                if (memberStrMaybeWithComma.length > lineWidthRemaining) {
                  None
                } else {
                  Some(objectStrSoFar + memberStrMaybeWithComma)
                }
              }
            }
          }
        })
      maybeMembersStr match {
        case None => None
        case Some(membersStr) => {
          val wholeObjectStr = membersStr + printObjectEnd(members.nonEmpty)
          if (wholeObjectStr.length > lineWidthRemainingForWholeObject) {
            None
          } else {
            Some(wholeObjectStr)
          }
        }
      }
    }
  }
  */
  /*
  def printArraySingleLine(
    array: VonArray,
    lineWidthRemainingForWholeObject: Int):
  // None if we failed to put it on the one line.
  Option[String] = {
    val VonArray(id, members) = array

    val prefix = printArrayStart()

    val lineWidthRemainingAfterPrefix = lineWidthRemainingForWholeObject - prefix.length
    if (lineWidthRemainingAfterPrefix <= 0) {
      // identifier took up too much space, bail!
      None
    } else {
      // None means we failed to put it within lineWidthRemaining
      val initialObjectStr: Option[String] = Some(prefix)
      val maybeMembersStr =
        members.zipWithIndex.foldLeft((initialObjectStr))({
          case (None, _) => None
          case (Some(objectStrSoFar), (member, index)) => {
            val lineWidthRemaining = lineWidthRemainingForWholeObject - objectStrSoFar.length
            // If we get here, we're trying to fit things on one line.
            printSingleLine(member, lineWidthRemaining) match {
              case None => None
              case Some(memberStr) => {
                val memberStrMaybeWithComma = memberStr + (if (index == members.size - 1) "" else memberSeparator)
                if (memberStrMaybeWithComma.length > lineWidthRemaining) {
                  None
                } else {
                  Some(objectStrSoFar + memberStrMaybeWithComma)
                }
              }
            }
          }
        })
      maybeMembersStr match {
        case None => None
        case Some(membersStr) => {
          val wholeObjectStr = membersStr + printArrayEnd()
          if (wholeObjectStr.length > lineWidthRemainingForWholeObject) {
            None
          } else {
            Some(wholeObjectStr)
          }
        }
      }
    }
  }
  */
  /*
  def printMemberPrefix(name: String): String = {
    syntax match {
      case VonSyntax(true, _, _, _) => name + " = "
      case VonSyntax(false, _, _, _) => ""
      case JsonSyntax => "\"" + name + "\": "
    }
  }
  */
  /*
  def printMemberSingleLine(
    member: VonMember,
    // None means it will get its own line.
    // Some(n) means we should only try to print to n characters then give up.
    lineWidthRemaining: Int):
  // None if we failed to put it on the one line.
  Option[String] = {
    val VonMember(fieldName, value) = member

    val identifier = printMemberPrefix(fieldName)

    val lineWidthRemainingForValue = lineWidthRemaining - identifier.length
    if (lineWidthRemainingForValue <= 0) {
      // identifier took up too much space, bail!
      None
    } else {
      printSingleLine(value, lineWidthRemainingForValue) match {
        case None => {
          // We failed to put it in the remaining space, which means the entire member string is too long.
          None
        }
        case Some(valueStr) => {
          val result = identifier + valueStr
          if (result.length >= lineWidthRemaining) {
            None
          } else {
            Some(result)
          }
        }
      }
    }
  }
  */
}

impl Default for VonPrinter {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_object() {
    let printer = VonPrinter::new();
    let obj = IVonData::object(
      "TestType".to_string(),
      vec![
        VonMember::new("field1".to_string(), IVonData::int(42)),
        VonMember::new("field2".to_string(), IVonData::str("hello".to_string())),
      ],
    );

    let json = printer.print(&obj);
    assert!(json.contains("\"__type\": \"TestType\""));
    assert!(json.contains("\"field1\": 42"));
    assert!(json.contains("\"field2\": \"hello\""));
  }

  #[test]
  fn test_array() {
    let printer = VonPrinter::new();
    let arr = IVonData::array(vec![IVonData::int(1), IVonData::int(2), IVonData::int(3)]);

    let json = printer.print(&arr);
    assert!(json.contains("["));
    assert!(json.contains("1"));
    assert!(json.contains("2"));
    assert!(json.contains("3"));
    assert!(json.contains("]"));
  }

  #[test]
  fn test_escape_java_style_sequences() {
    let printer = VonPrinter::new();
    let raw = "\u{0008}\u{000C}\u{001B}\"\n\r\t\\\u{00E9}\u{1F600}";
    assert_eq!(printer.escape(raw), "\\b\\f\\u001B\\\"\\n\\r\\t\\\\\\u00E9\\uD83D\\uDE00");
  }
}

/*
}

  */
