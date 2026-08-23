// MetalLowerer: instantiated IR (HinputsI) -> MetalCache, via the onion FFI builders.
//
// A faithful ~1:1 translation: each ExpressionIE node maps to the matching `cache.expr_*`
// builder, each KindIT to a `cache.get_*` kind, names are mangled via the instantiated
// humanizer. No lowering logic beyond that mangling. Placement / index resolution / deref
// semantics all live downstream in C++ codegen.
//
// TODO(onion): edges/vtables (interface_to_sub_citizen_to_edge), interface super-lists, and
// static/runtime array *definitions* are not yet reconstructed here. Virtual dispatch and
// array codegen need them, but nothing runs until step 4, so they're deferred to a focused
// follow-up. The expression/kind/function/struct/interface/export/extern spine is complete.

use std::collections::HashMap;

use crate::backend_ffi::metal_cache::{
    Edge, Expression, Function, InterfaceDef, InterfaceMethod, Kind, Local, MetalCache, Mutability,
    Name, PackageCoord, Program, Prototype, StructDef, StructMember, Weakability,
};
use crate::instantiating::ast::ast::{FunctionDefinitionI, PrototypeI};
use crate::instantiating::ast::citizens::{InterfaceDefinitionI, StructDefinitionI};
use crate::instantiating::ast::expressions::ExpressionIE;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::{IdI, IVarNameI};
use crate::instantiating::ast::types::{
    BorrowRefIT, InterfaceIT, KindIT, SharednessI, StaticSizedArrayIT, StructIT,
};
use crate::instantiating::instantiated_humanizer::humanize_id;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::CodeLocationS;

/// Walk a `HinputsI` and populate the `MetalCache`, returning a fully constructed `Program`.
pub fn populate_metal_cache<'cache, 's, 'i>(
    cache: &'cache MetalCache,
    monouts: &HinputsI<'s, 'i>,
) -> Program<'cache>
where
    's: 'i,
{
    let lowerer = Lowerer { cache };

    // HinputsI is flat; group defs by their id's package coordinate (first-seen order).
    let mut package_coords: Vec<&'s PackageCoordinate<'s>> = Vec::new();
    let mut seen: HashMap<usize, ()> = HashMap::new();
    let mut note = |pc: &'s PackageCoordinate<'s>, out: &mut Vec<&'s PackageCoordinate<'s>>| {
        if seen.insert(pc as *const _ as usize, ()).is_none() {
            out.push(pc);
        }
    };
    for f in monouts.functions.iter() {
        note(f.header.id.package_coord, &mut package_coords);
    }
    for s in monouts.structs.iter() {
        note(s.instantiated_citizen.id.package_coord, &mut package_coords);
    }
    for it in monouts.interfaces.iter() {
        note(it.instantiated_interface.id.package_coord, &mut package_coords);
    }

    let pb = cache.new_program_builder();
    for pc in package_coords {
        let coord = lowerer.lower_package_coord(pc);
        let package = lowerer.lower_package(monouts, pc, coord);
        pb.add_package(coord, package);
    }
    pb.finish()
}

struct Lowerer<'cache> {
    cache: &'cache MetalCache,
}

fn code_map<'s>(loc: CodeLocationS<'s>) -> String {
    format!("{:?}", loc)
}

