// Implementation of metal_cache_ffi.h. Each function is a thin, faithful 1:1 wrapper that
// reinterpret_casts the opaque handles to the underlying onion C++ types and forwards to
// MetalCache::get* or `new`s the instruction. No lowering here. Types are onion Kind*, and
// placement / index resolution / deref lowering all happen downstream in codegen.

#include "metal_cache_ffi.h"

#include <cassert>
#include <string>
#include <vector>

#include "addresshasher.h"
#include "metal/ast.h"
#include "metal/instructions.h"
#include "metal/metalcache.h"
#include "metal/name.h"
#include "metal/types.h"

namespace {

struct CacheOwner {
  AddressNumberer addressNumberer;
  MetalCache cache;
  CacheOwner() : addressNumberer(), cache(&addressNumberer) {}
};

inline CacheOwner*        owner(MetalCacheHandle* h)     { return reinterpret_cast<CacheOwner*>(h); }
inline MetalCache*        cache(MetalCacheHandle* h)     { return &owner(h)->cache; }
inline PackageCoordinate* pc(PackageCoordHandle* h)      { return reinterpret_cast<PackageCoordinate*>(h); }
inline RegionId*          rid(RegionIdHandle* h)         { return reinterpret_cast<RegionId*>(h); }
inline Name*              nm(NameHandle* h)              { return reinterpret_cast<Name*>(h); }
inline Kind*              knd(KindHandle* h)             { return reinterpret_cast<Kind*>(h); }
// Always-borrow operands: the instantiator types these `&BorrowRefIT`, so the handle is a
// BorrowRef. Fail loud (mirroring the instantiator) if it ever isn't.
inline BorrowRef*         brf(KindHandle* h)             { auto b = dynamic_cast<BorrowRef*>(knd(h)); assert(b != nullptr); return b; }
inline Prototype*         proto(PrototypeHandle* h)      { return reinterpret_cast<Prototype*>(h); }
inline Local*             loc(LocalHandle* h)            { return reinterpret_cast<Local*>(h); }
inline Expression*        ex(ExpressionHandle* h)        { return reinterpret_cast<Expression*>(h); }

inline std::string str(const char* p, size_t n) { return std::string(p, n); }

inline std::vector<Expression*> exprs(ExpressionHandle* const* ptr, size_t count) {
  std::vector<Expression*> vec;
  vec.reserve(count);
  for (size_t i = 0; i < count; i++) vec.push_back(ex(ptr[i]));
  return vec;
}

inline std::vector<Local*> locals(LocalHandle* const* ptr, size_t count) {
  std::vector<Local*> vec;
  vec.reserve(count);
  for (size_t i = 0; i < count; i++) vec.push_back(loc(ptr[i]));
  return vec;
}

}  // namespace

#define VIS __attribute__((visibility("default")))

// --- Lifecycle ---

extern "C" VIS MetalCacheHandle* metal_cache_new(void) {
  return reinterpret_cast<MetalCacheHandle*>(new CacheOwner());
}

extern "C" VIS void metal_cache_free(MetalCacheHandle* h) {
  delete owner(h);
}

// Internal accessor used by ffi.cpp's backend_compile_program to reach the inner MetalCache.
extern "C" __attribute__((visibility("default")))
MetalCache* metal_cache_ffi_inner(MetalCacheHandle* h) {
  return &owner(h)->cache;
}

// --- Singletons ---

extern "C" VIS PackageCoordHandle* metal_cache_builtin_package_coord(MetalCacheHandle* h) {
  return reinterpret_cast<PackageCoordHandle*>(cache(h)->builtinPackageCoord);
}
extern "C" VIS RegionIdHandle* metal_cache_rcimm_region_id(MetalCacheHandle* h) {
  return reinterpret_cast<RegionIdHandle*>(cache(h)->rcImmRegionId);
}
extern "C" VIS RegionIdHandle* metal_cache_mut_region_id(MetalCacheHandle* h) {
  return reinterpret_cast<RegionIdHandle*>(cache(h)->mutRegionId);
}

