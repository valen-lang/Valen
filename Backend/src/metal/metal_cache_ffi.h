// C ABI for MetalCache and its onion IR node types. Consumed by FrontendRust's
// `backend_ffi::metal_cache` module to populate the cache in-process from the Rust
// instantiated IR (HinputsI).
//
// This layer is dumb, faithful 1:1 plumbing: each builder constructs the corresponding
// onion node (metal/instructions.h) with exactly the fields the IR carries. There is no
// Reference / ownership / location, no member-name→index, no Deref/load fusion, no
// placement. All lowering is done downstream in C++ codegen. Types are the onion
// `KindHandle*` (a bare kind is owned; the wrap builders express references).
//
// Conventions:
//   - All handle types are opaque pointers, kept distinct for type safety on the Rust side.
//   - Strings pass as `(const char* ptr, size_t len)` and are NOT NUL-terminated.
//   - Returned interned pointers are owned by the MetalCache; do NOT free.

#ifndef METAL_CACHE_FFI_H_
#define METAL_CACHE_FFI_H_

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct MetalCacheHandle      MetalCacheHandle;
typedef struct PackageCoordHandle    PackageCoordHandle;
typedef struct RegionIdHandle        RegionIdHandle;
typedef struct NameHandle            NameHandle;
typedef struct KindHandle            KindHandle;       // base; Int*, StructKind*, BorrowRef*, ... all use this
typedef struct PrototypeHandle       PrototypeHandle;
typedef struct LocalHandle           LocalHandle;
typedef struct InterfaceMethodHandle InterfaceMethodHandle;
typedef struct StructMemberHandle    StructMemberHandle;
typedef struct EdgeHandle            EdgeHandle;
typedef struct StructDefHandle       StructDefHandle;
typedef struct InterfaceDefHandle    InterfaceDefHandle;
typedef struct FunctionHandle        FunctionHandle;
typedef struct ExpressionHandle      ExpressionHandle;
typedef struct PackageHandle         PackageHandle;
typedef struct ProgramHandle         ProgramHandle;
typedef struct StaticSizedArrayDefHandle StaticSizedArrayDefHandle;
typedef struct RuntimeSizedArrayDefHandle RuntimeSizedArrayDefHandle;
typedef struct PackageBuilderHandle  PackageBuilderHandle;
typedef struct ProgramBuilderHandle  ProgramBuilderHandle;

// --- Lifecycle ---

MetalCacheHandle* metal_cache_new(void);
void              metal_cache_free(MetalCacheHandle*);

// --- Singletons (mirror the fields auto-initialized in MetalCache's ctor) ---

PackageCoordHandle* metal_cache_builtin_package_coord(MetalCacheHandle*);
RegionIdHandle*     metal_cache_rcimm_region_id(MetalCacheHandle*);
RegionIdHandle*     metal_cache_mut_region_id(MetalCacheHandle*);

KindHandle*         metal_cache_i32(MetalCacheHandle*);
KindHandle*         metal_cache_i64(MetalCacheHandle*);
KindHandle*         metal_cache_bool(MetalCacheHandle*);
KindHandle*         metal_cache_float(MetalCacheHandle*);
KindHandle*         metal_cache_str(MetalCacheHandle*);
KindHandle*         metal_cache_never(MetalCacheHandle*);
KindHandle*         metal_cache_void(MetalCacheHandle*);

// --- Interned getters ---
//
// Each mirrors a `MetalCache::get*` method; structurally-equal args return the same pointer.

PackageCoordHandle* metal_cache_get_package_coordinate(
    MetalCacheHandle*,
    const char* project_name_ptr, size_t project_name_len,
    const char* const* steps_ptrs, const size_t* steps_lens, size_t steps_count);

RegionIdHandle* metal_cache_get_region_id(
    MetalCacheHandle*,
    PackageCoordHandle* package_coord,
    const char* id_ptr, size_t id_len);

NameHandle* metal_cache_get_name(
    MetalCacheHandle*,
    PackageCoordHandle* package_coord,
    const char* name_ptr, size_t name_len);