impl<'cache> Lowerer<'cache> {
    fn lower_package_coord<'s>(&self, pc: &PackageCoordinate<'s>) -> PackageCoord<'cache> {
        // Empty module -> "__vale" (Backend's userFuncName convention).
        let project = if pc.module.0.is_empty() { "__vale" } else { pc.module.0 };
        let steps: Vec<&str> = pc.packages.iter().map(|s| s.0).collect();
        self.cache.get_package_coordinate(project, &steps)
    }

    fn lower_id_to_name<'s, 'i>(&self, id: &IdI<'s, 'i>) -> Name<'cache> {
        let name_str = humanize_id(&code_map, id, None);
        let coord = self.lower_package_coord(id.package_coord);
        self.cache.get_name(coord, &name_str)
    }

    fn lower_kind<'s, 'i>(&self, kind: KindIT<'s, 'i>) -> Kind<'cache> {
        let c = self.cache;
        match kind {
            KindIT::NeverIT(_) => c.never_kind(),
            KindIT::VoidIT(_) => c.void_kind(),
            KindIT::BoolIT(_) => c.bool_kind(),
            KindIT::StrIT(_) => c.str_kind(),
            KindIT::FloatIT(_) => c.float_kind(),
            KindIT::IntIT(i) => c.get_int(c.mut_region_id(), i.bits),
            KindIT::USizeIT(_) => c.get_usize(c.mut_region_id()),
            KindIT::StructIT(s) => c.get_struct_kind(self.lower_id_to_name(&s.id)),
            KindIT::InterfaceIT(i) => c.get_interface_kind(self.lower_id_to_name(&i.id)),
            KindIT::StaticSizedArrayIT(a) => c.get_static_sized_array(self.lower_id_to_name(&a.name)),
            KindIT::RuntimeSizedArrayIT(a) => c.get_runtime_sized_array(self.lower_id_to_name(&a.name)),
            KindIT::BorrowRefIT(r) => c.get_borrow_ref(self.lower_kind(r.inner)),
            KindIT::OwnRefIT(r) => c.get_own_ref(self.lower_kind(r.inner)),
            KindIT::ShareRefIT(r) => c.get_share_ref(self.lower_kind(r.inner)),
            KindIT::WeakRefIT(r) => c.get_weak_ref(self.lower_kind(r.inner)),
        }
    }

    fn lower_borrow<'s, 'i>(&self, r: &BorrowRefIT<'s, 'i>) -> Kind<'cache> {
        self.cache.get_borrow_ref(self.lower_kind(r.inner))
    }

    fn lower_struct_kind<'s, 'i>(&self, s: StructIT<'s, 'i>) -> Kind<'cache> {
        self.cache.get_struct_kind(self.lower_id_to_name(&s.id))
    }
    fn lower_interface_kind<'s, 'i>(&self, i: InterfaceIT<'s, 'i>) -> Kind<'cache> {
        self.cache.get_interface_kind(self.lower_id_to_name(&i.id))
    }
    fn lower_static_array_kind<'s, 'i>(&self, a: &StaticSizedArrayIT<'s, 'i>) -> Kind<'cache> {
        self.cache.get_static_sized_array(self.lower_id_to_name(&a.name))
    }

    fn lower_prototype<'s, 'i>(&self, proto: &PrototypeI<'s, 'i>) -> Prototype<'cache> {
        let name = self.lower_id_to_name(&proto.id);
        let return_type = self.lower_kind(proto.return_type);
        let params: Vec<Kind<'cache>> = proto.param_types().into_iter().map(|k| self.lower_kind(k)).collect();
        self.cache.get_prototype(name, return_type, &params)
    }

    /// Forward the local's identity to the metal `Local`. The instantiator reallocates a fresh
    /// `LocalVariableI` per mention, so we don't dedup here — the `id` (the structurally-unique
    /// `IVarNameI`) is the identity, and the backend `BlockState` keys on it. `name` is the
    /// display/LLVM name only.
    fn lower_local<'s, 'i>(&self, var: &crate::instantiating::ast::ast::LocalVariableI<'s, 'i>) -> Local<'cache> {
        let id = format!("{:?}", var.name);
        let name_str = humanize_var_name(var.name);
        self.cache.get_local(&id, &name_str, self.lower_kind(var.tyype))
    }

    fn lower_package<'s, 'i>(
        &self,
        monouts: &HinputsI<'s, 'i>,
        pc: &'s PackageCoordinate<'s>,
        coord: PackageCoord<'cache>,
    ) -> crate::backend_ffi::metal_cache::Package<'cache> {
        let pkg_key = pc as *const _ as usize;
        let pb = self.cache.new_package_builder(coord);

        for f in monouts.functions.iter() {
            if f.header.id.package_coord as *const _ as usize != pkg_key {
                continue;
            }
            let func = self.lower_function(f);
            pb.add_function(&humanize_id(&code_map, &f.header.id, None), func);
        }
        for s in monouts.structs.iter() {
            if s.instantiated_citizen.id.package_coord as *const _ as usize != pkg_key {
                continue;
            }
            let sd = self.lower_struct_def(s);
            pb.add_struct(&humanize_id(&code_map, &s.instantiated_citizen.id, None), sd);
        }
        for it in monouts.interfaces.iter() {
            if it.instantiated_interface.id.package_coord as *const _ as usize != pkg_key {
                continue;
            }
            let id = self.lower_interface_def(it);
            pb.add_interface(&humanize_id(&code_map, &it.instantiated_interface.id, None), id);
        }

        for e in monouts.function_exports.iter() {
            if e.export_id.package_coord as *const _ as usize != pkg_key {
                continue;
            }
            pb.add_export_function(e.exported_name.0, self.lower_prototype(e.prototype));
        }
        for e in monouts.kind_exports.iter() {
            if e.id.package_coord as *const _ as usize != pkg_key {
                continue;
            }
            pb.add_export_kind(e.exported_name.0, self.lower_kind(e.tyype));
        }
        for e in monouts.function_externs.iter() {
            if e.prototype.id.package_coord as *const _ as usize != pkg_key {
                continue;
            }
            // The extern's wire name is its mangled id; codegen maps it to the native symbol.
            pb.add_extern_function(&humanize_id(&code_map, &e.prototype.id, None), self.lower_prototype(e.prototype));
        }
        for (struct_it, _extern) in monouts.kind_externs.iter() {
            if struct_it.id.package_coord as *const _ as usize != pkg_key {
                continue;
            }
            pb.add_extern_kind(&humanize_id(&code_map, &struct_it.id, None), self.lower_struct_kind(**struct_it));
        }

        pb.finish()
    }

    fn lower_function<'s, 'i>(&self, f: &FunctionDefinitionI<'s, 'i>) -> Function<'cache> {
        let proto = self.lower_prototype(&f.header.to_prototype());
        let body = self.lower_expression(&f.body);
        self.cache.new_function(proto, Some(body))
    }

    fn lower_struct_def<'s, 'i>(&self, s: &StructDefinitionI<'s, 'i>) -> StructDef<'cache> {
        let name = self.lower_id_to_name(&s.instantiated_citizen.id);
        let kind = self.lower_struct_kind(*s.instantiated_citizen);
        let region_id = self.region_for(s.sharedness);
        let members: Vec<StructMember<'cache>> = s
            .members
            .iter()
            .map(|m| {
                let member_name = humanize_var_name(m.name);
                self.cache.new_struct_member(&member_name, &member_name, self.lower_kind(m.tyype))
            })
            .collect();
        // TODO(onion): edges are in HinputsI.interface_to_sub_citizen_to_edge, not inline; reconstruct.
        let edges: Vec<Edge<'cache>> = Vec::new();
        self.cache.new_struct_def(
            name,
            kind,
            region_id,
            self.mutability_for(s.sharedness),
            &edges,
            &members,
            if s.weakable { Weakability::Weakable } else { Weakability::NonWeakable },
        )
    }

    fn lower_interface_def<'s, 'i>(&self, it: &InterfaceDefinitionI<'s, 'i>) -> InterfaceDef<'cache> {
        let name = self.lower_id_to_name(&it.instantiated_interface.id);
        let kind = self.lower_interface_kind(*it.instantiated_interface);
        let region_id = self.region_for(it.sharedness);
        let methods: Vec<InterfaceMethod<'cache>> = it
            .internal_methods
            .iter()
            .map(|(proto, vindex)| {
                self.cache.get_interface_method(self.lower_prototype(proto), *vindex)
            })
            .collect();
        // TODO(onion): super-interface list isn't carried on InterfaceDefinitionI; empty for now.
        let super_interfaces: Vec<Name<'cache>> = Vec::new();
        self.cache.new_interface_def(
            name,
            kind,
            region_id,
            self.mutability_for(it.sharedness),
            &super_interfaces,
            &methods,
            if it.weakable { Weakability::Weakable } else { Weakability::NonWeakable },
        )
    }

    fn region_for(&self, sharedness: SharednessI) -> crate::backend_ffi::metal_cache::RegionId<'cache> {
        match sharedness {
            SharednessI::Shared => self.cache.rcimm_region_id(),
            SharednessI::Single => self.cache.mut_region_id(),
        }
    }
    fn mutability_for(&self, sharedness: SharednessI) -> Mutability {
        match sharedness {
            SharednessI::Shared => Mutability::Immutable,
            SharednessI::Single => Mutability::Mutable,
        }
    }

    fn lower_exprs<'s, 'i>(&self, exprs: &[ExpressionIE<'s, 'i>]) -> Vec<Expression<'cache>> {
        exprs.iter().map(|e| self.lower_expression(e)).collect()
    }

    fn lower_expression<'s, 'i>(&self, expr: &ExpressionIE<'s, 'i>) -> Expression<'cache> {
        let c = self.cache;
        match expr {
            ExpressionIE::ConstantInt(x) => c.expr_constant_int(x.value, x.bits),
            ExpressionIE::ConstantBool(x) => c.expr_constant_bool(x.value),
            ExpressionIE::ConstantFloat(x) => c.expr_constant_f64(x.value),
            ExpressionIE::ConstantStr(x) => c.expr_constant_str(x.value, self.lower_kind(x.result)),
            ExpressionIE::VoidLiteral(_) => c.expr_constant_void(),
            ExpressionIE::Break(_) => c.expr_break(),

            ExpressionIE::Return(x) => c.expr_return(self.lower_expression(&x.source_expr), self.lower_kind(x.source_type)),
            ExpressionIE::Discard(x) => c.expr_discard(self.lower_expression(&x.expr), self.lower_kind(x.source_type)),
            ExpressionIE::Block(x) => c.expr_block(self.lower_expression(&x.inner), self.lower_kind(x.result)),
            ExpressionIE::Consecutor(x) => c.expr_consecutor(&self.lower_exprs(x.exprs), self.lower_kind(x.result)),

            ExpressionIE::ArgLookup(x) => c.expr_argument(x.param_index, self.lower_kind(x.tyype)),
            ExpressionIE::LetNormal(x) => {
                let local = self.lower_local(x.variable);
                c.expr_stackify(local, self.lower_expression(&x.expr), self.lower_kind(x.result))
            }
            ExpressionIE::LetAndLend(x) => {
                let local = self.lower_local(x.variable);
                c.expr_let_and_lend(local, self.lower_expression(&x.expr), self.lower_kind(x.result))
            }
            ExpressionIE::Restackify(x) => {
                let local = self.lower_local(x.variable);
                c.expr_restackify(local, self.lower_expression(&x.source_expr), self.lower_kind(x.result))
            }
            ExpressionIE::Unlet(x) => {
                let local = self.lower_local(x.variable);
                c.expr_unstackify(local, self.lower_kind(x.result))
            }
            ExpressionIE::LocalLookup(x) => {
                let local = self.lower_local(x.local_variable);
                c.expr_local_lookup(local, self.lower_borrow(x.result))
            }

            ExpressionIE::Deref(x) => c.expr_deref(self.lower_expression(&x.inner), self.lower_kind(x.source_type), self.lower_kind(x.result)),
            ExpressionIE::MemberLookup(x) => c.expr_member_lookup(
                self.lower_expression(&x.struct_expr),
                self.lower_borrow(x.struct_type),
                x.member_index,
                &humanize_var_name(x.member_name),
                self.lower_kind(x.member_type),
                self.lower_borrow(x.result),
            ),
            ExpressionIE::StaticSizedArrayLookup(x) => c.expr_static_sized_array_lookup(
                self.lower_expression(&x.array_expr),
                self.lower_borrow(x.array_type),
                self.lower_expression(&x.index_expr),
                self.lower_kind(x.index_type),
                self.lower_borrow(x.result),
            ),
            ExpressionIE::RuntimeSizedArrayLookup(x) => c.expr_runtime_sized_array_lookup(
                self.lower_expression(&x.array_expr),
                self.lower_borrow(x.array_type),
                self.lower_expression(&x.index_expr),
                self.lower_kind(x.index_type),
                self.lower_borrow(x.result),
            ),

            ExpressionIE::Mutate(x) => c.expr_mutate(
                self.lower_expression(&x.destination_expr),
                self.lower_borrow(x.destination_type),
                self.lower_expression(&x.source_expr),
                self.lower_kind(x.source_type),
                self.lower_kind(x.result),
            ),

            ExpressionIE::Construct(x) => c.expr_new_struct(
                self.lower_struct_kind(x.struct_tt),
                self.lower_kind(x.result),
                &self.lower_exprs(x.args),
            ),
            ExpressionIE::Destroy(x) => c.expr_destroy(
                self.lower_expression(&x.expr),
                self.lower_struct_kind(x.struct_tt),
                &self.lower_locals(x.destination_reference_variables),
            ),
            ExpressionIE::CopyPrim(x) => c.expr_copy_prim(self.lower_expression(&x.inner), self.lower_kind(x.source_type), self.lower_kind(x.result)),

            ExpressionIE::Upcast(x) => c.expr_struct_to_interface_upcast(
                self.lower_expression(&x.inner_expr),
                self.lower_kind(x.source_type),
                self.lower_interface_kind(x.target_interface),
                self.lower_id_to_name(&x.impl_name),
                self.lower_kind(x.result),
            ),
            ExpressionIE::InterfaceToInterfaceUpcast(x) => c.expr_interface_to_interface_upcast(
                self.lower_expression(&x.inner_expr),
                self.lower_interface_kind(x.target_interface),
                self.lower_kind(x.result),
            ),
            ExpressionIE::AsSubtype(x) => c.expr_as_subtype(
                self.lower_expression(&x.source_expr),
                self.lower_kind(x.source_type),
                self.lower_kind(x.target_type),
                self.lower_prototype(x.ok_constructor),
                self.lower_prototype(x.err_constructor),
                self.lower_id_to_name(&x.impl_name),
                self.lower_id_to_name(&x.ok_impl_name),
                self.lower_id_to_name(&x.err_impl_name),
                self.lower_kind(x.result),
            ),
            ExpressionIE::IsSameInstance(x) => {
                c.expr_is_same_instance(self.lower_expression(&x.left), self.lower_kind(x.left_type), self.lower_expression(&x.right), self.lower_kind(x.right_type))
            }

            ExpressionIE::BorrowToWeak(x) => c.expr_weak_alias(self.lower_expression(&x.inner_expr), self.lower_kind(x.source_type), self.lower_kind(x.result)),
            ExpressionIE::LockWeak(x) => c.expr_lock_weak(
                self.lower_expression(&x.inner_expr),
                self.lower_kind(x.source_type),
                self.lower_prototype(&x.some_constructor),
                self.lower_prototype(&x.none_constructor),
                self.lower_id_to_name(&x.some_impl_name),
                self.lower_id_to_name(&x.none_impl_name),
                self.lower_kind(x.result),
            ),

            ExpressionIE::FunctionCall(x) => c.expr_call(
                self.lower_prototype(&x.callable),
                &self.lower_exprs(x.args),
                self.lower_kind(x.result),
            ),
            ExpressionIE::ExternFunctionCall(x) => c.expr_extern_call(
                self.lower_prototype(&x.prototype2),
                &self.lower_exprs(x.args),
                self.lower_kind(x.result),
            ),
            ExpressionIE::InterfaceFunctionCall(x) => c.expr_interface_call(
                self.lower_prototype(x.super_function_prototype),
                x.virtual_param_index,
                x.index_in_edge,
                &self.lower_exprs(x.args),
                self.lower_kind(x.result),
            ),

            ExpressionIE::If(x) => c.expr_if(
                self.lower_expression(&x.condition),
                self.lower_expression(&x.then_call),
                self.lower_expression(&x.else_call),
                self.lower_kind(x.then_result_type),
                self.lower_kind(x.else_result_type),
                self.lower_kind(x.result),
            ),
            ExpressionIE::While(x) => {
                let block = c.expr_block(self.lower_expression(&x.block.inner), self.lower_kind(x.block.result));
                c.expr_while(block, self.lower_kind(x.result))
            }

            ExpressionIE::StaticArrayFromValues(x) => c.expr_new_array_from_values(
                &self.lower_exprs(x.elements),
                self.lower_kind(x.result),
                self.lower_static_array_kind(x.array_type),
            ),
            ExpressionIE::NewRuntimeSizedArray(x) => c.expr_new_mut_runtime_sized_array(
                self.lower_kind(KindIT::RuntimeSizedArrayIT(&x.array_type)),
                self.lower_expression(&x.capacity_expr),
                self.lower_kind(x.result),
            ),
            ExpressionIE::StaticArrayFromCallable(x) => c.expr_static_array_from_callable(
                self.lower_kind(KindIT::StaticSizedArrayIT(&x.array_type)),
                self.lower_expression(&x.generator),
                self.lower_prototype(&x.generator_method),
                self.lower_kind(x.result),
            ),
            ExpressionIE::ArrayLength(x) => c.expr_array_length(self.lower_expression(&x.array_expr), self.lower_borrow(x.array_type)),
            ExpressionIE::RuntimeSizedArrayCapacity(x) => c.expr_array_capacity(self.lower_expression(&x.array_expr), self.lower_borrow(x.array_type)),
            ExpressionIE::ArraySize(x) => c.expr_array_size(self.lower_expression(&x.array), self.lower_kind(x.result)),
            ExpressionIE::PushRuntimeSizedArray(x) => c.expr_push_runtime_sized_array(
                self.lower_expression(&x.array_expr),
                self.lower_borrow(x.array_type),
                self.lower_expression(&x.new_element_expr),
                self.lower_kind(x.element_type),
            ),
            ExpressionIE::PopRuntimeSizedArray(x) => c.expr_pop_runtime_sized_array(
                self.lower_expression(&x.array_expr),
                self.lower_borrow(x.array_type),
                self.lower_kind(x.result),
            ),
            ExpressionIE::DestroyStaticSizedArrayIntoFunction(x) => c.expr_destroy_static_sized_array_into_function(
                self.lower_expression(&x.array_expr),
                self.lower_kind(KindIT::StaticSizedArrayIT(&x.array_type)),
                self.lower_expression(&x.consumer),
                self.lower_prototype(&x.consumer_method),
            ),
            ExpressionIE::DestroyStaticSizedArrayIntoLocals(x) => c.expr_destroy_static_sized_array_into_locals(
                self.lower_expression(&x.expr),
                self.lower_kind(KindIT::StaticSizedArrayIT(&x.static_sized_array)),
                &self.lower_locals(x.destination_reference_variables),
            ),
            ExpressionIE::DestroyRuntimeSizedArray(x) => {
                c.expr_destroy_mut_runtime_sized_array(self.lower_expression(&x.array_expr))
            }
        }
    }

    fn lower_locals<'s, 'i>(
        &self,
        vars: &[&crate::instantiating::ast::ast::LocalVariableI<'s, 'i>],
    ) -> Vec<Local<'cache>> {
        vars.iter().map(|v| self.lower_local(v)).collect()
    }
}

fn humanize_var_name<'s, 'i>(name: IVarNameI<'s, 'i>) -> String {
    crate::instantiating::instantiated_humanizer::humanize_name(&code_map, name.into(), None)
}
