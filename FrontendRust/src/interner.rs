use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::postparsing::names::{
  IImpreciseNameS, IImpreciseNameValS, INameS, INameValS, IRuneS, IRuneValS,
  IFunctionDeclarationNameS, IFunctionDeclarationNameValS, IVarNameS, IVarNameValS,
  ImplicitRegionRuneS, ImplicitCoercionOwnershipRuneS, ImplicitCoercionKindRuneS,
  RuneNameS,
  ImplicitCoercionTemplateRuneS, AnonymousSubstructMethodInheritedRuneS,
  DispatcherRuneFromImplS, CaseRuneFromImplS,
  LambdaStructImpreciseNameS,
  AnonymousSubstructTemplateImpreciseNameS,
  AnonymousSubstructConstructorTemplateImpreciseNameS, ImplImpreciseNameS,
  ImplSubCitizenImpreciseNameS, ImplSuperInterfaceImpreciseNameS,
};
use bumpalo::Bump;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;


/// Interned string: a by-value wrapper around arena-backed `&'a str`.
/// Never arena-allocated; just holds a reference to canonical storage.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct StrI<'a>(pub &'a str);

impl<'a> StrI<'a> {
  pub fn as_str(&self) -> &'a str {
    self.0
  }
}

impl Deref for StrI<'_> {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl Debug for StrI<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(self.0, f)
  }
}

impl Display for StrI<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Display::fmt(self.0, f)
  }
}

impl PartialEq<&str> for StrI<'_> {
  fn eq(&self, other: &&str) -> bool {
    self.0 == *other
  }
}

impl PartialEq<str> for StrI<'_> {
  fn eq(&self, other: &str) -> bool {
    self.0 == other
  }
}

#[derive(Copy, Clone)]
pub struct InternedSlice<'a, T: Copy> {
  slice: &'a [T],
}

impl<'a, T: Copy> InternedSlice<'a, T> {
  pub fn new(slice: &'a [T]) -> Self {
    InternedSlice { slice }
  }

  pub fn as_slice(&self) -> &'a [T] {
    self.slice
  }

  pub fn iter(&self) -> std::slice::Iter<'a, T> {
    self.slice.iter()
  }

  pub fn is_empty(&self) -> bool {
    self.slice.is_empty()
  }

  pub fn len(&self) -> usize {
    self.slice.len()
  }
}

impl<'a, T: Copy> IntoIterator for &InternedSlice<'a, T> {
  type Item = &'a T;
  type IntoIter = std::slice::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.slice.iter()
  }
}

impl<T: Copy + Debug> Debug for InternedSlice<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(self.slice, f)
  }
}

impl<T: Copy + PartialEq> PartialEq for InternedSlice<'_, T> {
  fn eq(&self, other: &Self) -> bool {
    self.slice == other.slice
  }
}

impl<T: Copy + Eq> Eq for InternedSlice<'_, T> {}

impl<T: Copy + Hash> Hash for InternedSlice<'_, T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.slice.hash(state);
  }
}

/// Generic interning system with interior mutability
/// Matches Scala's Interner
pub struct Interner<'a> {
  arena: &'a Bump,
  inner: RefCell<InternerInner<'a>>,
  _marker: PhantomData<&'a str>,
}

/// Lookup key for file coordinates; uses String for filepath to allow lookup with arbitrary &str.
#[derive(Clone)]
struct FileCoordLookupKey<'a> {
  package_coord: &'a PackageCoordinate<'a>,
  filepath: String,
}

impl<'a> PartialEq for FileCoordLookupKey<'a> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.package_coord, other.package_coord) && self.filepath == other.filepath
  }
}
impl<'a> Eq for FileCoordLookupKey<'a> {}

impl<'a> std::hash::Hash for FileCoordLookupKey<'a> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    (self.package_coord as *const PackageCoordinate<'_>).hash(state);
    self.filepath.hash(state);
  }
}

struct InternerInner<'a> {
  string_to_interned: HashMap<String, &'a str>,
  package_coord_to_ref: HashMap<PackageCoordinate<'a>, &'a PackageCoordinate<'a>>,
  file_coord_to_ref: HashMap<FileCoordLookupKey<'a>, &'a FileCoordinate<'a>>,
  imprecise_name_val_to_ref: HashMap<IImpreciseNameValS<'a>, IImpreciseNameS<'a>>,
  name_val_to_ref: HashMap<INameValS<'a>, INameS<'a>>,
  rune_val_to_ref: HashMap<IRuneValS<'a>, IRuneS<'a>>,
}