// Base kinds.
KindHandle* metal_cache_get_int(MetalCacheHandle*, RegionIdHandle* region, int32_t bits);
KindHandle* metal_cache_get_bool(MetalCacheHandle*, RegionIdHandle* region);
KindHandle* metal_cache_get_str(MetalCacheHandle*, RegionIdHandle* region);
KindHandle* metal_cache_get_float(MetalCacheHandle*, RegionIdHandle* region);
KindHandle* metal_cache_get_void(MetalCacheHandle*, RegionIdHandle* region);
KindHandle* metal_cache_get_never(MetalCacheHandle*, RegionIdHandle* region);
KindHandle* metal_cache_get_usize(MetalCacheHandle*, RegionIdHandle* region);

KindHandle* metal_cache_get_struct_kind(MetalCacheHandle*, NameHandle* name);
KindHandle* metal_cache_get_interface_kind(MetalCacheHandle*, NameHandle* name);
KindHandle* metal_cache_get_static_sized_array(MetalCacheHandle*, NameHandle* name);
KindHandle* metal_cache_get_runtime_sized_array(MetalCacheHandle*, NameHandle* name);

// Onion wrap kinds: ownership as a layer around the base kind (mirrors KindIT's wraps).
KindHandle* metal_cache_get_borrow_ref(MetalCacheHandle*, KindHandle* inner);
KindHandle* metal_cache_get_own_ref(MetalCacheHandle*, KindHandle* inner);
KindHandle* metal_cache_get_share_ref(MetalCacheHandle*, KindHandle* inner);
KindHandle* metal_cache_get_weak_ref(MetalCacheHandle*, KindHandle* inner);

PrototypeHandle* metal_cache_get_prototype(
    MetalCacheHandle*,
    NameHandle* name,
    KindHandle* return_type,
    KindHandle* const* param_types, size_t param_count);

InterfaceMethodHandle* metal_cache_get_interface_method(
    MetalCacheHandle*, PrototypeHandle* prototype, int32_t virtual_param_index);

// A local is a name + its onion kind; the lowerer constructs each once and reuses the handle.
LocalHandle* metal_cache_get_local(
    MetalCacheHandle*, const char* name_ptr, size_t name_len, KindHandle* kind);

// --- Non-interned constructors (raw `new` on the C++ side) ---

StructMemberHandle* metal_struct_member_new(
    const char* full_name_ptr, size_t full_name_len,
    const char* name_ptr, size_t name_len,
    KindHandle* type);

EdgeHandle* metal_edge_new(
    KindHandle* struct_kind,
    KindHandle* interface_kind,
    InterfaceMethodHandle* const* interface_methods,
    PrototypeHandle* const* struct_prototypes,
    size_t pair_count);

// Mutability encoding: 0=IMMUTABLE, 1=MUTABLE
// Weakability encoding: 0=WEAKABLE, 1=NON_WEAKABLE
StructDefHandle* metal_struct_def_new(
    NameHandle* name,
    KindHandle* struct_kind,
    RegionIdHandle* region_id,
    uint32_t mutability,
    EdgeHandle* const* edges, size_t edge_count,
    StructMemberHandle* const* members, size_t member_count,
    uint32_t weakability);

InterfaceDefHandle* metal_interface_def_new(
    NameHandle* name,
    KindHandle* interface_kind,
    RegionIdHandle* region_id,
    uint32_t mutability,
    NameHandle* const* super_interfaces, size_t super_count,
    InterfaceMethodHandle* const* methods, size_t method_count,
    uint32_t weakability);

FunctionHandle* metal_function_new(PrototypeHandle* prototype, ExpressionHandle* body);

// --- Expression constructors (one per onion ExpressionIE node) ---
//
// Each produces a freshly-allocated Expression*; no interning. Type fields are onion
// KindHandle*; the `result` mirrors the IR node's result kind where it carries one.

