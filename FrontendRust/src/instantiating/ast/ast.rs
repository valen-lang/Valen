use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{CoordI, KindIT, ICitizenIT, MutabilityI, VariabilityI, StructIT};
use crate::instantiating::ast::names::{
    IdI, INameI,
    IFunctionNameI, IImplNameI, IInterfaceNameI, IStructNameI, ICitizenNameI,
    IRegionNameI, IVarNameI,
    ExportNameI, FunctionBoundNameI, ImplBoundNameI,
};
use crate::instantiating::instantiating_interner::MustIntern;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::ast::expressions::ReferenceExpressionIE;
use crate::instantiating::ast::types::InterfaceIT;
use crate::utils::code_hierarchy::PackageCoordinate;


// mig: struct KindExportI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindExportI<'s, 'i> {
    pub range: RangeS<'s>,
    pub tyype: KindIT<'s, 'i>,
    pub id: IdI<'s, 'i>,
    pub exported_name: StrI<'s>,
}

// mig: impl KindExportI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for KindExportI` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for KindExportI` below.)

// mig: struct FunctionExportI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionExportI<'s, 'i> where 's: 'i {
    pub range: RangeS<'s>,
    pub prototype: &'i PrototypeI<'s, 'i>,
    pub export_id: IdI<'s, 'i>,
    pub exported_name: StrI<'s>,
}

// mig: impl FunctionExportI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionExportI` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionExportI` below.)

// mig: struct FunctionExternI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionExternI<'s, 'i> where 's: 'i {
    pub prototype: &'i PrototypeI<'s, 'i>,
    // How many of the function's trailing generic-arg slots were inherited from a parent
    // citizen template, per @PRIIROZ (0 = no inheritance / top-level extern). Hammer uses
    // this to reshape the wire-format SimpleId so container template args land on the
    // citizen step (e.g. Vec<i32>::capacity rather than Vec::capacity<i32>), which is
    // what the Backend's rustifySimpleId expects per @SMLRZ.
    pub num_inherited_generic_parameters: i32,
}

// mig: impl FunctionExternI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionExternI` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Canonical groups equals/hashCode on one physical line — see the eq block above.)

// mig: struct KindExternI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindExternI<'s, 'i> where 's: 'i {
    pub r#struct: &'i StructIT<'s, 'i>,
}

// mig: impl KindExternI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by the #[derive(PartialEq, Eq)] above.)

// mig: fn hash_code (realized-by-impl Hash)
// (Canonical groups equals/hashCode on one physical line — see the eq block above.)


// mig: struct InterfaceEdgeBlueprintI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceEdgeBlueprintI<'s, 'i> where 's: 'i {
    pub interface: IdI<'s, 'i>,
    pub super_family_root_headers: &'i [(&'i PrototypeI<'s, 'i>, i32)],
}

// mig: impl InterfaceEdgeBlueprintI
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceEdgeBlueprintI` below.)

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceEdgeBlueprintI` below.)

// mig: struct EdgeI
/// Temporary state
#[derive(PartialEq, Eq, Debug)]
pub struct EdgeI<'s, 'i> where 's: 'i {
    pub edge_id: IdI<'s, 'i>,
    pub sub_citizen: ICitizenIT<'s, 'i>,
    pub super_interface: IdI<'s, 'i>,
    pub rune_to_func_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub abstract_func_to_override_func: ArenaIndexMap<'i, IdI<'s, 'i>, &'i PrototypeI<'s, 'i>>,
}

// mig: impl EdgeI
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for EdgeI` below.)

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for EdgeI` below.)

// mig: struct FunctionDefinitionI
/// Temporary state
#[derive(Debug)]
pub struct FunctionDefinitionI<'s, 'i> where 's: 'i {
    pub header: FunctionHeaderI<'s, 'i>,
    pub rune_to_func_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub body: ReferenceExpressionIE<'s, 'i>,
}

// mig: impl FunctionDefinitionI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionDefinitionI` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionDefinitionI` below.)

// mig: fn is_pure
impl<'s, 'i> FunctionDefinitionI<'s, 'i> {
    pub fn is_pure(&self) -> bool {
        panic!("Unimplemented: is_pure")
        // header.isPure
    }
}

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<FunctionDefinitionI> for IFunctionNameI` or inline match.)

// mig: struct LocationInFunctionEnvironmentI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocationInFunctionEnvironmentI<'i> {
    pub path: &'i [i32],
}
// mig: impl LocationInFunctionEnvironmentI

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LocationInFunctionEnvironmentI` below.)

// mig: fn add
impl<'i> LocationInFunctionEnvironmentI<'i> {
    pub fn add(&self, sub_location: i32) -> LocationInFunctionEnvironmentI<'i> {
        panic!("Unimplemented: add")
        // LocationInFunctionEnvironmentI(path :+ subLocation)
    }

