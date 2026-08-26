// ScoutArena: arena + interning maps for the postparsing (scout) pass.
// Has string/coord interning (like ParseArena) plus name/rune/imprecise-name interning.

use crate::interner::{InternedSlice, StrI};
use crate::postparsing::ast::{LocationInDenizen, LocationInDenizenVal};
use crate::utils::range::{CodeLocationS, RangeS};
use crate::postparsing::names::AnonymousSubstructImplDeclarationNameS;
use crate::postparsing::names::AnonymousSubstructTemplateNameS;
use crate::postparsing::names::IImpreciseNameValS::*;
use crate::postparsing::names::RuneValQuery;
use crate::postparsing::names::{
  AnonymousSubstructConstructorTemplateImpreciseNameS, AnonymousSubstructMethodInheritedRuneS,
  AnonymousSubstructTemplateImpreciseNameS, CallPureMergeRegionRuneS, CallPureMergeRegionRuneValS,
  AnonymousSubstructMemberNameS, CallRegionRuneS, CallRegionRuneValS, CaseRuneFromImplS,
  ClosureParamImpreciseNameS, ClosureParamImpreciseNameValS, CodeNameS, CodeNameValS,
  ConstructingMemberImpreciseNameS, ConstructingMemberImpreciseNameValS, DesugaredParamNameS,
  DesugaredParamNameValS, MagicParamNameValS,
  AnonymousSubstructMemberNameValS, LambdaImpreciseNameS, LambdaImpreciseNameValS,
  PlaceholderImpreciseNameS, PrototypeNameS, ArbitraryNameS, IFunctionImpreciseNameS,
  IFunctionImpreciseNameValS, ForwarderFunctionImpreciseNameS,
  DispatcherRuneFromImplS, IFunctionDeclarationNameS, IImpreciseNameS,
  IImpreciseNameValS, INameS, INameValS, IRuneS, IRuneValS, IVarDeclarationNameS,
  IterableNameS, IterationOptionNameS, IteratorNameS, MagicParamImpreciseNameS, SelfNameS, SelfNameValS,
  IterableNameValS, IterationOptionNameValS, IteratorNameValS, WhileCondResultNameValS,
  WhileCondResultNameS, ImplImpreciseNameS,
  ImplSubCitizenImpreciseNameS, ImplSuperInterfaceImpreciseNameS, ImplicitCoercionTemplateRuneS,
  ImplicitRegionRuneS, ImplicitRuneS, ImplicitRuneValS, LambdaStructImpreciseNameS,
  LetImplicitRuneS, LetImplicitRuneValS, LocalDefaultRegionRuneS, LocalDefaultRegionRuneValS,
  MagicParamRuneS, MagicParamRuneValS, RuneNameS, TopLevelInterfaceDeclarationNameS,
  TopLevelStructDeclarationNameS,
};
use crate::postparsing::names::{
  ForwarderFunctionDeclarationNameS, ForwarderFunctionDeclarationNameValS,
};
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::utils::fx::HashMap;
use bumpalo::Bump;
use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr::eq;
use IRuneValS::*;

#[derive(Clone)]
struct FileCoordLookupKey<'s> {
  package_coord: &'s PackageCoordinate<'s>,
  filepath: String,
}

impl<'s> PartialEq for FileCoordLookupKey<'s> {
  fn eq(&self, other: &Self) -> bool {
    eq(self.package_coord, other.package_coord) && self.filepath == other.filepath
  }
}
impl<'s> Eq for FileCoordLookupKey<'s> {}

impl<'s> Hash for FileCoordLookupKey<'s> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    (self.package_coord as *const PackageCoordinate<'_>).hash(state);
    self.filepath.hash(state);
  }
}

/// Construction-witness token for postparse interned payloads (mirrors typing's `MustIntern`, see
/// @SICZ). The inner unit field is private to this module, so only `scout_arena.rs` — specifically
/// the `alloc_*_canonical` helpers — can write `ScoutInterned(())`. A sealed payload carries a
/// `pub _must_intern: ScoutInterned` field; because the constructor is unnameable elsewhere, the only
/// way to obtain the payload is via an `intern_*` method (E0423 otherwise). This makes "interned" a
/// compiler-enforced category rather than a convention.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScoutInterned(());

/// The `self` imprecise name is an empty marker, so a single blessed value serves every variable's
/// `imprecise_name()` without allocating. It lives here (not at the use site) because `SelfNameS` is
/// sealed (@SICZ) — only this module can name `ScoutInterned(())`. Non-canonical by design: compare
/// it by value (`==`), never `ptr_eq`, like every other result of `IVarNameT::imprecise_name()`.
pub static SELF_IMPRECISE_NAME: SelfNameS = SelfNameS { _must_intern: ScoutInterned(()) };

pub struct ScoutArena<'s> {
  bump: &'s Bump,
  inner: RefCell<ScoutArenaInner<'s>>,
}