extern "C" VIS KindHandle* metal_cache_i32(MetalCacheHandle* h) {
  return reinterpret_cast<KindHandle*>(cache(h)->i32Type);
}
extern "C" VIS KindHandle* metal_cache_i64(MetalCacheHandle* h) {
  return reinterpret_cast<KindHandle*>(cache(h)->i64Type);
}
extern "C" VIS KindHandle* metal_cache_bool(MetalCacheHandle* h) {
  return reinterpret_cast<KindHandle*>(cache(h)->boolType);
}
extern "C" VIS KindHandle* metal_cache_float(MetalCacheHandle* h) {
  return reinterpret_cast<KindHandle*>(cache(h)->floatType);
}
extern "C" VIS KindHandle* metal_cache_str(MetalCacheHandle* h) {
  return reinterpret_cast<KindHandle*>(cache(h)->str);
}
extern "C" VIS KindHandle* metal_cache_never(MetalCacheHandle* h) {
  return reinterpret_cast<KindHandle*>(cache(h)->neverType);
}
extern "C" VIS KindHandle* metal_cache_void(MetalCacheHandle* h) {
  return reinterpret_cast<KindHandle*>(cache(h)->voidType);
}

// --- Interned getters ---

extern "C" VIS PackageCoordHandle* metal_cache_get_package_coordinate(
    MetalCacheHandle* h,
    const char* project_name_ptr, size_t project_name_len,
    const char* const* steps_ptrs, const size_t* steps_lens, size_t steps_count) {
  std::vector<std::string> steps;
  steps.reserve(steps_count);
  for (size_t i = 0; i < steps_count; i++) {
    steps.emplace_back(steps_ptrs[i], steps_lens[i]);
  }
  return reinterpret_cast<PackageCoordHandle*>(
      cache(h)->getPackageCoordinate(str(project_name_ptr, project_name_len), steps));
}

extern "C" VIS RegionIdHandle* metal_cache_get_region_id(
    MetalCacheHandle* h, PackageCoordHandle* package_coord,
    const char* id_ptr, size_t id_len) {
  return reinterpret_cast<RegionIdHandle*>(
      cache(h)->getRegionId(pc(package_coord), str(id_ptr, id_len)));
}

extern "C" VIS NameHandle* metal_cache_get_name(
    MetalCacheHandle* h, PackageCoordHandle* package_coord,
    const char* name_ptr, size_t name_len) {
  return reinterpret_cast<NameHandle*>(
      cache(h)->getName(pc(package_coord), str(name_ptr, name_len)));
}

extern "C" VIS KindHandle* metal_cache_get_int(MetalCacheHandle* h, RegionIdHandle* region, int32_t bits) {
  return reinterpret_cast<KindHandle*>(cache(h)->getInt(rid(region), bits));
}
extern "C" VIS KindHandle* metal_cache_get_bool(MetalCacheHandle* h, RegionIdHandle* region) {
  return reinterpret_cast<KindHandle*>(cache(h)->getBool(rid(region)));
}
extern "C" VIS KindHandle* metal_cache_get_str(MetalCacheHandle* h, RegionIdHandle* region) {
  return reinterpret_cast<KindHandle*>(cache(h)->getStr(rid(region)));
}
extern "C" VIS KindHandle* metal_cache_get_float(MetalCacheHandle* h, RegionIdHandle* region) {
  return reinterpret_cast<KindHandle*>(cache(h)->getFloat(rid(region)));
}
extern "C" VIS KindHandle* metal_cache_get_void(MetalCacheHandle* h, RegionIdHandle* region) {
  return reinterpret_cast<KindHandle*>(cache(h)->getVoid(rid(region)));
}
extern "C" VIS KindHandle* metal_cache_get_never(MetalCacheHandle* h, RegionIdHandle* region) {
  return reinterpret_cast<KindHandle*>(cache(h)->getNever(rid(region)));
}
extern "C" VIS KindHandle* metal_cache_get_usize(MetalCacheHandle* h, RegionIdHandle* region) {
  return reinterpret_cast<KindHandle*>(cache(h)->getUSize(rid(region)));
}

