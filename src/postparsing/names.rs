use crate::interner::StrI;
use crate::postparsing::ast::{LocationInDenizen, LocationInDenizenVal};
use crate::scout_arena::{ScoutArena, ScoutInterned};
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr::eq;
use IRuneValS::*;

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
  VarName(&'s IVarDeclarationNameS<'s>),
}

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

  /// Returns true iff both refer to the same canonical interned value.
  #[inline(always)]
  pub fn ptr_eq(&self, other: &INameS<'s>) -> bool {
    eq(self.canonical_ptr(), other.canonical_ptr())
  }

  pub fn as_top_level_citizen_name(&self) -> Option<TopLevelCitizenDeclarationNameS<'s>> {
    match self {
      INameS::TopLevelStructDeclaration(s) => {
        Some(TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName((*s).clone()))
      }
      INameS::TopLevelInterfaceDeclaration(i) => {
        Some(TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName((*i).clone()))
      }
      _ => None,
    }
  }
}

/// Value/key form for interner lookups. Shallow Val structs reference canonical INameS/IFunctionDeclarationNameS/etc.
/// Per @DSAUIMZ, if a variant gains a slice field, add a 'tmp lifetime and use a transient ValS struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum INameValS<'s> {
  // FunctionDeclaration is NOT here: function declaration names are identity (not interned),
  // built directly and wrapped in `INameS::FunctionDeclaration` (like `INameS::VarName`).
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
  ArbitraryName(ArbitraryNameValS),
}

/// Shallow: inner already canonical.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructImplDeclarationNameValS<'s> {
  pub interface: &'s TopLevelInterfaceDeclarationNameS<'s>,
}

/// Shallow: interface_name already canonical.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateNameValS<'s> {
  pub interface_name: &'s TopLevelInterfaceDeclarationNameS<'s>,
}

// AFTERM: Add arcana for how these sometimes contain INameS even though
// INameS arent interned. Should be fine, but worth looking out for.
/// Interned (see @TFITCX)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImpreciseNameS<'s> {
  CodeName(&'s CodeNameS<'s>),
  ConstructingMemberImpreciseName(&'s ConstructingMemberImpreciseNameS<'s>),
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
  MagicParamName(&'s MagicParamImpreciseNameS<'s>),
  WhileCondResultName(&'s WhileCondResultNameS<'s>),
  AnonymousSubstructMemberName(&'s AnonymousSubstructMemberNameS),
  DesugaredParamName(&'s DesugaredParamNameS<'s>),
}

impl<'s> IImpreciseNameS<'s> {
  /// Pointer to the canonical interned payload. Use `std::ptr::eq(a.canonical_ptr(), b.canonical_ptr())` for identity comparison.
  pub fn canonical_ptr(&self) -> *const () {
    match self {
      IImpreciseNameS::CodeName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ConstructingMemberImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::IterableName(r) => *r as *const _ as *const (),
      IImpreciseNameS::IteratorName(r) => *r as *const _ as *const (),
      IImpreciseNameS::IterationOptionName(r) => *r as *const _ as *const (),
      IImpreciseNameS::LambdaImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::PlaceholderImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::LambdaStructImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ClosureParamImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::PrototypeName(r) => *r as *const _ as *const (),
      IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::AnonymousSubstructConstructorTemplateImpreciseName(r) => {
        *r as *const _ as *const ()
      }
      IImpreciseNameS::ImplImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplSubCitizenImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplSuperInterfaceImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::SelfName(r) => *r as *const _ as *const (),
      IImpreciseNameS::RuneName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ArbitraryName(r) => *r as *const _ as *const (),
      IImpreciseNameS::MagicParamName(r) => *r as *const _ as *const (),
      IImpreciseNameS::WhileCondResultName(r) => *r as *const _ as *const (),
      IImpreciseNameS::AnonymousSubstructMemberName(r) => *r as *const _ as *const (),
      IImpreciseNameS::DesugaredParamName(r) => *r as *const _ as *const (),
    }
  }

  /// Returns true iff both refer to the same canonical interned value.
  #[inline(always)]
  pub fn ptr_eq(&self, other: &IImpreciseNameS<'s>) -> bool {
    eq(self.canonical_ptr(), other.canonical_ptr())
  }
}

/// Value-struct for LambdaStructImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructImpreciseNameValS<'s> {
  pub lambda_name: IImpreciseNameS<'s>,
}

/// Value-struct for AnonymousSubstructTemplateImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateImpreciseNameValS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
}

/// Value-struct for AnonymousSubstructConstructorTemplateImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructConstructorTemplateImpreciseNameValS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
}

/// Value-struct for ImplImpreciseNameS key. Shallow: references canonical children.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplImpreciseNameValS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
}

/// Value-struct for ImplSubCitizenImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSubCitizenImpreciseNameValS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
}

/// Value-struct for ImplSuperInterfaceImpreciseNameS key. Shallow: references canonical child.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSuperInterfaceImpreciseNameValS<'s> {
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
}

/// Value-struct for RuneNameS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneNameValS<'s> {
  pub rune: IRuneS<'s>,
}

