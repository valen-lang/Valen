/*
package dev.vale.parsing

import dev.vale.lexing.{Lexer, LexingIterator}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.ConstantStrPE
import dev.vale.{Collector, Interner, Keywords, vassert, vimpl}
import net.liftweb.json._
import dev.vale.parsing.ast.ConstantStrPE
import dev.vale.von.{JsonSyntax, VonPrinter}
import org.scalatest._

import java.nio.charset.Charset


class LoadTests extends FunSuite with Matchers with Collector with TestParseUtils {
*/

use bumpalo::Bump;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::lexing_iterator::LexingIterator;
use crate::lexing::lexer::Lexer;
use crate::parsing::parsed_loader;
use crate::parsing::tests::utils::compile_file;
use crate::parsing::vonifier::ParserVonifier;
use crate::von::printer::VonPrinter;

#[test]
fn simple_program() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let original_file = compile_file(&interner, &keywords, "exported func main() int { return 42; }").unwrap();
  let von = ParserVonifier::vonify_file(&original_file);
  let json = VonPrinter::new().print(&von);
  let loaded_file = parsed_loader::load(&interner, &json).unwrap();
  // This is because we don't want to enable .equals, see EHCFBD.
  assert_eq!(format!("{:?}", original_file), format!("{:?}", loaded_file));
}
/*
  test("Simple program") {
    val interner = new Interner()
    val originalFile = compileFile("""exported func main() int { return 42; }""").getOrDie()
    val von = ParserVonifier.vonifyFile(originalFile)
    val json = new VonPrinter(JsonSyntax, 120).print(von)
    val loadedFile = new ParsedLoader(interner).load(json).getOrDie()
    // This is because we don't want to enable .equals, see EHCFBD.
    originalFile.toString == loadedFile.toString
  }
*/

#[test]
fn strings_with_special_characters() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let lexer = Lexer::new(&interner, &keywords);
  let mut iter = LexingIterator::new("000a".to_string());
  assert_eq!(lexer.parse_four_digit_hex_num(&mut iter, 0), Some(10));

  let code = "exported func main() str { \"hello\\u001bworld\" }";
  // FALL NOT TO TEMPTATION
  // Scala has some issues here.
  // The above "\"\\u001b\"" seems like it could be expressed """"\\u001b"""" but it can't.
  // Nothing seems to work:
  // - vassert("\"\\u001b\"" == """"\u001b"""") fails
  // - vassert("\"\\u001b\"" == """"\\u001b"""") fails
  // - vassert("\"\\u001b\"" == """\"\\u001b\"""") fails
  // This took quite a while to figure out.
  // So, just stick with regular scala string literals, scala's good with those.
  // Other tests have this, search TEMPTATION.
  // NOW GO YE AND PROSPER

  // This assert makes sure the above is making the input we actually intend.
  // Real source files from disk are going to have a backslash character and then a u,
  // they won't have the 0x1b byte.
  assert!(code.contains("\\u001b"));

  let original_file = compile_file(&interner, &keywords, code).unwrap();
  let von = ParserVonifier::vonify_file(&original_file);
  let generated_json_str = VonPrinter::new().print(&von);
  let generated_bytes = generated_json_str.as_bytes();
  let loaded_json_str = String::from_utf8(generated_bytes.to_vec()).unwrap();
  let loaded_file = parsed_loader::load(&interner, &loaded_json_str).unwrap();
  // This is because we don't want to enable .equals, see EHCFBD.
  assert_eq!(format!("{:?}", original_file), format!("{:?}", loaded_file));
}
/*
  test("Strings with special characters") {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val lexer = new Lexer(interner, keywords)
    lexer.parseFourDigitHexNum(new LexingIterator("000a", 0)) shouldEqual Some(10)

    val code = "exported func main() str { \"hello\\u001bworld\" }"
    // FALL NOT TO TEMPTATION
    // Scala has some issues here.
    // The above "\"\\u001b\"" seems like it could be expressed """"\\u001b"""" but it can't.
    // Nothing seems to work:
    // - vassert("\"\\u001b\"" == """"\u001b"""") fails
    // - vassert("\"\\u001b\"" == """"\\u001b"""") fails
    // - vassert("\"\\u001b\"" == """\"\\u001b\"""") fails
    // This took quite a while to figure out.
    // So, just stick with regular scala string literals, scala's good with those.
    // Other tests have this, search TEMPTATION.
    // NOW GO YE AND PROSPER

    // This assert makes sure the above is making the input we actually intend.
    // Real source files from disk are going to have a backslash character and then a u,
    // they won't have the 0x1b byte.
    vassert(code.contains("\\u001b"))

    val expr = compileExpressionExpect(code)
    val von = ParserVonifier.vonifyExpression(expr)
    val generatedJsonStr = new VonPrinter(JsonSyntax, 120).print(von)
//    vassert(generatedJsonStr.contains("hello\u001bworld") || generatedJsonStr.contains("hello\u001Bworld"))
//    vassert(!generatedJsonStr.contains("hello\\\\u"))
    val generatedBytes = generatedJsonStr.getBytes(Charset.forName("UTF-8"))

    val loader = new ParsedLoader(interner)
    val loadedJsonStr = new String(generatedBytes, "UTF-8");
    val jnode = parse(loadedJsonStr)
    val jobj = loader.expectObject(jnode)
    val loadedExpr = loader.loadExpression(jobj)
    // This is because we don't want to enable .equals, see EHCFBD.
    expr.toString shouldEqual loadedExpr.toString
  }
}
*/