extern "C" VIS KindHandle* metal_cache_get_struct_kind(MetalCacheHandle* h, NameHandle* name) {
  return reinterpret_cast<KindHandle*>(cache(h)->getStructKind(nm(name)));
}
extern "C" VIS KindHandle* metal_cache_get_interface_kind(MetalCacheHandle* h, NameHandle* name) {
  return reinterpret_cast<KindHandle*>(cache(h)->getInterfaceKind(nm(name)));
}
extern "C" VIS KindHandle* metal_cache_get_static_sized_array(MetalCacheHandle* h, NameHandle* name) {
  return reinterpret_cast<KindHandle*>(cache(h)->getStaticSizedArray(nm(name)));
}
extern "C" VIS KindHandle* metal_cache_get_runtime_sized_array(MetalCacheHandle* h, NameHandle* name) {
  return reinterpret_cast<KindHandle*>(cache(h)->getRuntimeSizedArray(nm(name)));
}

extern "C" VIS KindHandle* metal_cache_get_borrow_ref(MetalCacheHandle* h, KindHandle* inner) {
  return reinterpret_cast<KindHandle*>(cache(h)->getBorrowRef(knd(inner)));
}
extern "C" VIS KindHandle* metal_cache_get_own_ref(MetalCacheHandle* h, KindHandle* inner) {
  return reinterpret_cast<KindHandle*>(cache(h)->getOwnRef(knd(inner)));
}
extern "C" VIS KindHandle* metal_cache_get_share_ref(MetalCacheHandle* h, KindHandle* inner) {
  return reinterpret_cast<KindHandle*>(cache(h)->getShareRef(knd(inner)));
}
extern "C" VIS KindHandle* metal_cache_get_weak_ref(MetalCacheHandle* h, KindHandle* inner) {
  return reinterpret_cast<KindHandle*>(cache(h)->getWeakRef(knd(inner)));
}

extern "C" VIS PrototypeHandle* metal_cache_get_prototype(
    MetalCacheHandle* h, NameHandle* name, KindHandle* return_type,
    KindHandle* const* param_types, size_t param_count) {
  std::vector<Kind*> params;
  params.reserve(param_count);
  for (size_t i = 0; i < param_count; i++) {
    params.push_back(knd(param_types[i]));
  }
  return reinterpret_cast<PrototypeHandle*>(
      cache(h)->getPrototype(nm(name), knd(return_type), std::move(params)));
}

extern "C" VIS InterfaceMethodHandle* metal_cache_get_interface_method(
    MetalCacheHandle* h, PrototypeHandle* prototype, int32_t virtual_param_index) {
  return reinterpret_cast<InterfaceMethodHandle*>(
      cache(h)->getInterfaceMethod(proto(prototype), virtual_param_index));
}

extern "C" VIS LocalHandle* metal_cache_get_local(
    MetalCacheHandle* h, const char* id_ptr, size_t id_len,
    const char* name_ptr, size_t name_len, KindHandle* kind) {
  (void)h;  // Locals are constructed per-mention; identity is the `id` string (BlockState keys
            // on it), so multiple handles for one source local are fine.
  return reinterpret_cast<LocalHandle*>(
      new Local(VarNameM{str(id_ptr, id_len)}, str(name_ptr, name_len), knd(kind)));
}

// --- Non-interned constructors ---

extern "C" VIS StructMemberHandle* metal_struct_member_new(
    const char* full_name_ptr, size_t full_name_len,
    const char* name_ptr, size_t name_len,
    KindHandle* type) {
  return reinterpret_cast<StructMemberHandle*>(new StructMember(
      str(full_name_ptr, full_name_len),
      str(name_ptr, name_len),
      knd(type)));
}

extern "C" VIS EdgeHandle* metal_edge_new(
    KindHandle* struct_kind, KindHandle* interface_kind,
    InterfaceMethodHandle* const* interface_methods,
    PrototypeHandle* const* struct_prototypes,
    size_t pair_count) {
  std::vector<std::pair<InterfaceMethod*, Prototype*>> pairs;
  pairs.reserve(pair_count);
  for (size_t i = 0; i < pair_count; i++) {
    pairs.emplace_back(
        reinterpret_cast<InterfaceMethod*>(interface_methods[i]),
        proto(struct_prototypes[i]));
  }
  return reinterpret_cast<EdgeHandle*>(new Edge(
      reinterpret_cast<StructKind*>(knd(struct_kind)),
      reinterpret_cast<InterfaceKind*>(knd(interface_kind)),
      std::move(pairs)));
}