// mig: fn to_string
    pub fn to_string(&self) -> String {
        self.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(".")
    }
}

// mig: struct AbstractI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AbstractI;
// mig: impl AbstractI

// mig: struct ParameterI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ParameterI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i>,
    pub virtuality: Option<AbstractI>,
    pub pre_checked: bool,
    pub tyype: CoordI<'s, 'i>,
}

// mig: impl ParameterI
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ParameterI` below.)

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ParameterI` below.)

// mig: fn same
impl<'s, 'i> ParameterI<'s, 'i> {
    pub fn same(&self, that: &ParameterI<'_, '_>) -> bool {
        panic!("Unimplemented: same")
        // name == that.name && virtuality == that.virtuality && tyype == that.tyype
    }
}

// mig: struct SignatureI
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SignatureI<'s, 'i> {
    pub id: IdI<'s, 'i>,
    pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SignatureIValI<'s, 'i> {
    pub id: IdI<'s, 'i>,
}
// mig: impl SignatureI

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for SignatureI` below.)

// mig: fn param_types
impl<'s, 'i> SignatureI<'s, 'i> {
    pub fn param_types(&self) -> Vec<()> {
        panic!("Unimplemented: param_types")
        // id.localName.parameters
    }
}

// mig: enum IFunctionAttributeI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionAttributeI<'s> {
    PureI,
    UserFunctionI,
    ExternI(ExternI<'s>),
}

// mig: impl IFunctionAttributeI
// mig: enum ICitizenAttributeI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenAttributeI<'s> {
    SealedI,
    ExternI(ExternI<'s>),
}

// mig: impl ICitizenAttributeI
// mig: struct ExternI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternI<'s> {
    pub package_coord: PackageCoordinate<'s>,
}
// mig: impl ExternI

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ExternI` below.)

// mig: struct RegionI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegionI<'s, 'i> where 's: 'i {
    pub name: IRegionNameI<'s, 'i>,
    pub mutable: bool,
}

// mig: impl RegionI
// mig: struct FunctionHeaderI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionHeaderI<'s, 'i> where 's: 'i {
    // This one little name field can illuminate much of how the compiler works, see UINIT.
    pub id: IdI<'s, 'i>,
    pub attributes: &'i [IFunctionAttributeI<'s>],
//  regions: Vector[cIegionI],
    pub params: &'i [ParameterI<'s, 'i>],
    pub return_type: CoordI<'s, 'i>,
}

// mig: impl FunctionHeaderI
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionHeaderI` below.)

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionHeaderI` below.)

// mig: fn is_extern
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
    pub fn is_extern(&self) -> bool {
        panic!("Unimplemented: is_extern")
        // attributes.exists({ case ExternI(_) => true case _ => false })
    }

// mig: fn is_user_function
    pub fn is_user_function(&self) -> bool {
        self.attributes.contains(&IFunctionAttributeI::UserFunctionI)
    }

// mig: fn get_abstract_interface
    pub fn get_abstract_interface(&self) -> Option<&'i InterfaceIT<'s, 'i>> {
        let abstract_interfaces: Vec<_> = self.params.iter().filter_map(|p| match (p.virtuality, p.tyype.kind) {
            (Some(AbstractI), KindIT::InterfaceIT(ir)) => Some(ir),
            _ => None,
        }).collect();
        assert!(abstract_interfaces.len() <= 1);
        abstract_interfaces.into_iter().next()
    }

// mig: fn get_virtual_index
    pub fn get_virtual_index(&self) -> Option<i32> {
        panic!("Unimplemented: get_virtual_index")
        // val indices = params.zipWithIndex.collect({ case (ParameterI(_, Some(AbstractI()), _, _), index) => index })
        // vassert(indices.size <= 1)
        // indices.headOption
    }

// mig: fn to_prototype
    pub fn to_prototype(&self, interner: &InstantiatingInterner<'s, 'i>) -> PrototypeI<'s, 'i> {
        //    val substituter = TemplataCompiler.getPlaceholderSubstituter(interner, fullName, templateArgs)
        //    val paramTypes = params.map(_.tyype).map(substituter.substituteForCoord)
        //    val newLastStep = fullName.last.makeFunctionName(interner, keywords, templateArgs, paramTypes)
        //    val newName = FullNameI(fullName.packageCoord, fullName.initSteps, newLastStep)
        *interner.intern_prototype_ci(PrototypeIValI { id: self.id, return_type: self.return_type })
    }

// mig: fn to_signature
    pub fn to_signature(&self) -> SignatureI<'_, '_> {
        panic!("Unimplemented: to_signature")
        // toPrototype.toSignature
    }
}

