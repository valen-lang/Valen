/*
package dev.vale.solver

import dev.vale.Err
import org.scalatest._

import scala.collection.immutable.Map

*/
// mig: trait IRule
pub trait IRule {
    fn all_runes(&self) -> Vec<i64>;
    fn all_puzzles(&self) -> Vec<Vec<i64>>;
}
/*
sealed trait IRule {
  def allRunes: Vector[Long]
  def allPuzzles: Vector[Vector[Long]]
}
*/
// mig: struct Lookup
pub struct Lookup {
    pub rune: i64,
    pub name: String,
}
// mig: impl Lookup
impl IRule for Lookup {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
case class Lookup(rune: Long, name: String) extends IRule {
  override def allRunes: Vector[Long] = Vector(rune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector())
}
*/
// mig: struct Literal
pub struct Literal {
    pub rune: i64,
    pub value: String,
}
// mig: impl Literal
impl IRule for Literal {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![vec![]]
    }
}
/*
case class Literal(rune: Long, value: String) extends IRule {
  override def allRunes: Vector[Long] = Vector(rune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector())
}
*/
// mig: struct Equals
pub struct Equals {
    pub left_rune: i64,
    pub right_rune: i64,
}
// mig: impl Equals
impl IRule for Equals {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
case class Equals(leftRune: Long, rightRune: Long) extends IRule {
  override def allRunes: Vector[Long] = Vector(leftRune, rightRune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector(leftRune), Vector(rightRune))
}
*/
// mig: struct CoordComponents
pub struct CoordComponents {
    pub coord_rune: i64,
    pub ownership_rune: i64,
    pub kind_rune: i64,
}
// mig: impl CoordComponents
impl IRule for CoordComponents {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
case class CoordComponents(coordRune: Long, ownershipRune: Long, kindRune: Long) extends IRule {
  override def allRunes: Vector[Long] = Vector(coordRune, ownershipRune, kindRune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector(coordRune), Vector(ownershipRune, kindRune))
}
*/
// mig: struct OneOf
pub struct OneOf {
    pub coord_rune: i64,
    pub possible_values: Vec<String>,
}
// mig: impl OneOf
impl IRule for OneOf {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
case class OneOf(coordRune: Long, possibleValues: Vector[String]) extends IRule {
  override def allRunes: Vector[Long] = Vector(coordRune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector(coordRune))
}
*/
// mig: struct Call
pub struct Call {
    pub result_rune: i64,
    pub name_rune: i64,
    pub arg_rune: i64,
}
// mig: impl Call
impl IRule for Call {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
case class Call(resultRune: Long, nameRune: Long, argRune: Long) extends IRule {
  override def allRunes: Vector[Long] = Vector(resultRune, nameRune, argRune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector(resultRune, nameRune), Vector(nameRune, argRune))
}
*/
// mig: struct Send
pub struct Send {
    pub sender_rune: i64,
    pub receiver_rune: i64,
}
// mig: impl Send
impl IRule for Send {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
// See IRFU and SRCAMP for what this rule is doing
case class Send(senderRune: Long, receiverRune: Long) extends IRule {
  override def allRunes: Vector[Long] = Vector(receiverRune, senderRune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector(receiverRune))
}
*/
// mig: struct Implements
pub struct Implements {
    pub sub_rune: i64,
    pub super_rune: i64,
}
// mig: impl Implements
impl IRule for Implements {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
case class Implements(subRune: Long, superRune: Long) extends IRule {
  override def allRunes: Vector[Long] = Vector(subRune, superRune)
  override def allPuzzles: Vector[Vector[Long]] = Vector(Vector(subRune, superRune))
}
*/
// mig: struct Pack
pub struct Pack {
    pub result_rune: i64,
    pub member_runes: Vec<i64>,
}
// mig: impl Pack
impl IRule for Pack {
    fn all_runes(&self) -> Vec<i64> {
        panic!("Unimplemented: all_runes");
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        panic!("Unimplemented: all_puzzles");
    }
}
/*
case class Pack(resultRune: Long, memberRunes: Vector[Long]) extends IRule {
  override def allRunes: Vector[Long] = Vector(resultRune) ++ memberRunes
  override def allPuzzles: Vector[Vector[Long]] = {
    if (memberRunes.isEmpty) {
      Vector(Vector(resultRune))
    } else {
      Vector(Vector(resultRune), memberRunes)
    }
  }
}
*/