extern "C" VIS StructDefHandle* metal_struct_def_new(
    NameHandle* name, KindHandle* struct_kind, RegionIdHandle* region_id,
    uint32_t mutability,
    EdgeHandle* const* edges, size_t edge_count,
    StructMemberHandle* const* members, size_t member_count,
    uint32_t weakability) {
  std::vector<Edge*> edge_vec;
  edge_vec.reserve(edge_count);
  for (size_t i = 0; i < edge_count; i++) {
    edge_vec.push_back(reinterpret_cast<Edge*>(edges[i]));
  }
  std::vector<StructMember*> member_vec;
  member_vec.reserve(member_count);
  for (size_t i = 0; i < member_count; i++) {
    member_vec.push_back(reinterpret_cast<StructMember*>(members[i]));
  }
  return reinterpret_cast<StructDefHandle*>(new StructDefinition(
      nm(name),
      reinterpret_cast<StructKind*>(knd(struct_kind)),
      rid(region_id),
      static_cast<Sharedness>(mutability),
      std::move(edge_vec),
      std::move(member_vec),
      static_cast<Weakability>(weakability)));
}

extern "C" VIS InterfaceDefHandle* metal_interface_def_new(
    NameHandle* name, KindHandle* interface_kind, RegionIdHandle* region_id,
    uint32_t mutability,
    NameHandle* const* super_interfaces, size_t super_count,
    InterfaceMethodHandle* const* methods, size_t method_count,
    uint32_t weakability) {
  std::vector<Name*> supers;
  supers.reserve(super_count);
  for (size_t i = 0; i < super_count; i++) {
    supers.push_back(nm(super_interfaces[i]));
  }
  std::vector<InterfaceMethod*> method_vec;
  method_vec.reserve(method_count);
  for (size_t i = 0; i < method_count; i++) {
    method_vec.push_back(reinterpret_cast<InterfaceMethod*>(methods[i]));
  }
  return reinterpret_cast<InterfaceDefHandle*>(new InterfaceDefinition(
      nm(name),
      reinterpret_cast<InterfaceKind*>(knd(interface_kind)),
      rid(region_id),
      static_cast<Sharedness>(mutability),
      supers,
      method_vec,
      static_cast<Weakability>(weakability)));
}

extern "C" VIS FunctionHandle* metal_function_new(
    PrototypeHandle* prototype, ExpressionHandle* body) {
  return reinterpret_cast<FunctionHandle*>(new Function(proto(prototype), ex(body)));
}

// --- Expression constructors (onion nodes, 1:1) ---

