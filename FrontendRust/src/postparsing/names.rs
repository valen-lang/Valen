use crate::interner::StrI;
use crate::scout_arena::ScoutArena;
use crate::postparsing::ast::{LocationInDenizen, LocationInDenizenVal};
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use IRuneValS::*;
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr::eq;
/*
package dev.vale.postparsing

import dev.vale.{CodeLocationS, IInterning, Interner, PackageCoordinate, RangeS, StrI, vassert, vcheck, vimpl, vpass}
*/

/// Canonical interned name. Storage uses arena-backed refs; use `ptr_eq` for identity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum INameS<'s> {
  FunctionDeclaration(&'s IFunctionDeclarationNameS<'s>),
  ImplDeclaration(&'s ImplDeclarationNameS<'s>),
  AnonymousSubstructImplDeclaration(&'s AnonymousSubstructImplDeclarationNameS<'s>),
  ExportAsName(&'s ExportAsNameS<'s>),
  LetName(&'s LetNameS<'s>),
  TopLevelStructDeclaration(&'s TopLevelStructDeclarationNameS<'s>),
  TopLevelInterfaceDeclaration(&'s TopLevelInterfaceDeclarationNameS<'s>),
  LambdaStructDeclaration(&'s LambdaStructDeclarationNameS<'s>),
  AnonymousSubstructTemplateName(&'s AnonymousSubstructTemplateNameS<'s>),
  RuneName(&'s RuneNameS<'s>),
  RuntimeSizedArrayDeclarationName(&'s RuntimeSizedArrayDeclarationNameS),
  StaticSizedArrayDeclarationName(&'s StaticSizedArrayDeclarationNameS),
  GlobalFunctionFamilyName(&'s GlobalFunctionFamilyNameS<'s>),
  ArbitraryName(&'s ArbitraryNameS),
  VarName(&'s IVarNameS<'s>),
}
/*
trait INameS extends IInterning
*/

impl<'s> INameS<'s> {
  /// Pointer to the canonical interned payload.
  pub fn canonical_ptr(&self) -> *const () {
    match self {
      INameS::FunctionDeclaration(r) => *r as *const _ as *const (),
      INameS::ImplDeclaration(r) => *r as *const _ as *const (),
      INameS::AnonymousSubstructImplDeclaration(r) => *r as *const _ as *const (),
      INameS::ExportAsName(r) => *r as *const _ as *const (),
      INameS::LetName(r) => *r as *const _ as *const (),
      INameS::TopLevelStructDeclaration(r) => *r as *const _ as *const (),
      INameS::TopLevelInterfaceDeclaration(r) => *r as *const _ as *const (),
      INameS::LambdaStructDeclaration(r) => *r as *const _ as *const (),
      INameS::AnonymousSubstructTemplateName(r) => *r as *const _ as *const (),
      INameS::RuneName(r) => *r as *const _ as *const (),
      INameS::RuntimeSizedArrayDeclarationName(r) => *r as *const _ as *const (),
      INameS::StaticSizedArrayDeclarationName(r) => *r as *const _ as *const (),
      INameS::GlobalFunctionFamilyName(r) => *r as *const _ as *const (),
      INameS::ArbitraryName(r) => *r as *const _ as *const (),
      INameS::VarName(r) => *r as *const _ as *const (),
    }
  }
  /* Guardian: disable-all */

  /// Returns true iff both refer to the same canonical interned value.
  #[inline(always)]
  pub fn ptr_eq(&self, other: &INameS<'s>) -> bool {
    eq(self.canonical_ptr(), other.canonical_ptr())
  }
  /* Guardian: disable-all */

  pub fn as_top_level_citizen_name(&self) -> Option<TopLevelCitizenDeclarationNameS<'s>> {
    match self {
      INameS::TopLevelStructDeclaration(s) => Some(TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName((*s).clone())),
      INameS::TopLevelInterfaceDeclaration(i) => Some(TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName((*i).clone())),
      _ => None,
    }
  }
  /* Guardian: disable-all */
}
/*
Guardian: disable-all
*/

/// Value/key form for interner lookups. Shallow Val structs reference canonical INameS/IFunctionDeclarationNameS/etc.
/// Per @DSAUIMZ, if a variant gains a slice field, add a 'tmp lifetime and use a transient ValS struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum INameValS<'s> {
  FunctionDeclaration(IFunctionDeclarationNameValS<'s>),
  ImplDeclaration(ImplDeclarationNameS<'s>),
  AnonymousSubstructImplDeclaration(AnonymousSubstructImplDeclarationNameValS<'s>),
  ExportAsName(ExportAsNameS<'s>),
  LetName(LetNameS<'s>),
  TopLevelStructDeclaration(TopLevelStructDeclarationNameS<'s>),
  TopLevelInterfaceDeclaration(TopLevelInterfaceDeclarationNameS<'s>),
  LambdaStructDeclaration(LambdaStructDeclarationNameS<'s>),
  AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameValS<'s>),
  RuneName(RuneNameValS<'s>),
  RuntimeSizedArrayDeclarationName(RuntimeSizedArrayDeclarationNameS),
  StaticSizedArrayDeclarationName(StaticSizedArrayDeclarationNameS),
  GlobalFunctionFamilyName(GlobalFunctionFamilyNameS<'s>),
  ArbitraryName(ArbitraryNameS),
  VarName(IVarNameValS<'s>),
}
/* Guardian: disable-all */

/// Shallow: inner already canonical.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructImplDeclarationNameValS<'s> {
  pub interface: &'s TopLevelInterfaceDeclarationNameS<'s>,
}
/* Guardian: disable-all */

/// Shallow: interface_name already canonical.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateNameValS<'s> {
  pub interface_name: &'s TopLevelInterfaceDeclarationNameS<'s>,
}
/* Guardian: disable-all */

// AFTERM: Add arcana for how these sometimes contain INameS even though
// INameS arent interned. Should be fine, but worth looking out for.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImpreciseNameS<'s> {
  CodeName(&'s CodeNameS<'s>),
  IterableName(&'s IterableNameS<'s>),
  IteratorName(&'s IteratorNameS<'s>),
  IterationOptionName(&'s IterationOptionNameS<'s>),
  LambdaImpreciseName(&'s LambdaImpreciseNameS),
  PlaceholderImpreciseName(&'s PlaceholderImpreciseNameS),
  LambdaStructImpreciseName(&'s LambdaStructImpreciseNameS<'s>),
  ClosureParamImpreciseName(&'s ClosureParamImpreciseNameS),
  PrototypeName(&'s PrototypeNameS),
  AnonymousSubstructTemplateImpreciseName(&'s AnonymousSubstructTemplateImpreciseNameS<'s>),
  AnonymousSubstructConstructorTemplateImpreciseName(
    &'s AnonymousSubstructConstructorTemplateImpreciseNameS<'s>,
  ),
  ImplImpreciseName(&'s ImplImpreciseNameS<'s>),
  ImplSubCitizenImpreciseName(&'s ImplSubCitizenImpreciseNameS<'s>),
  ImplSuperInterfaceImpreciseName(&'s ImplSuperInterfaceImpreciseNameS<'s>),
  SelfName(&'s SelfNameS),
  RuneName(&'s RuneNameS<'s>),
  ArbitraryName(&'s ArbitraryNameS),
}
/*
trait IImpreciseNameS extends IInterning
*/

impl<'s> IImpreciseNameS<'s> {
  /// Pointer to the canonical interned payload. Use `std::ptr::eq(a.canonical_ptr(), b.canonical_ptr())` for identity comparison.
  pub fn canonical_ptr(&self) -> *const () {
    match self {
      IImpreciseNameS::CodeName(r) => *r as *const _ as *const (),
      IImpreciseNameS::IterableName(r) => *r as *const _ as *const (),
      IImpreciseNameS::IteratorName(r) => *r as *const _ as *const (),
      IImpreciseNameS::IterationOptionName(r) => *r as *const _ as *const (),
      IImpreciseNameS::LambdaImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::PlaceholderImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::LambdaStructImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ClosureParamImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::PrototypeName(r) => *r as *const _ as *const (),
      IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::AnonymousSubstructConstructorTemplateImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplSubCitizenImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplSuperInterfaceImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::SelfName(r) => *r as *const _ as *const (),
      IImpreciseNameS::RuneName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ArbitraryName(r) => *r as *const _ as *const (),
    }
  }
  /* Guardian: disable-all */

  /// Returns true iff both refer to the same canonical interned value.
  #[inline(always)]
  pub fn ptr_eq(&self, other: &IImpreciseNameS<'s>) -> bool {
    eq(self.canonical_ptr(), other.canonical_ptr())
  }
  /* Guardian: disable-all */
}
/*
Guardian: disable-all
*/

/// Value-struct for LambdaStructImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructImpreciseNameValS<'s> {
  pub lambda_name: IImpreciseNameS<'s>,
}
/* Guardian: disable-all */

/// Value-struct for AnonymousSubstructTemplateImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateImpreciseNameValS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
}
/* Guardian: disable-all */

/// Value-struct for AnonymousSubstructConstructorTemplateImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructConstructorTemplateImpreciseNameValS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
}
/* Guardian: disable-all */

/// Value-struct for ImplImpreciseNameS key. Shallow: references canonical children.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplImpreciseNameValS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
}
/* Guardian: disable-all */

/// Value-struct for ImplSubCitizenImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSubCitizenImpreciseNameValS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
}
/* Guardian: disable-all */

/// Value-struct for ImplSuperInterfaceImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSuperInterfaceImpreciseNameValS<'s> {
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
}
/* Guardian: disable-all */

/// Value-struct for RuneNameS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneNameValS<'s> {
  pub rune: IRuneS<'s>,
}
/* Guardian: disable-all */

/// Value/key form of imprecise name for interner lookups. Storage uses canonical `IImpreciseNameS<'s>`.
/// Per @DSAUIMZ, if a variant gains a slice field, add a 'tmp lifetime and use a transient ValS struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImpreciseNameValS<'s> {
  CodeName(CodeNameS<'s>),
  IterableName(IterableNameS<'s>),
  IteratorName(IteratorNameS<'s>),
  IterationOptionName(IterationOptionNameS<'s>),
  LambdaImpreciseName(LambdaImpreciseNameS),
  PlaceholderImpreciseName(PlaceholderImpreciseNameS),
  LambdaStructImpreciseName(LambdaStructImpreciseNameValS<'s>),
  ClosureParamImpreciseName(ClosureParamImpreciseNameS),
  PrototypeName(PrototypeNameS),
  AnonymousSubstructTemplateImpreciseName(AnonymousSubstructTemplateImpreciseNameValS<'s>),
  AnonymousSubstructConstructorTemplateImpreciseName(
    AnonymousSubstructConstructorTemplateImpreciseNameValS<'s>,
  ),
  ImplImpreciseName(ImplImpreciseNameValS<'s>),
  ImplSubCitizenImpreciseName(ImplSubCitizenImpreciseNameValS<'s>),
  ImplSuperInterfaceImpreciseName(ImplSuperInterfaceImpreciseNameValS<'s>),
  SelfName(SelfNameS),
  RuneName(RuneNameValS<'s>),
  ArbitraryName(ArbitraryNameS),
}
/* Guardian: disable-all */


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IVarNameS<'s> {
  CodeVarName(StrI<'s>),
  ConstructingMemberName(StrI<'s>),
  ClosureParamName(&'s ClosureParamNameS<'s>),
  MagicParamName(CodeLocationS<'s>),
  IterableName(RangeS<'s>),
  IteratorName(RangeS<'s>),
  IterationOptionName(RangeS<'s>),
  WhileCondResultName(RangeS<'s>),
  SelfName,
  AnonymousSubstructMemberName(i32),
}
/*
sealed trait IVarNameS extends INameS
*/

/// Value form for interner lookups.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IVarNameValS<'s> {
  CodeVarName(StrI<'s>),
  ConstructingMemberName(StrI<'s>),
  ClosureParamName(ClosureParamNameS<'s>),
  MagicParamName(CodeLocationS<'s>),
  IterableName(RangeS<'s>),
  IteratorName(RangeS<'s>),
  IterationOptionName(RangeS<'s>),
  WhileCondResultName(RangeS<'s>),
  SelfName,
  AnonymousSubstructMemberName(i32),
}
/* Guardian: disable-all */

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IFunctionDeclarationNameS<'s> {
  FunctionName(FunctionNameS<'s>),
  LambdaDeclarationName(LambdaDeclarationNameS<'s>),
  ForwarderFunctionDeclarationName(&'s ForwarderFunctionDeclarationNameS<'s>),
  ConstructorName(&'s ConstructorNameS<'s>),
  ImmConcreteDestructorName(&'s ImmConcreteDestructorNameS<'s>),
  ImmInterfaceDestructorName(&'s ImmInterfaceDestructorNameS<'s>),
}
/*
trait IFunctionDeclarationNameS extends INameS {
  def packageCoordinate: PackageCoordinate
  def getImpreciseName(interner: Interner): IImpreciseNameS
}
*/


/// Value form for interner lookups. Shallow variant holds canonical IFunctionDeclarationNameS.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IFunctionDeclarationNameValS<'s> {
  FunctionName(FunctionNameS<'s>),
  LambdaDeclarationName(LambdaDeclarationNameS<'s>),
  ForwarderFunctionDeclarationName(ForwarderFunctionDeclarationNameValS<'s>),
  ConstructorName(ConstructorNameS<'s>),
  ImmConcreteDestructorName(ImmConcreteDestructorNameS<'s>),
  ImmInterfaceDestructorName(ImmInterfaceDestructorNameS<'s>),
}
/* Guardian: disable-all */

/// Shallow: inner already canonical.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwarderFunctionDeclarationNameValS<'s> {
  pub inner: IFunctionDeclarationNameS<'s>,
  pub index: i32,
}
/* Guardian: disable-all */

impl<'s> IFunctionDeclarationNameS<'s> {
  pub fn package_coordinate(&self) -> &'s PackageCoordinate<'s> {
    match self {
      IFunctionDeclarationNameS::FunctionName(x) => x.code_location.file.package_coord,
      IFunctionDeclarationNameS::LambdaDeclarationName(x) => x.code_location.file.package_coord,
      IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(r) => r.inner.package_coordinate(),
      IFunctionDeclarationNameS::ConstructorName(r) => {
        match &r.tlcd {
          ICitizenDeclarationNameS::TopLevelStructDeclarationName(s) => s.range.begin.file.package_coord,
          ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(i) => i.range.begin.file.package_coord,
          ICitizenDeclarationNameS::AnonymousSubstructTemplateName(n) => n.interface_name.range.begin.file.package_coord,
        }
      }
      IFunctionDeclarationNameS::ImmConcreteDestructorName(r) => &r.package_coordinate,
      IFunctionDeclarationNameS::ImmInterfaceDestructorName(r) => &r.package_coordinate,
    }
  }

  /// Convert to value form for interning. Clones through refs.
  pub fn to_val(&self) -> IFunctionDeclarationNameValS<'s> {
    match self {
      IFunctionDeclarationNameS::FunctionName(x) => {
        IFunctionDeclarationNameValS::FunctionName(x.clone())
      }
      IFunctionDeclarationNameS::LambdaDeclarationName(x) => {
        IFunctionDeclarationNameValS::LambdaDeclarationName(x.clone())
      }
      IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(r) => {
        IFunctionDeclarationNameValS::ForwarderFunctionDeclarationName(ForwarderFunctionDeclarationNameValS {
          inner: r.inner.clone(),
          index: r.index,
        })
      }
      IFunctionDeclarationNameS::ConstructorName(r) => {
        IFunctionDeclarationNameValS::ConstructorName((*r).clone())
      }
      IFunctionDeclarationNameS::ImmConcreteDestructorName(r) => {
        IFunctionDeclarationNameValS::ImmConcreteDestructorName((*r).clone())
      }
      IFunctionDeclarationNameS::ImmInterfaceDestructorName(r) => {
        IFunctionDeclarationNameValS::ImmInterfaceDestructorName((*r).clone())
      }
    }
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImplDeclarationNameS<'s> {
  ImplDeclarationName(ImplDeclarationNameS<'s>),
  AnonymousSubstructImplDeclarationName(AnonymousSubstructImplDeclarationNameS<'s>),
}
/*
trait IImplDeclarationNameS extends INameS {
  def packageCoordinate: PackageCoordinate
}
*/
impl<'s> IImplDeclarationNameS<'s> {
  pub fn package_coordinate(&self) -> &'s PackageCoordinate<'s> {
    match self {
      IImplDeclarationNameS::ImplDeclarationName(x) => x.code_location.file.package_coord,
      IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(x) => x.interface.range.begin.file.package_coord,
    }
  }

  // Rust adaptation: Scala's `IImplDeclarationNameS extends INameS` allows implicit
  // subtype upcast; Rust restates it as an explicit re-intern through the scout arena
  // so the caller can hand the value to `INameS`-typed APIs (e.g. `translateNameStep`).
  // Inverse of `INameS::as_top_level_citizen_name` (which narrows the other direction).
  pub fn to_i_name_s(self, scout_arena: &ScoutArena<'s>) -> INameS<'s> {
    match self {
      IImplDeclarationNameS::ImplDeclarationName(p) => {
        scout_arena.intern_name(INameValS::ImplDeclaration(p))
      }
      IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(p) => {
        let interface_ref = match scout_arena.intern_name(
          INameValS::TopLevelInterfaceDeclaration(p.interface)
        ) {
          INameS::TopLevelInterfaceDeclaration(r) => r,
          _ => unreachable!(),
        };
        scout_arena.intern_name(INameValS::AnonymousSubstructImplDeclaration(
          AnonymousSubstructImplDeclarationNameValS { interface: interface_ref }
        ))
      }
    }
  }
}
/* Guardian: disable-all */

/*
trait ICitizenDeclarationNameS extends INameS {
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ICitizenDeclarationNameS<'s> {
  TopLevelStructDeclarationName(TopLevelStructDeclarationNameS<'s>),
  TopLevelInterfaceDeclarationName(TopLevelInterfaceDeclarationNameS<'s>),
  AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameS<'s>),
}
// Rust adaptation: Scala's `ICitizenDeclarationNameS` is a parent trait covering both
// `TopLevelCitizenDeclarationNameS` subtypes and `AnonymousSubstructTemplateNameS`.
// Rust restates it as an explicit enum so fields like `ConstructorNameS.tlcd` and the
// citizen rune carriers can hold any citizen-shaped name.
impl<'s> From<TopLevelCitizenDeclarationNameS<'s>> for ICitizenDeclarationNameS<'s> {
  fn from(value: TopLevelCitizenDeclarationNameS<'s>) -> Self {
    match value {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(n) =>
        ICitizenDeclarationNameS::TopLevelStructDeclarationName(n),
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(n) =>
        ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(n),
    }
  }
}
/* Guardian: disable-all */

impl<'s> From<IStructDeclarationNameS<'s>> for ICitizenDeclarationNameS<'s> {
  fn from(value: IStructDeclarationNameS<'s>) -> Self {
    match value {
      IStructDeclarationNameS::TopLevelStructDeclarationName(n) =>
        ICitizenDeclarationNameS::TopLevelStructDeclarationName(n),
      IStructDeclarationNameS::AnonymousSubstructTemplateName(n) =>
        ICitizenDeclarationNameS::AnonymousSubstructTemplateName(n),
    }
  }
}
/* Guardian: disable-all */

impl<'s> IStructDeclarationNameS<'s> {
  pub fn range(&self) -> RangeS<'s> {
    match self {
      IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n.range,
      IStructDeclarationNameS::AnonymousSubstructTemplateName(n) => n.interface_name.range,
    }
  }
  /*
  def range: RangeS
  */
  /* Guardian: disable-all */
/*
  def packageCoordinate: PackageCoordinate
*/
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    match self {
      IStructDeclarationNameS::TopLevelStructDeclarationName(n) => {
        scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: n.name }))
      }
      IStructDeclarationNameS::AnonymousSubstructTemplateName(n) => {
        let interface_imprecise_name = n.interface_name.get_imprecise_name(scout_arena);
        scout_arena.intern_imprecise_name(IImpreciseNameValS::AnonymousSubstructTemplateImpreciseName(AnonymousSubstructTemplateImpreciseNameValS { interface_imprecise_name }))
      }
    }
  }
  /*
  def getImpreciseName(interner: Interner): IImpreciseNameS
  */
  /* Guardian: disable-all */
}
/*
}
*/
/*
//case class FreeDeclarationNameS(codeLocationS: CodeLocationS) extends IFunctionDeclarationNameS {
//
//  override def packageCoordinate: PackageCoordinate = codeLocationS.file.packageCoordinate
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(FreeImpreciseNameS())
//}
//case class FreeImpreciseNameS() extends IImpreciseNameS {
//
//}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaDeclarationNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}
/*
case class LambdaDeclarationNameS(
//  parentName: INameS,
  codeLocation: CodeLocationS
) extends IFunctionDeclarationNameS {
*/
impl<'s> LambdaDeclarationNameS<'s> {
/*
  override def packageCoordinate: PackageCoordinate = codeLocation.file.packageCoordinate
*/
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    scout_arena.intern_imprecise_name(IImpreciseNameValS::LambdaImpreciseName(LambdaImpreciseNameS {}))
  }
/*
  override def getImpreciseName(interner: Interner): LambdaImpreciseNameS = interner.intern(LambdaImpreciseNameS())
*/
/*
}
*/
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaImpreciseNameS {}
/*
case class LambdaImpreciseNameS() extends IImpreciseNameS {
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlaceholderImpreciseNameS {
  pub index: i32,
}
/*
case class PlaceholderImpreciseNameS(index: Int) extends IImpreciseNameS {
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionNameS<'s> {
  pub name: StrI<'s>,
  pub code_location: CodeLocationS<'s>,
}

/*
case class FunctionNameS(name: StrI, codeLocation: CodeLocationS) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = codeLocation.file.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(name))
}
//case class AbstractVirtualDropFunctionDeclarationNameS(interfaceName: TopLevelCitizenDeclarationNameS) extends IFunctionDeclarationNameS {
//  override def packageCoordinate: PackageCoordinate = interfaceName.packageCoord
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME))
//}
//case class OverrideVirtualDropFunctionDeclarationNameS(implName: IImplDeclarationNameS) extends IFunctionDeclarationNameS {
//  override def packageCoordinate: PackageCoordinate = implName.packageCoord
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME))
//}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwarderFunctionDeclarationNameS<'s> {
  pub inner: IFunctionDeclarationNameS<'s>,
  pub index: i32,
}
/*
case class ForwarderFunctionDeclarationNameS(inner: IFunctionDeclarationNameS, index: Int) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = inner.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = inner.getImpreciseName(interner)
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TopLevelCitizenDeclarationNameS<'s> {
  TopLevelStructDeclarationName(TopLevelStructDeclarationNameS<'s>),
  TopLevelInterfaceDeclarationName(TopLevelInterfaceDeclarationNameS<'s>),
}

/*
sealed trait TopLevelCitizenDeclarationNameS extends ICitizenDeclarationNameS {
*/
impl<'s> TopLevelCitizenDeclarationNameS<'s> {
  pub fn name(&self) -> StrI<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => x.name,
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => x.name,
    }
  }
  /*
  def name: StrI
  */
  /* Guardian: disable-all */
  pub fn range(&self) -> RangeS<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => x.range,
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => x.range,
    }
  }
  /*
  def range: RangeS
  */
  /* Guardian: disable-all */
  pub fn package_coordinate(&self) -> &'s PackageCoordinate<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => x.range.begin.file.package_coord,
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => x.range.begin.file.package_coord,
    }
  }
  /*
  override def packageCoordinate: PackageCoordinate = range.file.packageCoordinate
  */
  /* Guardian: disable-all */
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => {
        scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: x.name }))
      }
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => {
        scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: x.name }))
      }
    }
  }
  /*
  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(name))
  */
  /* Guardian: disable-all */
}
/*
}
*/
/*
object TopLevelCitizenDeclarationNameS {
  def unapply(n: TopLevelCitizenDeclarationNameS): Option[(StrI, RangeS)] = {
    Some((n.name, n.range))
  }
}
*/
/*
sealed trait IStructDeclarationNameS extends ICitizenDeclarationNameS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IStructDeclarationNameS<'s> {
  TopLevelStructDeclarationName(TopLevelStructDeclarationNameS<'s>),
  AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameS<'s>),
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TopLevelStructDeclarationNameS<'s> {
  pub name: StrI<'s>,
  pub range: RangeS<'s>,
}
/*
case class TopLevelStructDeclarationNameS(name: StrI, range: RangeS) extends IStructDeclarationNameS with TopLevelCitizenDeclarationNameS {
}
*/
/*
sealed trait IInterfaceDeclarationNameS extends ICitizenDeclarationNameS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TopLevelInterfaceDeclarationNameS<'s> {
  pub name: StrI<'s>,
  pub range: RangeS<'s>,
}
/*
case class TopLevelInterfaceDeclarationNameS(name: StrI, range: RangeS) extends IInterfaceDeclarationNameS with TopLevelCitizenDeclarationNameS {
}
*/
// Rust adaptation: Scala inherits `getImpreciseName` from `TopLevelCitizenDeclarationNameS`
// (`= interner.intern(CodeNameS(name))`). Rust doesn't auto-inherit across the
// struct/enum split, so we restate the dispatch on the child struct.
impl<'s> TopLevelInterfaceDeclarationNameS<'s> {
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: self.name }))
  }
}
/* Guardian: disable-all */
impl<'s> From<&TopLevelStructDeclarationNameS<'s>> for TopLevelCitizenDeclarationNameS<'s> {
  fn from(value: &TopLevelStructDeclarationNameS<'s>) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(value.clone())
  }
}
/*
Guardian: disable-all
*/

impl<'s> From<&TopLevelInterfaceDeclarationNameS<'s>> for TopLevelCitizenDeclarationNameS<'s> {
  fn from(value: &TopLevelInterfaceDeclarationNameS<'s>) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(value.clone())
  }
}
/*
Guardian: disable-all
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructDeclarationNameS<'s> {
  pub lambda_name: LambdaDeclarationNameS<'s>,
}
/*
case class LambdaStructDeclarationNameS(lambdaName: LambdaDeclarationNameS) extends INameS {
*/
impl<'s> LambdaStructDeclarationNameS<'s> {
/*
  def getImpreciseName(interner: Interner): LambdaStructImpreciseNameS = interner.intern(LambdaStructImpreciseNameS(lambdaName.getImpreciseName(interner)))
*/
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    let lambda_imprecise_name = self.lambda_name.get_imprecise_name(scout_arena);
    scout_arena.intern_imprecise_name(IImpreciseNameValS::LambdaStructImpreciseName(
      LambdaStructImpreciseNameValS {
        lambda_name: lambda_imprecise_name,
      },
    ))
  }
}
/*
}
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructImpreciseNameS<'s> {
  pub lambda_name: IImpreciseNameS<'s>,
}
/*
case class LambdaStructImpreciseNameS(lambdaName: LambdaImpreciseNameS) extends IImpreciseNameS {  }
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDeclarationNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}
/*
case class ImplDeclarationNameS(codeLocation: CodeLocationS) extends IImplDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = codeLocation.file.packageCoordinate
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructImplDeclarationNameS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructImplDeclarationNameS(interface: TopLevelInterfaceDeclarationNameS) extends IImplDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = interface.packageCoordinate
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportAsNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}

/*
case class ExportAsNameS(codeLocation: CodeLocationS) extends INameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}
/*
case class LetNameS(codeLocation: CodeLocationS) extends INameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}
/*
case class ClosureParamNameS(codeLocation: CodeLocationS) extends IVarNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamImpreciseNameS {}
/*
case class ClosureParamImpreciseNameS() extends IImpreciseNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeNameS {}
/*
// All prototypes can be looked up via this name.
case class PrototypeNameS() extends IImpreciseNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}
/*
case class MagicParamNameS(codeLocation: CodeLocationS) extends IVarNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateNameS<'s> {
  pub interface_name: TopLevelInterfaceDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructTemplateNameS(interfaceName: TopLevelInterfaceDeclarationNameS) extends IStructDeclarationNameS {
  vpass()
  override def packageCoordinate: PackageCoordinate = interfaceName.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(AnonymousSubstructTemplateImpreciseNameS(interfaceName.getImpreciseName(interner)))
  override def range: RangeS = interfaceName.range
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateImpreciseNameS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
}
/*
case class AnonymousSubstructTemplateImpreciseNameS(interfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS {

}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructConstructorTemplateImpreciseNameS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
}
/*
case class AnonymousSubstructConstructorTemplateImpreciseNameS(interfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS {

}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberNameS {
  pub index: i32,
}
/*
case class AnonymousSubstructMemberNameS(index: Int) extends IVarNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeVarNameS<'s> {
  pub name: StrI<'s>,
}
/*
case class CodeVarNameS(name: StrI) extends IVarNameS {
  vcheck(name.str != "set", "Can't name a variable 'set'")
  vcheck(name.str != "mut", "Can't name a variable 'mut'")
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructingMemberNameS<'s> {
  pub name: StrI<'s>,
}
/*
case class ConstructingMemberNameS(name: StrI) extends IVarNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterableNameS<'s> {
  pub range: RangeS<'s>,
}
/*
case class IterableNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IteratorNameS<'s> {
  pub range: RangeS<'s>,
}
/*
case class IteratorNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterationOptionNameS<'s> {
  pub range: RangeS<'s>,
}
/*
case class IterationOptionNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileCondResultNameS<'s> {
  pub range: RangeS<'s>,
}
/*
case class WhileCondResultNameS(range: RangeS) extends IVarNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneNameS<'s> {
  pub rune: IRuneS<'s>,
}
/*
case class RuneNameS(rune: IRuneS) extends INameS with IImpreciseNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimeSizedArrayDeclarationNameS {}
/*
case class RuntimeSizedArrayDeclarationNameS() extends INameS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticSizedArrayDeclarationNameS {}
/*
case class StaticSizedArrayDeclarationNameS() extends INameS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IRuneS<'s> {
  CodeRune(&'s CodeRuneS<'s>),
  ImplDropCoordRune(&'s ImplDropCoordRuneS),
  ImplDropVoidRune(&'s ImplDropVoidRuneS),
  ImplicitRune(&'s ImplicitRuneS<'s>),
  PureBlockRegionRune(&'s PureBlockRegionRuneS<'s>),
  CallRegionRune(&'s CallRegionRuneS<'s>),
  CallPureMergeRegionRune(&'s CallPureMergeRegionRuneS<'s>),
  ImplicitRegionRune(&'s ImplicitRegionRuneS<'s>),
  ReachablePrototypeRune(&'s ReachablePrototypeRuneS),
  FreeOverrideStructTemplateRune(&'s FreeOverrideStructTemplateRuneS),
  FreeOverrideStructRune(&'s FreeOverrideStructRuneS),
  FreeOverrideInterfaceRune(&'s FreeOverrideInterfaceRuneS),
  LetImplicitRune(&'s LetImplicitRuneS<'s>),
  MagicParamRune(&'s MagicParamRuneS<'s>),
  MemberRune(&'s MemberRuneS),
  LocalDefaultRegionRune(&'s LocalDefaultRegionRuneS<'s>),
  DenizenDefaultRegionRune(&'s DenizenDefaultRegionRuneS<'s>),
  ExportDefaultRegionRune(&'s ExportDefaultRegionRuneS<'s>),
  ExternDefaultRegionRune(&'s ExternDefaultRegionRuneS<'s>),
  ImplicitCoercionOwnershipRune(&'s ImplicitCoercionOwnershipRuneS<'s>),
  ImplicitCoercionKindRune(&'s ImplicitCoercionKindRuneS<'s>),
  ImplicitCoercionTemplateRune(&'s ImplicitCoercionTemplateRuneS<'s>),
  ArraySizeImplicitRune(&'s ArraySizeImplicitRuneS),
  ArrayMutabilityImplicitRune(&'s ArrayMutabilityImplicitRuneS),
  ArrayVariabilityImplicitRune(&'s ArrayVariabilityImplicitRuneS),
  ReturnRune(&'s ReturnRuneS),
  StructNameRune(&'s StructNameRuneS<'s>),
  InterfaceNameRune(&'s InterfaceNameRuneS<'s>),
  SelfRune(&'s SelfRuneS),
  SelfOwnershipRune(&'s SelfOwnershipRuneS),
  SelfKindRune(&'s SelfKindRuneS),
  SelfKindTemplateRune(&'s SelfKindTemplateRuneS<'s>),
  SelfCoordRune(&'s SelfCoordRuneS),
  MacroVoidKindRune(&'s MacroVoidKindRuneS),
  MacroVoidCoordRune(&'s MacroVoidCoordRuneS),
  MacroSelfKindRune(&'s MacroSelfKindRuneS),
  MacroSelfKindTemplateRune(&'s MacroSelfKindTemplateRuneS),
  MacroSelfCoordRune(&'s MacroSelfCoordRuneS),
  ArgumentRune(&'s ArgumentRuneS),
  PatternInputRune(&'s PatternInputRuneS<'s>),
  ExplicitTemplateArgRune(&'s ExplicitTemplateArgRuneS),
  AnonymousSubstructParentInterfaceTemplateRune(
    &'s AnonymousSubstructParentInterfaceTemplateRuneS,
  ),
  AnonymousSubstructParentInterfaceKindRune(&'s AnonymousSubstructParentInterfaceKindRuneS),
  AnonymousSubstructParentInterfaceCoordRune(&'s AnonymousSubstructParentInterfaceCoordRuneS),
  AnonymousSubstructTemplateRune(&'s AnonymousSubstructTemplateRuneS),
  AnonymousSubstructKindRune(&'s AnonymousSubstructKindRuneS),
  AnonymousSubstructCoordRune(&'s AnonymousSubstructCoordRuneS),
  AnonymousSubstructVoidKindRune(&'s AnonymousSubstructVoidKindRuneS),
  AnonymousSubstructVoidCoordRune(&'s AnonymousSubstructVoidCoordRuneS),
  AnonymousSubstructMemberRune(&'s AnonymousSubstructMemberRuneS<'s>),
  AnonymousSubstructMethodSelfBorrowCoordRune(&'s AnonymousSubstructMethodSelfBorrowCoordRuneS<'s>),
  AnonymousSubstructMethodSelfOwnCoordRune(&'s AnonymousSubstructMethodSelfOwnCoordRuneS<'s>),
  AnonymousSubstructDropBoundPrototypeRune(&'s AnonymousSubstructDropBoundPrototypeRuneS<'s>),
  AnonymousSubstructDropBoundParamsListRune(&'s AnonymousSubstructDropBoundParamsListRuneS<'s>),
  AnonymousSubstructFunctionBoundPrototypeRune(&'s AnonymousSubstructFunctionBoundPrototypeRuneS<'s>),
  AnonymousSubstructFunctionBoundParamsListRune(&'s AnonymousSubstructFunctionBoundParamsListRuneS<'s>),
  AnonymousSubstructFunctionInterfaceTemplateRune(
    &'s AnonymousSubstructFunctionInterfaceTemplateRuneS<'s>,
  ),
  AnonymousSubstructFunctionInterfaceKindRune(&'s AnonymousSubstructFunctionInterfaceKindRuneS<'s>),
  AnonymousSubstructMethodInheritedRune(&'s AnonymousSubstructMethodInheritedRuneS<'s>),
  FunctorPrototypeRuneName(&'s FunctorPrototypeRuneNameS),
  FunctorParamRuneName(&'s FunctorParamRuneNameS),
  FunctorReturnRuneName(&'s FunctorReturnRuneNameS),
  DispatcherRuneFromImpl(&'s DispatcherRuneFromImplS<'s>),
  CaseRuneFromImpl(&'s CaseRuneFromImplS<'s>),
}

impl<'s> IRuneS<'s> {
  /// Pointer to the canonical interned payload. Use `std::ptr::eq(a.canonical_ptr(), b.canonical_ptr())` for identity comparison.
  pub fn canonical_ptr(&self) -> *const () {
    match self {
      IRuneS::CodeRune(r) => *r as *const _ as *const (),
      IRuneS::ImplDropCoordRune(r) => *r as *const _ as *const (),
      IRuneS::ImplDropVoidRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::PureBlockRegionRune(r) => *r as *const _ as *const (),
      IRuneS::CallRegionRune(r) => *r as *const _ as *const (),
      IRuneS::CallPureMergeRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ReachablePrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::FreeOverrideStructTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::FreeOverrideStructRune(r) => *r as *const _ as *const (),
      IRuneS::FreeOverrideInterfaceRune(r) => *r as *const _ as *const (),
      IRuneS::LetImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::MagicParamRune(r) => *r as *const _ as *const (),
      IRuneS::MemberRune(r) => *r as *const _ as *const (),
      IRuneS::LocalDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::DenizenDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ExportDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ExternDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitCoercionOwnershipRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitCoercionKindRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitCoercionTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::ArraySizeImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ArrayMutabilityImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ArrayVariabilityImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ReturnRune(r) => *r as *const _ as *const (),
      IRuneS::StructNameRune(r) => *r as *const _ as *const (),
      IRuneS::InterfaceNameRune(r) => *r as *const _ as *const (),
      IRuneS::SelfRune(r) => *r as *const _ as *const (),
      IRuneS::SelfOwnershipRune(r) => *r as *const _ as *const (),
      IRuneS::SelfKindRune(r) => *r as *const _ as *const (),
      IRuneS::SelfKindTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::SelfCoordRune(r) => *r as *const _ as *const (),
      IRuneS::MacroVoidKindRune(r) => *r as *const _ as *const (),
      IRuneS::MacroVoidCoordRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfKindRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfKindTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfCoordRune(r) => *r as *const _ as *const (),
      IRuneS::ArgumentRune(r) => *r as *const _ as *const (),
      IRuneS::PatternInputRune(r) => *r as *const _ as *const (),
      IRuneS::ExplicitTemplateArgRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructVoidKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructVoidCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMemberRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMethodSelfBorrowCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMethodSelfOwnCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructDropBoundPrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructDropBoundParamsListRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionBoundParamsListRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionInterfaceKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMethodInheritedRune(r) => *r as *const _ as *const (),
      IRuneS::FunctorPrototypeRuneName(r) => *r as *const _ as *const (),
      IRuneS::FunctorParamRuneName(r) => *r as *const _ as *const (),
      IRuneS::FunctorReturnRuneName(r) => *r as *const _ as *const (),
      IRuneS::DispatcherRuneFromImpl(r) => *r as *const _ as *const (),
      IRuneS::CaseRuneFromImpl(r) => *r as *const _ as *const (),
    }
  }

  /// Returns true iff both refer to the same canonical interned value.
  #[inline(always)]
  pub fn ptr_eq(&self, other: &IRuneS<'s>) -> bool {
    eq(self.canonical_ptr(), other.canonical_ptr())
  }
  /*
  Guardian: disable-all
  */
}
/*
Guardian: disable-all
*/

/// Value-struct for ImplicitRegionRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRegionRuneValS<'s> {
  pub original_rune: IRuneS<'s>,
}
/*
Guardian: disable-all
*/

/// Value-struct for ImplicitCoercionOwnershipRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionOwnershipRuneValS<'s> {
  pub range: RangeS<'s>,
  pub original_coord_rune: IRuneS<'s>,
}
/*
Guardian: disable-all
*/

/// Value-struct for ImplicitCoercionKindRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionKindRuneValS<'s> {
  pub range: RangeS<'s>,
  pub original_coord_rune: IRuneS<'s>,
}
/*
Guardian: disable-all
*/

/// Value-struct for ImplicitCoercionTemplateRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionTemplateRuneValS<'s> {
  pub range: RangeS<'s>,
  pub original_kind_rune: IRuneS<'s>,
}
/*
Guardian: disable-all
*/

/// Value-struct for AnonymousSubstructMethodInheritedRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodInheritedRuneValS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
  pub inner: IRuneS<'s>,
}
/*
Guardian: disable-all
*/

/// Value-struct for DispatcherRuneFromImplS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DispatcherRuneFromImplValS<'s> {
  pub inner_rune: IRuneS<'s>,
}
/*
Guardian: disable-all
*/

/// Value-struct for CaseRuneFromImplS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseRuneFromImplValS<'s> {
  pub inner_rune: IRuneS<'s>,
}
/*
Guardian: disable-all
*/

// Per @DSAUIMZ, these Val structs have private lid fields to prevent pre-allocation.
// Only constructible via new() which takes a LocationInDenizenVal from borrow_val().

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRuneValS<'tmp> { lid: LocationInDenizenVal<'tmp> }
impl<'tmp> ImplicitRuneValS<'tmp> { pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self { Self { lid } } pub fn lid(&self) -> LocationInDenizenVal<'tmp> { self.lid } }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PureBlockRegionRuneValS<'tmp> { lid: LocationInDenizenVal<'tmp> }
impl<'tmp> PureBlockRegionRuneValS<'tmp> { pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self { Self { lid } } pub fn lid(&self) -> LocationInDenizenVal<'tmp> { self.lid } }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallRegionRuneValS<'tmp> { lid: LocationInDenizenVal<'tmp> }
impl<'tmp> CallRegionRuneValS<'tmp> { pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self { Self { lid } } pub fn lid(&self) -> LocationInDenizenVal<'tmp> { self.lid } }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallPureMergeRegionRuneValS<'tmp> { lid: LocationInDenizenVal<'tmp> }
impl<'tmp> CallPureMergeRegionRuneValS<'tmp> { pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self { Self { lid } } pub fn lid(&self) -> LocationInDenizenVal<'tmp> { self.lid } }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetImplicitRuneValS<'tmp> { lid: LocationInDenizenVal<'tmp> }
impl<'tmp> LetImplicitRuneValS<'tmp> { pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self { Self { lid } } pub fn lid(&self) -> LocationInDenizenVal<'tmp> { self.lid } }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamRuneValS<'tmp> { lid: LocationInDenizenVal<'tmp> }
impl<'tmp> MagicParamRuneValS<'tmp> { pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self { Self { lid } } pub fn lid(&self) -> LocationInDenizenVal<'tmp> { self.lid } }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalDefaultRegionRuneValS<'tmp> { lid: LocationInDenizenVal<'tmp> }
impl<'tmp> LocalDefaultRegionRuneValS<'tmp> { pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self { Self { lid } } pub fn lid(&self) -> LocationInDenizenVal<'tmp> { self.lid } }

/// Per @DSAUIMZ, 'tmp carries a temporary borrow to defer slice allocation.
/// Value/key form of rune for interner lookups. Used when constructing runes before
/// canonicalizing via `intern_rune`. Storage fields use canonical `IRuneS<'s>`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IRuneValS<'s, 'tmp> {
  CodeRune(CodeRuneS<'s>),
  ImplDropCoordRune(ImplDropCoordRuneS),
  ImplDropVoidRune(ImplDropVoidRuneS),
  ImplicitRune(ImplicitRuneValS<'tmp>),
  PureBlockRegionRune(PureBlockRegionRuneValS<'tmp>),
  CallRegionRune(CallRegionRuneValS<'tmp>),
  CallPureMergeRegionRune(CallPureMergeRegionRuneValS<'tmp>),
  ImplicitRegionRune(ImplicitRegionRuneValS<'s>),
  ReachablePrototypeRune(ReachablePrototypeRuneS),
  FreeOverrideStructTemplateRune(FreeOverrideStructTemplateRuneS),
  FreeOverrideStructRune(FreeOverrideStructRuneS),
  FreeOverrideInterfaceRune(FreeOverrideInterfaceRuneS),
  LetImplicitRune(LetImplicitRuneValS<'tmp>),
  MagicParamRune(MagicParamRuneValS<'tmp>),
  MemberRune(MemberRuneS),
  LocalDefaultRegionRune(LocalDefaultRegionRuneValS<'tmp>),
  DenizenDefaultRegionRune(DenizenDefaultRegionRuneS<'s>),
  ExportDefaultRegionRune(ExportDefaultRegionRuneS<'s>),
  ExternDefaultRegionRune(ExternDefaultRegionRuneS<'s>),
  ImplicitCoercionOwnershipRune(ImplicitCoercionOwnershipRuneValS<'s>),
  ImplicitCoercionKindRune(ImplicitCoercionKindRuneValS<'s>),
  ImplicitCoercionTemplateRune(ImplicitCoercionTemplateRuneValS<'s>),
  ArraySizeImplicitRune(ArraySizeImplicitRuneS),
  ArrayMutabilityImplicitRune(ArrayMutabilityImplicitRuneS),
  ArrayVariabilityImplicitRune(ArrayVariabilityImplicitRuneS),
  ReturnRune(ReturnRuneS),
  StructNameRune(StructNameRuneS<'s>),
  InterfaceNameRune(InterfaceNameRuneS<'s>),
  SelfRune(SelfRuneS),
  SelfOwnershipRune(SelfOwnershipRuneS),
  SelfKindRune(SelfKindRuneS),
  SelfKindTemplateRune(SelfKindTemplateRuneS<'s>),
  SelfCoordRune(SelfCoordRuneS),
  MacroVoidKindRune(MacroVoidKindRuneS),
  MacroVoidCoordRune(MacroVoidCoordRuneS),
  MacroSelfKindRune(MacroSelfKindRuneS),
  MacroSelfKindTemplateRune(MacroSelfKindTemplateRuneS),
  MacroSelfCoordRune(MacroSelfCoordRuneS),
  ArgumentRune(ArgumentRuneS),
  PatternInputRune(PatternInputRuneS<'s>),
  ExplicitTemplateArgRune(ExplicitTemplateArgRuneS),
  AnonymousSubstructParentInterfaceTemplateRune(AnonymousSubstructParentInterfaceTemplateRuneS),
  AnonymousSubstructParentInterfaceKindRune(AnonymousSubstructParentInterfaceKindRuneS),
  AnonymousSubstructParentInterfaceCoordRune(AnonymousSubstructParentInterfaceCoordRuneS),
  AnonymousSubstructTemplateRune(AnonymousSubstructTemplateRuneS),
  AnonymousSubstructKindRune(AnonymousSubstructKindRuneS),
  AnonymousSubstructCoordRune(AnonymousSubstructCoordRuneS),
  AnonymousSubstructVoidKindRune(AnonymousSubstructVoidKindRuneS),
  AnonymousSubstructVoidCoordRune(AnonymousSubstructVoidCoordRuneS),
  AnonymousSubstructMemberRune(AnonymousSubstructMemberRuneS<'s>),
  AnonymousSubstructMethodSelfBorrowCoordRune(AnonymousSubstructMethodSelfBorrowCoordRuneS<'s>),
  AnonymousSubstructMethodSelfOwnCoordRune(AnonymousSubstructMethodSelfOwnCoordRuneS<'s>),
  AnonymousSubstructDropBoundPrototypeRune(AnonymousSubstructDropBoundPrototypeRuneS<'s>),
  AnonymousSubstructDropBoundParamsListRune(AnonymousSubstructDropBoundParamsListRuneS<'s>),
  AnonymousSubstructFunctionBoundPrototypeRune(AnonymousSubstructFunctionBoundPrototypeRuneS<'s>),
  AnonymousSubstructFunctionBoundParamsListRune(AnonymousSubstructFunctionBoundParamsListRuneS<'s>),
  AnonymousSubstructFunctionInterfaceTemplateRune(AnonymousSubstructFunctionInterfaceTemplateRuneS<'s>),
  AnonymousSubstructFunctionInterfaceKindRune(AnonymousSubstructFunctionInterfaceKindRuneS<'s>),
  AnonymousSubstructMethodInheritedRune(AnonymousSubstructMethodInheritedRuneValS<'s>),
  FunctorPrototypeRuneName(FunctorPrototypeRuneNameS),
  FunctorParamRuneName(FunctorParamRuneNameS),
  FunctorReturnRuneName(FunctorReturnRuneNameS),
  DispatcherRuneFromImpl(DispatcherRuneFromImplValS<'s>),
  CaseRuneFromImpl(CaseRuneFromImplValS<'s>),
}

/// Per @DSAUIMZ, wrapper enabling heterogeneous HashMap lookup.
///
/// The intern map stores `IRuneValS<'s, 's>` keys (both lifetimes = arena).
/// But callers build `IRuneValS<'s, 'tmp>` where 'tmp borrows a stack-local
/// builder (not the arena). We need to look up in the map using the 'tmp version.
///
/// We can't implement `Equivalent<IRuneValS<'s,'s>> for IRuneValS<'s,'tmp>` directly
/// because when 'tmp = 's, the two types are identical, and Rust's blanket impl
/// `Equivalent<K> for K` (from PartialEq) already covers that case. The orphan
/// rules see a potential overlap and reject our impl.
///
/// This wrapper is a distinct type that breaks the overlap. It holds a reference
/// to the query val and delegates Hash/Equivalent to the inner val's contents.
/// The Hash output is identical for equal values regardless of lifetime, because
/// both LocationInDenizenVal and LocationInDenizen hash by slice contents.
pub struct RuneValQuery<'a, 's, 'tmp>(pub &'a IRuneValS<'s, 'tmp>);

impl<'a, 's, 'tmp> Hash for RuneValQuery<'a, 's, 'tmp> {
  fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl<'a, 's, 'tmp> hashbrown::Equivalent<IRuneValS<'s, 's>> for RuneValQuery<'a, 's, 'tmp> {
  fn equivalent(&self, key: &IRuneValS<'s, 's>) -> bool {
    match (self.0, key) {
      // 7 lid variants: compare path contents
      (ImplicitRune(a), ImplicitRune(b)) => a.lid().path() == b.lid().path(),
      (PureBlockRegionRune(a), PureBlockRegionRune(b)) => a.lid().path() == b.lid().path(),
      (CallRegionRune(a), CallRegionRune(b)) => a.lid().path() == b.lid().path(),
      (CallPureMergeRegionRune(a), CallPureMergeRegionRune(b)) => a.lid().path() == b.lid().path(),
      (LetImplicitRune(a), LetImplicitRune(b)) => a.lid().path() == b.lid().path(),
      (MagicParamRune(a), MagicParamRune(b)) => a.lid().path() == b.lid().path(),
      (LocalDefaultRegionRune(a), LocalDefaultRegionRune(b)) => a.lid().path() == b.lid().path(),
      // All other variants: same inner type on both sides, delegate to PartialEq
      (CodeRune(a), CodeRune(b)) => a == b,
      (ImplDropCoordRune(a), ImplDropCoordRune(b)) => a == b,
      (ImplDropVoidRune(a), ImplDropVoidRune(b)) => a == b,
      (ImplicitRegionRune(a), ImplicitRegionRune(b)) => a == b,
      (ReachablePrototypeRune(a), ReachablePrototypeRune(b)) => a == b,
      (FreeOverrideStructTemplateRune(a), FreeOverrideStructTemplateRune(b)) => a == b,
      (FreeOverrideStructRune(a), FreeOverrideStructRune(b)) => a == b,
      (FreeOverrideInterfaceRune(a), FreeOverrideInterfaceRune(b)) => a == b,
      (MemberRune(a), MemberRune(b)) => a == b,
      (DenizenDefaultRegionRune(a), DenizenDefaultRegionRune(b)) => a == b,
      (ExportDefaultRegionRune(a), ExportDefaultRegionRune(b)) => a == b,
      (ExternDefaultRegionRune(a), ExternDefaultRegionRune(b)) => a == b,
      (ImplicitCoercionOwnershipRune(a), ImplicitCoercionOwnershipRune(b)) => a == b,
      (ImplicitCoercionKindRune(a), ImplicitCoercionKindRune(b)) => a == b,
      (ImplicitCoercionTemplateRune(a), ImplicitCoercionTemplateRune(b)) => a == b,
      (ArraySizeImplicitRune(a), ArraySizeImplicitRune(b)) => a == b,
      (ArrayMutabilityImplicitRune(a), ArrayMutabilityImplicitRune(b)) => a == b,
      (ArrayVariabilityImplicitRune(a), ArrayVariabilityImplicitRune(b)) => a == b,
      (ReturnRune(a), ReturnRune(b)) => a == b,
      (StructNameRune(a), StructNameRune(b)) => a == b,
      (InterfaceNameRune(a), InterfaceNameRune(b)) => a == b,
      (SelfRune(a), SelfRune(b)) => a == b,
      (SelfOwnershipRune(a), SelfOwnershipRune(b)) => a == b,
      (SelfKindRune(a), SelfKindRune(b)) => a == b,
      (SelfKindTemplateRune(a), SelfKindTemplateRune(b)) => a == b,
      (SelfCoordRune(a), SelfCoordRune(b)) => a == b,
      (MacroVoidKindRune(a), MacroVoidKindRune(b)) => a == b,
      (MacroVoidCoordRune(a), MacroVoidCoordRune(b)) => a == b,
      (MacroSelfKindRune(a), MacroSelfKindRune(b)) => a == b,
      (MacroSelfKindTemplateRune(a), MacroSelfKindTemplateRune(b)) => a == b,
      (MacroSelfCoordRune(a), MacroSelfCoordRune(b)) => a == b,
      (ArgumentRune(a), ArgumentRune(b)) => a == b,
      (PatternInputRune(a), PatternInputRune(b)) => a == b,
      (ExplicitTemplateArgRune(a), ExplicitTemplateArgRune(b)) => a == b,
      (AnonymousSubstructParentInterfaceTemplateRune(a), AnonymousSubstructParentInterfaceTemplateRune(b)) => a == b,
      (AnonymousSubstructParentInterfaceKindRune(a), AnonymousSubstructParentInterfaceKindRune(b)) => a == b,
      (AnonymousSubstructParentInterfaceCoordRune(a), AnonymousSubstructParentInterfaceCoordRune(b)) => a == b,
      (AnonymousSubstructTemplateRune(a), AnonymousSubstructTemplateRune(b)) => a == b,
      (AnonymousSubstructKindRune(a), AnonymousSubstructKindRune(b)) => a == b,
      (AnonymousSubstructCoordRune(a), AnonymousSubstructCoordRune(b)) => a == b,
      (AnonymousSubstructVoidKindRune(a), AnonymousSubstructVoidKindRune(b)) => a == b,
      (AnonymousSubstructVoidCoordRune(a), AnonymousSubstructVoidCoordRune(b)) => a == b,
      (AnonymousSubstructMemberRune(a), AnonymousSubstructMemberRune(b)) => a == b,
      (AnonymousSubstructMethodSelfBorrowCoordRune(a), AnonymousSubstructMethodSelfBorrowCoordRune(b)) => a == b,
      (AnonymousSubstructMethodSelfOwnCoordRune(a), AnonymousSubstructMethodSelfOwnCoordRune(b)) => a == b,
      (AnonymousSubstructDropBoundPrototypeRune(a), AnonymousSubstructDropBoundPrototypeRune(b)) => a == b,
      (AnonymousSubstructDropBoundParamsListRune(a), AnonymousSubstructDropBoundParamsListRune(b)) => a == b,
      (AnonymousSubstructFunctionBoundPrototypeRune(a), AnonymousSubstructFunctionBoundPrototypeRune(b)) => a == b,
      (AnonymousSubstructFunctionBoundParamsListRune(a), AnonymousSubstructFunctionBoundParamsListRune(b)) => a == b,
      (AnonymousSubstructFunctionInterfaceTemplateRune(a), AnonymousSubstructFunctionInterfaceTemplateRune(b)) => a == b,
      (AnonymousSubstructFunctionInterfaceKindRune(a), AnonymousSubstructFunctionInterfaceKindRune(b)) => a == b,
      (AnonymousSubstructMethodInheritedRune(a), AnonymousSubstructMethodInheritedRune(b)) => a == b,
      (FunctorPrototypeRuneName(a), FunctorPrototypeRuneName(b)) => a == b,
      (FunctorParamRuneName(a), FunctorParamRuneName(b)) => a == b,
      (FunctorReturnRuneName(a), FunctorReturnRuneName(b)) => a == b,
      (DispatcherRuneFromImpl(a), DispatcherRuneFromImpl(b)) => a == b,
      (CaseRuneFromImpl(a), CaseRuneFromImpl(b)) => a == b,
      _ => false,
    }
  }
  /* Guardian: disable-all */
}

/*
// We differentiate rune names from regular names, we scout out what's actually
// a rune so we can inform the typingpass. The typingpass wants to know so it can know
// how to handle this thing; if it's a name, we expect it to exist in the
// environment already, but if it's a rune we can assign something into it.
// Also, we might refer to a rune that was defined in our container's container's
// container, so asking "is this thing here a rune" involves looking at all our
// containers. That's much easier for the scout, so thats a nice bonus.
// We have all these subclasses instead of a string so we don't have to have
// prefixes and names like __implicit_0, __paramRune_0, etc.
// This extends INameS so we can use it as a lookup key in Compiler's environments.
trait IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeRuneS<'s> {
  pub name: StrI<'s>,
}

/*
case class CodeRuneS(name: StrI) extends IRuneS {
  vpass()
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDropCoordRuneS {}
/*
case class ImplDropCoordRuneS() extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDropVoidRuneS {}
/*
case class ImplDropVoidRuneS() extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}
/*
case class ImplicitRuneS(lid: LocationInDenizen) extends IRuneS {
  vpass()
  lid match {
    case LocationInDenizen(Vector(2, 1, 1, 2)) => {
      vpass()
    }
    case _ =>
  }
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PureBlockRegionRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}
/*
case class PureBlockRegionRuneS(lid: LocationInDenizen) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallRegionRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}
/*
case class CallRegionRuneS(lid: LocationInDenizen) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallPureMergeRegionRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}
/*
case class CallPureMergeRegionRuneS(lid: LocationInDenizen) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRegionRuneS<'s> {
  pub original_rune: IRuneS<'s>,
}
/*
case class ImplicitRegionRuneS(originalRune: IRuneS) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReachablePrototypeRuneS {
  pub num: i32,
}
/*
case class ReachablePrototypeRuneS(num: Int) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideStructTemplateRuneS {}
/*
case class FreeOverrideStructTemplateRuneS() extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideStructRuneS {}
/*
case class FreeOverrideStructRuneS() extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideInterfaceRuneS {}
/*
case class FreeOverrideInterfaceRuneS() extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetImplicitRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}
/*
case class LetImplicitRuneS(lid: LocationInDenizen) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}
/*
case class MagicParamRuneS(lid: LocationInDenizen) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemberRuneS {
  pub member_index: i32,
}
/*
case class MemberRuneS(memberIndex: Int) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalDefaultRegionRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}
/*

case class LocalDefaultRegionRuneS(lid: LocationInDenizen) extends IRuneS
// This has a name because there might be multiple default regions in play sometimes.
// When a function calls the constructor for a struct, the function has its own default region,
// but it's also evaluating the rules for the struct. Best not mix them up.
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DenizenDefaultRegionRuneS<'s> {
  pub denizen_name: INameS<'s>,
}

/*
case class DenizenDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportDefaultRegionRuneS<'s> {
  pub denizen_name: INameS<'s>,
}
/*
case class ExportDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternDefaultRegionRuneS<'s> {
  pub denizen_name: INameS<'s>,
}
/*
case class ExternDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionOwnershipRuneS<'s> {
  pub range: RangeS<'s>,
  pub original_coord_rune: IRuneS<'s>,
}
/*
case class ImplicitCoercionOwnershipRuneS(range: RangeS, originalCoordRune: IRuneS) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionKindRuneS<'s> {
  pub range: RangeS<'s>,
  pub original_coord_rune: IRuneS<'s>,
}
/*
case class ImplicitCoercionKindRuneS(range: RangeS, originalCoordRune: IRuneS) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionTemplateRuneS<'s> {
  pub range: RangeS<'s>,
  pub original_kind_rune: IRuneS<'s>,
}
/*
case class ImplicitCoercionTemplateRuneS(range: RangeS, originalKindRune: IRuneS) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArraySizeImplicitRuneS {}
/*
// Used to type the templex handed to the size part of the static sized array expressions
case class ArraySizeImplicitRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayMutabilityImplicitRuneS {}
/*
// Used to type the templex handed to the mutability part of the static sized array expressions
case class ArrayMutabilityImplicitRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayVariabilityImplicitRuneS {}
/*
// Used to type the templex handed to the variability part of the static sized array expressions
case class ArrayVariabilityImplicitRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReturnRuneS {}
/*
case class ReturnRuneS() extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructNameRuneS<'s> {
  pub struct_name: ICitizenDeclarationNameS<'s>,
}
/*
case class StructNameRuneS(structName: ICitizenDeclarationNameS) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceNameRuneS<'s> {
  pub interface_name: ICitizenDeclarationNameS<'s>,
}
/*
case class InterfaceNameRuneS(interfaceName: ICitizenDeclarationNameS) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfRuneS {}
/*
// Vale has no notion of Self, it's just a convenient name for a first parameter.
case class SelfRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfOwnershipRuneS {}
/*
case class SelfOwnershipRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfKindRuneS {}
/*
case class SelfKindRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfKindTemplateRuneS<'s> {
  pub loc: CodeLocationS<'s>,
}
/*
case class SelfKindTemplateRuneS(loc: CodeLocationS) extends IRuneS {
  vpass()
}
  */
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfCoordRuneS {}
/*
case class SelfCoordRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroVoidKindRuneS {}
/*
case class MacroVoidKindRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroVoidCoordRuneS {}
/*
case class MacroVoidCoordRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfKindRuneS {}
/*
case class MacroSelfKindRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfKindTemplateRuneS {}
/*
case class MacroSelfKindTemplateRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfCoordRuneS {}
/*
case class MacroSelfCoordRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeNameS<'s> {
  pub name: StrI<'s>,
}

/*
case class CodeNameS(name: StrI) extends IImpreciseNameS {
  vpass()
  vassert(name.str != "_")
}
  */
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalFunctionFamilyNameS<'s> {
  pub name: StrI<'s>,
}
/*
// When we're calling a function, we're addressing an overload set, not a specific function.
// If we want a specific function, we use TopLevelDeclarationNameS.
case class GlobalFunctionFamilyNameS(name: String) extends INameS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArgumentRuneS {
  pub arg_index: i32,
}
/*
// These are only made by the typingpass
case class ArgumentRuneS(argIndex: Int) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PatternInputRuneS<'s> {
  pub code_loc: CodeLocationS<'s>,
}
/*
case class PatternInputRuneS(codeLoc: CodeLocationS) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExplicitTemplateArgRuneS {
  pub index: i32,
}
/*
case class ExplicitTemplateArgRuneS(index: Int) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceTemplateRuneS {}
/*
case class AnonymousSubstructParentInterfaceTemplateRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceKindRuneS {}
/*
case class AnonymousSubstructParentInterfaceKindRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceCoordRuneS {}
/*
case class AnonymousSubstructParentInterfaceCoordRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateRuneS {}
/*
case class AnonymousSubstructTemplateRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructKindRuneS {}
/*
case class AnonymousSubstructKindRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructCoordRuneS {}
/*
case class AnonymousSubstructCoordRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructVoidKindRuneS {}
/*
case class AnonymousSubstructVoidKindRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructVoidCoordRuneS {}
/*
case class AnonymousSubstructVoidCoordRuneS() extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructMemberRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodSelfBorrowCoordRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructMethodSelfBorrowCoordRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodSelfOwnCoordRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructMethodSelfOwnCoordRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundPrototypeRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructDropBoundPrototypeRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundParamsListRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructDropBoundParamsListRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundPrototypeRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructFunctionBoundPrototypeRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundParamsListRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructFunctionBoundParamsListRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
/*
//case class AnonymousSubstructFunctionInterfaceKindRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
/*
//case class AnonymousSubstructFunctionInterfaceOwnershipRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceTemplateRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructFunctionInterfaceTemplateRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceKindRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}
/*
case class AnonymousSubstructFunctionInterfaceKindRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodInheritedRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
  pub inner: IRuneS<'s>,
}
/*
case class AnonymousSubstructMethodInheritedRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS, inner: IRuneS) extends IRuneS {
  this match {
    case AnonymousSubstructMethodInheritedRuneS(TopLevelInterfaceDeclarationNameS(StrI("Bork"),_),FunctionNameS(StrI("bork"),_),ImplicitRuneS(LocationInDenizen(Vector(2, 1, 1, 2, 1, 1)))) => {
      vpass()
    }
    case _ =>
  }
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorPrototypeRuneNameS {}
/*
case class FunctorPrototypeRuneNameS() extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorParamRuneNameS {
  pub index: i32,
}
/*
case class FunctorParamRuneNameS(index: Int) extends IRuneS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorReturnRuneNameS {}
/*
case class FunctorReturnRuneNameS() extends IRuneS
*/
// Vale has no notion of Self, it's just a convenient name for a first parameter.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfNameS {}
/*
// Vale has no notion of Self, it's just a convenient name for a first parameter.
case class SelfNameS() extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArbitraryNameS {}
/*
// A miscellaneous name, for when a name doesn't really make sense, like it's the only entry in the environment or something.
case class ArbitraryNameS() extends INameS with IImpreciseNameS
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DispatcherRuneFromImplS<'s> {
  pub inner_rune: IRuneS<'s>,
}
/*
case class DispatcherRuneFromImplS(innerRune: IRuneS) extends IRuneS
*/
// Only made by typingpass, see if we can take these out
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseRuneFromImplS<'s> {
  pub inner_rune: IRuneS<'s>,
}
/*
case class CaseRuneFromImplS(innerRune: IRuneS) extends IRuneS

// Only made by typingpass, see if we can take these out
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructorNameS<'s> {
  pub tlcd: ICitizenDeclarationNameS<'s>,
}
/*
case class ConstructorNameS(tlcd: ICitizenDeclarationNameS) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = tlcd.range.begin.file.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = tlcd.getImpreciseName(interner)
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImmConcreteDestructorNameS<'s> {
  pub package_coordinate: PackageCoordinate<'s>,
}
/*
case class ImmConcreteDestructorNameS(packageCoordinate: PackageCoordinate) extends IFunctionDeclarationNameS {
  override def getImpreciseName(interner: Interner): IImpreciseNameS = vimpl()
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImmInterfaceDestructorNameS<'s> {
  pub package_coordinate: PackageCoordinate<'s>,
}
/*
case class ImmInterfaceDestructorNameS(packageCoordinate: PackageCoordinate) extends IFunctionDeclarationNameS {
  override def getImpreciseName(interner: Interner): IImpreciseNameS = vimpl()
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplImpreciseNameS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSubCitizenImpreciseNameS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSuperInterfaceImpreciseNameS<'s> {
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
}
/*
case class ImplImpreciseNameS(subCitizenImpreciseName: IImpreciseNameS, superInterfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS { }
case class ImplSubCitizenImpreciseNameS(subCitizenImpreciseName: IImpreciseNameS) extends IImpreciseNameS { }
case class ImplSuperInterfaceImpreciseNameS(superInterfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS { }

// See NSIDN for why we need this virtual name
//case class VirtualFreeImpreciseNameS() extends IImpreciseNameS { }
//case class AbstractVirtualFreeDeclarationNameS(codeLoc: CodeLocationS) extends IFunctionDeclarationNameS {
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(VirtualFreeImpreciseNameS())
//  override def packageCoordinate: PackageCoordinate = codeLoc.file.packageCoord
//}
//case class OverrideVirtualFreeDeclarationNameS(codeLoc: CodeLocationS) extends IFunctionDeclarationNameS {
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(VirtualFreeImpreciseNameS())
//  override def packageCoordinate: PackageCoordinate = codeLoc.file.packageCoord
//}
*/