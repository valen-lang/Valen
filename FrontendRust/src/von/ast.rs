/*
*/

/// Von data types - intermediate representation for JSON serialization
/// Matches Scala's IVonData

#[derive(Clone, Debug, PartialEq)]
pub enum IVonData {
  Int(VonInt),
  Float(VonFloat),
  Bool(VonBool),
  Str(VonStr),
  Object(VonObject),
  Array(VonArray),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VonInt {
  pub value: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VonFloat {
  pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VonBool {
  pub value: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VonStr {
  pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VonObject {
  pub tyype: String,
  pub id: Option<String>,
  pub members: Vec<VonMember>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VonMember {
  pub field_name: String,
  pub value: IVonData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VonArray {
  pub id: Option<String>,
  pub members: Vec<IVonData>,
}

impl IVonData {
  pub fn int(value: i64) -> Self {
    IVonData::Int(VonInt { value })
  }

  pub fn float(value: f64) -> Self {
    IVonData::Float(VonFloat { value })
  }

  pub fn bool(value: bool) -> Self {
    IVonData::Bool(VonBool { value })
  }

  pub fn str(value: String) -> Self {
    IVonData::Str(VonStr { value })
  }

  pub fn object(tyype: String, members: Vec<VonMember>) -> Self {
    IVonData::Object(VonObject {
      tyype,
      id: None,
      members,
    })
  }

  pub fn array(members: Vec<IVonData>) -> Self {
    IVonData::Array(VonArray { id: None, members })
  }
}

impl VonMember {
  pub fn new(field_name: String, value: IVonData) -> Self {
    VonMember { field_name, value }
  }
}

/*
package dev.vale.von

import dev.vale.vcurious
import dev.vale.vimpl

sealed trait IVonData
*/
/*
case class VonInt(value: Long) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*
case class VonFloat(value: Double) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*
case class VonBool(value: Boolean) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*
case class VonStr(value: String) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*

case class VonReference(id: String) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*

case class VonObject(
  tyype: String,
  id: Option[String],
  members: Vector[VonMember]
) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
case class VonMember(fieldName: String, value: IVonData) { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*
case class VonArray(
  id: Option[String],
  members: Vector[IVonData]
) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*
case class VonListMap(
  id: Option[String],
  members: Vector[VonMapEntry]
) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*
case class VonMap(
  id: Option[String],
  members: Vector[VonMapEntry]
) extends IVonData { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
/*
case class VonMapEntry(
  key: IVonData,
  value: IVonData) { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious(); }
*/
