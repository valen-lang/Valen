/*
Guardian: disable-all
*/

// ScoutArena: arena + interning maps for the postparsing (scout) pass.
// Has string/coord interning (like ParseArena) plus name/rune/imprecise-name interning.

use crate::interner::{InternedSlice, StrI};
use crate::postparsing::names::{
  IImpreciseNameS, IImpreciseNameValS, INameS, INameValS, IRuneS, IRuneValS,
  IFunctionDeclarationNameS, IFunctionDeclarationNameValS, IVarNameS, IVarNameValS,
  ImplicitRegionRuneS, ImplicitCoercionOwnershipRuneS, ImplicitCoercionKindRuneS,
  RuneNameS, TopLevelStructDeclarationNameS, TopLevelInterfaceDeclarationNameS,
  ImplicitCoercionTemplateRuneS, AnonymousSubstructMethodInheritedRuneS,
  DispatcherRuneFromImplS, CaseRuneFromImplS,
  LambdaStructImpreciseNameS,
  AnonymousSubstructTemplateImpreciseNameS,
  AnonymousSubstructConstructorTemplateImpreciseNameS, ImplImpreciseNameS,
  ImplSubCitizenImpreciseNameS, ImplSuperInterfaceImpreciseNameS,
};
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use bumpalo::Bump;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone)]
struct FileCoordLookupKey<'s> {
  package_coord: &'s PackageCoordinate<'s>,
  filepath: String,
}

impl<'s> PartialEq for FileCoordLookupKey<'s> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.package_coord, other.package_coord) && self.filepath == other.filepath
  }
}
impl<'s> Eq for FileCoordLookupKey<'s> {}

impl<'s> std::hash::Hash for FileCoordLookupKey<'s> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    (self.package_coord as *const PackageCoordinate<'_>).hash(state);
    self.filepath.hash(state);
  }
}

pub struct ScoutArena<'s> {
  bump: &'s Bump,
  inner: RefCell<ScoutArenaInner<'s>>,
}

struct ScoutArenaInner<'s> {
  string_to_interned: HashMap<String, &'s str>,
  package_coord_to_ref: HashMap<PackageCoordinate<'s>, &'s PackageCoordinate<'s>>,
  file_coord_to_ref: HashMap<FileCoordLookupKey<'s>, &'s FileCoordinate<'s>>,
  imprecise_name_val_to_ref: HashMap<IImpreciseNameValS<'s>, IImpreciseNameS<'s>>,
  name_val_to_ref: HashMap<INameValS<'s>, INameS<'s>>,
  rune_val_to_ref: HashMap<IRuneValS<'s>, IRuneS<'s>>,
}

impl<'s> ScoutArena<'s> {
  pub fn new(bump: &'s Bump) -> Self {
    ScoutArena {
      bump,
      inner: RefCell::new(ScoutArenaInner {
        string_to_interned: HashMap::with_capacity(256),
        package_coord_to_ref: HashMap::new(),
        file_coord_to_ref: HashMap::new(),
        imprecise_name_val_to_ref: HashMap::new(),
        name_val_to_ref: HashMap::new(),
        rune_val_to_ref: HashMap::new(),
      }),
    }
  }