extern "C" VIS ExpressionHandle* metal_expr_constant_void(void) {
  return reinterpret_cast<ExpressionHandle*>(new ConstantVoid());
}
extern "C" VIS ExpressionHandle* metal_expr_constant_int(int64_t value, int32_t bits) {
  return reinterpret_cast<ExpressionHandle*>(new ConstantInt(value, bits));
}
extern "C" VIS ExpressionHandle* metal_expr_constant_bool(int32_t value) {
  return reinterpret_cast<ExpressionHandle*>(new ConstantBool(value != 0));
}
extern "C" VIS ExpressionHandle* metal_expr_constant_f64(double value) {
  return reinterpret_cast<ExpressionHandle*>(new ConstantF64(value));
}
extern "C" VIS ExpressionHandle* metal_expr_constant_str(const char* p, size_t n, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new ConstantStr(str(p, n), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_break(void) {
  return reinterpret_cast<ExpressionHandle*>(new Break());
}
extern "C" VIS ExpressionHandle* metal_expr_return(ExpressionHandle* source_expr, KindHandle* source_type) {
  return reinterpret_cast<ExpressionHandle*>(new Return(ex(source_expr), knd(source_type)));
}
extern "C" VIS ExpressionHandle* metal_expr_discard(ExpressionHandle* expr, KindHandle* source_type) {
  return reinterpret_cast<ExpressionHandle*>(new Discard(ex(expr), knd(source_type)));
}
extern "C" VIS ExpressionHandle* metal_expr_block(ExpressionHandle* inner, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Block(ex(inner), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_consecutor(
    ExpressionHandle* const* es, size_t expr_count, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Consecutor(exprs(es, expr_count), knd(result)));
}

extern "C" VIS ExpressionHandle* metal_expr_argument(int32_t param_index, KindHandle* tyype) {
  return reinterpret_cast<ExpressionHandle*>(new Argument(param_index, knd(tyype)));
}

extern "C" VIS ExpressionHandle* metal_expr_stackify(
    LocalHandle* variable, ExpressionHandle* expr, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Stackify(loc(variable), ex(expr), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_let_and_lend(
    LocalHandle* variable, ExpressionHandle* expr, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new LetAndLend(loc(variable), ex(expr), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_restackify(
    LocalHandle* variable, ExpressionHandle* source_expr, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Restackify(loc(variable), ex(source_expr), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_unstackify(LocalHandle* variable, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Unstackify(loc(variable), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_local_lookup(LocalHandle* local_variable, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new LocalLookup(loc(local_variable), knd(result)));
}

extern "C" VIS ExpressionHandle* metal_expr_deref(ExpressionHandle* inner, KindHandle* source_type, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Deref(ex(inner), knd(source_type), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_member_lookup(
    ExpressionHandle* struct_expr, KindHandle* struct_type, const char* member_name_ptr, size_t member_name_len, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new MemberLookup(
      ex(struct_expr), brf(struct_type), str(member_name_ptr, member_name_len), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_static_sized_array_lookup(
    ExpressionHandle* array_expr, KindHandle* array_type, ExpressionHandle* index_expr, KindHandle* index_type, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new StaticSizedArrayLookup(
      ex(array_expr), brf(array_type), ex(index_expr), knd(index_type), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_runtime_sized_array_lookup(
    ExpressionHandle* array_expr, KindHandle* array_type, ExpressionHandle* index_expr, KindHandle* index_type, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new RuntimeSizedArrayLookup(
      ex(array_expr), brf(array_type), ex(index_expr), knd(index_type), knd(result)));
}

extern "C" VIS ExpressionHandle* metal_expr_mutate(
    ExpressionHandle* destination_expr, KindHandle* destination_type, ExpressionHandle* source_expr, KindHandle* source_type, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Mutate(ex(destination_expr), brf(destination_type), ex(source_expr), knd(source_type), knd(result)));
}

extern "C" VIS ExpressionHandle* metal_expr_new_struct(
    KindHandle* struct_kind, KindHandle* result,
    ExpressionHandle* const* args, size_t arg_count) {
  return reinterpret_cast<ExpressionHandle*>(new NewStruct(
      reinterpret_cast<StructKind*>(knd(struct_kind)), knd(result), exprs(args, arg_count)));
}
extern "C" VIS ExpressionHandle* metal_expr_destroy(
    ExpressionHandle* expr, KindHandle* struct_kind,
    LocalHandle* const* destination_locals, size_t local_count) {
  return reinterpret_cast<ExpressionHandle*>(new Destroy(
      ex(expr), reinterpret_cast<StructKind*>(knd(struct_kind)), locals(destination_locals, local_count)));
}
extern "C" VIS ExpressionHandle* metal_expr_copy_prim(ExpressionHandle* inner, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new CopyPrim(ex(inner), knd(result)));
}

extern "C" VIS ExpressionHandle* metal_expr_struct_to_interface_upcast(
    ExpressionHandle* inner_expr, KindHandle* source_type, KindHandle* target_interface, NameHandle* impl_name, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new StructToInterfaceUpcast(
      ex(inner_expr), knd(source_type), reinterpret_cast<InterfaceKind*>(knd(target_interface)), nm(impl_name), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_interface_to_interface_upcast(
    ExpressionHandle* inner_expr, KindHandle* target_interface, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new InterfaceToInterfaceUpcast(
      ex(inner_expr), reinterpret_cast<InterfaceKind*>(knd(target_interface)), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_as_subtype(
    ExpressionHandle* source_expr, KindHandle* source_type, KindHandle* target_type,
    PrototypeHandle* ok_constructor, PrototypeHandle* err_constructor,
    NameHandle* impl_name, NameHandle* ok_impl_name, NameHandle* err_impl_name,
    KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new AsSubtype(
      ex(source_expr), knd(source_type), knd(target_type),
      proto(ok_constructor), proto(err_constructor),
      nm(impl_name), nm(ok_impl_name), nm(err_impl_name), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_is_same_instance(
    ExpressionHandle* left, KindHandle* left_type, ExpressionHandle* right, KindHandle* right_type) {
  return reinterpret_cast<ExpressionHandle*>(new IsSameInstance(ex(left), knd(left_type), ex(right), knd(right_type)));
}

extern "C" VIS ExpressionHandle* metal_expr_weak_alias(ExpressionHandle* inner_expr, KindHandle* source_type, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new WeakAlias(ex(inner_expr), knd(source_type), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_lock_weak(
    ExpressionHandle* inner_expr, KindHandle* source_type,
    PrototypeHandle* some_constructor, PrototypeHandle* none_constructor,
    NameHandle* some_impl_name, NameHandle* none_impl_name,
    KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new LockWeak(
      ex(inner_expr), knd(source_type), proto(some_constructor), proto(none_constructor),
      nm(some_impl_name), nm(none_impl_name), knd(result)));
}

extern "C" VIS ExpressionHandle* metal_expr_call(
    PrototypeHandle* callable, ExpressionHandle* const* args, size_t arg_count, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new Call(proto(callable), exprs(args, arg_count), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_extern_call(
    PrototypeHandle* prototype, ExpressionHandle* const* args, size_t arg_count, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new ExternCall(proto(prototype), exprs(args, arg_count), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_interface_call(
    PrototypeHandle* super_function_prototype, int32_t virtual_param_index, int32_t index_in_edge,
    ExpressionHandle* const* args, size_t arg_count, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new InterfaceCall(
      proto(super_function_prototype), virtual_param_index, index_in_edge, exprs(args, arg_count), knd(result)));
}

extern "C" VIS ExpressionHandle* metal_expr_if(
    ExpressionHandle* condition, ExpressionHandle* then_call, ExpressionHandle* else_call,
    KindHandle* then_result_type, KindHandle* else_result_type, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new If(
      ex(condition), ex(then_call), ex(else_call), knd(then_result_type), knd(else_result_type), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_while(ExpressionHandle* block, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new While(ex(block), knd(result)));
}

// --- Arrays ---

extern "C" VIS ExpressionHandle* metal_expr_new_array_from_values(
    ExpressionHandle* const* elements, size_t element_count, KindHandle* result, KindHandle* array_type) {
  return reinterpret_cast<ExpressionHandle*>(new NewArrayFromValues(
      exprs(elements, element_count), knd(result),
      reinterpret_cast<StaticSizedArrayT*>(knd(array_type))));
}
extern "C" VIS ExpressionHandle* metal_expr_new_mut_runtime_sized_array(
    KindHandle* array_type, ExpressionHandle* capacity_expr, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new NewRuntimeSizedArray(
      reinterpret_cast<RuntimeSizedArrayT*>(knd(array_type)), ex(capacity_expr), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_static_array_from_callable(
    KindHandle* array_type, ExpressionHandle* generator, PrototypeHandle* generator_method, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new StaticArrayFromCallable(
      reinterpret_cast<StaticSizedArrayT*>(knd(array_type)), ex(generator), proto(generator_method), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_array_length(ExpressionHandle* array_expr, KindHandle* array_type) {
  return reinterpret_cast<ExpressionHandle*>(new ArrayLength(ex(array_expr), brf(array_type)));
}
extern "C" VIS ExpressionHandle* metal_expr_array_capacity(ExpressionHandle* array_expr, KindHandle* array_type) {
  return reinterpret_cast<ExpressionHandle*>(new ArrayCapacity(ex(array_expr), brf(array_type)));
}
extern "C" VIS ExpressionHandle* metal_expr_array_size(ExpressionHandle* array, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new ArraySize(ex(array), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_push_runtime_sized_array(
    ExpressionHandle* array_expr, KindHandle* array_type, ExpressionHandle* new_element_expr, KindHandle* element_type) {
  return reinterpret_cast<ExpressionHandle*>(new PushRuntimeSizedArray(ex(array_expr), brf(array_type), ex(new_element_expr), knd(element_type)));
}
extern "C" VIS ExpressionHandle* metal_expr_pop_runtime_sized_array(ExpressionHandle* array_expr, KindHandle* array_type, KindHandle* result) {
  return reinterpret_cast<ExpressionHandle*>(new PopRuntimeSizedArray(ex(array_expr), brf(array_type), knd(result)));
}
extern "C" VIS ExpressionHandle* metal_expr_destroy_static_sized_array_into_function(
    ExpressionHandle* array_expr, KindHandle* array_type,
    ExpressionHandle* consumer, PrototypeHandle* consumer_method) {
  return reinterpret_cast<ExpressionHandle*>(new DestroyStaticSizedArrayIntoFunction(
      ex(array_expr), reinterpret_cast<StaticSizedArrayT*>(knd(array_type)), ex(consumer), proto(consumer_method)));
}
extern "C" VIS ExpressionHandle* metal_expr_destroy_static_sized_array_into_locals(
    ExpressionHandle* expr, KindHandle* static_sized_array,
    LocalHandle* const* destination_locals, size_t local_count) {
  return reinterpret_cast<ExpressionHandle*>(new DestroyStaticSizedArrayIntoLocals(
      ex(expr), reinterpret_cast<StaticSizedArrayT*>(knd(static_sized_array)),
      locals(destination_locals, local_count)));
}
extern "C" VIS ExpressionHandle* metal_expr_destroy_mut_runtime_sized_array(ExpressionHandle* array_expr) {
  return reinterpret_cast<ExpressionHandle*>(new DestroyRuntimeSizedArray(ex(array_expr)));
}

// --- Package builder ---

struct PackageBuilder {
  MetalCache* cache;
  PackageCoordinate* packageCoord;
  std::unordered_map<std::string, InterfaceDefinition*> interfaces;
  std::unordered_map<std::string, StructDefinition*> structs;
  std::unordered_map<std::string, StaticSizedArrayDefinitionT*> staticSizedArrays;
  std::unordered_map<std::string, RuntimeSizedArrayDefinitionT*> runtimeSizedArrays;
  std::unordered_map<std::string, Function*> functions;
  std::unordered_map<std::string, Prototype*> exportNameToFunction;
  std::unordered_map<std::string, Kind*> exportNameToKind;
  std::unordered_map<std::string, Prototype*> externNameToFunction;
  std::unordered_map<std::string, Kind*> externNameToKind;
};

extern "C" VIS PackageBuilderHandle* metal_package_builder_new(
    MetalCacheHandle* h, PackageCoordHandle* package_coord) {
  auto* b = new PackageBuilder();
  b->cache = cache(h);
  b->packageCoord = pc(package_coord);
  return reinterpret_cast<PackageBuilderHandle*>(b);
}

#define PB(h) reinterpret_cast<PackageBuilder*>(h)

extern "C" VIS void metal_package_builder_add_interface(
    PackageBuilderHandle* h, const char* p, size_t n, InterfaceDefHandle* v) {
  PB(h)->interfaces[str(p, n)] = reinterpret_cast<InterfaceDefinition*>(v);
}
extern "C" VIS void metal_package_builder_add_struct(
    PackageBuilderHandle* h, const char* p, size_t n, StructDefHandle* v) {
  PB(h)->structs[str(p, n)] = reinterpret_cast<StructDefinition*>(v);
}
extern "C" VIS void metal_package_builder_add_function(
    PackageBuilderHandle* h, const char* p, size_t n, FunctionHandle* v) {
  PB(h)->functions[str(p, n)] = reinterpret_cast<Function*>(v);
}
extern "C" VIS void metal_package_builder_add_static_sized_array(
    PackageBuilderHandle* h, const char* p, size_t n, StaticSizedArrayDefHandle* v) {
  PB(h)->staticSizedArrays[str(p, n)] = reinterpret_cast<StaticSizedArrayDefinitionT*>(v);
}
extern "C" VIS void metal_package_builder_add_runtime_sized_array(
    PackageBuilderHandle* h, const char* p, size_t n, RuntimeSizedArrayDefHandle* v) {
  PB(h)->runtimeSizedArrays[str(p, n)] = reinterpret_cast<RuntimeSizedArrayDefinitionT*>(v);
}

extern "C" VIS StaticSizedArrayDefHandle* metal_static_sized_array_def_new(
    NameHandle* name, KindHandle* array_kind, int32_t size,
    RegionIdHandle* region_id,
    KindHandle* element_type) {
  return reinterpret_cast<StaticSizedArrayDefHandle*>(new StaticSizedArrayDefinitionT(
      nm(name), reinterpret_cast<StaticSizedArrayT*>(knd(array_kind)),
      size, rid(region_id),
      knd(element_type)));
}

extern "C" VIS RuntimeSizedArrayDefHandle* metal_runtime_sized_array_def_new(
    NameHandle* name, KindHandle* array_kind,
    RegionIdHandle* region_id,
    KindHandle* element_type) {
  return reinterpret_cast<RuntimeSizedArrayDefHandle*>(new RuntimeSizedArrayDefinitionT(
      nm(name), reinterpret_cast<RuntimeSizedArrayT*>(knd(array_kind)),
      rid(region_id),
      knd(element_type)));
}
extern "C" VIS void metal_package_builder_add_export_function(
    PackageBuilderHandle* h, const char* p, size_t n, PrototypeHandle* v) {
  PB(h)->exportNameToFunction[str(p, n)] = proto(v);
}
extern "C" VIS void metal_package_builder_add_export_kind(
    PackageBuilderHandle* h, const char* p, size_t n, KindHandle* v) {
  PB(h)->exportNameToKind[str(p, n)] = knd(v);
}
extern "C" VIS void metal_package_builder_add_extern_function(
    PackageBuilderHandle* h, const char* p, size_t n, PrototypeHandle* v) {
  PB(h)->externNameToFunction[str(p, n)] = proto(v);
}
extern "C" VIS void metal_package_builder_add_extern_kind(
    PackageBuilderHandle* h, const char* p, size_t n, KindHandle* v) {
  PB(h)->externNameToKind[str(p, n)] = knd(v);
}

extern "C" VIS PackageHandle* metal_package_builder_finish(PackageBuilderHandle* h) {
  auto* b = PB(h);
  auto* pkg = new Package(
      b->cache->addressNumberer,
      b->packageCoord,
      std::move(b->interfaces),
      std::move(b->structs),
      std::move(b->staticSizedArrays),
      std::move(b->runtimeSizedArrays),
      std::move(b->functions),
      std::move(b->exportNameToFunction),
      std::move(b->exportNameToKind),
      std::move(b->externNameToFunction),
      std::move(b->externNameToKind));
  delete b;
  return reinterpret_cast<PackageHandle*>(pkg);
}

// --- Program builder ---

struct ProgramBuilder {
  MetalCache* cache;
  std::unordered_map<PackageCoordinate*, Package*,
      AddressHasher<PackageCoordinate*>, std::equal_to<PackageCoordinate*>> packages;

  ProgramBuilder(MetalCache* cache_)
    : cache(cache_),
      packages(0, cache_->addressNumberer->makeHasher<PackageCoordinate*>(),
               std::equal_to<PackageCoordinate*>()) {}
};

extern "C" VIS ProgramBuilderHandle* metal_program_builder_new(MetalCacheHandle* h) {
  return reinterpret_cast<ProgramBuilderHandle*>(new ProgramBuilder(cache(h)));
}

#define ProgB(h) reinterpret_cast<ProgramBuilder*>(h)

extern "C" VIS void metal_program_builder_add_package(
    ProgramBuilderHandle* h, PackageCoordHandle* coord, PackageHandle* package) {
  ProgB(h)->packages[pc(coord)] = reinterpret_cast<Package*>(package);
}

extern "C" VIS ProgramHandle* metal_program_builder_finish(ProgramBuilderHandle* h) {
  auto* b = ProgB(h);
  auto* program = new Program(std::move(b->packages));
  delete b;
  return reinterpret_cast<ProgramHandle*>(program);
}

extern "C" VIS void metal_program_free(ProgramHandle* h) {
  auto* p = reinterpret_cast<Program*>(h);
  for (auto& [_, pkg] : p->packages) {
    delete pkg;
  }
  delete p;
}