ExpressionHandle* metal_expr_constant_void(void);
ExpressionHandle* metal_expr_constant_int(int64_t value, int32_t bits);
ExpressionHandle* metal_expr_constant_bool(int32_t value /* 0 or 1 */);
ExpressionHandle* metal_expr_constant_f64(double value);
ExpressionHandle* metal_expr_constant_str(const char* value_ptr, size_t value_len, KindHandle* result);
ExpressionHandle* metal_expr_break(void);
ExpressionHandle* metal_expr_return(ExpressionHandle* source_expr);
ExpressionHandle* metal_expr_discard(ExpressionHandle* expr);
ExpressionHandle* metal_expr_block(ExpressionHandle* inner, KindHandle* result);
ExpressionHandle* metal_expr_consecutor(ExpressionHandle* const* exprs, size_t expr_count, KindHandle* result);

// ArgLookup { param_index, tyype }
ExpressionHandle* metal_expr_argument(int32_t param_index, KindHandle* tyype);

// Locals / lets.
ExpressionHandle* metal_expr_stackify(LocalHandle* variable, ExpressionHandle* expr, KindHandle* result);
ExpressionHandle* metal_expr_let_and_lend(LocalHandle* variable, ExpressionHandle* expr, KindHandle* result);
ExpressionHandle* metal_expr_restackify(LocalHandle* variable, ExpressionHandle* source_expr, KindHandle* result);
ExpressionHandle* metal_expr_unstackify(LocalHandle* variable, KindHandle* result);
ExpressionHandle* metal_expr_local_lookup(LocalHandle* local_variable, KindHandle* result);

// Deref / member & array lookups.
ExpressionHandle* metal_expr_deref(ExpressionHandle* inner, KindHandle* result);
ExpressionHandle* metal_expr_member_lookup(
    ExpressionHandle* struct_expr, const char* member_name_ptr, size_t member_name_len, KindHandle* result);
ExpressionHandle* metal_expr_static_sized_array_lookup(
    ExpressionHandle* array_expr, KindHandle* array_type, ExpressionHandle* index_expr, KindHandle* result);
ExpressionHandle* metal_expr_runtime_sized_array_lookup(
    ExpressionHandle* array_expr, KindHandle* array_type, ExpressionHandle* index_expr, KindHandle* result);

// Mutate (unified store over a destination lvalue).
ExpressionHandle* metal_expr_mutate(
    ExpressionHandle* destination_expr, ExpressionHandle* source_expr, KindHandle* result);

// Construct / destroy.
ExpressionHandle* metal_expr_new_struct(
    KindHandle* struct_kind, KindHandle* result,
    ExpressionHandle* const* args, size_t arg_count);
ExpressionHandle* metal_expr_destroy(
    ExpressionHandle* expr, KindHandle* struct_kind,
    LocalHandle* const* destination_locals, size_t local_count);
ExpressionHandle* metal_expr_copy_prim(ExpressionHandle* inner, KindHandle* result);

// Upcast / subtype.
ExpressionHandle* metal_expr_struct_to_interface_upcast(
    ExpressionHandle* inner_expr, KindHandle* target_interface, NameHandle* impl_name, KindHandle* result);
ExpressionHandle* metal_expr_interface_to_interface_upcast(
    ExpressionHandle* inner_expr, KindHandle* target_interface, KindHandle* result);
ExpressionHandle* metal_expr_as_subtype(
    ExpressionHandle* source_expr, KindHandle* target_type,
    PrototypeHandle* ok_constructor, PrototypeHandle* err_constructor,
    NameHandle* impl_name, NameHandle* ok_impl_name, NameHandle* err_impl_name,
    KindHandle* result);
ExpressionHandle* metal_expr_is_same_instance(ExpressionHandle* left, ExpressionHandle* right);

// Weak refs.
ExpressionHandle* metal_expr_weak_alias(ExpressionHandle* inner_expr, KindHandle* result);
ExpressionHandle* metal_expr_lock_weak(
    ExpressionHandle* inner_expr,
    PrototypeHandle* some_constructor, PrototypeHandle* none_constructor,
    NameHandle* some_impl_name, NameHandle* none_impl_name,
    KindHandle* result);

// Calls.
ExpressionHandle* metal_expr_call(
    PrototypeHandle* callable, ExpressionHandle* const* args, size_t arg_count, KindHandle* result);
