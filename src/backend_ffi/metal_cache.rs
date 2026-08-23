// Safe-ish Rust bindings for Backend/src/metal/metal_cache_ffi.h.
//
// Dumb, faithful 1:1 mirror of the onion FFI: each method wraps one builder call, converting
// handle newtypes to `*mut c_void` and back. Types are the onion `Kind` (a bare kind is owned;
// the wrap getters express references). There is no Reference/ownership/location here, and no
// lowering; all of that is downstream in C++ codegen.
//
// Handles are `*mut Opaque` newtypes (`Copy`, so a borrow can be passed to several FFI calls);
// lifetime safety is enforced via `&MetalCache` on the wrappers: handles returned by `get_*`
// borrow from the cache for as long as it lives.

use std::ffi::c_void;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr::NonNull;

#[repr(C)]
pub struct MetalCacheHandleRaw {
    _opaque: [u8; 0],
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct PackageCoord<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RegionId<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Name<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Kind<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Prototype<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InterfaceMethod<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StructMember<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Edge<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StructDef<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InterfaceDef<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Function<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
/// Opaque handle to a populated `Expression*` tree.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Expression<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
/// A local is a name + its onion kind; the lowerer constructs each once and reuses the handle.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Local<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mutability { Immutable = 0, Mutable = 1 }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Weakability { Weakable = 0, NonWeakable = 1 }

extern "C" {
    fn metal_cache_new() -> *mut MetalCacheHandleRaw;
    fn metal_cache_free(_: *mut MetalCacheHandleRaw);

    fn metal_cache_builtin_package_coord(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_rcimm_region_id(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_mut_region_id(_: *mut MetalCacheHandleRaw) -> *mut c_void;

    fn metal_cache_i32(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_i64(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_bool(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_float(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_str(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_never(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_cache_void(_: *mut MetalCacheHandleRaw) -> *mut c_void;

    fn metal_cache_get_package_coordinate(
        _: *mut MetalCacheHandleRaw,
        project_name_ptr: *const c_char, project_name_len: usize,
        steps_ptrs: *const *const c_char, steps_lens: *const usize, steps_count: usize,
    ) -> *mut c_void;

    fn metal_cache_get_region_id(
        _: *mut MetalCacheHandleRaw, package_coord: *mut c_void,
        id_ptr: *const c_char, id_len: usize,
    ) -> *mut c_void;

    fn metal_cache_get_name(
        _: *mut MetalCacheHandleRaw, package_coord: *mut c_void,
        name_ptr: *const c_char, name_len: usize,
    ) -> *mut c_void;

    fn metal_cache_get_int(_: *mut MetalCacheHandleRaw, region: *mut c_void, bits: i32) -> *mut c_void;
    fn metal_cache_get_bool(_: *mut MetalCacheHandleRaw, region: *mut c_void) -> *mut c_void;
    fn metal_cache_get_str(_: *mut MetalCacheHandleRaw, region: *mut c_void) -> *mut c_void;
    fn metal_cache_get_float(_: *mut MetalCacheHandleRaw, region: *mut c_void) -> *mut c_void;
    fn metal_cache_get_void(_: *mut MetalCacheHandleRaw, region: *mut c_void) -> *mut c_void;
    fn metal_cache_get_never(_: *mut MetalCacheHandleRaw, region: *mut c_void) -> *mut c_void;
    fn metal_cache_get_usize(_: *mut MetalCacheHandleRaw, region: *mut c_void) -> *mut c_void;

    fn metal_cache_get_struct_kind(_: *mut MetalCacheHandleRaw, name: *mut c_void) -> *mut c_void;
    fn metal_cache_get_interface_kind(_: *mut MetalCacheHandleRaw, name: *mut c_void) -> *mut c_void;
    fn metal_cache_get_static_sized_array(_: *mut MetalCacheHandleRaw, name: *mut c_void) -> *mut c_void;
    fn metal_cache_get_runtime_sized_array(_: *mut MetalCacheHandleRaw, name: *mut c_void) -> *mut c_void;

    fn metal_cache_get_borrow_ref(_: *mut MetalCacheHandleRaw, inner: *mut c_void) -> *mut c_void;
    fn metal_cache_get_own_ref(_: *mut MetalCacheHandleRaw, inner: *mut c_void) -> *mut c_void;
    fn metal_cache_get_share_ref(_: *mut MetalCacheHandleRaw, inner: *mut c_void) -> *mut c_void;
    fn metal_cache_get_weak_ref(_: *mut MetalCacheHandleRaw, inner: *mut c_void) -> *mut c_void;

    fn metal_cache_get_prototype(
        _: *mut MetalCacheHandleRaw, name: *mut c_void, return_type: *mut c_void,
        param_types: *const *mut c_void, param_count: usize,
    ) -> *mut c_void;

    fn metal_cache_get_interface_method(
        _: *mut MetalCacheHandleRaw, prototype: *mut c_void, virtual_param_index: i32,
    ) -> *mut c_void;

    fn metal_cache_get_local(
        _: *mut MetalCacheHandleRaw,
        id_ptr: *const c_char, id_len: usize,
        name_ptr: *const c_char, name_len: usize, kind: *mut c_void,
    ) -> *mut c_void;

    fn metal_struct_member_new(
        full_name_ptr: *const c_char, full_name_len: usize,
        name_ptr: *const c_char, name_len: usize,
        ty: *mut c_void,
    ) -> *mut c_void;

    fn metal_edge_new(
        struct_kind: *mut c_void, interface_kind: *mut c_void,
        interface_methods: *const *mut c_void,
        struct_prototypes: *const *mut c_void,
        pair_count: usize,
    ) -> *mut c_void;

    fn metal_struct_def_new(
        name: *mut c_void, struct_kind: *mut c_void, region_id: *mut c_void,
        mutability: u32,
        edges: *const *mut c_void, edge_count: usize,
        members: *const *mut c_void, member_count: usize,
        weakability: u32,
    ) -> *mut c_void;

    fn metal_interface_def_new(
        name: *mut c_void, interface_kind: *mut c_void, region_id: *mut c_void,
        mutability: u32,
        super_interfaces: *const *mut c_void, super_count: usize,
        methods: *const *mut c_void, method_count: usize,
        weakability: u32,
    ) -> *mut c_void;

    fn metal_function_new(prototype: *mut c_void, body: *mut c_void) -> *mut c_void;

    // Onion expression constructors.
    fn metal_expr_constant_void() -> *mut c_void;
    fn metal_expr_constant_int(value: i64, bits: i32) -> *mut c_void;
    fn metal_expr_constant_bool(value: i32) -> *mut c_void;
    fn metal_expr_constant_f64(value: f64) -> *mut c_void;
    fn metal_expr_constant_str(value_ptr: *const c_char, value_len: usize, result: *mut c_void) -> *mut c_void;
    fn metal_expr_break() -> *mut c_void;
    fn metal_expr_return(source_expr: *mut c_void, source_type: *mut c_void) -> *mut c_void;
    fn metal_expr_discard(expr: *mut c_void, source_type: *mut c_void) -> *mut c_void;
    fn metal_expr_block(inner: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_consecutor(exprs: *const *mut c_void, expr_count: usize, result: *mut c_void) -> *mut c_void;

    fn metal_expr_argument(param_index: i32, tyype: *mut c_void) -> *mut c_void;
    fn metal_expr_stackify(variable: *mut c_void, expr: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_let_and_lend(variable: *mut c_void, expr: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_restackify(variable: *mut c_void, source_expr: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_unstackify(variable: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_local_lookup(local_variable: *mut c_void, result: *mut c_void) -> *mut c_void;

    fn metal_expr_deref(inner: *mut c_void, source_type: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_member_lookup(
        struct_expr: *mut c_void, struct_type: *mut c_void, member_index: i32, member_name_ptr: *const c_char, member_name_len: usize, member_type: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_static_sized_array_lookup(
        array_expr: *mut c_void, array_type: *mut c_void, index_expr: *mut c_void, index_type: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_runtime_sized_array_lookup(
        array_expr: *mut c_void, array_type: *mut c_void, index_expr: *mut c_void, index_type: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;

    fn metal_expr_mutate(destination_expr: *mut c_void, destination_type: *mut c_void, source_expr: *mut c_void, source_type: *mut c_void, result: *mut c_void) -> *mut c_void;

    fn metal_expr_new_struct(
        struct_kind: *mut c_void, result: *mut c_void,
        args: *const *mut c_void, arg_count: usize,
    ) -> *mut c_void;
    fn metal_expr_destroy(
        expr: *mut c_void, struct_kind: *mut c_void,
        destination_locals: *const *mut c_void, local_count: usize,
    ) -> *mut c_void;
    fn metal_expr_copy_prim(inner: *mut c_void, source_type: *mut c_void, result: *mut c_void) -> *mut c_void;

    fn metal_expr_struct_to_interface_upcast(
        inner_expr: *mut c_void, source_type: *mut c_void, target_interface: *mut c_void, impl_name: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_interface_to_interface_upcast(
        inner_expr: *mut c_void, target_interface: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_as_subtype(
        source_expr: *mut c_void, source_type: *mut c_void, target_type: *mut c_void,
        ok_constructor: *mut c_void, err_constructor: *mut c_void,
        impl_name: *mut c_void, ok_impl_name: *mut c_void, err_impl_name: *mut c_void,
        result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_is_same_instance(left: *mut c_void, left_type: *mut c_void, right: *mut c_void, right_type: *mut c_void) -> *mut c_void;

    fn metal_expr_weak_alias(inner_expr: *mut c_void, source_type: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_lock_weak(
        inner_expr: *mut c_void, source_type: *mut c_void,
        some_constructor: *mut c_void, none_constructor: *mut c_void,
        some_impl_name: *mut c_void, none_impl_name: *mut c_void,
        result: *mut c_void,
    ) -> *mut c_void;

    fn metal_expr_call(
        callable: *mut c_void, args: *const *mut c_void, arg_count: usize, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_extern_call(
        prototype: *mut c_void, args: *const *mut c_void, arg_count: usize, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_interface_call(
        super_function_prototype: *mut c_void, virtual_param_index: i32, index_in_edge: i32,
        args: *const *mut c_void, arg_count: usize, result: *mut c_void,
    ) -> *mut c_void;

    fn metal_expr_if(
        condition: *mut c_void, then_call: *mut c_void, else_call: *mut c_void,
        then_result_type: *mut c_void, else_result_type: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_while(block: *mut c_void, result: *mut c_void) -> *mut c_void;

    fn metal_expr_new_array_from_values(
        elements: *const *mut c_void, element_count: usize, result: *mut c_void, array_type: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_new_mut_runtime_sized_array(
        array_type: *mut c_void, capacity_expr: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_static_array_from_callable(
        array_type: *mut c_void, generator: *mut c_void, generator_method: *mut c_void, result: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_array_length(array_expr: *mut c_void, array_type: *mut c_void) -> *mut c_void;
    fn metal_expr_array_capacity(array_expr: *mut c_void, array_type: *mut c_void) -> *mut c_void;
    fn metal_expr_array_size(array: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_push_runtime_sized_array(array_expr: *mut c_void, array_type: *mut c_void, new_element_expr: *mut c_void, element_type: *mut c_void) -> *mut c_void;
    fn metal_expr_pop_runtime_sized_array(array_expr: *mut c_void, array_type: *mut c_void, result: *mut c_void) -> *mut c_void;
    fn metal_expr_destroy_static_sized_array_into_function(
        array_expr: *mut c_void, array_type: *mut c_void, consumer: *mut c_void, consumer_method: *mut c_void,
    ) -> *mut c_void;
    fn metal_expr_destroy_static_sized_array_into_locals(
        expr: *mut c_void, static_sized_array: *mut c_void,
        destination_locals: *const *mut c_void, local_count: usize,
    ) -> *mut c_void;
    fn metal_expr_destroy_mut_runtime_sized_array(array_expr: *mut c_void) -> *mut c_void;

    fn metal_package_builder_new(_: *mut MetalCacheHandleRaw, package_coord: *mut c_void) -> *mut c_void;
    fn metal_package_builder_add_interface(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_add_struct(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_add_function(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_add_static_sized_array(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_add_runtime_sized_array(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_static_sized_array_def_new(
        name: *mut c_void, array_kind: *mut c_void, size: i32,
        region_id: *mut c_void,
        element_type: *mut c_void,
    ) -> *mut c_void;
    fn metal_runtime_sized_array_def_new(
        name: *mut c_void, array_kind: *mut c_void,
        region_id: *mut c_void,
        element_type: *mut c_void,
    ) -> *mut c_void;
    fn metal_package_builder_add_export_function(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_add_export_kind(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_add_extern_function(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_add_extern_kind(_: *mut c_void, name_ptr: *const c_char, name_len: usize, v: *mut c_void);
    fn metal_package_builder_finish(_: *mut c_void) -> *mut c_void;

    fn metal_program_builder_new(_: *mut MetalCacheHandleRaw) -> *mut c_void;
    fn metal_program_builder_add_package(_: *mut c_void, coord: *mut c_void, package: *mut c_void);
    fn metal_program_builder_finish(_: *mut c_void) -> *mut c_void;
    fn metal_program_free(program: *mut c_void);
}

/// Owning wrapper around a MetalCache. Frees the underlying cache on drop.
pub struct MetalCache {
    raw: *mut MetalCacheHandleRaw,
}

// Small helper to build a `Vec<*mut c_void>` from a slice of handle newtypes via their `.0`.
macro_rules! ptrs {
    ($slice:expr) => {{
        let v: Vec<*mut c_void> = $slice.iter().map(|h| h.0.as_ptr()).collect();
        v
    }};
}

impl MetalCache {
    pub fn new() -> Self {
        let raw = unsafe { metal_cache_new() };
        assert!(!raw.is_null(), "metal_cache_new returned null");
        MetalCache { raw }
    }

    pub fn raw(&self) -> *mut MetalCacheHandleRaw { self.raw }

    // --- Singletons ---

    pub fn builtin_package_coord(&self) -> PackageCoord<'_> {
        unsafe { PackageCoord(NonNull::new(metal_cache_builtin_package_coord(self.raw)).unwrap(), PhantomData) }
    }
    pub fn rcimm_region_id(&self) -> RegionId<'_> {
        unsafe { RegionId(NonNull::new(metal_cache_rcimm_region_id(self.raw)).unwrap(), PhantomData) }
    }
    pub fn mut_region_id(&self) -> RegionId<'_> {
        unsafe { RegionId(NonNull::new(metal_cache_mut_region_id(self.raw)).unwrap(), PhantomData) }
    }
    pub fn i32(&self) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_i32(self.raw)).unwrap(), PhantomData) }
    }
    pub fn i64(&self) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_i64(self.raw)).unwrap(), PhantomData) }
    }
    pub fn bool_kind(&self) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_bool(self.raw)).unwrap(), PhantomData) }
    }
    pub fn float_kind(&self) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_float(self.raw)).unwrap(), PhantomData) }
    }
    pub fn str_kind(&self) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_str(self.raw)).unwrap(), PhantomData) }
    }
    pub fn never_kind(&self) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_never(self.raw)).unwrap(), PhantomData) }
    }
    pub fn void_kind(&self) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_void(self.raw)).unwrap(), PhantomData) }
    }

    // --- Interned getters ---

    pub fn get_package_coordinate(&self, project_name: &str, steps: &[&str]) -> PackageCoord<'_> {
        let step_ptrs: Vec<*const c_char> = steps.iter().map(|s| s.as_ptr() as *const c_char).collect();
        let step_lens: Vec<usize> = steps.iter().map(|s| s.len()).collect();
        unsafe {
            PackageCoord(
                NonNull::new(metal_cache_get_package_coordinate(
                    self.raw,
                    project_name.as_ptr() as *const c_char, project_name.len(),
                    step_ptrs.as_ptr(), step_lens.as_ptr(), step_ptrs.len(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn get_region_id(&self, pkg: PackageCoord<'_>, id: &str) -> RegionId<'_> {
        unsafe {
            RegionId(
                NonNull::new(metal_cache_get_region_id(
                    self.raw, pkg.0.as_ptr(), id.as_ptr() as *const c_char, id.len(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn get_name(&self, pkg: PackageCoord<'_>, name: &str) -> Name<'_> {
        unsafe {
            Name(
                NonNull::new(metal_cache_get_name(
                    self.raw, pkg.0.as_ptr(), name.as_ptr() as *const c_char, name.len(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn get_int(&self, region: RegionId<'_>, bits: i32) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_int(self.raw, region.0.as_ptr(), bits)).unwrap(), PhantomData) }
    }
    pub fn get_bool(&self, region: RegionId<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_bool(self.raw, region.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_str(&self, region: RegionId<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_str(self.raw, region.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_float(&self, region: RegionId<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_float(self.raw, region.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_void(&self, region: RegionId<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_void(self.raw, region.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_never(&self, region: RegionId<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_never(self.raw, region.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_usize(&self, region: RegionId<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_usize(self.raw, region.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn get_struct_kind(&self, name: Name<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_struct_kind(self.raw, name.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_interface_kind(&self, name: Name<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_interface_kind(self.raw, name.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_static_sized_array(&self, name: Name<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_static_sized_array(self.raw, name.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_runtime_sized_array(&self, name: Name<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_runtime_sized_array(self.raw, name.0.as_ptr())).unwrap(), PhantomData) }
    }

    // Onion wrap kinds.
    pub fn get_borrow_ref(&self, inner: Kind<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_borrow_ref(self.raw, inner.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_own_ref(&self, inner: Kind<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_own_ref(self.raw, inner.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_share_ref(&self, inner: Kind<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_share_ref(self.raw, inner.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn get_weak_ref(&self, inner: Kind<'_>) -> Kind<'_> {
        unsafe { Kind(NonNull::new(metal_cache_get_weak_ref(self.raw, inner.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn get_prototype(&self, name: Name<'_>, return_type: Kind<'_>, param_types: &[Kind<'_>]) -> Prototype<'_> {
        let param_ptrs = ptrs!(param_types);
        unsafe {
            Prototype(
                NonNull::new(metal_cache_get_prototype(
                    self.raw, name.0.as_ptr(), return_type.0.as_ptr(),
                    param_ptrs.as_ptr(), param_ptrs.len(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn get_interface_method(&self, prototype: Prototype<'_>, virtual_param_index: i32) -> InterfaceMethod<'_> {
        unsafe {
            InterfaceMethod(
                NonNull::new(metal_cache_get_interface_method(
                    self.raw, prototype.0.as_ptr(), virtual_param_index,
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn get_local(&self, id: &str, name: &str, kind: Kind<'_>) -> Local<'_> {
        unsafe {
            Local(
                NonNull::new(metal_cache_get_local(
                    self.raw,
                    id.as_ptr() as *const c_char, id.len(),
                    name.as_ptr() as *const c_char, name.len(), kind.0.as_ptr(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    // --- Non-interned constructors ---

    pub fn new_struct_member(&self, full_name: &str, name: &str, ty: Kind<'_>) -> StructMember<'_> {
        unsafe {
            StructMember(
                NonNull::new(metal_struct_member_new(
                    full_name.as_ptr() as *const c_char, full_name.len(),
                    name.as_ptr() as *const c_char, name.len(),
                    ty.0.as_ptr(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn new_edge<'c>(
        &'c self, struct_kind: Kind<'c>, interface_kind: Kind<'c>,
        pairs: &[(InterfaceMethod<'c>, Prototype<'c>)],
    ) -> Edge<'c> {
        let methods: Vec<*mut c_void> = pairs.iter().map(|(im, _)| im.0.as_ptr()).collect();
        let protos: Vec<*mut c_void> = pairs.iter().map(|(_, p)| p.0.as_ptr()).collect();
        unsafe {
            Edge(
                NonNull::new(metal_edge_new(
                    struct_kind.0.as_ptr(), interface_kind.0.as_ptr(),
                    methods.as_ptr(), protos.as_ptr(), pairs.len(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn new_struct_def<'c>(
        &'c self, name: Name<'c>, struct_kind: Kind<'c>, region_id: RegionId<'c>,
        mutability: Mutability, edges: &[Edge<'c>], members: &[StructMember<'c>],
        weakability: Weakability,
    ) -> StructDef<'c> {
        let edge_ptrs = ptrs!(edges);
        let member_ptrs = ptrs!(members);
        unsafe {
            StructDef(
                NonNull::new(metal_struct_def_new(
                    name.0.as_ptr(), struct_kind.0.as_ptr(), region_id.0.as_ptr(),
                    mutability as u32,
                    edge_ptrs.as_ptr(), edge_ptrs.len(),
                    member_ptrs.as_ptr(), member_ptrs.len(),
                    weakability as u32,
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn new_interface_def<'c>(
        &'c self, name: Name<'c>, interface_kind: Kind<'c>, region_id: RegionId<'c>,
        mutability: Mutability, super_interfaces: &[Name<'c>], methods: &[InterfaceMethod<'c>],
        weakability: Weakability,
    ) -> InterfaceDef<'c> {
        let super_ptrs = ptrs!(super_interfaces);
        let method_ptrs = ptrs!(methods);
        unsafe {
            InterfaceDef(
                NonNull::new(metal_interface_def_new(
                    name.0.as_ptr(), interface_kind.0.as_ptr(), region_id.0.as_ptr(),
                    mutability as u32,
                    super_ptrs.as_ptr(), super_ptrs.len(),
                    method_ptrs.as_ptr(), method_ptrs.len(),
                    weakability as u32,
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn new_function<'c>(&'c self, prototype: Prototype<'c>, body: Option<Expression<'c>>) -> Function<'c> {
        let body_ptr = body.map(|e| e.0.as_ptr()).unwrap_or(std::ptr::null_mut());
        unsafe {
            Function(NonNull::new(metal_function_new(prototype.0.as_ptr(), body_ptr)).unwrap(), PhantomData)
        }
    }

    pub fn new_static_sized_array_def<'c>(
        &'c self, name: Name<'c>, kind: Kind<'c>, size: i32, region_id: RegionId<'c>, element_type: Kind<'c>,
    ) -> StaticSizedArrayDef<'c> {
        unsafe {
            StaticSizedArrayDef(
                NonNull::new(metal_static_sized_array_def_new(
                    name.0.as_ptr(), kind.0.as_ptr(), size, region_id.0.as_ptr(), element_type.0.as_ptr(),
                )).unwrap(),
                PhantomData,
            )
        }
    }
    pub fn new_runtime_sized_array_def<'c>(
        &'c self, name: Name<'c>, kind: Kind<'c>, region_id: RegionId<'c>, element_type: Kind<'c>,
    ) -> RuntimeSizedArrayDef<'c> {
        unsafe {
            RuntimeSizedArrayDef(
                NonNull::new(metal_runtime_sized_array_def_new(
                    name.0.as_ptr(), kind.0.as_ptr(), region_id.0.as_ptr(), element_type.0.as_ptr(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    // --- Expression constructors (onion) ---

    pub fn expr_constant_void(&self) -> Expression<'_> {
        unsafe { Expression(NonNull::new(metal_expr_constant_void()).unwrap(), PhantomData) }
    }
    pub fn expr_constant_int(&self, value: i64, bits: i32) -> Expression<'_> {
        unsafe { Expression(NonNull::new(metal_expr_constant_int(value, bits)).unwrap(), PhantomData) }
    }
    pub fn expr_constant_bool(&self, value: bool) -> Expression<'_> {
        unsafe { Expression(NonNull::new(metal_expr_constant_bool(value as i32)).unwrap(), PhantomData) }
    }
    pub fn expr_constant_f64(&self, value: f64) -> Expression<'_> {
        unsafe { Expression(NonNull::new(metal_expr_constant_f64(value)).unwrap(), PhantomData) }
    }
    pub fn expr_constant_str<'c>(&'c self, value: &str, result: Kind<'c>) -> Expression<'c> {
        unsafe {
            Expression(
                NonNull::new(metal_expr_constant_str(value.as_ptr() as *const c_char, value.len(), result.0.as_ptr())).unwrap(),
                PhantomData,
            )
        }
    }
    pub fn expr_break(&self) -> Expression<'_> {
        unsafe { Expression(NonNull::new(metal_expr_break()).unwrap(), PhantomData) }
    }
    pub fn expr_return<'c>(&'c self, source_expr: Expression<'c>, source_type: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_return(source_expr.0.as_ptr(), source_type.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_discard<'c>(&'c self, expr: Expression<'c>, source_type: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_discard(expr.0.as_ptr(), source_type.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_block<'c>(&'c self, inner: Expression<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_block(inner.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_consecutor<'c>(&'c self, exprs: &[Expression<'c>], result: Kind<'c>) -> Expression<'c> {
        let ptrs = ptrs!(exprs);
        unsafe { Expression(NonNull::new(metal_expr_consecutor(ptrs.as_ptr(), ptrs.len(), result.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_argument<'c>(&'c self, param_index: i32, tyype: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_argument(param_index, tyype.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_stackify<'c>(&'c self, variable: Local<'c>, expr: Expression<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_stackify(variable.0.as_ptr(), expr.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_let_and_lend<'c>(&'c self, variable: Local<'c>, expr: Expression<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_let_and_lend(variable.0.as_ptr(), expr.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_restackify<'c>(&'c self, variable: Local<'c>, source_expr: Expression<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_restackify(variable.0.as_ptr(), source_expr.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_unstackify<'c>(&'c self, variable: Local<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_unstackify(variable.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_local_lookup<'c>(&'c self, local_variable: Local<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_local_lookup(local_variable.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_deref<'c>(&'c self, inner: Expression<'c>, source_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_deref(inner.0.as_ptr(), source_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_member_lookup<'c>(&'c self, struct_expr: Expression<'c>, struct_type: Kind<'c>, member_index: i32, member_name: &str, member_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe {
            Expression(
                NonNull::new(metal_expr_member_lookup(
                    struct_expr.0.as_ptr(), struct_type.0.as_ptr(), member_index, member_name.as_ptr() as *const c_char, member_name.len(), member_type.0.as_ptr(), result.0.as_ptr(),
                )).unwrap(),
                PhantomData,
            )
        }
    }
    pub fn expr_static_sized_array_lookup<'c>(&'c self, array_expr: Expression<'c>, array_type: Kind<'c>, index_expr: Expression<'c>, index_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_static_sized_array_lookup(array_expr.0.as_ptr(), array_type.0.as_ptr(), index_expr.0.as_ptr(), index_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_runtime_sized_array_lookup<'c>(&'c self, array_expr: Expression<'c>, array_type: Kind<'c>, index_expr: Expression<'c>, index_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_runtime_sized_array_lookup(array_expr.0.as_ptr(), array_type.0.as_ptr(), index_expr.0.as_ptr(), index_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_mutate<'c>(&'c self, destination_expr: Expression<'c>, destination_type: Kind<'c>, source_expr: Expression<'c>, source_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_mutate(destination_expr.0.as_ptr(), destination_type.0.as_ptr(), source_expr.0.as_ptr(), source_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_new_struct<'c>(&'c self, struct_kind: Kind<'c>, result: Kind<'c>, args: &[Expression<'c>]) -> Expression<'c> {
        let ptrs = ptrs!(args);
        unsafe { Expression(NonNull::new(metal_expr_new_struct(struct_kind.0.as_ptr(), result.0.as_ptr(), ptrs.as_ptr(), ptrs.len())).unwrap(), PhantomData) }
    }
    pub fn expr_destroy<'c>(&'c self, expr: Expression<'c>, struct_kind: Kind<'c>, destination_locals: &[Local<'c>]) -> Expression<'c> {
        let ptrs = ptrs!(destination_locals);
        unsafe { Expression(NonNull::new(metal_expr_destroy(expr.0.as_ptr(), struct_kind.0.as_ptr(), ptrs.as_ptr(), ptrs.len())).unwrap(), PhantomData) }
    }
    pub fn expr_copy_prim<'c>(&'c self, inner: Expression<'c>, source_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_copy_prim(inner.0.as_ptr(), source_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_struct_to_interface_upcast<'c>(&'c self, inner_expr: Expression<'c>, source_type: Kind<'c>, target_interface: Kind<'c>, impl_name: Name<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_struct_to_interface_upcast(inner_expr.0.as_ptr(), source_type.0.as_ptr(), target_interface.0.as_ptr(), impl_name.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_interface_to_interface_upcast<'c>(&'c self, inner_expr: Expression<'c>, target_interface: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_interface_to_interface_upcast(inner_expr.0.as_ptr(), target_interface.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_as_subtype<'c>(
        &'c self, source_expr: Expression<'c>, source_type: Kind<'c>, target_type: Kind<'c>,
        ok_constructor: Prototype<'c>, err_constructor: Prototype<'c>,
        impl_name: Name<'c>, ok_impl_name: Name<'c>, err_impl_name: Name<'c>, result: Kind<'c>,
    ) -> Expression<'c> {
        unsafe {
            Expression(
                NonNull::new(metal_expr_as_subtype(
                    source_expr.0.as_ptr(), source_type.0.as_ptr(), target_type.0.as_ptr(),
                    ok_constructor.0.as_ptr(), err_constructor.0.as_ptr(),
                    impl_name.0.as_ptr(), ok_impl_name.0.as_ptr(), err_impl_name.0.as_ptr(), result.0.as_ptr(),
                )).unwrap(),
                PhantomData,
            )
        }
    }
    pub fn expr_is_same_instance<'c>(&'c self, left: Expression<'c>, left_type: Kind<'c>, right: Expression<'c>, right_type: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_is_same_instance(left.0.as_ptr(), left_type.0.as_ptr(), right.0.as_ptr(), right_type.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_weak_alias<'c>(&'c self, inner_expr: Expression<'c>, source_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_weak_alias(inner_expr.0.as_ptr(), source_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_lock_weak<'c>(
        &'c self, inner_expr: Expression<'c>, source_type: Kind<'c>,
        some_constructor: Prototype<'c>, none_constructor: Prototype<'c>,
        some_impl_name: Name<'c>, none_impl_name: Name<'c>, result: Kind<'c>,
    ) -> Expression<'c> {
        unsafe {
            Expression(
                NonNull::new(metal_expr_lock_weak(
                    inner_expr.0.as_ptr(), source_type.0.as_ptr(), some_constructor.0.as_ptr(), none_constructor.0.as_ptr(),
                    some_impl_name.0.as_ptr(), none_impl_name.0.as_ptr(), result.0.as_ptr(),
                )).unwrap(),
                PhantomData,
            )
        }
    }

    pub fn expr_call<'c>(&'c self, callable: Prototype<'c>, args: &[Expression<'c>], result: Kind<'c>) -> Expression<'c> {
        let ptrs = ptrs!(args);
        unsafe { Expression(NonNull::new(metal_expr_call(callable.0.as_ptr(), ptrs.as_ptr(), ptrs.len(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_extern_call<'c>(&'c self, prototype: Prototype<'c>, args: &[Expression<'c>], result: Kind<'c>) -> Expression<'c> {
        let ptrs = ptrs!(args);
        unsafe { Expression(NonNull::new(metal_expr_extern_call(prototype.0.as_ptr(), ptrs.as_ptr(), ptrs.len(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_interface_call<'c>(&'c self, super_function_prototype: Prototype<'c>, virtual_param_index: i32, index_in_edge: i32, args: &[Expression<'c>], result: Kind<'c>) -> Expression<'c> {
        let ptrs = ptrs!(args);
        unsafe { Expression(NonNull::new(metal_expr_interface_call(super_function_prototype.0.as_ptr(), virtual_param_index, index_in_edge, ptrs.as_ptr(), ptrs.len(), result.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_if<'c>(&'c self, condition: Expression<'c>, then_call: Expression<'c>, else_call: Expression<'c>, then_result_type: Kind<'c>, else_result_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_if(condition.0.as_ptr(), then_call.0.as_ptr(), else_call.0.as_ptr(), then_result_type.0.as_ptr(), else_result_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_while<'c>(&'c self, block: Expression<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_while(block.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }

    pub fn expr_new_array_from_values<'c>(&'c self, elements: &[Expression<'c>], result: Kind<'c>, array_type: Kind<'c>) -> Expression<'c> {
        let ptrs = ptrs!(elements);
        unsafe { Expression(NonNull::new(metal_expr_new_array_from_values(ptrs.as_ptr(), ptrs.len(), result.0.as_ptr(), array_type.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_new_mut_runtime_sized_array<'c>(&'c self, array_type: Kind<'c>, capacity_expr: Expression<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_new_mut_runtime_sized_array(array_type.0.as_ptr(), capacity_expr.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_static_array_from_callable<'c>(&'c self, array_type: Kind<'c>, generator: Expression<'c>, generator_method: Prototype<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_static_array_from_callable(array_type.0.as_ptr(), generator.0.as_ptr(), generator_method.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_array_length<'c>(&'c self, array_expr: Expression<'c>, array_type: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_array_length(array_expr.0.as_ptr(), array_type.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_array_capacity<'c>(&'c self, array_expr: Expression<'c>, array_type: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_array_capacity(array_expr.0.as_ptr(), array_type.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_array_size<'c>(&'c self, array: Expression<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_array_size(array.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_push_runtime_sized_array<'c>(&'c self, array_expr: Expression<'c>, array_type: Kind<'c>, new_element_expr: Expression<'c>, element_type: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_push_runtime_sized_array(array_expr.0.as_ptr(), array_type.0.as_ptr(), new_element_expr.0.as_ptr(), element_type.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_pop_runtime_sized_array<'c>(&'c self, array_expr: Expression<'c>, array_type: Kind<'c>, result: Kind<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_pop_runtime_sized_array(array_expr.0.as_ptr(), array_type.0.as_ptr(), result.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_destroy_static_sized_array_into_function<'c>(&'c self, array_expr: Expression<'c>, array_type: Kind<'c>, consumer: Expression<'c>, consumer_method: Prototype<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_destroy_static_sized_array_into_function(array_expr.0.as_ptr(), array_type.0.as_ptr(), consumer.0.as_ptr(), consumer_method.0.as_ptr())).unwrap(), PhantomData) }
    }
    pub fn expr_destroy_static_sized_array_into_locals<'c>(&'c self, expr: Expression<'c>, static_sized_array: Kind<'c>, destination_locals: &[Local<'c>]) -> Expression<'c> {
        let ptrs = ptrs!(destination_locals);
        unsafe { Expression(NonNull::new(metal_expr_destroy_static_sized_array_into_locals(expr.0.as_ptr(), static_sized_array.0.as_ptr(), ptrs.as_ptr(), ptrs.len())).unwrap(), PhantomData) }
    }
    pub fn expr_destroy_mut_runtime_sized_array<'c>(&'c self, array_expr: Expression<'c>) -> Expression<'c> {
        unsafe { Expression(NonNull::new(metal_expr_destroy_mut_runtime_sized_array(array_expr.0.as_ptr())).unwrap(), PhantomData) }
    }

    // --- Builders ---

    pub fn new_package_builder<'c>(&'c self, package_coord: PackageCoord<'c>) -> PackageBuilder<'c> {
        let raw = unsafe { metal_package_builder_new(self.raw, package_coord.0.as_ptr()) };
        assert!(!raw.is_null());
        PackageBuilder { raw, _cache: PhantomData }
    }

    pub fn new_program_builder<'c>(&'c self) -> ProgramBuilder<'c> {
        let raw = unsafe { metal_program_builder_new(self.raw) };
        assert!(!raw.is_null());
        ProgramBuilder { raw, _cache: PhantomData }
    }
}

impl Drop for MetalCache {
    fn drop(&mut self) {
        unsafe { metal_cache_free(self.raw) };
    }
}

/// Accumulates Package contents for one PackageCoordinate. `finish` consumes the builder and
/// returns the constructed Package; the builder is also freed.
pub struct PackageBuilder<'cache> {
    raw: *mut c_void,
    _cache: PhantomData<&'cache ()>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Package<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StaticSizedArrayDef<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RuntimeSizedArrayDef<'cache>(NonNull<c_void>, PhantomData<&'cache ()>);

impl<'cache> PackageBuilder<'cache> {
    pub fn add_interface(&self, name: &str, v: InterfaceDef<'cache>) {
        unsafe { metal_package_builder_add_interface(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_struct(&self, name: &str, v: StructDef<'cache>) {
        unsafe { metal_package_builder_add_struct(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_function(&self, name: &str, v: Function<'cache>) {
        unsafe { metal_package_builder_add_function(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_static_sized_array(&self, name: &str, v: StaticSizedArrayDef<'cache>) {
        unsafe { metal_package_builder_add_static_sized_array(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_runtime_sized_array(&self, name: &str, v: RuntimeSizedArrayDef<'cache>) {
        unsafe { metal_package_builder_add_runtime_sized_array(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_export_function(&self, name: &str, v: Prototype<'cache>) {
        unsafe { metal_package_builder_add_export_function(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_export_kind(&self, name: &str, v: Kind<'cache>) {
        unsafe { metal_package_builder_add_export_kind(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_extern_function(&self, name: &str, v: Prototype<'cache>) {
        unsafe { metal_package_builder_add_extern_function(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }
    pub fn add_extern_kind(&self, name: &str, v: Kind<'cache>) {
        unsafe { metal_package_builder_add_extern_kind(self.raw, name.as_ptr() as *const c_char, name.len(), v.0.as_ptr()) }
    }

    pub fn finish(self) -> Package<'cache> {
        let pkg = unsafe { metal_package_builder_finish(self.raw) };
        std::mem::forget(self);
        Package(NonNull::new(pkg).unwrap(), PhantomData)
    }
}

impl<'cache> Drop for PackageBuilder<'cache> {
    fn drop(&mut self) {
        let pkg = unsafe { metal_package_builder_finish(self.raw) };
        let _ = pkg;
    }
}

pub struct ProgramBuilder<'cache> {
    raw: *mut c_void,
    _cache: PhantomData<&'cache ()>,
}

pub struct Program<'cache> {
    raw: *mut c_void,
    _cache: PhantomData<&'cache ()>,
}

impl<'cache> ProgramBuilder<'cache> {
    pub fn add_package(&self, coord: PackageCoord<'cache>, package: Package<'cache>) {
        unsafe { metal_program_builder_add_package(self.raw, coord.0.as_ptr(), package.0.as_ptr()) }
    }

    pub fn finish(self) -> Program<'cache> {
        let prog = unsafe { metal_program_builder_finish(self.raw) };
        std::mem::forget(self);
        Program { raw: NonNull::new(prog).unwrap().as_ptr(), _cache: PhantomData }
    }
}

impl<'cache> Drop for ProgramBuilder<'cache> {
    fn drop(&mut self) {
        let prog = unsafe { metal_program_builder_finish(self.raw) };
        unsafe { metal_program_free(prog) };
    }
}

impl<'cache> Program<'cache> {
    pub fn raw(&self) -> *mut c_void { self.raw }
}

impl<'cache> Drop for Program<'cache> {
    fn drop(&mut self) {
        unsafe { metal_program_free(self.raw) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singletons_match_constructor_inits() {
        let cache = MetalCache::new();
        let i32_via_singleton = cache.i32();
        let i32_via_get = cache.get_int(cache.mut_region_id(), 32);
        assert_eq!(i32_via_singleton, i32_via_get, "i32 singleton must dedupe with get_int(mut, 32)");

        let i32_again = cache.get_int(cache.mut_region_id(), 32);
        assert_eq!(i32_via_get, i32_again);

        let i64 = cache.get_int(cache.mut_region_id(), 64);
        assert_ne!(i32_via_get, i64);
    }

    #[test]
    fn name_and_struct_kind_intern() {
        let cache = MetalCache::new();
        let pkg = cache.get_package_coordinate("test", &[]);
        let n1 = cache.get_name(pkg, "Widget");
        let n2 = cache.get_name(pkg, "Widget");
        assert_eq!(n1, n2, "names must intern by (package, string)");

        let n3 = cache.get_name(pkg, "Other");
        assert_ne!(n1, n3);

        let s1 = cache.get_struct_kind(n1);
        let s2 = cache.get_struct_kind(n1);
        assert_eq!(s1, s2, "struct kinds must intern by Name pointer");
    }

    #[test]
    fn wrap_kinds_intern_on_inner() {
        let cache = MetalCache::new();
        let i32 = cache.i32();
        // A borrow-wrap interns by its inner kind.
        assert_eq!(cache.get_borrow_ref(i32), cache.get_borrow_ref(i32));
        // Different wraps of the same inner are distinct kinds.
        assert_ne!(cache.get_borrow_ref(i32), cache.get_share_ref(i32));
        // A bare kind is not its own wrap.
        assert_ne!(cache.get_borrow_ref(i32), i32);
    }

    #[test]
    fn prototype_interns_on_signature() {
        let cache = MetalCache::new();
        let pkg = cache.get_package_coordinate("test", &[]);
        let main_name = cache.get_name(pkg, "main");
        let p1 = cache.get_prototype(main_name, cache.i32(), &[]);
        let p2 = cache.get_prototype(main_name, cache.i32(), &[]);
        assert_eq!(p1, p2);
    }

    #[test]
    fn interface_method_interns() {
        let cache = MetalCache::new();
        let pkg = cache.get_package_coordinate("test", &[]);
        let foo = cache.get_prototype(cache.get_name(pkg, "foo"), cache.i32(), &[]);
        let m1 = cache.get_interface_method(foo, 0);
        let m2 = cache.get_interface_method(foo, 0);
        assert_eq!(m1, m2);
        let m3 = cache.get_interface_method(foo, 1);
        assert_ne!(m1, m3);
    }

    #[test]
    fn non_interned_constructors_allocate_fresh_each_time() {
        let cache = MetalCache::new();
        let i32 = cache.i32();
        let m1 = cache.new_struct_member("x", "x", i32);
        let m2 = cache.new_struct_member("x", "x", i32);
        assert_ne!(m1, m2);
    }

    #[test]
    fn build_empty_program() {
        let cache = MetalCache::new();
        let coord = cache.get_package_coordinate("test", &[]);
        let pkg = cache.new_package_builder(coord).finish();
        let pb = cache.new_program_builder();
        pb.add_package(coord, pkg);
        let _program = pb.finish();
    }

    #[test]
    fn build_hello_world_program_structure() {
        // Mirrors `exported func main() int { return 7; }` onion-shaped:
        //   Function { prototype: main(): int, body: Block(Return(ConstantInt(7,32)), int) }
        let cache = MetalCache::new();
        let coord = cache.get_package_coordinate("test", &[]);
        let main_name = cache.get_name(coord, "main");
        let proto = cache.get_prototype(main_name, cache.i32(), &[]);

        let seven = cache.expr_constant_int(7, 32);
        let ret = cache.expr_return(seven, cache.i32());
        let body = cache.expr_block(ret, cache.i32());
        let func = cache.new_function(proto, Some(body));

        let pb = cache.new_package_builder(coord);
        pb.add_function("main", func);
        pb.add_export_function("main", proto);
        let pkg = pb.finish();

        let progb = cache.new_program_builder();
        progb.add_package(coord, pkg);
        let _program = progb.finish();
    }

    #[test]
    fn build_program_with_one_function_no_body() {
        let cache = MetalCache::new();
        let coord = cache.get_package_coordinate("test", &[]);
        let main_name = cache.get_name(coord, "main");
        let proto = cache.get_prototype(main_name, cache.i32(), &[]);
        let func = cache.new_function(proto, None);

        let pb = cache.new_package_builder(coord);
        pb.add_function("main", func);
        pb.add_export_function("main", proto);
        let pkg = pb.finish();

        let progb = cache.new_program_builder();
        progb.add_package(coord, pkg);
        let _program = progb.finish();
    }
}