impl<'a> Interner<'a> {
  pub fn with_arena(arena: &'a Bump) -> Self {
    Interner {
      arena,
      inner: RefCell::new(InternerInner {
        string_to_interned: HashMap::new(),
        package_coord_to_ref: HashMap::new(),
        file_coord_to_ref: HashMap::new(),
        imprecise_name_val_to_ref: HashMap::new(),
        name_val_to_ref: HashMap::new(),
        rune_val_to_ref: HashMap::new(),
      }),
      _marker: PhantomData,
    }
  }

  /// Intern a string, returning a canonical value.
  pub fn intern(&self, s: &str) -> StrI<'a> {
    let mut inner = self.inner.borrow_mut();
    if let Some(&existing) = inner.string_to_interned.get(s) {
      return StrI(existing);
    }

    let arena_str = self.arena.alloc_str(s);
    inner.string_to_interned.insert(s.to_string(), arena_str);
    StrI(arena_str)
  }


  /// Intern a PackageCoordinate.
  pub fn intern_package_coordinate(
    &self,
    module: StrI<'a>,
    packages: &[StrI<'a>],
  ) -> &'a PackageCoordinate<'a> {
    let mut inner = self.inner.borrow_mut();
    let lookup_coord = PackageCoordinate {
      module,
      packages: InternedSlice::new(packages),
    };
    if let Some(existing) = inner.package_coord_to_ref.get(&lookup_coord) {
      return *existing;
    }
    let arena_packages = self.arena.alloc_slice_copy(packages);
    let coord = PackageCoordinate {
      module,
      packages: InternedSlice::new(arena_packages),
    };
    let new_ref: &'a PackageCoordinate<'a> = self.arena.alloc(coord.clone());
    inner.package_coord_to_ref.insert(coord, new_ref);
    new_ref
  }

  /// Intern a FileCoordinate
  pub fn intern_file_coordinate(
    &self,
    package_coord: &'a PackageCoordinate<'a>,
    filepath: &str,
  ) -> &'a FileCoordinate<'a> {
    let mut inner = self.inner.borrow_mut();
    let lookup_key = FileCoordLookupKey {
      package_coord,
      filepath: filepath.to_string(),
    };
    if let Some(existing) = inner.file_coord_to_ref.get(&lookup_key) {
      return *existing;
    }
    let arena_filepath = self.arena.alloc_str(filepath);
    let coord = FileCoordinate {
      package_coord,
      filepath: StrI(arena_filepath),
    };
    let new_ref: &'a FileCoordinate<'a> = self.arena.alloc(coord.clone());
    let insert_key = FileCoordLookupKey {
      package_coord,
      filepath: filepath.to_string(),
    };
    inner.file_coord_to_ref.insert(insert_key, new_ref);
    new_ref
  }

  /// Canonical imprecise-name entrypoint: intern an IImpreciseNameValS value key and return canonical IImpreciseNameS<'a>.
  pub fn intern_imprecise_name(&self, val: IImpreciseNameValS<'a>) -> IImpreciseNameS<'a> {
    {
      let inner = self.inner.borrow();
      if let Some(existing) = inner.imprecise_name_val_to_ref.get(&val) {
        return existing.clone();
      }
    }
    let canonical: IImpreciseNameS<'a> = self.alloc_imprecise_name_canonical(val.clone());
    let mut inner = self.inner.borrow_mut();
    inner.imprecise_name_val_to_ref.insert(val, canonical.clone());
    canonical
  }

  fn alloc_imprecise_name_canonical(&self, val: IImpreciseNameValS<'a>) -> IImpreciseNameS<'a> {
    use crate::postparsing::names::IImpreciseNameValS::*;
    match val {
      CodeName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::CodeName(r)
      }
      IterableName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::IterableName(r)
      }
      IteratorName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::IteratorName(r)
      }
      IterationOptionName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::IterationOptionName(r)
      }
      LambdaImpreciseName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::LambdaImpreciseName(r)
      }
      PlaceholderImpreciseName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::PlaceholderImpreciseName(r)
      }
      LambdaStructImpreciseName(v) => {
        // Shallow key: v.lambda_name is already canonical; use directly.
        let payload = LambdaStructImpreciseNameS {
          lambda_name: v.lambda_name,
        };
        let r = self.arena.alloc(payload);
        IImpreciseNameS::LambdaStructImpreciseName(r)
      }
      ClosureParamImpreciseName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::ClosureParamImpreciseName(r)
      }
      PrototypeName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::PrototypeName(r)
      }
      AnonymousSubstructTemplateImpreciseName(v) => {
        // Shallow key: v.interface_imprecise_name is already canonical; use directly.
        let payload = AnonymousSubstructTemplateImpreciseNameS {
          interface_imprecise_name: v.interface_imprecise_name,
        };
        let r = self.arena.alloc(payload);
        IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(r)
      }
      AnonymousSubstructConstructorTemplateImpreciseName(v) => {
        // Shallow key: v.interface_imprecise_name is already canonical; use directly.
        let payload = AnonymousSubstructConstructorTemplateImpreciseNameS {
          interface_imprecise_name: v.interface_imprecise_name,
        };
        let r = self.arena.alloc(payload);
        IImpreciseNameS::AnonymousSubstructConstructorTemplateImpreciseName(r)
      }
      ImplImpreciseName(v) => {
        // Shallow key: both children already canonical; use directly.
        let payload = ImplImpreciseNameS {
          sub_citizen_imprecise_name: v.sub_citizen_imprecise_name,
          super_interface_imprecise_name: v.super_interface_imprecise_name,
        };
        let r = self.arena.alloc(payload);
        IImpreciseNameS::ImplImpreciseName(r)
      }
      ImplSubCitizenImpreciseName(v) => {
        // Shallow key: v.sub_citizen_imprecise_name is already canonical; use directly.
        let payload = ImplSubCitizenImpreciseNameS {
          sub_citizen_imprecise_name: v.sub_citizen_imprecise_name,
        };
        let r = self.arena.alloc(payload);
        IImpreciseNameS::ImplSubCitizenImpreciseName(r)
      }
      ImplSuperInterfaceImpreciseName(v) => {
        // Shallow key: v.super_interface_imprecise_name is already canonical; use directly.
        let payload = ImplSuperInterfaceImpreciseNameS {
          super_interface_imprecise_name: v.super_interface_imprecise_name,
        };
        let r = self.arena.alloc(payload);
        IImpreciseNameS::ImplSuperInterfaceImpreciseName(r)
      }
      SelfName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::SelfName(r)
      }
      RuneName(v) => {
        // Shallow key: v.rune is already canonical; use directly.
        let payload = RuneNameS { rune: v.rune };
        let r = self.arena.alloc(payload);
        IImpreciseNameS::RuneName(r)
      }
      ArbitraryName(p) => {
        let r = self.arena.alloc(p);
        IImpreciseNameS::ArbitraryName(r)
      }
    }
  }

  /// Canonical name entrypoint: intern an INameValS value key and return canonical INameS<'a>.
  pub fn intern_name(&self, val: INameValS<'a>) -> INameS<'a> {
    {
      let inner = self.inner.borrow();
      if let Some(existing) = inner.name_val_to_ref.get(&val) {
        return existing.clone();
      }
    }
    let canonical = self.alloc_name_canonical(val.clone());
    let mut inner = self.inner.borrow_mut();
    inner.name_val_to_ref.insert(val, canonical.clone());
    canonical
  }

  fn alloc_name_canonical(&self, val: INameValS<'a>) -> INameS<'a> {
    use crate::postparsing::names::{
      AnonymousSubstructImplDeclarationNameValS, AnonymousSubstructTemplateNameValS,
    };
    match val {
      INameValS::FunctionDeclaration(v) => {
        let inner = self.alloc_function_declaration_name_canonical(v);
        let r = self.arena.alloc(inner);
        INameS::FunctionDeclaration(r)
      }
      INameValS::ImplDeclaration(p) => {
        let r = self.arena.alloc(p);
        INameS::ImplDeclaration(r)
      }
      INameValS::AnonymousSubstructImplDeclaration(AnonymousSubstructImplDeclarationNameValS {
        interface,
      }) => {
        let payload = crate::postparsing::names::AnonymousSubstructImplDeclarationNameS {
          interface: interface.clone(),
        };
        let r = self.arena.alloc(payload);
        INameS::AnonymousSubstructImplDeclaration(r)
      }
      INameValS::ExportAsName(p) => {
        let r = self.arena.alloc(p);
        INameS::ExportAsName(r)
      }
      INameValS::LetName(p) => {
        let r = self.arena.alloc(p);
        INameS::LetName(r)
      }
      INameValS::TopLevelStructDeclaration(p) => {
        let r = self.arena.alloc(p);
        INameS::TopLevelStructDeclaration(r)
      }
      INameValS::TopLevelInterfaceDeclaration(p) => {
        let r = self.arena.alloc(p);
        INameS::TopLevelInterfaceDeclaration(r)
      }
      INameValS::LambdaStructDeclaration(p) => {
        let r = self.arena.alloc(p);
        INameS::LambdaStructDeclaration(r)
      }
      INameValS::AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameValS {
        interface_name,
      }) => {
        let payload = crate::postparsing::names::AnonymousSubstructTemplateNameS {
          interface_name: interface_name.clone(),
        };
        let r = self.arena.alloc(payload);
        INameS::AnonymousSubstructTemplateName(r)
      }
      INameValS::RuneName(v) => {
        let payload = RuneNameS { rune: v.rune };
        let r = self.arena.alloc(payload);
        INameS::RuneName(r)
      }
      INameValS::RuntimeSizedArrayDeclarationName(p) => {
        let r = self.arena.alloc(p);
        INameS::RuntimeSizedArrayDeclarationName(r)
      }
      INameValS::StaticSizedArrayDeclarationName(p) => {
        let r = self.arena.alloc(p);
        INameS::StaticSizedArrayDeclarationName(r)
      }
      INameValS::GlobalFunctionFamilyName(p) => {
        let r = self.arena.alloc(p);
        INameS::GlobalFunctionFamilyName(r)
      }
      INameValS::ArbitraryName(p) => {
        let r = self.arena.alloc(p);
        INameS::ArbitraryName(r)
      }
      INameValS::VarName(v) => {
        let inner = self.alloc_var_name_canonical(v);
        let r = self.arena.alloc(inner);
        INameS::VarName(r)
      }
    }
  }

  fn alloc_function_declaration_name_canonical(
    &self,
    val: IFunctionDeclarationNameValS<'a>,
  ) -> IFunctionDeclarationNameS<'a> {
    use crate::postparsing::names::{ForwarderFunctionDeclarationNameS, ForwarderFunctionDeclarationNameValS};
    match val {
      IFunctionDeclarationNameValS::FunctionName(p) => IFunctionDeclarationNameS::FunctionName(p),
      IFunctionDeclarationNameValS::LambdaDeclarationName(p) => {
        IFunctionDeclarationNameS::LambdaDeclarationName(p)
      }
      IFunctionDeclarationNameValS::ForwarderFunctionDeclarationName(
        ForwarderFunctionDeclarationNameValS { inner, index },
      ) => {
        let payload = ForwarderFunctionDeclarationNameS {
          inner: inner.clone(),
          index,
        };
        let r = self.arena.alloc(payload);
        IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(r)
      }
      IFunctionDeclarationNameValS::ConstructorName(p) => {
        let r = self.arena.alloc(p);
        IFunctionDeclarationNameS::ConstructorName(r)
      }
      IFunctionDeclarationNameValS::ImmConcreteDestructorName(p) => {
        let r = self.arena.alloc(p);
        IFunctionDeclarationNameS::ImmConcreteDestructorName(r)
      }
      IFunctionDeclarationNameValS::ImmInterfaceDestructorName(p) => {
        let r = self.arena.alloc(p);
        IFunctionDeclarationNameS::ImmInterfaceDestructorName(r)
      }
    }
  }

  fn alloc_var_name_canonical(&self, val: IVarNameValS<'a>) -> IVarNameS<'a> {
    match val {
      IVarNameValS::CodeVarName(n) => IVarNameS::CodeVarName(n),
      IVarNameValS::ConstructingMemberName(n) => IVarNameS::ConstructingMemberName(n),
      IVarNameValS::ClosureParamName(p) => {
        let r = self.arena.alloc(p);
        IVarNameS::ClosureParamName(r)
      }
      IVarNameValS::MagicParamName(p) => IVarNameS::MagicParamName(p),
      IVarNameValS::IterableName(p) => IVarNameS::IterableName(p),
      IVarNameValS::IteratorName(p) => IVarNameS::IteratorName(p),
      IVarNameValS::IterationOptionName(p) => IVarNameS::IterationOptionName(p),
      IVarNameValS::WhileCondResultName(p) => IVarNameS::WhileCondResultName(p),
      IVarNameValS::SelfName => IVarNameS::SelfName,
      IVarNameValS::AnonymousSubstructMemberName(i) => IVarNameS::AnonymousSubstructMemberName(i),
    }
  }

  /// Canonical rune entrypoint: intern an IRuneValS value key and return canonical IRuneS.
  pub fn intern_rune(&self, val: IRuneValS<'a>) -> IRuneS<'a> {
    {
      let inner = self.inner.borrow();
      if let Some(existing) = inner.rune_val_to_ref.get(&val) {
        // Fast path: return the already-canonicalized payload.
        return existing.clone();
      }
    }
    let canonical = self.alloc_rune_canonical(val.clone());
    let mut inner = self.inner.borrow_mut();
    // Store by value-key and return canonical ref payload.
    // If you need to verify canonicalization, use `ptr_eq` (not just `==`).
    inner.rune_val_to_ref.insert(val, canonical.clone());
    canonical
  }

  fn alloc_rune_canonical(&self, val: IRuneValS<'a>) -> IRuneS<'a> {
    use IRuneValS::*;
    match val {
      CodeRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::CodeRune(r)
      }
      LetImplicitRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::LetImplicitRune(r)
      }
      ImplicitRegionRune(v) => {
        // Shallow key: v.original_rune is already canonical; use directly.
        let payload = ImplicitRegionRuneS {
          original_rune: v.original_rune,
        };
        let r = self.arena.alloc(payload);
        IRuneS::ImplicitRegionRune(r)
      }
      ImplicitCoercionOwnershipRune(v) => {
        // Shallow key: v.original_coord_rune is already canonical; use directly.
        let payload = ImplicitCoercionOwnershipRuneS {
          range: v.range,
          original_coord_rune: v.original_coord_rune,
        };
        let r = self.arena.alloc(payload);
        IRuneS::ImplicitCoercionOwnershipRune(r)
      }
      ImplicitCoercionKindRune(v) => {
        // Shallow key: v.original_coord_rune is already canonical; use directly.
        let payload = ImplicitCoercionKindRuneS {
          range: v.range,
          original_coord_rune: v.original_coord_rune,
        };
        let r = self.arena.alloc(payload);
        IRuneS::ImplicitCoercionKindRune(r)
      }
      ImplicitCoercionTemplateRune(v) => {
        // Shallow key: v.original_kind_rune is already canonical; use directly.
        let payload = ImplicitCoercionTemplateRuneS {
          range: v.range,
          original_kind_rune: v.original_kind_rune,
        };
        let r = self.arena.alloc(payload);
        IRuneS::ImplicitCoercionTemplateRune(r)
      }
      AnonymousSubstructMethodInheritedRune(v) => {
        // Shallow key: v.inner is already canonical; use directly.
        let payload = AnonymousSubstructMethodInheritedRuneS {
          interface: v.interface,
          method: v.method,
          inner: v.inner,
        };
        let r = self.arena.alloc(payload);
        IRuneS::AnonymousSubstructMethodInheritedRune(r)
      }
      DispatcherRuneFromImpl(v) => {
        // Shallow key: v.inner_rune is already canonical; use directly.
        let payload = DispatcherRuneFromImplS {
          inner_rune: v.inner_rune,
        };
        let r = self.arena.alloc(payload);
        IRuneS::DispatcherRuneFromImpl(r)
      }
      CaseRuneFromImpl(v) => {
        // Shallow key: v.inner_rune is already canonical; use directly.
        let payload = CaseRuneFromImplS {
          inner_rune: v.inner_rune,
        };
        let r = self.arena.alloc(payload);
        IRuneS::CaseRuneFromImpl(r)
      }
      ImplDropCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ImplDropCoordRune(r)
      }
      ImplDropVoidRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ImplDropVoidRune(r)
      }
      ImplicitRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ImplicitRune(r)
      }
      PureBlockRegionRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::PureBlockRegionRune(r)
      }
      CallRegionRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::CallRegionRune(r)
      }
      CallPureMergeRegionRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::CallPureMergeRegionRune(r)
      }
      ReachablePrototypeRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ReachablePrototypeRune(r)
      }
      FreeOverrideStructTemplateRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::FreeOverrideStructTemplateRune(r)
      }
      FreeOverrideStructRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::FreeOverrideStructRune(r)
      }
      FreeOverrideInterfaceRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::FreeOverrideInterfaceRune(r)
      }
      MagicParamRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::MagicParamRune(r)
      }
      MemberRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::MemberRune(r)
      }
      LocalDefaultRegionRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::LocalDefaultRegionRune(r)
      }
      DenizenDefaultRegionRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::DenizenDefaultRegionRune(r)
      }
      ExportDefaultRegionRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ExportDefaultRegionRune(r)
      }
      ExternDefaultRegionRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ExternDefaultRegionRune(r)
      }
      ArraySizeImplicitRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ArraySizeImplicitRune(r)
      }
      ArrayMutabilityImplicitRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ArrayMutabilityImplicitRune(r)
      }
      ArrayVariabilityImplicitRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ArrayVariabilityImplicitRune(r)
      }
      ReturnRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ReturnRune(r)
      }
      StructNameRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::StructNameRune(r)
      }
      InterfaceNameRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::InterfaceNameRune(r)
      }
      SelfRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::SelfRune(r)
      }
      SelfOwnershipRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::SelfOwnershipRune(r)
      }
      SelfKindRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::SelfKindRune(r)
      }
      SelfKindTemplateRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::SelfKindTemplateRune(r)
      }
      SelfCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::SelfCoordRune(r)
      }
      MacroVoidKindRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::MacroVoidKindRune(r)
      }
      MacroVoidCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::MacroVoidCoordRune(r)
      }
      MacroSelfKindRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::MacroSelfKindRune(r)
      }
      MacroSelfKindTemplateRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::MacroSelfKindTemplateRune(r)
      }
      MacroSelfCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::MacroSelfCoordRune(r)
      }
      ArgumentRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ArgumentRune(r)
      }
      PatternInputRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::PatternInputRune(r)
      }
      ExplicitTemplateArgRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::ExplicitTemplateArgRune(r)
      }
      AnonymousSubstructParentInterfaceTemplateRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructParentInterfaceTemplateRune(r)
      }
      AnonymousSubstructParentInterfaceKindRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructParentInterfaceKindRune(r)
      }
      AnonymousSubstructParentInterfaceCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructParentInterfaceCoordRune(r)
      }
      AnonymousSubstructTemplateRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructTemplateRune(r)
      }
      AnonymousSubstructKindRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructKindRune(r)
      }
      AnonymousSubstructCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructCoordRune(r)
      }
      AnonymousSubstructVoidKindRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructVoidKindRune(r)
      }
      AnonymousSubstructVoidCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructVoidCoordRune(r)
      }
      AnonymousSubstructMemberRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructMemberRune(r)
      }
      AnonymousSubstructMethodSelfBorrowCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructMethodSelfBorrowCoordRune(r)
      }
      AnonymousSubstructMethodSelfOwnCoordRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructMethodSelfOwnCoordRune(r)
      }
      AnonymousSubstructDropBoundPrototypeRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructDropBoundPrototypeRune(r)
      }
      AnonymousSubstructDropBoundParamsListRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructDropBoundParamsListRune(r)
      }
      AnonymousSubstructFunctionBoundPrototypeRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(r)
      }
      AnonymousSubstructFunctionBoundParamsListRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructFunctionBoundParamsListRune(r)
      }
      AnonymousSubstructFunctionInterfaceTemplateRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(r)
      }
      AnonymousSubstructFunctionInterfaceKindRune(p) => {
        let r = self.arena.alloc(p);
        IRuneS::AnonymousSubstructFunctionInterfaceKindRune(r)
      }
      FunctorPrototypeRuneName(p) => {
        let r = self.arena.alloc(p);
        IRuneS::FunctorPrototypeRuneName(r)
      }
      FunctorParamRuneName(p) => {
        let r = self.arena.alloc(p);
        IRuneS::FunctorParamRuneName(r)
      }
      FunctorReturnRuneName(p) => {
        let r = self.arena.alloc(p);
        IRuneS::FunctorReturnRuneName(r)
      }
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_interning() {
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);
    let s1 = interner.intern("hello");
    let s2 = interner.intern("hello");

    // Same string should be same canonical instance.
    assert_eq!(s1, s2);
    assert_eq!(s1.as_str(), "hello");
  }

  #[test]
  fn test_long_string_interning() {
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);
    let s1 = interner.intern("this is a long string");
    let s2 = interner.intern("this is a long string");

    assert_eq!(s1, s2);
    assert_eq!(s1.as_str(), "this is a long string");
  }

  #[test]
  fn test_different_strings() {
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);
    let s1 = interner.intern("foo");
    let s2 = interner.intern("bar");

    assert_ne!(s1, s2);
  }

  #[test]
  fn test_coordinate_interning_canonicalizes() {
    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let module = interner.intern("my_module");
    let pkg1 = interner.intern_package_coordinate(module, &[]);
    let pkg2 = interner.intern_package_coordinate(module, &[]);
    assert!(std::ptr::eq(pkg1, pkg2));

    let file1 = interner.intern_file_coordinate(pkg1, "main.vale");
    let file2 = interner.intern_file_coordinate(pkg1, "main.vale");
    assert!(std::ptr::eq(file1, file2));
    assert_eq!(file1.filepath, "main.vale");
  }

  #[test]
  fn test_imprecise_name_canonicalization_same_key() {
    use crate::postparsing::names::{CodeNameS, IImpreciseNameValS};

    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let name = interner.intern("foo");
    let n1 = interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name }));
    let n2 = interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name }));
    assert_eq!(n1, n2);
  }

  #[test]
  fn test_rune_canonicalization_same_key() {
    use crate::postparsing::ast::LocationInDenizen;
    use crate::postparsing::names::{CodeRuneS, IRuneValS, LetImplicitRuneS};

    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let name = interner.intern("T");
    let r1 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS { name }));
    let r2 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS { name }));
    // `==` checks value semantics; ptr-based tests below check canonical identity.
    assert_eq!(r1, r2);

    let r3 = interner.intern_rune(IRuneValS::LetImplicitRune(LetImplicitRuneS {
      lid: LocationInDenizen { path: vec![1] },
    }));
    let r4 = interner.intern_rune(IRuneValS::LetImplicitRune(LetImplicitRuneS {
      lid: LocationInDenizen { path: vec![1] },
    }));
    assert_eq!(r3, r4);
  }

  #[test]
  fn test_rune_canonicalization_distinct_values() {
    use crate::postparsing::names::{CodeRuneS, IRuneValS};

    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let r1 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
      name: interner.intern("A"),
    }));
    let r2 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
      name: interner.intern("B"),
    }));
    assert_ne!(r1, r2);
  }

  #[test]
  fn test_rune_ptr_eq_same_canonical() {
    use crate::postparsing::names::{CodeRuneS, IRuneS, IRuneValS};

    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let name = interner.intern("X");
    let r1 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS { name }));
    let r2 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS { name }));

    // Guardrail: this checks payload pointer identity directly, so it catches
    // accidental regressions where equality stays true but canonicalization breaks.
    let (ref1, ref2) = match (&r1, &r2) {
      (IRuneS::CodeRune(a), IRuneS::CodeRune(b)) => (*a as *const _, *b as *const _),
      _ => panic!("expected CodeRune"),
    };
    assert!(std::ptr::eq(ref1, ref2), "same key should yield same canonical ref");

    assert!(r1.ptr_eq(&r2));
    assert!(std::ptr::eq(r1.canonical_ptr(), r2.canonical_ptr()));
  }

  #[test]
  fn test_rune_ptr_eq_distinct() {
    use crate::postparsing::names::{CodeRuneS, IRuneValS};

    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let r1 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
      name: interner.intern("P"),
    }));
    let r2 = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
      name: interner.intern("Q"),
    }));

    assert!(!r1.ptr_eq(&r2));
    assert!(!std::ptr::eq(r1.canonical_ptr(), r2.canonical_ptr()));
  }

  #[test]
  fn test_imprecise_name_ptr_eq_same_canonical() {
    use crate::postparsing::names::{CodeNameS, IImpreciseNameValS};

    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let name = interner.intern("bar");
    let n1 = interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name }));
    let n2 = interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name }));

    assert!(n1.ptr_eq(&n2));
    assert!(std::ptr::eq(n1.canonical_ptr(), n2.canonical_ptr()));
  }

  #[test]
  fn test_imprecise_name_ptr_eq_distinct() {
    use crate::postparsing::names::{CodeNameS, IImpreciseNameValS};

    let arena = Bump::new();
    let interner = Interner::with_arena(&arena);

    let n1 = interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
      name: interner.intern("x"),
    }));
    let n2 = interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
      name: interner.intern("y"),
    }));

    assert!(!n1.ptr_eq(&n2));
    assert!(!std::ptr::eq(n1.canonical_ptr(), n2.canonical_ptr()));
  }

}