struct ScoutArenaInner<'s> {
  string_to_interned: HashMap<String, &'s str>,
  package_coord_to_ref: HashMap<PackageCoordinate<'s>, &'s PackageCoordinate<'s>>,
  file_coord_to_ref: HashMap<FileCoordLookupKey<'s>, &'s FileCoordinate<'s>>,
  imprecise_name_val_to_ref: HashMap<IImpreciseNameValS<'s>, IImpreciseNameS<'s>>,
  function_imprecise_name_val_to_ref:
    HashMap<IFunctionImpreciseNameValS<'s>, IFunctionImpreciseNameS<'s>>,
  name_val_to_ref: HashMap<INameValS<'s>, INameS<'s>>,
  // Per @DSAUIMZ, uses hashbrown for heterogeneous lookup (IRuneValS<'s, 'tmp> against IRuneValS<'s, 's> keys).
  rune_val_to_ref: hashbrown::HashMap<IRuneValS<'s, 's>, IRuneS<'s>>,
}

impl<'s> ScoutArena<'s> {
  pub fn new(bump: &'s Bump) -> Self {
    ScoutArena {
      bump,
      inner: RefCell::new(ScoutArenaInner {
        string_to_interned: HashMap::with_capacity_and_hasher(256, Default::default()),
        package_coord_to_ref: HashMap::default(),
        file_coord_to_ref: HashMap::default(),
        imprecise_name_val_to_ref: HashMap::default(),
        function_imprecise_name_val_to_ref: HashMap::default(),
        name_val_to_ref: HashMap::default(),
        rune_val_to_ref: hashbrown::HashMap::default(),
      }),
    }
  }