// mig: fn param_types
impl<'s, 'i> FunctionHeaderI<'s, 'i> where 's: 'i {
    pub fn param_types(&self) -> Vec<CoordI<'s, 'i>> {
        IFunctionNameI::try_from(self.id.local_name).unwrap().parameters().to_vec()
    }
}

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<FunctionHeaderI> for (IdI, Vec<ParameterI>, CoordI)` or inline match.)

// mig: fn is_pure
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
    pub fn is_pure(&self) -> bool {
        panic!("Unimplemented: is_pure")
        // attributes.collectFirst({ case PureI => }).nonEmpty
    }
}

// mig: struct PrototypeI
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeI<'s, 'i> {
    pub id: IdI<'s, 'i>,
    pub return_type: CoordI<'s, 'i>,
    pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeIValI<'s, 'i> {
    pub id: IdI<'s, 'i>,
    pub return_type: CoordI<'s, 'i>,
}
// mig: impl PrototypeI

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for PrototypeI` below.)

// mig: fn param_types
impl<'s, 'i> PrototypeI<'s, 'i> where 's: 'i {
    pub fn param_types(&self) -> Vec<CoordI<'s, 'i>> {
        IFunctionNameI::try_from(self.id.local_name).unwrap().parameters().to_vec()
    }
}

// mig: fn to_signature
impl<'s, 'i> PrototypeI<'s, 'i> {
    pub fn to_signature(&self) -> SignatureIValI<'s, 'i> {
        SignatureIValI { id: self.id }
    }
}

// mig: enum IVariableI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVariableI<'s, 'i> where 's: 'i {
    AddressibleLocalVariableI(&'i AddressibleLocalVariableI<'s, 'i>),
    ReferenceLocalVariableI(&'i ReferenceLocalVariableI<'s, 'i>),
    AddressibleClosureVariableI(&'i AddressibleClosureVariableI<'s, 'i>),
    ReferenceClosureVariableI(&'i ReferenceClosureVariableI<'s, 'i>),
}

// mig: impl IVariableI
// mig: fn name
impl<'s, 'i> IVariableI<'s, 'i> {
    pub fn name(&self) -> () {
        panic!("Unimplemented: name")
    }

// mig: fn variability
    pub fn variability(&self) -> () {
        panic!("Unimplemented: variability")
    }

// mig: fn collapsed_coord
    pub fn collapsed_coord(&self) -> () {
        panic!("Unimplemented: collapsed_coord")
    }
}

// mig: enum ILocalVariableI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ILocalVariableI<'s, 'i> where 's: 'i {
    AddressibleLocalVariableI(&'i AddressibleLocalVariableI<'s, 'i>),
    ReferenceLocalVariableI(&'i ReferenceLocalVariableI<'s, 'i>),
}

// mig: impl ILocalVariableI
// mig: fn name
impl<'s, 'i> ILocalVariableI<'s, 'i> {
    pub fn name(&self) -> IVarNameI<'s, 'i> {
        match self {
            ILocalVariableI::ReferenceLocalVariableI(rlv) => rlv.name,
            ILocalVariableI::AddressibleLocalVariableI(alv) => alv.name,
        }
    }

// mig: fn collapsed_coord
    pub fn collapsed_coord(&self) -> CoordI<'s, 'i> {
        match self {
            ILocalVariableI::AddressibleLocalVariableI(alv) => alv.collapsed_coord,
            ILocalVariableI::ReferenceLocalVariableI(rlv) => rlv.collapsed_coord,
        }
    }

// mig: fn variability (inherited from IVariableI, no direct ILocalVariableI Scala decl)
    pub fn variability(&self) -> VariabilityI {
        match self {
            ILocalVariableI::AddressibleLocalVariableI(alv) => alv.variability,
            ILocalVariableI::ReferenceLocalVariableI(rlv) => rlv.variability,
        }
    }
}

// mig: struct AddressibleLocalVariableI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleLocalVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i>,
}

// mig: impl AddressibleLocalVariableI
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for AddressibleLocalVariableI` below.)

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for AddressibleLocalVariableI` below.)

// mig: struct ReferenceLocalVariableI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceLocalVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i>,
}

// mig: impl ReferenceLocalVariableI
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReferenceLocalVariableI` below.)

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReferenceLocalVariableI` below.)

// mig: struct AddressibleClosureVariableI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleClosureVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i>,
    pub closured_vars_struct_type: StructIT<'s, 'i>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i>,
}

// mig: impl AddressibleClosureVariableI
// mig: struct ReferenceClosureVariableI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceClosureVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i>,
    pub closured_vars_struct_type: StructIT<'s, 'i>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i>,
}

// mig: impl ReferenceClosureVariableI
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReferenceClosureVariableI` below.)

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReferenceClosureVariableI` below.)