  pub fn bump(&self) -> &'s Bump {
    self.bump
  }

  pub fn arena(&self) -> &'s Bump {
    self.bump
  }

  pub fn alloc<T>(&self, val: T) -> &'s mut T {
    self.bump.alloc(val)
  }

  pub fn alloc_slice_copy<T: Copy>(&self, src: &[T]) -> &'s [T] {
    self.bump.alloc_slice_copy(src)
  }

  // --- String interning ---

  pub fn intern_str(&self, s: &str) -> StrI<'s> {
    let mut inner = self.inner.borrow_mut();
    if let Some(&existing) = inner.string_to_interned.get(s) {
      return StrI(existing);
    }
    let arena_str = self.bump.alloc_str(s);
    inner.string_to_interned.insert(s.to_string(), arena_str);
    StrI(arena_str)
  }

  // --- Package/File coordinate interning ---

  pub fn intern_package_coordinate(
    &self,
    module: StrI<'s>,
    packages: &[StrI<'s>],
  ) -> &'s PackageCoordinate<'s> {
    let mut inner = self.inner.borrow_mut();
    let lookup_coord = PackageCoordinate {
      module,
      packages: InternedSlice::new(packages),
    };
    if let Some(existing) = inner.package_coord_to_ref.get(&lookup_coord) {
      return *existing;
    }
    let arena_packages = self.bump.alloc_slice_copy(packages);
    let coord = PackageCoordinate {
      module,
      packages: InternedSlice::new(arena_packages),
    };
    let new_ref: &'s PackageCoordinate<'s> = self.bump.alloc(coord.clone());
    inner.package_coord_to_ref.insert(coord, new_ref);
    new_ref
  }

  pub fn intern_file_coordinate(
    &self,
    package_coord: &'s PackageCoordinate<'s>,
    filepath: &str,
  ) -> &'s FileCoordinate<'s> {
    let mut inner = self.inner.borrow_mut();
    let lookup_key = FileCoordLookupKey {
      package_coord,
      filepath: filepath.to_string(),
    };
    if let Some(existing) = inner.file_coord_to_ref.get(&lookup_key) {
      return *existing;
    }
    let arena_filepath = self.bump.alloc_str(filepath);
    let coord = FileCoordinate {
      package_coord,
      filepath: StrI(arena_filepath),
    };
    let new_ref: &'s FileCoordinate<'s> = self.bump.alloc(coord.clone());
    let insert_key = FileCoordLookupKey {
      package_coord,
      filepath: filepath.to_string(),
    };
    inner.file_coord_to_ref.insert(insert_key, new_ref);
    new_ref
  }

  // --- Imprecise name interning ---

  pub fn intern_imprecise_name(&self, val: IImpreciseNameValS<'s>) -> IImpreciseNameS<'s> {
    {
      let inner = self.inner.borrow();
      if let Some(existing) = inner.imprecise_name_val_to_ref.get(&val) {
        return existing.clone();
      }
    }
    let canonical: IImpreciseNameS<'s> = self.alloc_imprecise_name_canonical(val.clone());
    let mut inner = self.inner.borrow_mut();
    inner.imprecise_name_val_to_ref.insert(val, canonical.clone());
    canonical
  }

  fn alloc_imprecise_name_canonical(&self, val: IImpreciseNameValS<'s>) -> IImpreciseNameS<'s> {
    use crate::postparsing::names::IImpreciseNameValS::*;
    match val {
      CodeName(p) => IImpreciseNameS::CodeName(self.bump.alloc(p)),
      IterableName(p) => IImpreciseNameS::IterableName(self.bump.alloc(p)),
      IteratorName(p) => IImpreciseNameS::IteratorName(self.bump.alloc(p)),
      IterationOptionName(p) => IImpreciseNameS::IterationOptionName(self.bump.alloc(p)),
      LambdaImpreciseName(p) => IImpreciseNameS::LambdaImpreciseName(self.bump.alloc(p)),
      PlaceholderImpreciseName(p) => IImpreciseNameS::PlaceholderImpreciseName(self.bump.alloc(p)),
      LambdaStructImpreciseName(v) => {
        let payload = LambdaStructImpreciseNameS { lambda_name: v.lambda_name };
        IImpreciseNameS::LambdaStructImpreciseName(self.bump.alloc(payload))
      }
      ClosureParamImpreciseName(p) => IImpreciseNameS::ClosureParamImpreciseName(self.bump.alloc(p)),
      PrototypeName(p) => IImpreciseNameS::PrototypeName(self.bump.alloc(p)),
      AnonymousSubstructTemplateImpreciseName(v) => {
        let payload = AnonymousSubstructTemplateImpreciseNameS { interface_imprecise_name: v.interface_imprecise_name };
        IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(self.bump.alloc(payload))
      }
      AnonymousSubstructConstructorTemplateImpreciseName(v) => {
        let payload = AnonymousSubstructConstructorTemplateImpreciseNameS { interface_imprecise_name: v.interface_imprecise_name };
        IImpreciseNameS::AnonymousSubstructConstructorTemplateImpreciseName(self.bump.alloc(payload))
      }
      ImplImpreciseName(v) => {
        let payload = ImplImpreciseNameS { sub_citizen_imprecise_name: v.sub_citizen_imprecise_name, super_interface_imprecise_name: v.super_interface_imprecise_name };
        IImpreciseNameS::ImplImpreciseName(self.bump.alloc(payload))
      }
      ImplSubCitizenImpreciseName(v) => {
        let payload = ImplSubCitizenImpreciseNameS { sub_citizen_imprecise_name: v.sub_citizen_imprecise_name };
        IImpreciseNameS::ImplSubCitizenImpreciseName(self.bump.alloc(payload))
      }
      ImplSuperInterfaceImpreciseName(v) => {
        let payload = ImplSuperInterfaceImpreciseNameS { super_interface_imprecise_name: v.super_interface_imprecise_name };
        IImpreciseNameS::ImplSuperInterfaceImpreciseName(self.bump.alloc(payload))
      }
      SelfName(p) => IImpreciseNameS::SelfName(self.bump.alloc(p)),
      RuneName(v) => {
        let payload = RuneNameS { rune: v.rune };
        IImpreciseNameS::RuneName(self.bump.alloc(payload))
      }
      ArbitraryName(p) => IImpreciseNameS::ArbitraryName(self.bump.alloc(p)),
    }
  }

  // --- Name interning ---

  pub fn intern_struct_declaration_name(&self, val: TopLevelStructDeclarationNameS<'s>) -> &'s TopLevelStructDeclarationNameS<'s> {
    match self.intern_name(INameValS::TopLevelStructDeclaration(val)) {
      INameS::TopLevelStructDeclaration(r) => r,
      _ => unreachable!(),
    }
  }

  pub fn intern_interface_declaration_name(&self, val: TopLevelInterfaceDeclarationNameS<'s>) -> &'s TopLevelInterfaceDeclarationNameS<'s> {
    match self.intern_name(INameValS::TopLevelInterfaceDeclaration(val)) {
      INameS::TopLevelInterfaceDeclaration(r) => r,
      _ => unreachable!(),
    }
  }

  pub fn intern_name(&self, val: INameValS<'s>) -> INameS<'s> {
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

  fn alloc_name_canonical(&self, val: INameValS<'s>) -> INameS<'s> {
    use crate::postparsing::names::{
      AnonymousSubstructImplDeclarationNameValS, AnonymousSubstructTemplateNameValS,
    };
    match val {
      INameValS::FunctionDeclaration(v) => {
        let inner = self.alloc_function_declaration_name_canonical(v);
        INameS::FunctionDeclaration(self.bump.alloc(inner))
      }
      INameValS::ImplDeclaration(p) => INameS::ImplDeclaration(self.bump.alloc(p)),
      INameValS::AnonymousSubstructImplDeclaration(AnonymousSubstructImplDeclarationNameValS { interface }) => {
        let payload = crate::postparsing::names::AnonymousSubstructImplDeclarationNameS { interface: interface.clone() };
        INameS::AnonymousSubstructImplDeclaration(self.bump.alloc(payload))
      }
      INameValS::ExportAsName(p) => INameS::ExportAsName(self.bump.alloc(p)),
      INameValS::LetName(p) => INameS::LetName(self.bump.alloc(p)),
      INameValS::TopLevelStructDeclaration(p) => INameS::TopLevelStructDeclaration(self.bump.alloc(p)),
      INameValS::TopLevelInterfaceDeclaration(p) => INameS::TopLevelInterfaceDeclaration(self.bump.alloc(p)),
      INameValS::LambdaStructDeclaration(p) => INameS::LambdaStructDeclaration(self.bump.alloc(p)),
      INameValS::AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameValS { interface_name }) => {
        let payload = crate::postparsing::names::AnonymousSubstructTemplateNameS { interface_name: interface_name.clone() };
        INameS::AnonymousSubstructTemplateName(self.bump.alloc(payload))
      }
      INameValS::RuneName(v) => {
        let payload = RuneNameS { rune: v.rune };
        INameS::RuneName(self.bump.alloc(payload))
      }
      INameValS::RuntimeSizedArrayDeclarationName(p) => INameS::RuntimeSizedArrayDeclarationName(self.bump.alloc(p)),
      INameValS::StaticSizedArrayDeclarationName(p) => INameS::StaticSizedArrayDeclarationName(self.bump.alloc(p)),
      INameValS::GlobalFunctionFamilyName(p) => INameS::GlobalFunctionFamilyName(self.bump.alloc(p)),
      INameValS::ArbitraryName(p) => INameS::ArbitraryName(self.bump.alloc(p)),
      INameValS::VarName(v) => {
        let inner = self.alloc_var_name_canonical(v);
        INameS::VarName(self.bump.alloc(inner))
      }
    }
  }

  fn alloc_function_declaration_name_canonical(&self, val: IFunctionDeclarationNameValS<'s>) -> IFunctionDeclarationNameS<'s> {
    use crate::postparsing::names::{ForwarderFunctionDeclarationNameS, ForwarderFunctionDeclarationNameValS};
    match val {
      IFunctionDeclarationNameValS::FunctionName(p) => IFunctionDeclarationNameS::FunctionName(p),
      IFunctionDeclarationNameValS::LambdaDeclarationName(p) => IFunctionDeclarationNameS::LambdaDeclarationName(p),
      IFunctionDeclarationNameValS::ForwarderFunctionDeclarationName(ForwarderFunctionDeclarationNameValS { inner, index }) => {
        let payload = ForwarderFunctionDeclarationNameS { inner: inner.clone(), index };
        IFunctionDeclarationNameS::ForwarderFunctionDeclarationName(self.bump.alloc(payload))
      }
      IFunctionDeclarationNameValS::ConstructorName(p) => IFunctionDeclarationNameS::ConstructorName(self.bump.alloc(p)),
      IFunctionDeclarationNameValS::ImmConcreteDestructorName(p) => IFunctionDeclarationNameS::ImmConcreteDestructorName(self.bump.alloc(p)),
      IFunctionDeclarationNameValS::ImmInterfaceDestructorName(p) => IFunctionDeclarationNameS::ImmInterfaceDestructorName(self.bump.alloc(p)),
    }
  }

  fn alloc_var_name_canonical(&self, val: IVarNameValS<'s>) -> IVarNameS<'s> {
    match val {
      IVarNameValS::CodeVarName(n) => IVarNameS::CodeVarName(n),
      IVarNameValS::ConstructingMemberName(n) => IVarNameS::ConstructingMemberName(n),
      IVarNameValS::ClosureParamName(p) => IVarNameS::ClosureParamName(self.bump.alloc(p)),
      IVarNameValS::MagicParamName(p) => IVarNameS::MagicParamName(p),
      IVarNameValS::IterableName(p) => IVarNameS::IterableName(p),
      IVarNameValS::IteratorName(p) => IVarNameS::IteratorName(p),
      IVarNameValS::IterationOptionName(p) => IVarNameS::IterationOptionName(p),
      IVarNameValS::WhileCondResultName(p) => IVarNameS::WhileCondResultName(p),
      IVarNameValS::SelfName => IVarNameS::SelfName,
      IVarNameValS::AnonymousSubstructMemberName(i) => IVarNameS::AnonymousSubstructMemberName(i),
    }
  }

  // --- Rune interning ---

  pub fn intern_rune(&self, val: IRuneValS<'s>) -> IRuneS<'s> {
    {
      let inner = self.inner.borrow();
      if let Some(existing) = inner.rune_val_to_ref.get(&val) {
        return existing.clone();
      }
    }
    let canonical = self.alloc_rune_canonical(val.clone());
    let mut inner = self.inner.borrow_mut();
    inner.rune_val_to_ref.insert(val, canonical.clone());
    canonical
  }

  fn alloc_rune_canonical(&self, val: IRuneValS<'s>) -> IRuneS<'s> {
    use IRuneValS::*;
    match val {
      CodeRune(p) => IRuneS::CodeRune(self.bump.alloc(p)),
      LetImplicitRune(p) => IRuneS::LetImplicitRune(self.bump.alloc(p)),
      ImplicitRegionRune(v) => {
        let payload = ImplicitRegionRuneS { original_rune: v.original_rune };
        IRuneS::ImplicitRegionRune(self.bump.alloc(payload))
      }
      ImplicitCoercionOwnershipRune(v) => {
        let payload = ImplicitCoercionOwnershipRuneS { range: v.range, original_coord_rune: v.original_coord_rune };
        IRuneS::ImplicitCoercionOwnershipRune(self.bump.alloc(payload))
      }
      ImplicitCoercionKindRune(v) => {
        let payload = ImplicitCoercionKindRuneS { range: v.range, original_coord_rune: v.original_coord_rune };
        IRuneS::ImplicitCoercionKindRune(self.bump.alloc(payload))
      }
      ImplicitCoercionTemplateRune(v) => {
        let payload = ImplicitCoercionTemplateRuneS { range: v.range, original_kind_rune: v.original_kind_rune };
        IRuneS::ImplicitCoercionTemplateRune(self.bump.alloc(payload))
      }
      AnonymousSubstructMethodInheritedRune(v) => {
        let payload = AnonymousSubstructMethodInheritedRuneS { interface: v.interface, method: v.method, inner: v.inner };
        IRuneS::AnonymousSubstructMethodInheritedRune(self.bump.alloc(payload))
      }
      DispatcherRuneFromImpl(v) => {
        let payload = DispatcherRuneFromImplS { inner_rune: v.inner_rune };
        IRuneS::DispatcherRuneFromImpl(self.bump.alloc(payload))
      }
      CaseRuneFromImpl(v) => {
        let payload = CaseRuneFromImplS { inner_rune: v.inner_rune };
        IRuneS::CaseRuneFromImpl(self.bump.alloc(payload))
      }
      ImplDropCoordRune(p) => IRuneS::ImplDropCoordRune(self.bump.alloc(p)),
      ImplDropVoidRune(p) => IRuneS::ImplDropVoidRune(self.bump.alloc(p)),
      ImplicitRune(p) => IRuneS::ImplicitRune(self.bump.alloc(p)),
      PureBlockRegionRune(p) => IRuneS::PureBlockRegionRune(self.bump.alloc(p)),
      CallRegionRune(p) => IRuneS::CallRegionRune(self.bump.alloc(p)),
      CallPureMergeRegionRune(p) => IRuneS::CallPureMergeRegionRune(self.bump.alloc(p)),
      ReachablePrototypeRune(p) => IRuneS::ReachablePrototypeRune(self.bump.alloc(p)),
      FreeOverrideStructTemplateRune(p) => IRuneS::FreeOverrideStructTemplateRune(self.bump.alloc(p)),
      FreeOverrideStructRune(p) => IRuneS::FreeOverrideStructRune(self.bump.alloc(p)),
      FreeOverrideInterfaceRune(p) => IRuneS::FreeOverrideInterfaceRune(self.bump.alloc(p)),
      MagicParamRune(p) => IRuneS::MagicParamRune(self.bump.alloc(p)),
      MemberRune(p) => IRuneS::MemberRune(self.bump.alloc(p)),
      LocalDefaultRegionRune(p) => IRuneS::LocalDefaultRegionRune(self.bump.alloc(p)),
      DenizenDefaultRegionRune(p) => IRuneS::DenizenDefaultRegionRune(self.bump.alloc(p)),
      ExportDefaultRegionRune(p) => IRuneS::ExportDefaultRegionRune(self.bump.alloc(p)),
      ExternDefaultRegionRune(p) => IRuneS::ExternDefaultRegionRune(self.bump.alloc(p)),
      ArraySizeImplicitRune(p) => IRuneS::ArraySizeImplicitRune(self.bump.alloc(p)),
      ArrayMutabilityImplicitRune(p) => IRuneS::ArrayMutabilityImplicitRune(self.bump.alloc(p)),
      ArrayVariabilityImplicitRune(p) => IRuneS::ArrayVariabilityImplicitRune(self.bump.alloc(p)),
      ReturnRune(p) => IRuneS::ReturnRune(self.bump.alloc(p)),
      StructNameRune(p) => IRuneS::StructNameRune(self.bump.alloc(p)),
      InterfaceNameRune(p) => IRuneS::InterfaceNameRune(self.bump.alloc(p)),
      SelfRune(p) => IRuneS::SelfRune(self.bump.alloc(p)),
      SelfOwnershipRune(p) => IRuneS::SelfOwnershipRune(self.bump.alloc(p)),
      SelfKindRune(p) => IRuneS::SelfKindRune(self.bump.alloc(p)),
      SelfKindTemplateRune(p) => IRuneS::SelfKindTemplateRune(self.bump.alloc(p)),
      SelfCoordRune(p) => IRuneS::SelfCoordRune(self.bump.alloc(p)),
      MacroVoidKindRune(p) => IRuneS::MacroVoidKindRune(self.bump.alloc(p)),
      MacroVoidCoordRune(p) => IRuneS::MacroVoidCoordRune(self.bump.alloc(p)),
      MacroSelfKindRune(p) => IRuneS::MacroSelfKindRune(self.bump.alloc(p)),
      MacroSelfKindTemplateRune(p) => IRuneS::MacroSelfKindTemplateRune(self.bump.alloc(p)),
      MacroSelfCoordRune(p) => IRuneS::MacroSelfCoordRune(self.bump.alloc(p)),
      ArgumentRune(p) => IRuneS::ArgumentRune(self.bump.alloc(p)),
      PatternInputRune(p) => IRuneS::PatternInputRune(self.bump.alloc(p)),
      ExplicitTemplateArgRune(p) => IRuneS::ExplicitTemplateArgRune(self.bump.alloc(p)),
      AnonymousSubstructParentInterfaceTemplateRune(p) => IRuneS::AnonymousSubstructParentInterfaceTemplateRune(self.bump.alloc(p)),
      AnonymousSubstructParentInterfaceKindRune(p) => IRuneS::AnonymousSubstructParentInterfaceKindRune(self.bump.alloc(p)),
      AnonymousSubstructParentInterfaceCoordRune(p) => IRuneS::AnonymousSubstructParentInterfaceCoordRune(self.bump.alloc(p)),
      AnonymousSubstructTemplateRune(p) => IRuneS::AnonymousSubstructTemplateRune(self.bump.alloc(p)),
      AnonymousSubstructKindRune(p) => IRuneS::AnonymousSubstructKindRune(self.bump.alloc(p)),
      AnonymousSubstructCoordRune(p) => IRuneS::AnonymousSubstructCoordRune(self.bump.alloc(p)),
      AnonymousSubstructVoidKindRune(p) => IRuneS::AnonymousSubstructVoidKindRune(self.bump.alloc(p)),
      AnonymousSubstructVoidCoordRune(p) => IRuneS::AnonymousSubstructVoidCoordRune(self.bump.alloc(p)),
      AnonymousSubstructMemberRune(p) => IRuneS::AnonymousSubstructMemberRune(self.bump.alloc(p)),
      AnonymousSubstructMethodSelfBorrowCoordRune(p) => IRuneS::AnonymousSubstructMethodSelfBorrowCoordRune(self.bump.alloc(p)),
      AnonymousSubstructMethodSelfOwnCoordRune(p) => IRuneS::AnonymousSubstructMethodSelfOwnCoordRune(self.bump.alloc(p)),
      AnonymousSubstructDropBoundPrototypeRune(p) => IRuneS::AnonymousSubstructDropBoundPrototypeRune(self.bump.alloc(p)),
      AnonymousSubstructDropBoundParamsListRune(p) => IRuneS::AnonymousSubstructDropBoundParamsListRune(self.bump.alloc(p)),
      AnonymousSubstructFunctionBoundPrototypeRune(p) => IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(self.bump.alloc(p)),
      AnonymousSubstructFunctionBoundParamsListRune(p) => IRuneS::AnonymousSubstructFunctionBoundParamsListRune(self.bump.alloc(p)),
      AnonymousSubstructFunctionInterfaceTemplateRune(p) => IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(self.bump.alloc(p)),
      AnonymousSubstructFunctionInterfaceKindRune(p) => IRuneS::AnonymousSubstructFunctionInterfaceKindRune(self.bump.alloc(p)),
      FunctorPrototypeRuneName(p) => IRuneS::FunctorPrototypeRuneName(self.bump.alloc(p)),
      FunctorParamRuneName(p) => IRuneS::FunctorParamRuneName(self.bump.alloc(p)),
      FunctorReturnRuneName(p) => IRuneS::FunctorReturnRuneName(self.bump.alloc(p)),
    }
  }
}