ExpressionHandle* metal_expr_extern_call(
    PrototypeHandle* prototype, ExpressionHandle* const* args, size_t arg_count, KindHandle* result);
ExpressionHandle* metal_expr_interface_call(
    PrototypeHandle* super_function_prototype, int32_t virtual_param_index,
    ExpressionHandle* const* args, size_t arg_count, KindHandle* result);

// Control flow.
ExpressionHandle* metal_expr_if(
    ExpressionHandle* condition, ExpressionHandle* then_call, ExpressionHandle* else_call, KindHandle* result);
ExpressionHandle* metal_expr_while(ExpressionHandle* block, KindHandle* result);

// Arrays.
ExpressionHandle* metal_expr_new_array_from_values(
    ExpressionHandle* const* elements, size_t element_count, KindHandle* result, KindHandle* array_type);
ExpressionHandle* metal_expr_new_mut_runtime_sized_array(
    KindHandle* array_type, ExpressionHandle* capacity_expr, KindHandle* result);
ExpressionHandle* metal_expr_static_array_from_callable(
    KindHandle* array_type, ExpressionHandle* generator, PrototypeHandle* generator_method, KindHandle* result);
ExpressionHandle* metal_expr_array_length(ExpressionHandle* array_expr);
ExpressionHandle* metal_expr_array_capacity(ExpressionHandle* array_expr);
ExpressionHandle* metal_expr_array_size(ExpressionHandle* array, KindHandle* result);
ExpressionHandle* metal_expr_push_runtime_sized_array(
    ExpressionHandle* array_expr, ExpressionHandle* new_element_expr);
ExpressionHandle* metal_expr_pop_runtime_sized_array(ExpressionHandle* array_expr, KindHandle* result);
ExpressionHandle* metal_expr_destroy_static_sized_array_into_function(
    ExpressionHandle* array_expr, KindHandle* array_type,
    ExpressionHandle* consumer, PrototypeHandle* consumer_method);
ExpressionHandle* metal_expr_destroy_static_sized_array_into_locals(
    ExpressionHandle* expr, KindHandle* static_sized_array,
    LocalHandle* const* destination_locals, size_t local_count);
ExpressionHandle* metal_expr_destroy_mut_runtime_sized_array(ExpressionHandle* array_expr);

// --- Package builder ---

PackageBuilderHandle* metal_package_builder_new(
    MetalCacheHandle* cache, PackageCoordHandle* package_coord);

void metal_package_builder_add_interface(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, InterfaceDefHandle*);
void metal_package_builder_add_struct(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, StructDefHandle*);
void metal_package_builder_add_function(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, FunctionHandle*);
void metal_package_builder_add_static_sized_array(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, StaticSizedArrayDefHandle*);
void metal_package_builder_add_runtime_sized_array(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, RuntimeSizedArrayDefHandle*);

StaticSizedArrayDefHandle* metal_static_sized_array_def_new(
    NameHandle* name, KindHandle* array_kind, int32_t size,
    RegionIdHandle* region_id,
    KindHandle* element_type);

RuntimeSizedArrayDefHandle* metal_runtime_sized_array_def_new(
    NameHandle* name, KindHandle* array_kind,
    RegionIdHandle* region_id,
    KindHandle* element_type);
void metal_package_builder_add_export_function(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, PrototypeHandle*);
void metal_package_builder_add_export_kind(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, KindHandle*);
void metal_package_builder_add_extern_function(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, PrototypeHandle*);
void metal_package_builder_add_extern_kind(
    PackageBuilderHandle*, const char* name_ptr, size_t name_len, KindHandle*);

PackageHandle* metal_package_builder_finish(PackageBuilderHandle*);

// --- Program builder ---

ProgramBuilderHandle* metal_program_builder_new(MetalCacheHandle* cache);
void metal_program_builder_add_package(
    ProgramBuilderHandle*, PackageCoordHandle*, PackageHandle*);
ProgramHandle* metal_program_builder_finish(ProgramBuilderHandle*);

void metal_program_free(ProgramHandle*);

#ifdef __cplusplus
}
#endif

#endif  // METAL_CACHE_FFI_H_