  // VCOORD: people can use this to forge things that were supposed to be interned
  pub fn alloc<T>(&self, val: T) -> &'s mut T {
    self.bump.alloc(val)
  }

  pub fn alloc_slice_copy<T: Copy>(&self, src: &[T]) -> &'s [T] {
    self.bump.alloc_slice_copy(src)
  }

  /// Allocate a slice from a Vec into the arena.
  pub fn alloc_slice_from_vec<T>(&self, vec: Vec<T>) -> &'s [T] {
    self.bump.alloc_slice_fill_iter(vec.into_iter())
  }

  /// Create an empty ArenaIndexMap allocated in this arena.
  pub fn alloc_index_map<K: Hash + Eq + Clone, V>(&self) -> ArenaIndexMap<'s, K, V> {
    ArenaIndexMap::new_in(self.bump)
  }

  /// Create an ArenaIndexMap from an iterator, allocated in this arena.
  pub fn alloc_index_map_from_iter<K: Hash + Eq + Clone, V, I: IntoIterator<Item = (K, V)>>(
    &self,
    iter: I,
  ) -> ArenaIndexMap<'s, K, V> {
    ArenaIndexMap::from_iter_in(iter, self.bump)
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
    let lookup_coord = PackageCoordinate { module, packages: InternedSlice::new(packages) };
    if let Some(existing) = inner.package_coord_to_ref.get(&lookup_coord) {
      return *existing;
    }
    let arena_packages = self.bump.alloc_slice_copy(packages);
    let coord = PackageCoordinate { module, packages: InternedSlice::new(arena_packages) };
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
    let lookup_key = FileCoordLookupKey { package_coord, filepath: filepath.to_string() };
    if let Some(existing) = inner.file_coord_to_ref.get(&lookup_key) {
      return *existing;
    }
    let arena_filepath = self.bump.alloc_str(filepath);
    let coord = FileCoordinate { package_coord, filepath: StrI(arena_filepath) };
    let new_ref: &'s FileCoordinate<'s> = self.bump.alloc(coord.clone());
    let insert_key = FileCoordLookupKey { package_coord, filepath: filepath.to_string() };
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

  // Interned-ref helpers: return the canonical `&'s <payload>` for the imprecise-name variants a
  // declaration name embeds. Declarations hold their imprecise name by interned ref, always
  // (typing-design "Names"), so these are the only sanctioned way to fill those fields.

  pub fn intern_code_name(&self, name: StrI<'s>) -> &'s CodeNameS<'s> {
    match self.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameValS { name })) {
      IImpreciseNameS::CodeName(r) => r,
      _ => unreachable!("intern_imprecise_name(CodeName) yields CodeName"),
    }
  }

  pub fn intern_lambda_imprecise_name(&self) -> &'s LambdaImpreciseNameS {
    match self.intern_imprecise_name(IImpreciseNameValS::LambdaImpreciseName(
      LambdaImpreciseNameValS {},
    )) {
      IImpreciseNameS::LambdaImpreciseName(r) => r,
      _ => unreachable!("intern_imprecise_name(LambdaImpreciseName) yields it"),
    }
  }

  pub fn intern_closure_param_imprecise_name(&self) -> &'s ClosureParamImpreciseNameS {
    match self.intern_imprecise_name(IImpreciseNameValS::ClosureParamImpreciseName(
      ClosureParamImpreciseNameValS {},
    )) {
      IImpreciseNameS::ClosureParamImpreciseName(r) => r,
      _ => unreachable!("intern_imprecise_name(ClosureParamImpreciseName) yields it"),
    }
  }

  pub fn intern_constructing_member_imprecise_name(
    &self,
    name: StrI<'s>,
  ) -> &'s ConstructingMemberImpreciseNameS<'s> {
    match self.intern_imprecise_name(IImpreciseNameValS::ConstructingMemberImpreciseName(
      ConstructingMemberImpreciseNameValS { name },
    )) {
      IImpreciseNameS::ConstructingMemberImpreciseName(r) => r,
      _ => unreachable!("intern_imprecise_name(ConstructingMemberImpreciseName) yields it"),
    }
  }

  pub fn intern_magic_param_name(
    &self,
    code_location: CodeLocationS<'s>,
    lid: LocationInDenizen<'s>,
  ) -> &'s MagicParamImpreciseNameS<'s> {
    match self.intern_imprecise_name(IImpreciseNameValS::MagicParamName(MagicParamNameValS {
      code_location,
      lid,
    })) {
      IImpreciseNameS::MagicParamName(r) => r,
      _ => unreachable!("intern_imprecise_name(MagicParamName) yields it"),
    }
  }

  pub fn intern_iterable_name(&self, range: RangeS<'s>) -> &'s IterableNameS<'s> {
    match self.intern_imprecise_name(IImpreciseNameValS::IterableName(IterableNameValS { range })) {
      IImpreciseNameS::IterableName(r) => r,
      _ => unreachable!("intern_imprecise_name(IterableName) yields it"),
    }
  }

  pub fn intern_iterator_name(&self, range: RangeS<'s>) -> &'s IteratorNameS<'s> {
    match self.intern_imprecise_name(IImpreciseNameValS::IteratorName(IteratorNameValS { range })) {
      IImpreciseNameS::IteratorName(r) => r,
      _ => unreachable!("intern_imprecise_name(IteratorName) yields it"),
    }
  }

  pub fn intern_iteration_option_name(&self, range: RangeS<'s>) -> &'s IterationOptionNameS<'s> {
    match self
      .intern_imprecise_name(IImpreciseNameValS::IterationOptionName(IterationOptionNameValS { range }))
    {
      IImpreciseNameS::IterationOptionName(r) => r,
      _ => unreachable!("intern_imprecise_name(IterationOptionName) yields it"),
    }
  }

  pub fn intern_while_cond_result_name(&self, range: RangeS<'s>) -> &'s WhileCondResultNameS<'s> {
    match self
      .intern_imprecise_name(IImpreciseNameValS::WhileCondResultName(WhileCondResultNameValS { range }))
    {
      IImpreciseNameS::WhileCondResultName(r) => r,
      _ => unreachable!("intern_imprecise_name(WhileCondResultName) yields it"),
    }
  }

  pub fn intern_self_imprecise_name(&self) -> &'s SelfNameS {
    match self.intern_imprecise_name(IImpreciseNameValS::SelfName(SelfNameValS {})) {
      IImpreciseNameS::SelfName(r) => r,
      _ => unreachable!("intern_imprecise_name(SelfName) yields it"),
    }
  }

  pub fn intern_anonymous_substruct_member_name(
    &self,
    index: i32,
  ) -> &'s AnonymousSubstructMemberNameS {
    match self.intern_imprecise_name(IImpreciseNameValS::AnonymousSubstructMemberName(
      AnonymousSubstructMemberNameValS { index },
    )) {
      IImpreciseNameS::AnonymousSubstructMemberName(r) => r,
      _ => unreachable!("intern_imprecise_name(AnonymousSubstructMemberName) yields it"),
    }
  }

  pub fn intern_desugared_param_name(
    &self,
    code_location: CodeLocationS<'s>,
  ) -> &'s DesugaredParamNameS<'s> {
    match self.intern_imprecise_name(IImpreciseNameValS::DesugaredParamName(DesugaredParamNameValS {
      code_location,
    })) {
      IImpreciseNameS::DesugaredParamName(r) => r,
      _ => unreachable!("intern_imprecise_name(DesugaredParamName) yields it"),
    }
  }

  fn alloc_imprecise_name_canonical(&self, val: IImpreciseNameValS<'s>) -> IImpreciseNameS<'s> {
    match val {
      CodeName(p) => IImpreciseNameS::CodeName(
        self.bump.alloc(CodeNameS { name: p.name, _must_intern: ScoutInterned(()) }),
      ),
      ConstructingMemberImpreciseName(p) => IImpreciseNameS::ConstructingMemberImpreciseName(
        self.bump.alloc(ConstructingMemberImpreciseNameS {
          name: p.name,
          _must_intern: ScoutInterned(()),
        }),
      ),
      IterableName(p) => IImpreciseNameS::IterableName(
        self.bump.alloc(IterableNameS { range: p.range, _must_intern: ScoutInterned(()) }),
      ),
      IteratorName(p) => IImpreciseNameS::IteratorName(
        self.bump.alloc(IteratorNameS { range: p.range, _must_intern: ScoutInterned(()) }),
      ),
      IterationOptionName(p) => IImpreciseNameS::IterationOptionName(
        self.bump.alloc(IterationOptionNameS { range: p.range, _must_intern: ScoutInterned(()) }),
      ),
      LambdaImpreciseName(_p) => IImpreciseNameS::LambdaImpreciseName(
        self.bump.alloc(LambdaImpreciseNameS { _must_intern: ScoutInterned(()) }),
      ),
      PlaceholderImpreciseName(p) => IImpreciseNameS::PlaceholderImpreciseName(
        self.bump.alloc(PlaceholderImpreciseNameS { index: p.index, _must_intern: ScoutInterned(()) }),
      ),
      LambdaStructImpreciseName(v) => {
        let payload = LambdaStructImpreciseNameS {
          lambda_name: v.lambda_name,
          _must_intern: ScoutInterned(()),
        };
        IImpreciseNameS::LambdaStructImpreciseName(self.bump.alloc(payload))
      }
      ClosureParamImpreciseName(_p) => IImpreciseNameS::ClosureParamImpreciseName(
        self.bump.alloc(ClosureParamImpreciseNameS { _must_intern: ScoutInterned(()) }),
      ),
      PrototypeName(_p) => IImpreciseNameS::PrototypeName(
        self.bump.alloc(PrototypeNameS { _must_intern: ScoutInterned(()) }),
      ),
      AnonymousSubstructTemplateImpreciseName(v) => {
        let payload = AnonymousSubstructTemplateImpreciseNameS {
          interface_imprecise_name: v.interface_imprecise_name,
          _must_intern: ScoutInterned(()),
        };
        IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(self.bump.alloc(payload))
      }
      AnonymousSubstructConstructorTemplateImpreciseName(v) => {
        let payload = AnonymousSubstructConstructorTemplateImpreciseNameS {
          interface_imprecise_name: v.interface_imprecise_name,
          _must_intern: ScoutInterned(()),
        };
        IImpreciseNameS::AnonymousSubstructConstructorTemplateImpreciseName(
          self.bump.alloc(payload),
        )
      }
      ImplImpreciseName(v) => {
        let payload = ImplImpreciseNameS {
          sub_citizen_imprecise_name: v.sub_citizen_imprecise_name,
          super_interface_imprecise_name: v.super_interface_imprecise_name,
          _must_intern: ScoutInterned(()),
        };
        IImpreciseNameS::ImplImpreciseName(self.bump.alloc(payload))
      }
      ImplSubCitizenImpreciseName(v) => {
        let payload = ImplSubCitizenImpreciseNameS {
          sub_citizen_imprecise_name: v.sub_citizen_imprecise_name,
          _must_intern: ScoutInterned(()),
        };
        IImpreciseNameS::ImplSubCitizenImpreciseName(self.bump.alloc(payload))
      }
      ImplSuperInterfaceImpreciseName(v) => {
        let payload = ImplSuperInterfaceImpreciseNameS {
          super_interface_imprecise_name: v.super_interface_imprecise_name,
          _must_intern: ScoutInterned(()),
        };
        IImpreciseNameS::ImplSuperInterfaceImpreciseName(self.bump.alloc(payload))
      }
      SelfName(_p) => IImpreciseNameS::SelfName(
        self.bump.alloc(SelfNameS { _must_intern: ScoutInterned(()) }),
      ),
      RuneName(v) => {
        let payload = RuneNameS { rune: v.rune, _must_intern: ScoutInterned(()) };
        IImpreciseNameS::RuneName(self.bump.alloc(payload))
      }
      ArbitraryName(_p) => IImpreciseNameS::ArbitraryName(
        self.bump.alloc(ArbitraryNameS { _must_intern: ScoutInterned(()) }),
      ),
      MagicParamName(p) => IImpreciseNameS::MagicParamName(self.bump.alloc(MagicParamImpreciseNameS {
        code_location: p.code_location,
        lid: p.lid,
        _must_intern: ScoutInterned(()),
      })),
      WhileCondResultName(p) => IImpreciseNameS::WhileCondResultName(
        self.bump.alloc(WhileCondResultNameS { range: p.range, _must_intern: ScoutInterned(()) }),
      ),
      AnonymousSubstructMemberName(p) => {
        IImpreciseNameS::AnonymousSubstructMemberName(self.bump.alloc(
          AnonymousSubstructMemberNameS { index: p.index, _must_intern: ScoutInterned(()) },
        ))
      }
      DesugaredParamName(p) => IImpreciseNameS::DesugaredParamName(self.bump.alloc(DesugaredParamNameS {
        code_location: p.code_location,
        _must_intern: ScoutInterned(()),
      })),
    }
  }

  // --- Name interning ---

  pub fn intern_struct_declaration_name(
    &self,
    val: TopLevelStructDeclarationNameS<'s>,
  ) -> &'s TopLevelStructDeclarationNameS<'s> {
    match self.intern_name(INameValS::TopLevelStructDeclaration(val)) {
      INameS::TopLevelStructDeclaration(r) => r,
      _ => unreachable!(),
    }
  }

  pub fn intern_interface_declaration_name(
    &self,
    val: TopLevelInterfaceDeclarationNameS<'s>,
  ) -> &'s TopLevelInterfaceDeclarationNameS<'s> {
    match self.intern_name(INameValS::TopLevelInterfaceDeclaration(val)) {
      INameS::TopLevelInterfaceDeclaration(r) => r,
      _ => unreachable!(),
    }
  }

  pub fn intern_function_imprecise_name(
    &self,
    val: IFunctionImpreciseNameValS<'s>,
  ) -> IFunctionImpreciseNameS<'s> {
    {
      let inner = self.inner.borrow();
      if let Some(existing) = inner.function_imprecise_name_val_to_ref.get(&val) {
        return existing.clone();
      }
    }
    let canonical = self.alloc_function_imprecise_name_canonical(val.clone());
    let mut inner = self.inner.borrow_mut();
    inner.function_imprecise_name_val_to_ref.insert(val, canonical.clone());
    canonical
  }

  /// The forwarder payload is a shallow already-canonical-children case (its `inner` is an
  /// already-canonical `IFunctionImpreciseNameS`), so no `'tmp` machinery is needed. The other
  /// variants reduce to the shared imprecise-name interner (`CodeNameS`/`LambdaImpreciseNameS`).
  fn alloc_function_imprecise_name_canonical(
    &self,
    val: IFunctionImpreciseNameValS<'s>,
  ) -> IFunctionImpreciseNameS<'s> {
    match val {
      IFunctionImpreciseNameValS::FunctionName(CodeNameValS { name }) => {
        IFunctionImpreciseNameS::FunctionName(self.intern_code_name(name))
      }
      IFunctionImpreciseNameValS::ConstructorName(CodeNameValS { name }) => {
        IFunctionImpreciseNameS::ConstructorName(self.intern_code_name(name))
      }
      IFunctionImpreciseNameValS::LambdaDeclarationName(_) => {
        IFunctionImpreciseNameS::LambdaDeclarationName(self.intern_lambda_imprecise_name())
      }
      IFunctionImpreciseNameValS::ForwarderFunctionDeclarationName(v) => {
        IFunctionImpreciseNameS::ForwarderFunctionDeclarationName(self.bump.alloc(
          ForwarderFunctionImpreciseNameS {
            inner: v.inner,
            index: v.index,
            _must_intern: ScoutInterned(()),
          },
        ))
      }
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
      INameValS::ImplDeclaration(p) => INameS::ImplDeclaration(self.bump.alloc(p)),
      INameValS::AnonymousSubstructImplDeclaration(AnonymousSubstructImplDeclarationNameValS {
        interface,
      }) => {
        let payload = AnonymousSubstructImplDeclarationNameS { interface: interface.clone() };
        INameS::AnonymousSubstructImplDeclaration(self.bump.alloc(payload))
      }
      INameValS::ExportAsName(p) => INameS::ExportAsName(self.bump.alloc(p)),
      INameValS::LetName(p) => INameS::LetName(self.bump.alloc(p)),
      INameValS::TopLevelStructDeclaration(p) => {
        INameS::TopLevelStructDeclaration(self.bump.alloc(p))
      }
      INameValS::TopLevelInterfaceDeclaration(p) => {
        INameS::TopLevelInterfaceDeclaration(self.bump.alloc(p))
      }
      INameValS::LambdaStructDeclaration(p) => INameS::LambdaStructDeclaration(self.bump.alloc(p)),
      INameValS::AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameValS {
        interface_name,
      }) => {
        let payload = AnonymousSubstructTemplateNameS { interface_name: interface_name.clone() };
        INameS::AnonymousSubstructTemplateName(self.bump.alloc(payload))
      }
      INameValS::RuneName(v) => {
        let payload = RuneNameS { rune: v.rune, _must_intern: ScoutInterned(()) };
        INameS::RuneName(self.bump.alloc(payload))
      }
      INameValS::RuntimeSizedArrayDeclarationName(p) => {
        INameS::RuntimeSizedArrayDeclarationName(self.bump.alloc(p))
      }
      INameValS::StaticSizedArrayDeclarationName(p) => {
        INameS::StaticSizedArrayDeclarationName(self.bump.alloc(p))
      }
      INameValS::GlobalFunctionFamilyName(p) => {
        INameS::GlobalFunctionFamilyName(self.bump.alloc(p))
      }
      INameValS::ArbitraryName(_p) => INameS::ArbitraryName(
        self.bump.alloc(ArbitraryNameS { _must_intern: ScoutInterned(()) }),
      ),
    }
  }

  /// Function declaration names are identity (not interned, @WVSBIZ): arena-alloc directly, no
  /// dedup. Callers wrap the result in `INameS::FunctionDeclaration` where a denizen name is needed.
  pub fn alloc_function_declaration_name(
    &self,
    val: IFunctionDeclarationNameS<'s>,
  ) -> &'s IFunctionDeclarationNameS<'s> {
    self.bump.alloc(val)
  }


  // --- Rune interning ---

  // Per @DSAUIMZ, slices are arena-allocated here on miss, not by the caller.
  pub fn intern_rune<'tmp>(&self, val: IRuneValS<'s, 'tmp>) -> IRuneS<'s> {
    {
      let inner = self.inner.borrow();
      let query = RuneValQuery(&val);
      if let Some(existing) = inner.rune_val_to_ref.get(&query) {
        return existing.clone(); // HIT — zero allocation
      }
    }
    // MISS — promote val (arena-alloc slices) and build canonical
    let (promoted_key, canonical) = self.alloc_rune_canonical(val);
    let mut inner = self.inner.borrow_mut();
    inner.rune_val_to_ref.insert(promoted_key, canonical.clone());
    canonical
  }

  /// Promotes a Val (which may borrow temporaries via 'tmp) into an arena-allocated
  /// canonical IRuneS and a stored key IRuneValS<'s, 's>.
  /// Per @DSAUIMZ, this is where lid slices get arena-allocated — only on intern miss.
  fn alloc_rune_canonical<'tmp>(
    &self,
    val: IRuneValS<'s, 'tmp>,
  ) -> (IRuneValS<'s, 's>, IRuneS<'s>) {
    match val {
      // ── 7 lid variants: promote LocationInDenizenVal → LocationInDenizen ──
      ImplicitRune(v) => {
        let lid = v.lid().promote_in(self.bump);
        let canonical = IRuneS::ImplicitRune(self.bump.alloc(ImplicitRuneS { lid }));
        (
          IRuneValS::ImplicitRune(ImplicitRuneValS::new(LocationInDenizenVal::from_canonical(
            &lid,
          ))),
          canonical,
        )
      }
      CallRegionRune(v) => {
        let lid = v.lid().promote_in(self.bump);
        let canonical = IRuneS::CallRegionRune(self.bump.alloc(CallRegionRuneS { lid }));
        (
          IRuneValS::CallRegionRune(CallRegionRuneValS::new(LocationInDenizenVal::from_canonical(
            &lid,
          ))),
          canonical,
        )
      }
      CallPureMergeRegionRune(v) => {
        let lid = v.lid().promote_in(self.bump);
        let canonical =
          IRuneS::CallPureMergeRegionRune(self.bump.alloc(CallPureMergeRegionRuneS { lid }));
        (
          IRuneValS::CallPureMergeRegionRune(CallPureMergeRegionRuneValS::new(
            LocationInDenizenVal::from_canonical(&lid),
          )),
          canonical,
        )
      }
      LetImplicitRune(v) => {
        let lid = v.lid().promote_in(self.bump);
        let canonical = IRuneS::LetImplicitRune(self.bump.alloc(LetImplicitRuneS { lid }));
        (
          IRuneValS::LetImplicitRune(LetImplicitRuneValS::new(
            LocationInDenizenVal::from_canonical(&lid),
          )),
          canonical,
        )
      }
      MagicParamRune(v) => {
        let lid = v.lid().promote_in(self.bump);
        let canonical = IRuneS::MagicParamRune(self.bump.alloc(MagicParamRuneS { lid }));
        (
          IRuneValS::MagicParamRune(MagicParamRuneValS::new(LocationInDenizenVal::from_canonical(
            &lid,
          ))),
          canonical,
        )
      }
      LocalDefaultRegionRune(v) => {
        let lid = v.lid().promote_in(self.bump);
        let canonical =
          IRuneS::LocalDefaultRegionRune(self.bump.alloc(LocalDefaultRegionRuneS { lid }));
        (
          IRuneValS::LocalDefaultRegionRune(LocalDefaultRegionRuneValS::new(
            LocationInDenizenVal::from_canonical(&lid),
          )),
          canonical,
        )
      }
      // ── Shallow Val variants (already have separate Val structs) ──
      // Clone v for the stored key before moving fields into the canonical payload.
      // These inner fields are all small Copy-ish types (IRuneS is a tagged pointer).
      ImplicitRegionRune(v) => {
        let key = v.clone();
        let payload = ImplicitRegionRuneS { original_rune: v.original_rune };
        let canonical = IRuneS::ImplicitRegionRune(self.bump.alloc(payload));
        (IRuneValS::ImplicitRegionRune(key), canonical)
      }
      ImplicitCoercionTemplateRune(v) => {
        let key = v.clone();
        let payload = ImplicitCoercionTemplateRuneS {
          range: v.range,
          original_kind_rune: v.original_kind_rune,
        };
        let canonical = IRuneS::ImplicitCoercionTemplateRune(self.bump.alloc(payload));
        (IRuneValS::ImplicitCoercionTemplateRune(key), canonical)
      }
      AnonymousSubstructMethodInheritedRune(v) => {
        let key = v.clone();
        let payload = AnonymousSubstructMethodInheritedRuneS {
          interface: v.interface,
          method: v.method,
          inner: v.inner,
        };
        let canonical = IRuneS::AnonymousSubstructMethodInheritedRune(self.bump.alloc(payload));
        (IRuneValS::AnonymousSubstructMethodInheritedRune(key), canonical)
      }
      DispatcherRuneFromImpl(v) => {
        let key = v.clone();
        let payload = DispatcherRuneFromImplS { inner_rune: v.inner_rune };
        let canonical = IRuneS::DispatcherRuneFromImpl(self.bump.alloc(payload));
        (IRuneValS::DispatcherRuneFromImpl(key), canonical)
      }
      CaseRuneFromImpl(v) => {
        let key = v.clone();
        let payload = CaseRuneFromImplS { inner_rune: v.inner_rune };
        let canonical = IRuneS::CaseRuneFromImpl(self.bump.alloc(payload));
        (IRuneValS::CaseRuneFromImpl(key), canonical)
      }
      // ── Simple Val variants (same struct in both enums) ──
      CodeRune(p) => {
        let c = IRuneS::CodeRune(self.bump.alloc(p.clone()));
        (IRuneValS::CodeRune(p), c)
      }
      ImplDropKindRune(p) => {
        let c = IRuneS::ImplDropKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::ImplDropKindRune(p), c)
      }
      ImplDropVoidRune(p) => {
        let c = IRuneS::ImplDropVoidRune(self.bump.alloc(p.clone()));
        (IRuneValS::ImplDropVoidRune(p), c)
      }
      ReachablePrototypeRune(p) => {
        let c = IRuneS::ReachablePrototypeRune(self.bump.alloc(p.clone()));
        (IRuneValS::ReachablePrototypeRune(p), c)
      }
      FreeOverrideStructTemplateRune(p) => {
        let c = IRuneS::FreeOverrideStructTemplateRune(self.bump.alloc(p.clone()));
        (IRuneValS::FreeOverrideStructTemplateRune(p), c)
      }
      FreeOverrideStructRune(p) => {
        let c = IRuneS::FreeOverrideStructRune(self.bump.alloc(p.clone()));
        (IRuneValS::FreeOverrideStructRune(p), c)
      }
      FreeOverrideInterfaceRune(p) => {
        let c = IRuneS::FreeOverrideInterfaceRune(self.bump.alloc(p.clone()));
        (IRuneValS::FreeOverrideInterfaceRune(p), c)
      }
      MemberRune(p) => {
        let c = IRuneS::MemberRune(self.bump.alloc(p.clone()));
        (IRuneValS::MemberRune(p), c)
      }
      DenizenDefaultRegionRune(p) => {
        let c = IRuneS::DenizenDefaultRegionRune(self.bump.alloc(p.clone()));
        (IRuneValS::DenizenDefaultRegionRune(p), c)
      }
      ExportDefaultRegionRune(p) => {
        let c = IRuneS::ExportDefaultRegionRune(self.bump.alloc(p.clone()));
        (IRuneValS::ExportDefaultRegionRune(p), c)
      }
      ExternDefaultRegionRune(p) => {
        let c = IRuneS::ExternDefaultRegionRune(self.bump.alloc(p.clone()));
        (IRuneValS::ExternDefaultRegionRune(p), c)
      }
      ArraySizeImplicitRune(p) => {
        let c = IRuneS::ArraySizeImplicitRune(self.bump.alloc(p.clone()));
        (IRuneValS::ArraySizeImplicitRune(p), c)
      }
      ArrayMutabilityImplicitRune(p) => {
        let c = IRuneS::ArrayMutabilityImplicitRune(self.bump.alloc(p.clone()));
        (IRuneValS::ArrayMutabilityImplicitRune(p), c)
      }
      ReturnRune(p) => {
        let c = IRuneS::ReturnRune(self.bump.alloc(p.clone()));
        (IRuneValS::ReturnRune(p), c)
      }
      StructNameRune(p) => {
        let c = IRuneS::StructNameRune(self.bump.alloc(p.clone()));
        (IRuneValS::StructNameRune(p), c)
      }
      InterfaceNameRune(p) => {
        let c = IRuneS::InterfaceNameRune(self.bump.alloc(p.clone()));
        (IRuneValS::InterfaceNameRune(p), c)
      }
      SelfRune(p) => {
        let c = IRuneS::SelfRune(self.bump.alloc(p.clone()));
        (IRuneValS::SelfRune(p), c)
      }
      SelfKindRune(p) => {
        let c = IRuneS::SelfKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::SelfKindRune(p), c)
      }
      SelfFullTypeRune(p) => {
        let c = IRuneS::SelfFullTypeRune(self.bump.alloc(p.clone()));
        (IRuneValS::SelfFullTypeRune(p), c)
      }
      SelfKindTemplateRune(p) => {
        let c = IRuneS::SelfKindTemplateRune(self.bump.alloc(p.clone()));
        (IRuneValS::SelfKindTemplateRune(p), c)
      }
      MacroVoidKindRune(p) => {
        let c = IRuneS::MacroVoidKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::MacroVoidKindRune(p), c)
      }
      MacroSelfKindRune(p) => {
        let c = IRuneS::MacroSelfKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::MacroSelfKindRune(p), c)
      }
      MacroSelfKindTemplateRune(p) => {
        let c = IRuneS::MacroSelfKindTemplateRune(self.bump.alloc(p.clone()));
        (IRuneValS::MacroSelfKindTemplateRune(p), c)
      }
      ArgumentRune(p) => {
        let c = IRuneS::ArgumentRune(self.bump.alloc(p.clone()));
        (IRuneValS::ArgumentRune(p), c)
      }
      PatternInputRune(p) => {
        let c = IRuneS::PatternInputRune(self.bump.alloc(p.clone()));
        (IRuneValS::PatternInputRune(p), c)
      }
      ExplicitTemplateArgRune(p) => {
        let c = IRuneS::ExplicitTemplateArgRune(self.bump.alloc(p.clone()));
        (IRuneValS::ExplicitTemplateArgRune(p), c)
      }
      AnonymousSubstructParentInterfaceTemplateRune(p) => {
        let c = IRuneS::AnonymousSubstructParentInterfaceTemplateRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructParentInterfaceTemplateRune(p), c)
      }
      AnonymousSubstructParentInterfaceKindRune(p) => {
        let c = IRuneS::AnonymousSubstructParentInterfaceKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructParentInterfaceKindRune(p), c)
      }
      AnonymousSubstructTemplateRune(p) => {
        let c = IRuneS::AnonymousSubstructTemplateRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructTemplateRune(p), c)
      }
      AnonymousSubstructKindRune(p) => {
        let c = IRuneS::AnonymousSubstructKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructKindRune(p), c)
      }
      AnonymousSubstructVoidKindRune(p) => {
        let c = IRuneS::AnonymousSubstructVoidKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructVoidKindRune(p), c)
      }
      AnonymousSubstructMemberRune(p) => {
        let c = IRuneS::AnonymousSubstructMemberRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructMemberRune(p), c)
      }
      AnonymousSubstructMethodSelfBorrowKindRune(p) => {
        let c = IRuneS::AnonymousSubstructMethodSelfBorrowKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructMethodSelfBorrowKindRune(p), c)
      }
      AnonymousSubstructDropBoundPrototypeRune(p) => {
        let c = IRuneS::AnonymousSubstructDropBoundPrototypeRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructDropBoundPrototypeRune(p), c)
      }
      AnonymousSubstructDropBoundParamsListRune(p) => {
        let c = IRuneS::AnonymousSubstructDropBoundParamsListRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructDropBoundParamsListRune(p), c)
      }
      StructDropBoundPrototypeRune(p) => {
        let c = IRuneS::StructDropBoundPrototypeRune(self.bump.alloc(p.clone()));
        (IRuneValS::StructDropBoundPrototypeRune(p), c)
      }
      StructDropBoundParamsListRune(p) => {
        let c = IRuneS::StructDropBoundParamsListRune(self.bump.alloc(p.clone()));
        (IRuneValS::StructDropBoundParamsListRune(p), c)
      }
      AnonymousSubstructFunctionBoundPrototypeRune(p) => {
        let c = IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructFunctionBoundPrototypeRune(p), c)
      }
      AnonymousSubstructFunctionBoundParamsListRune(p) => {
        let c = IRuneS::AnonymousSubstructFunctionBoundParamsListRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructFunctionBoundParamsListRune(p), c)
      }
      AnonymousSubstructFunctionInterfaceTemplateRune(p) => {
        let c = IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructFunctionInterfaceTemplateRune(p), c)
      }
      AnonymousSubstructFunctionInterfaceKindRune(p) => {
        let c = IRuneS::AnonymousSubstructFunctionInterfaceKindRune(self.bump.alloc(p.clone()));
        (IRuneValS::AnonymousSubstructFunctionInterfaceKindRune(p), c)
      }
      FunctorPrototypeRuneName(p) => {
        let c = IRuneS::FunctorPrototypeRuneName(self.bump.alloc(p.clone()));
        (IRuneValS::FunctorPrototypeRuneName(p), c)
      }
      FunctorParamRuneName(p) => {
        let c = IRuneS::FunctorParamRuneName(self.bump.alloc(p.clone()));
        (IRuneValS::FunctorParamRuneName(p), c)
      }
      FunctorReturnRuneName(p) => {
        let c = IRuneS::FunctorReturnRuneName(self.bump.alloc(p.clone()));
        (IRuneValS::FunctorReturnRuneName(p), c)
      }
    }
  }
}