/// Value/key form of imprecise name for interner lookups. Storage uses canonical `IImpreciseNameS<'s>`.
/// Per @DSAUIMZ, if a variant gains a slice field, add a 'tmp lifetime and use a transient ValS struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImpreciseNameValS<'s> {
  CodeName(CodeNameValS<'s>),
  ConstructingMemberImpreciseName(ConstructingMemberImpreciseNameValS<'s>),
  IterableName(IterableNameValS<'s>),
  IteratorName(IteratorNameValS<'s>),
  IterationOptionName(IterationOptionNameValS<'s>),
  LambdaImpreciseName(LambdaImpreciseNameValS),
  PlaceholderImpreciseName(PlaceholderImpreciseNameValS),
  LambdaStructImpreciseName(LambdaStructImpreciseNameValS<'s>),
  ClosureParamImpreciseName(ClosureParamImpreciseNameValS),
  PrototypeName(PrototypeNameValS),
  AnonymousSubstructTemplateImpreciseName(AnonymousSubstructTemplateImpreciseNameValS<'s>),
  AnonymousSubstructConstructorTemplateImpreciseName(
    AnonymousSubstructConstructorTemplateImpreciseNameValS<'s>,
  ),
  ImplImpreciseName(ImplImpreciseNameValS<'s>),
  ImplSubCitizenImpreciseName(ImplSubCitizenImpreciseNameValS<'s>),
  ImplSuperInterfaceImpreciseName(ImplSuperInterfaceImpreciseNameValS<'s>),
  SelfName(SelfNameValS),
  RuneName(RuneNameValS<'s>),
  ArbitraryName(ArbitraryNameValS),
  MagicParamName(MagicParamNameValS<'s>),
  WhileCondResultName(WhileCondResultNameValS<'s>),
  AnonymousSubstructMemberName(AnonymousSubstructMemberNameValS),
  DesugaredParamName(DesugaredParamNameValS<'s>),
}

/// Value-type (see @TFITCX). Identity-bearing — each names one declaration — so never interned
/// per @WVSBIZ; the per-variant disambiguator (lid/location/range) makes structural eq be identity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IVarDeclarationNameS<'s> {
  CodeVarName(CodeVarNameS<'s>),
  ConstructingMemberName(ConstructingMemberNameDeclarationS<'s>),
  ClosureParamName(ClosureParamNameDeclarationS<'s>),
  MagicParamName(MagicParamNameDeclarationS<'s>),
  IterableName(IterableNameDeclarationS<'s>),
  IteratorName(IteratorNameDeclarationS<'s>),
  IterationOptionName(IterationOptionNameDeclarationS<'s>),
  WhileCondResultName(WhileCondResultNameDeclarationS<'s>),
  SelfName(SelfNameDeclarationS<'s>),
  AnonymousSubstructMemberName(AnonymousSubstructMemberNameDeclarationS<'s>),
  /// Synthetic ABI-slot identifier for a function parameter that has no user-written name
  /// (an anonymous destructure like `Pair[a, b]`, or an ignored `_ Pair`). Named params
  /// keep their real name instead.
  DesugaredParamName(DesugaredParamNameDeclarationS<'s>),
}

impl<'s> IVarDeclarationNameS<'s> {
  /// The imprecise (source) name a use-site uses to resolve this variable by its source
  /// spelling. Total: every declaration-name variant maps to a corresponding imprecise
  /// variant, so this never fails.
  /// The declaration's imprecise (source) name. A declaration stores its imprecise name as an
  /// already-interned `&'s` ref (typing-design "Names"), so this just wraps that canonical ref in the
  /// matching `IImpreciseNameS` variant — no re-interning needed.
  pub fn imprecise_name(self, _scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    match self {
      IVarDeclarationNameS::CodeVarName(n) => IImpreciseNameS::CodeName(n.imprecise_name),
      IVarDeclarationNameS::ConstructingMemberName(n) => {
        IImpreciseNameS::ConstructingMemberImpreciseName(n.imprecise_name)
      }
      IVarDeclarationNameS::ClosureParamName(n) => {
        IImpreciseNameS::ClosureParamImpreciseName(n.imprecise_name)
      }
      IVarDeclarationNameS::MagicParamName(n) => IImpreciseNameS::MagicParamName(n.imprecise_name),
      IVarDeclarationNameS::IterableName(n) => IImpreciseNameS::IterableName(n.imprecise_name),
      IVarDeclarationNameS::IteratorName(n) => IImpreciseNameS::IteratorName(n.imprecise_name),
      IVarDeclarationNameS::IterationOptionName(n) => {
        IImpreciseNameS::IterationOptionName(n.imprecise_name)
      }
      IVarDeclarationNameS::WhileCondResultName(n) => {
        IImpreciseNameS::WhileCondResultName(n.imprecise_name)
      }
      IVarDeclarationNameS::SelfName(n) => IImpreciseNameS::SelfName(n.imprecise_name),
      IVarDeclarationNameS::AnonymousSubstructMemberName(n) => {
        IImpreciseNameS::AnonymousSubstructMemberName(n.imprecise_name)
      }
      IVarDeclarationNameS::DesugaredParamName(n) => {
        IImpreciseNameS::DesugaredParamName(n.imprecise_name)
      }
    }
  }
}

/// Identity-bearing (each names one declaration — carries a `lid`), so **not interned**, mirroring
/// `IVarDeclarationNameS` (@WVSBIZ). Built directly and wrapped in `INameS::FunctionDeclaration`
/// (like `INameS::VarName` wraps `IVarDeclarationNameS`). Each variant embeds its interned imprecise
/// name; `imprecise_name()` hands back the matching `IFunctionImpreciseNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IFunctionDeclarationNameS<'s> {
  FunctionName(FunctionNameS<'s>),
  LambdaDeclarationName(LambdaDeclarationNameS<'s>),
  ForwarderFunctionDeclarationName(&'s ForwarderFunctionDeclarationNameS<'s>),
  ConstructorName(&'s ConstructorNameS<'s>),
}

/// Shallow: inner already canonical.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwarderFunctionDeclarationNameValS<'s> {
  pub inner: IFunctionDeclarationNameS<'s>,
  pub index: i32,
}

impl<'s> IFunctionDeclarationNameS<'s> {
  pub fn package_coordinate(&self) -> &'s PackageCoordinate<'s> {
    match self {
      IFunctionDeclarationNameS::FunctionName(x) => x.code_location.file.package_coord,
      IFunctionDeclarationNameS::LambdaDeclarationName(x) => x.code_location.file.package_coord,
      IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(r) => {
        r.inner.package_coordinate()
      }
      IFunctionDeclarationNameS::ConstructorName(r) => match &r.tlcd {
        ICitizenDeclarationNameS::TopLevelStructDeclarationName(s) => {
          s.range.begin.file.package_coord
        }
        ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(i) => {
          i.range.begin.file.package_coord
        }
        ICitizenDeclarationNameS::AnonymousSubstructTemplateName(n) => {
          n.interface_name.range.begin.file.package_coord
        }
      },
    }
  }

  /// The imprecise (spelling/lookup) name this declaration carries, wrapped in the matching
  /// `IFunctionImpreciseNameS` variant — mirrors `IVarDeclarationNameS::imprecise_name`.
  pub fn imprecise_name(self) -> IFunctionImpreciseNameS<'s> {
    match self {
      IFunctionDeclarationNameS::FunctionName(x) => {
        IFunctionImpreciseNameS::FunctionName(x.imprecise_name)
      }
      IFunctionDeclarationNameS::LambdaDeclarationName(x) => {
        IFunctionImpreciseNameS::LambdaDeclarationName(x.imprecise_name)
      }
      IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(r) => {
        IFunctionImpreciseNameS::ForwarderFunctionDeclarationName(r.imprecise_name)
      }
      IFunctionDeclarationNameS::ConstructorName(r) => {
        IFunctionImpreciseNameS::ConstructorName(r.imprecise_name)
      }
    }
  }
}

/// The imprecise (spelling/lookup) name of a function declaration — the counterpart to
/// `IFunctionDeclarationNameS`, mirroring how `IImpreciseNameS` is the imprecise side of the
/// variable declaration names. A Copy tagged-pointer enum whose payloads are interned (@SICZ).
/// The two lookup-relevant variants reduce to a shared `CodeNameS` spelling (so env resolution
/// stays on `IImpreciseNameS::CodeName`); the lambda reuses the empty marker (never looked up by
/// name); only the forwarder needs a bespoke payload (it wraps its inner's imprecise name).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IFunctionImpreciseNameS<'s> {
  FunctionName(&'s CodeNameS<'s>),
  ConstructorName(&'s CodeNameS<'s>),
  LambdaDeclarationName(&'s LambdaImpreciseNameS),
  ForwarderFunctionDeclarationName(&'s ForwarderFunctionImpreciseNameS<'s>),
}

/// Value/key form for interning `IFunctionImpreciseNameS` payloads.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IFunctionImpreciseNameValS<'s> {
  FunctionName(CodeNameValS<'s>),
  ConstructorName(CodeNameValS<'s>),
  LambdaDeclarationName(LambdaImpreciseNameValS),
  ForwarderFunctionDeclarationName(ForwarderFunctionImpreciseNameValS<'s>),
}

/// A forwarder's imprecise name wraps its inner function's imprecise name (see the closure/forwarder
/// model). Interned (@SICZ) — carries the witness.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwarderFunctionImpreciseNameS<'s> {
  pub inner: IFunctionImpreciseNameS<'s>,
  pub index: i32,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `ForwarderFunctionImpreciseNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwarderFunctionImpreciseNameValS<'s> {
  pub inner: IFunctionImpreciseNameS<'s>,
  pub index: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImplDeclarationNameS<'s> {
  ImplDeclarationName(ImplDeclarationNameS<'s>),
  AnonymousSubstructImplDeclarationName(AnonymousSubstructImplDeclarationNameS<'s>),
}

impl<'s> IImplDeclarationNameS<'s> {
  pub fn package_coordinate(&self) -> &'s PackageCoordinate<'s> {
    match self {
      IImplDeclarationNameS::ImplDeclarationName(x) => x.code_location.file.package_coord,
      IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(x) => {
        x.interface.range.begin.file.package_coord
      }
    }
  }

  // VCOORD: see if we can get rid of this panic
  // For sites that structurally can only encounter user-source impls (not
  // macro-generated anonymous-substruct impls). Panics on the anon variant.
  pub fn expect_top_level(&self) -> &ImplDeclarationNameS<'s> {
    match self {
      IImplDeclarationNameS::ImplDeclarationName(n) => n,
      IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(_) => {
        panic!("vwat: expected ImplDeclarationName, got AnonymousSubstructImplDeclarationName")
      }
    }
  }

  pub fn to_i_name_s(self, scout_arena: &ScoutArena<'s>) -> INameS<'s> {
    match self {
      IImplDeclarationNameS::ImplDeclarationName(p) => {
        scout_arena.intern_name(INameValS::ImplDeclaration(p))
      }
      IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(p) => {
        let interface_ref =
          match scout_arena.intern_name(INameValS::TopLevelInterfaceDeclaration(p.interface)) {
            INameS::TopLevelInterfaceDeclaration(r) => r,
            _ => unreachable!(),
          };
        scout_arena.intern_name(INameValS::AnonymousSubstructImplDeclaration(
          AnonymousSubstructImplDeclarationNameValS { interface: interface_ref },
        ))
      }
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ICitizenDeclarationNameS<'s> {
  TopLevelStructDeclarationName(TopLevelStructDeclarationNameS<'s>),
  TopLevelInterfaceDeclarationName(TopLevelInterfaceDeclarationNameS<'s>),
  AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameS<'s>),
}
impl<'s> From<TopLevelCitizenDeclarationNameS<'s>> for ICitizenDeclarationNameS<'s> {
  fn from(value: TopLevelCitizenDeclarationNameS<'s>) -> Self {
    match value {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(n) => {
        ICitizenDeclarationNameS::TopLevelStructDeclarationName(n)
      }
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(n) => {
        ICitizenDeclarationNameS::TopLevelInterfaceDeclarationName(n)
      }
    }
  }
}

impl<'s> From<IStructDeclarationNameS<'s>> for ICitizenDeclarationNameS<'s> {
  fn from(value: IStructDeclarationNameS<'s>) -> Self {
    match value {
      IStructDeclarationNameS::TopLevelStructDeclarationName(n) => {
        ICitizenDeclarationNameS::TopLevelStructDeclarationName(n)
      }
      IStructDeclarationNameS::AnonymousSubstructTemplateName(n) => {
        ICitizenDeclarationNameS::AnonymousSubstructTemplateName(n)
      }
    }
  }
}

impl<'s> IStructDeclarationNameS<'s> {
  pub fn range(&self) -> RangeS<'s> {
    match self {
      IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n.range,
      IStructDeclarationNameS::AnonymousSubstructTemplateName(n) => n.interface_name.range,
    }
  }

  // VCOORD: see if we can get rid of this
  // For sites that structurally can only encounter user-source structs (not
  // macro-generated anonymous substructs) — e.g., name-based lookups, top-level
  // citizen conversions. Panics if called on an anonymous substruct name.
  pub fn expect_top_level(&self) -> &TopLevelStructDeclarationNameS<'s> {
    match self {
      IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n,
      IStructDeclarationNameS::AnonymousSubstructTemplateName(_) => {
        panic!("vwat: expected TopLevelStructDeclarationName, got AnonymousSubstructTemplateName")
      }
    }
  }

  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    match self {
      IStructDeclarationNameS::TopLevelStructDeclarationName(n) => {
        scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: n.name }))
      }
      IStructDeclarationNameS::AnonymousSubstructTemplateName(n) => {
        let interface_imprecise_name = n.interface_name.get_imprecise_name(scout_arena);
        scout_arena.intern_imprecise_name(
          IImpreciseNameValS::AnonymousSubstructTemplateImpreciseName(
            AnonymousSubstructTemplateImpreciseNameValS { interface_imprecise_name },
          ),
        )
      }
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaDeclarationNameS<'s> {
  pub imprecise_name: &'s LambdaImpreciseNameS,
  pub code_location: CodeLocationS<'s>,
  pub lid: LocationInDenizen<'s>,
}

impl<'s> LambdaDeclarationNameS<'s> {
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    scout_arena
      .intern_imprecise_name(IImpreciseNameValS::LambdaImpreciseName(LambdaImpreciseNameValS {}))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaImpreciseNameS {
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `LambdaImpreciseNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaImpreciseNameValS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlaceholderImpreciseNameS {
  pub index: i32,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `PlaceholderImpreciseNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlaceholderImpreciseNameValS {
  pub index: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionNameS<'s> {
  pub imprecise_name: &'s CodeNameS<'s>,
  pub code_location: CodeLocationS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwarderFunctionDeclarationNameS<'s> {
  pub inner: IFunctionDeclarationNameS<'s>,
  pub index: i32,
  pub imprecise_name: &'s ForwarderFunctionImpreciseNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TopLevelCitizenDeclarationNameS<'s> {
  TopLevelStructDeclarationName(TopLevelStructDeclarationNameS<'s>),
  TopLevelInterfaceDeclarationName(TopLevelInterfaceDeclarationNameS<'s>),
}

impl<'s> TopLevelCitizenDeclarationNameS<'s> {
  pub fn name(&self) -> StrI<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => x.name,
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => x.name,
    }
  }

  pub fn range(&self) -> RangeS<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => x.range,
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => x.range,
    }
  }

  pub fn package_coordinate(&self) -> &'s PackageCoordinate<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => {
        x.range.begin.file.package_coord
      }
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => {
        x.range.begin.file.package_coord
      }
    }
  }

  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => {
        scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: x.name }))
      }
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => {
        scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: x.name }))
      }
    }
  }
}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TopLevelInterfaceDeclarationNameS<'s> {
  pub name: StrI<'s>,
  pub range: RangeS<'s>,
}

impl<'s> TopLevelInterfaceDeclarationNameS<'s> {
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name: self.name }))
  }
}

impl<'s> From<&TopLevelStructDeclarationNameS<'s>> for TopLevelCitizenDeclarationNameS<'s> {
  fn from(value: &TopLevelStructDeclarationNameS<'s>) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(value.clone())
  }
}

impl<'s> From<&TopLevelInterfaceDeclarationNameS<'s>> for TopLevelCitizenDeclarationNameS<'s> {
  fn from(value: &TopLevelInterfaceDeclarationNameS<'s>) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(value.clone())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructDeclarationNameS<'s> {
  pub lambda_name: LambdaDeclarationNameS<'s>,
}

impl<'s> LambdaStructDeclarationNameS<'s> {
  pub fn get_imprecise_name(&self, scout_arena: &ScoutArena<'s>) -> IImpreciseNameS<'s> {
    let lambda_imprecise_name = self.lambda_name.get_imprecise_name(scout_arena);
    scout_arena.intern_imprecise_name(IImpreciseNameValS::LambdaStructImpreciseName(
      LambdaStructImpreciseNameValS { lambda_name: lambda_imprecise_name },
    ))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructImpreciseNameS<'s> {
  pub lambda_name: IImpreciseNameS<'s>,
  pub _must_intern: ScoutInterned,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDeclarationNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructImplDeclarationNameS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportAsNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamNameS<'s> {
  pub code_location: CodeLocationS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamImpreciseNameS {
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `ClosureParamImpreciseNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamImpreciseNameValS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeNameS {
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `PrototypeNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeNameValS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamImpreciseNameS<'s> {
  pub code_location: CodeLocationS<'s>,
  /// Stable identity (see LIFE/LID design). Shared with the declaration's `lid`; the
  /// already-arena-allocated slice means this needs no `'tmp` deferral (see @DSAUIMZ), unlike
  /// a builder-borrowed lid — the imprecise name holds the canonical lid directly.
  pub lid: LocationInDenizen<'s>,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `MagicParamNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamNameValS<'s> {
  pub code_location: CodeLocationS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DesugaredParamNameS<'s> {
  pub code_location: CodeLocationS<'s>,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `DesugaredParamNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DesugaredParamNameValS<'s> {
  pub code_location: CodeLocationS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateNameS<'s> {
  pub interface_name: TopLevelInterfaceDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateImpreciseNameS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
  pub _must_intern: ScoutInterned,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructConstructorTemplateImpreciseNameS<'s> {
  pub interface_imprecise_name: IImpreciseNameS<'s>,
  pub _must_intern: ScoutInterned,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberNameS {
  pub index: i32,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `AnonymousSubstructMemberNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberNameValS {
  pub index: i32,
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeVarNameS<'s> {
  pub imprecise_name: &'s CodeNameS<'s>,
  /// Disambiguates same-named locals across scopes, so structural equality is identity
  /// equality — which is why a declaration name is never interned (see @WVSBIZ).
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructingMemberNameS<'s> {
  pub name: StrI<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterableNameS<'s> {
  pub range: RangeS<'s>,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `IterableNameS` (the Val side of the dual-enum).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterableNameValS<'s> {
  pub range: RangeS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IteratorNameS<'s> {
  pub range: RangeS<'s>,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `IteratorNameS` (the Val side of the dual-enum).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IteratorNameValS<'s> {
  pub range: RangeS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterationOptionNameS<'s> {
  pub range: RangeS<'s>,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `IterationOptionNameS` (the Val side of the dual-enum).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterationOptionNameValS<'s> {
  pub range: RangeS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileCondResultNameS<'s> {
  pub range: RangeS<'s>,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `WhileCondResultNameS` (the Val side of the dual-enum).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileCondResultNameValS<'s> {
  pub range: RangeS<'s>,
}

// Declaration-name payloads: each embeds its corresponding `*ImpreciseNameS` (the use-site
// name a source spelling resolves through) plus the declaration's `lid` (its unique identity,
// see @WVSBIZ). Built directly, never interned; the embedded imprecise name is a plain value
// here and only gets interned when `imprecise_name()` hands out the canonical form.

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructingMemberNameDeclarationS<'s> {
  pub imprecise_name: &'s ConstructingMemberImpreciseNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamNameDeclarationS<'s> {
  pub imprecise_name: &'s ClosureParamImpreciseNameS,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamNameDeclarationS<'s> {
  pub imprecise_name: &'s MagicParamImpreciseNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterableNameDeclarationS<'s> {
  pub imprecise_name: &'s IterableNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IteratorNameDeclarationS<'s> {
  pub imprecise_name: &'s IteratorNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterationOptionNameDeclarationS<'s> {
  pub imprecise_name: &'s IterationOptionNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileCondResultNameDeclarationS<'s> {
  pub imprecise_name: &'s WhileCondResultNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfNameDeclarationS<'s> {
  pub imprecise_name: &'s SelfNameS,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberNameDeclarationS<'s> {
  pub imprecise_name: &'s AnonymousSubstructMemberNameS,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DesugaredParamNameDeclarationS<'s> {
  pub imprecise_name: &'s DesugaredParamNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneNameS<'s> {
  pub rune: IRuneS<'s>,
  pub _must_intern: ScoutInterned,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimeSizedArrayDeclarationNameS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticSizedArrayDeclarationNameS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IRuneS<'s> {
  CodeRune(&'s CodeRuneS<'s>),
  ImplDropKindRune(&'s ImplDropKindRuneS),
  ImplDropVoidRune(&'s ImplDropVoidRuneS),
  ImplicitRune(&'s ImplicitRuneS<'s>),
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
  ImplicitCoercionTemplateRune(&'s ImplicitCoercionTemplateRuneS<'s>),
  ArraySizeImplicitRune(&'s ArraySizeImplicitRuneS),
  ArrayMutabilityImplicitRune(&'s ArrayMutabilityImplicitRuneS),
  ReturnRune(&'s ReturnRuneS),
  StructNameRune(&'s StructNameRuneS<'s>),
  InterfaceNameRune(&'s InterfaceNameRuneS<'s>),
  SelfRune(&'s SelfRuneS),
  SelfKindRune(&'s SelfKindRuneS),
  SelfFullTypeRune(&'s SelfFullTypeRuneS),
  SelfKindTemplateRune(&'s SelfKindTemplateRuneS<'s>),
  MacroVoidKindRune(&'s MacroVoidKindRuneS),
  MacroSelfKindRune(&'s MacroSelfKindRuneS),
  MacroSelfKindTemplateRune(&'s MacroSelfKindTemplateRuneS),
  ArgumentRune(&'s ArgumentRuneS),
  PatternInputRune(&'s PatternInputRuneS<'s>),
  ExplicitTemplateArgRune(&'s ExplicitTemplateArgRuneS),
  AnonymousSubstructParentInterfaceTemplateRune(&'s AnonymousSubstructParentInterfaceTemplateRuneS),
  AnonymousSubstructParentInterfaceKindRune(&'s AnonymousSubstructParentInterfaceKindRuneS),
  AnonymousSubstructTemplateRune(&'s AnonymousSubstructTemplateRuneS),
  AnonymousSubstructKindRune(&'s AnonymousSubstructKindRuneS),
  AnonymousSubstructVoidKindRune(&'s AnonymousSubstructVoidKindRuneS),
  AnonymousSubstructMemberRune(&'s AnonymousSubstructMemberRuneS<'s>),
  AnonymousSubstructMethodSelfBorrowKindRune(&'s AnonymousSubstructMethodSelfBorrowKindRuneS<'s>),
  AnonymousSubstructDropBoundPrototypeRune(&'s AnonymousSubstructDropBoundPrototypeRuneS<'s>),
  AnonymousSubstructDropBoundParamsListRune(&'s AnonymousSubstructDropBoundParamsListRuneS<'s>),
  StructDropBoundPrototypeRune(&'s StructDropBoundPrototypeRuneS<'s>),
  StructDropBoundParamsListRune(&'s StructDropBoundParamsListRuneS<'s>),
  AnonymousSubstructFunctionBoundPrototypeRune(
    &'s AnonymousSubstructFunctionBoundPrototypeRuneS<'s>,
  ),
  AnonymousSubstructFunctionBoundParamsListRune(
    &'s AnonymousSubstructFunctionBoundParamsListRuneS<'s>,
  ),
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
      IRuneS::ImplDropKindRune(r) => *r as *const _ as *const (),
      IRuneS::ImplDropVoidRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitRune(r) => *r as *const _ as *const (),
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
      IRuneS::ImplicitCoercionTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::ArraySizeImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ArrayMutabilityImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ReturnRune(r) => *r as *const _ as *const (),
      IRuneS::StructNameRune(r) => *r as *const _ as *const (),
      IRuneS::InterfaceNameRune(r) => *r as *const _ as *const (),
      IRuneS::SelfRune(r) => *r as *const _ as *const (),
      IRuneS::SelfKindRune(r) => *r as *const _ as *const (),
      IRuneS::SelfFullTypeRune(r) => *r as *const _ as *const (),
      IRuneS::SelfKindTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::MacroVoidKindRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfKindRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfKindTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::ArgumentRune(r) => *r as *const _ as *const (),
      IRuneS::PatternInputRune(r) => *r as *const _ as *const (),
      IRuneS::ExplicitTemplateArgRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructVoidKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMemberRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMethodSelfBorrowKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructDropBoundPrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructDropBoundParamsListRune(r) => *r as *const _ as *const (),
      IRuneS::StructDropBoundPrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::StructDropBoundParamsListRune(r) => *r as *const _ as *const (),
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
}

/// Value-struct for ImplicitRegionRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRegionRuneValS<'s> {
  pub original_rune: IRuneS<'s>,
}

/// Value-struct for ImplicitCoercionTemplateRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionTemplateRuneValS<'s> {
  pub range: RangeS<'s>,
  pub original_kind_rune: IRuneS<'s>,
}

/// Value-struct for AnonymousSubstructMethodInheritedRuneS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodInheritedRuneValS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
  pub inner: IRuneS<'s>,
}

/// Value-struct for DispatcherRuneFromImplS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DispatcherRuneFromImplValS<'s> {
  pub inner_rune: IRuneS<'s>,
}

/// Value-struct for CaseRuneFromImplS key. Shallow: references canonical child rune.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseRuneFromImplValS<'s> {
  pub inner_rune: IRuneS<'s>,
}

// Per @DSAUIMZ, these Val structs have private lid fields to prevent pre-allocation.
// Only constructible via new() which takes a LocationInDenizenVal from borrow_val().

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRuneValS<'tmp> {
  lid: LocationInDenizenVal<'tmp>,
}
impl<'tmp> ImplicitRuneValS<'tmp> {
  pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self {
    Self { lid }
  }
  pub fn lid(&self) -> LocationInDenizenVal<'tmp> {
    self.lid
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallRegionRuneValS<'tmp> {
  lid: LocationInDenizenVal<'tmp>,
}
impl<'tmp> CallRegionRuneValS<'tmp> {
  pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self {
    Self { lid }
  }
  pub fn lid(&self) -> LocationInDenizenVal<'tmp> {
    self.lid
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallPureMergeRegionRuneValS<'tmp> {
  lid: LocationInDenizenVal<'tmp>,
}
impl<'tmp> CallPureMergeRegionRuneValS<'tmp> {
  pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self {
    Self { lid }
  }
  pub fn lid(&self) -> LocationInDenizenVal<'tmp> {
    self.lid
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetImplicitRuneValS<'tmp> {
  lid: LocationInDenizenVal<'tmp>,
}
impl<'tmp> LetImplicitRuneValS<'tmp> {
  pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self {
    Self { lid }
  }
  pub fn lid(&self) -> LocationInDenizenVal<'tmp> {
    self.lid
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamRuneValS<'tmp> {
  lid: LocationInDenizenVal<'tmp>,
}
impl<'tmp> MagicParamRuneValS<'tmp> {
  pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self {
    Self { lid }
  }
  pub fn lid(&self) -> LocationInDenizenVal<'tmp> {
    self.lid
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalDefaultRegionRuneValS<'tmp> {
  lid: LocationInDenizenVal<'tmp>,
}
impl<'tmp> LocalDefaultRegionRuneValS<'tmp> {
  pub fn new(lid: LocationInDenizenVal<'tmp>) -> Self {
    Self { lid }
  }
  pub fn lid(&self) -> LocationInDenizenVal<'tmp> {
    self.lid
  }
}

/// Per @DSAUIMZ, 'tmp carries a temporary borrow to defer slice allocation.
/// Value/key form of rune for interner lookups. Used when constructing runes before
/// canonicalizing via `intern_rune`. Storage fields use canonical `IRuneS<'s>`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IRuneValS<'s, 'tmp> {
  CodeRune(CodeRuneS<'s>),
  ImplDropKindRune(ImplDropKindRuneS),
  ImplDropVoidRune(ImplDropVoidRuneS),
  ImplicitRune(ImplicitRuneValS<'tmp>),
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
  ImplicitCoercionTemplateRune(ImplicitCoercionTemplateRuneValS<'s>),
  ArraySizeImplicitRune(ArraySizeImplicitRuneS),
  ArrayMutabilityImplicitRune(ArrayMutabilityImplicitRuneS),
  ReturnRune(ReturnRuneS),
  StructNameRune(StructNameRuneS<'s>),
  InterfaceNameRune(InterfaceNameRuneS<'s>),
  SelfRune(SelfRuneS),
  SelfKindRune(SelfKindRuneS),
  SelfFullTypeRune(SelfFullTypeRuneS),
  SelfKindTemplateRune(SelfKindTemplateRuneS<'s>),
  MacroVoidKindRune(MacroVoidKindRuneS),
  MacroSelfKindRune(MacroSelfKindRuneS),
  MacroSelfKindTemplateRune(MacroSelfKindTemplateRuneS),
  ArgumentRune(ArgumentRuneS),
  PatternInputRune(PatternInputRuneS<'s>),
  ExplicitTemplateArgRune(ExplicitTemplateArgRuneS),
  AnonymousSubstructParentInterfaceTemplateRune(AnonymousSubstructParentInterfaceTemplateRuneS),
  AnonymousSubstructParentInterfaceKindRune(AnonymousSubstructParentInterfaceKindRuneS),
  AnonymousSubstructTemplateRune(AnonymousSubstructTemplateRuneS),
  AnonymousSubstructKindRune(AnonymousSubstructKindRuneS),
  AnonymousSubstructVoidKindRune(AnonymousSubstructVoidKindRuneS),
  AnonymousSubstructMemberRune(AnonymousSubstructMemberRuneS<'s>),
  AnonymousSubstructMethodSelfBorrowKindRune(AnonymousSubstructMethodSelfBorrowKindRuneS<'s>),
  AnonymousSubstructDropBoundPrototypeRune(AnonymousSubstructDropBoundPrototypeRuneS<'s>),
  AnonymousSubstructDropBoundParamsListRune(AnonymousSubstructDropBoundParamsListRuneS<'s>),
  StructDropBoundPrototypeRune(StructDropBoundPrototypeRuneS<'s>),
  StructDropBoundParamsListRune(StructDropBoundParamsListRuneS<'s>),
  AnonymousSubstructFunctionBoundPrototypeRune(AnonymousSubstructFunctionBoundPrototypeRuneS<'s>),
  AnonymousSubstructFunctionBoundParamsListRune(AnonymousSubstructFunctionBoundParamsListRuneS<'s>),
  AnonymousSubstructFunctionInterfaceTemplateRune(
    AnonymousSubstructFunctionInterfaceTemplateRuneS<'s>,
  ),
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
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl<'a, 's, 'tmp> hashbrown::Equivalent<IRuneValS<'s, 's>> for RuneValQuery<'a, 's, 'tmp> {
  fn equivalent(&self, key: &IRuneValS<'s, 's>) -> bool {
    match (self.0, key) {
      // 7 lid variants: compare path contents
      (ImplicitRune(a), ImplicitRune(b)) => a.lid().path() == b.lid().path(),
      (CallRegionRune(a), CallRegionRune(b)) => a.lid().path() == b.lid().path(),
      (CallPureMergeRegionRune(a), CallPureMergeRegionRune(b)) => a.lid().path() == b.lid().path(),
      (LetImplicitRune(a), LetImplicitRune(b)) => a.lid().path() == b.lid().path(),
      (MagicParamRune(a), MagicParamRune(b)) => a.lid().path() == b.lid().path(),
      (LocalDefaultRegionRune(a), LocalDefaultRegionRune(b)) => a.lid().path() == b.lid().path(),
      // All other variants: same inner type on both sides, delegate to PartialEq
      (CodeRune(a), CodeRune(b)) => a == b,
      (ImplDropKindRune(a), ImplDropKindRune(b)) => a == b,
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
      (ImplicitCoercionTemplateRune(a), ImplicitCoercionTemplateRune(b)) => a == b,
      (ArraySizeImplicitRune(a), ArraySizeImplicitRune(b)) => a == b,
      (ArrayMutabilityImplicitRune(a), ArrayMutabilityImplicitRune(b)) => a == b,
      (ReturnRune(a), ReturnRune(b)) => a == b,
      (StructNameRune(a), StructNameRune(b)) => a == b,
      (InterfaceNameRune(a), InterfaceNameRune(b)) => a == b,
      (SelfRune(a), SelfRune(b)) => a == b,
      (SelfKindRune(a), SelfKindRune(b)) => a == b,
      (SelfFullTypeRune(a), SelfFullTypeRune(b)) => a == b,
      (SelfKindTemplateRune(a), SelfKindTemplateRune(b)) => a == b,
      (MacroVoidKindRune(a), MacroVoidKindRune(b)) => a == b,
      (MacroSelfKindRune(a), MacroSelfKindRune(b)) => a == b,
      (MacroSelfKindTemplateRune(a), MacroSelfKindTemplateRune(b)) => a == b,
      (ArgumentRune(a), ArgumentRune(b)) => a == b,
      (PatternInputRune(a), PatternInputRune(b)) => a == b,
      (ExplicitTemplateArgRune(a), ExplicitTemplateArgRune(b)) => a == b,
      (
        AnonymousSubstructParentInterfaceTemplateRune(a),
        AnonymousSubstructParentInterfaceTemplateRune(b),
      ) => a == b,
      (
        AnonymousSubstructParentInterfaceKindRune(a),
        AnonymousSubstructParentInterfaceKindRune(b),
      ) => a == b,
      (AnonymousSubstructTemplateRune(a), AnonymousSubstructTemplateRune(b)) => a == b,
      (AnonymousSubstructKindRune(a), AnonymousSubstructKindRune(b)) => a == b,
      (AnonymousSubstructVoidKindRune(a), AnonymousSubstructVoidKindRune(b)) => a == b,
      (AnonymousSubstructMemberRune(a), AnonymousSubstructMemberRune(b)) => a == b,
      (
        AnonymousSubstructMethodSelfBorrowKindRune(a),
        AnonymousSubstructMethodSelfBorrowKindRune(b),
      ) => a == b,
      (
        AnonymousSubstructDropBoundPrototypeRune(a),
        AnonymousSubstructDropBoundPrototypeRune(b),
      ) => a == b,
      (
        AnonymousSubstructDropBoundParamsListRune(a),
        AnonymousSubstructDropBoundParamsListRune(b),
      ) => a == b,
      (StructDropBoundPrototypeRune(a), StructDropBoundPrototypeRune(b)) => a == b,
      (StructDropBoundParamsListRune(a), StructDropBoundParamsListRune(b)) => a == b,
      (
        AnonymousSubstructFunctionBoundPrototypeRune(a),
        AnonymousSubstructFunctionBoundPrototypeRune(b),
      ) => a == b,
      (
        AnonymousSubstructFunctionBoundParamsListRune(a),
        AnonymousSubstructFunctionBoundParamsListRune(b),
      ) => a == b,
      (
        AnonymousSubstructFunctionInterfaceTemplateRune(a),
        AnonymousSubstructFunctionInterfaceTemplateRune(b),
      ) => a == b,
      (
        AnonymousSubstructFunctionInterfaceKindRune(a),
        AnonymousSubstructFunctionInterfaceKindRune(b),
      ) => a == b,
      (AnonymousSubstructMethodInheritedRune(a), AnonymousSubstructMethodInheritedRune(b)) => {
        a == b
      }
      (FunctorPrototypeRuneName(a), FunctorPrototypeRuneName(b)) => a == b,
      (FunctorParamRuneName(a), FunctorParamRuneName(b)) => a == b,
      (FunctorReturnRuneName(a), FunctorReturnRuneName(b)) => a == b,
      (DispatcherRuneFromImpl(a), DispatcherRuneFromImpl(b)) => a == b,
      (CaseRuneFromImpl(a), CaseRuneFromImpl(b)) => a == b,
      _ => false,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeRuneS<'s> {
  pub name: StrI<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDropKindRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDropVoidRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallRegionRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallPureMergeRegionRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRegionRuneS<'s> {
  pub original_rune: IRuneS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReachablePrototypeRuneS {
  pub num: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideStructTemplateRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideStructRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideInterfaceRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetImplicitRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemberRuneS {
  pub member_index: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalDefaultRegionRuneS<'s> {
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DenizenDefaultRegionRuneS<'s> {
  pub denizen_name: INameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportDefaultRegionRuneS<'s> {
  pub denizen_name: INameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternDefaultRegionRuneS<'s> {
  pub denizen_name: INameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionTemplateRuneS<'s> {
  pub range: RangeS<'s>,
  pub original_kind_rune: IRuneS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArraySizeImplicitRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayMutabilityImplicitRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReturnRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructNameRuneS<'s> {
  pub struct_name: ICitizenDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceNameRuneS<'s> {
  pub interface_name: ICitizenDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfKindRuneS {}

/// Self's full type: the value type of `SelfKindRuneS` inside whatever reference
/// wraps the abstract method's self parameter declared (see @PFVSZ).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfFullTypeRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfKindTemplateRuneS<'s> {
  pub loc: CodeLocationS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroVoidKindRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfKindRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfKindTemplateRuneS {}

/// Interned imprecise name (sealed, @SICZ): obtainable only via `ScoutArena::intern_code_name` /
/// `intern_imprecise_name`. The `_must_intern` witness makes the canonical `&'s CodeNameS`
/// unforgeable. The freely-constructible lookup key is `CodeNameValS`.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CodeNameS<'s> {
  pub name: StrI<'s>,
  pub _must_intern: ScoutInterned,
}

// Hide the `_must_intern` witness (see @SICZ) from Debug: it's an internal sealing token
// with no information, and it otherwise leaks into every humanized name and error snapshot.
impl<'s> Debug for CodeNameS<'s> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("CodeNameS").field("name", &self.name).finish()
  }
}

// VCOORD: rename CodeNameValS to CodeNameKeyS or some other name to say its the interning key
/// Freely-constructible lookup key for `CodeNameS` (the Val side of the dual-enum).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeNameValS<'s> {
  pub name: StrI<'s>,
}

/// Imprecise (use-site) name for a `self.x` constructing-member reference. Distinct from
/// `CodeName` so a member access doesn't collide with a same-spelled local read.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructingMemberImpreciseNameS<'s> {
  pub name: StrI<'s>,
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `ConstructingMemberImpreciseNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructingMemberImpreciseNameValS<'s> {
  pub name: StrI<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalFunctionFamilyNameS<'s> {
  pub name: StrI<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArgumentRuneS {
  pub arg_index: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PatternInputRuneS<'s> {
  pub code_loc: CodeLocationS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExplicitTemplateArgRuneS {
  pub index: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceTemplateRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceKindRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructKindRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructVoidKindRuneS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodSelfBorrowKindRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundPrototypeRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundParamsListRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

/// The prototype rune of a struct's auto-generated drop's synthesized `where func drop(T)void`
/// bound. Keyed on the struct's generic-parameter rune `T`, so each stored type parameter gets its
/// own distinct bound within the drop.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructDropBoundPrototypeRuneS<'s> {
  pub param_rune: IRuneS<'s>,
}

/// The params-list rune of a struct's auto-generated drop's synthesized `where func drop(T)void`
/// bound (see `StructDropBoundPrototypeRuneS`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructDropBoundParamsListRuneS<'s> {
  pub param_rune: IRuneS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundPrototypeRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundParamsListRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceTemplateRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceKindRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodInheritedRuneS<'s> {
  pub interface: TopLevelInterfaceDeclarationNameS<'s>,
  pub method: IFunctionDeclarationNameS<'s>,
  pub inner: IRuneS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorPrototypeRuneNameS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorParamRuneNameS {
  pub index: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorReturnRuneNameS {}

// Vale has no notion of Self, it's just a convenient name for a first parameter.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfNameS {
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `SelfNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfNameValS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArbitraryNameS {
  pub _must_intern: ScoutInterned,
}
/// Freely-constructible lookup key for `ArbitraryNameS`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArbitraryNameValS {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DispatcherRuneFromImplS<'s> {
  pub inner_rune: IRuneS<'s>,
}

// Only made by typingpass, see if we can take these out
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseRuneFromImplS<'s> {
  pub inner_rune: IRuneS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructorNameS<'s> {
  pub tlcd: ICitizenDeclarationNameS<'s>,
  pub imprecise_name: &'s CodeNameS<'s>,
  pub lid: LocationInDenizen<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplImpreciseNameS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
  pub _must_intern: ScoutInterned,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSubCitizenImpreciseNameS<'s> {
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub _must_intern: ScoutInterned,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSuperInterfaceImpreciseNameS<'s> {
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
  pub _must_intern: ScoutInterned,
}
