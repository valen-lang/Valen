#include <iostream>
#include <utils/branch.h>
#include "../boundary.h"
#include "shared/shared.h"
#include "shared/string.h"
#include "../../region/common/controlblock.h"
#include "../../region/common/heap.h"
#include <region/common/migration.h>

#include "../../translatetype.h"

#include "../expression.h"


Ref buildResultOrEarlyReturnOfNever(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Prototype* prototype,
    Ref resultRef) {
  if (peel_all_references(prototype->returnType) == globalState->metalCache->neverType) {
    LLVMBuildRet(builder, LLVMGetUndef(functionState->returnTypeL));
    return toRef(globalState->getRegion(globalState->metalCache->neverType), globalState->metalCache->neverType, globalState->neverLE);
  } else {
    if (prototype->returnType == globalState->metalCache->voidType) {
      return makeVoidRef(globalState);
    } else {
      return resultRef;
    }
  }
}

Ref buildCallOrSideCall(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Prototype* prototype,
    const std::vector<Ref>& valeArgRefs) {
  // An interop extern carries an ABI descriptor (from rustc's FnAbi) saying how each argument and the
  // return crosses (the CoercionKind per arg/return, see metal/ast.h). A C extern has none; it takes the
  // descriptor-less path in each branch below.
  // VCOORD: this is temporary until we get much better C interop that does coerce to C ABI.
  const ExternAbi* abi = lookupExternAbi(globalState, prototype);
  bool hasAbi = abi != nullptr;

  // buildBoundarySignature is the single source of the declared LLVM parameter/return types (it is also
  // what declareExternFunction declares the extern from). The marshaling below builds each argument to
  // the type sig.paramTypesL declares, and reads sig.usesReturnOutParam for the return, so the call site
  // is driven from the same signature the declaration is, not kept in sync by hand.
  auto sig = buildBoundarySignature(globalState, prototype);

  auto hostArgsLE = std::vector<LLVMValueRef>{};
  hostArgsLE.reserve(valeArgRefs.size() + 1);

  for (int i = 0; i < valeArgRefs.size(); i++) {
    auto valeArgRefMT = prototype->params[i];
    auto valeArg = valeArgRefs[i];

    if (hasAbi) {
      const Coercion& c = abi->args[i];
      if (c.kind == CoercionKind::Ignore) {
        // A zero-sized argument doesn't cross at all.
        continue;
      }
      auto argValueKind = peel_all_references(valeArgRefMT);
      if ((c.kind == CoercionKind::DirectInt || c.kind == CoercionKind::Cast)
          && dynamic_cast<StructKind*>(argValueKind)) {
        // rustc passes this small struct in a single register integer (DirectInt for a scalar-repr
        // struct, Cast for a memory-repr one). Reinterpret the Vale struct's bytes as that integer, whose
        // type buildBoundarySignature already declared for this parameter. The slot is that integer type,
        // so its alloca carries the integer's alignment, which the struct's natural alignment could
        // underprovide.
        auto valeArgLE =
            globalState->getRegion(argValueKind)
                ->checkValidReference(FL(), functionState, builder, true, valeArgRefMT, valeArg);
        auto iN = sig.paramTypesL[hostArgsLE.size() + (sig.usesReturnOutParam ? 1u : 0u)];
        auto slot = makeBackendLocal(functionState, builder, iN, "argCoerceSlot", LLVMGetUndef(iN));
        LLVMBuildStore(
            builder, valeArgLE,
            LLVMBuildBitCast(builder, slot, LLVMPointerType(LLVMTypeOf(valeArgLE), 0), "argCoercePtr"));
        hostArgsLE.push_back(LLVMBuildLoad2(builder, iN, slot, "argAsInt"));
        continue;
      }
      if (c.kind == CoercionKind::DirectPtr) {
        // The host wants a pointer. A borrow already crosses as one, so its sent value is a pointer.
        // A consuming owned value (a drop's `*mut T`, where Vale holds the value inline) is not, so
        // spill it to a slot and pass the slot's address. `drop_in_place` runs on it in place; Vale
        // keeps ownership of the stack slot.
        auto sentLE = sendValeObjectIntoHost(globalState, functionState, builder, valeArgRefMT, valeArg);
        if (LLVMGetTypeKind(LLVMTypeOf(sentLE)) != LLVMPointerTypeKind) {
          sentLE = makeBackendLocal(functionState, builder, LLVMTypeOf(sentLE), "ptrCoerceSlot", sentLE);
        }
        hostArgsLE.push_back(sentLE);
        continue;
      }
      if (c.kind == CoercionKind::Indirect) {
        // Per @EACBIPZ, a large struct crosses as an indirect pointer, not byval: spill the moved owned
        // value to a slot (like the fall-through move below) and pass the slot's address, with no `byval`
        // attribute.
        auto sentLE = sendValeObjectIntoHost(globalState, functionState, builder, valeArgRefMT, valeArg);
        auto slot = makeBackendLocal(functionState, builder, LLVMTypeOf(sentLE), "indirectArgSlot", sentLE);
        hostArgsLE.push_back(slot);
        continue;
      }
      // A scalar DirectInt already is its integer. Fall through.
    }

    // Per @FRMACZ, the boundary does no RC. A share arg is *moved* into C by
    // normal Vale operations: the arg expression already produced an owned +1
    // (aliased if the value is used again, moved if it's a last use) exactly as
    // it would for a normal call, and that owned +1 crosses to C here. C owns
    // the arg and discharges it explicitly. Adding an alias here would be a
    // second +1 with no counterpart in the normal-call path — a leak.
    hostArgsLE.push_back(
        sendValeObjectIntoHost(globalState, functionState, builder, valeArgRefMT, valeArg));
  }

  auto externFuncIter = globalState->externFunctions.find(prototype->name->name);
  assert(externFuncIter != globalState->externFunctions.end());
  auto externFuncL = externFuncIter->second;

  buildFlare(FL(), globalState, functionState, builder, "Suspending function ", functionState->containingFuncName);
  buildFlare(FL(), globalState, functionState, builder, "Calling extern function ", prototype->name->name);

  auto returnKind = peel_all_references(prototype->returnType);

  // Whether the return crosses via an sret out-pointer comes straight from the signature, so the call
  // site and declareExternFunction agree by construction. An interop sret slot is the Vale value type
  // itself (sized by the struct-layout map), so the load already yields translateType(returnKind); a C
  // extern uses the `{i64}` handle type instead.
  bool retIndirect = sig.usesReturnOutParam;
  auto slotLT = hasAbi
      ? globalState->getRegion(returnKind)->translateType(returnKind)
      : globalState->getRegion(returnKind)->getExternalType(returnKind);

  LLVMValueRef hostReturnLE = nullptr;
  if (retIndirect) {
    auto localPtrLE =
        makeBackendLocal(functionState, builder, slotLT, "retOutParam", LLVMGetUndef(slotLT));
    buildFlare(FL(), globalState, functionState, builder, "Return ptr! ", ptrToIntLE(globalState, builder, localPtrLE));
    hostArgsLE.insert(hostArgsLE.begin(), localPtrLE);

    auto resultLE = buildMaybeNeverCall(globalState, builder, externFuncL, hostArgsLE);
    if (hasAbi) {
      // Match the declared `sret` attribute at the call site so the out-pointer is lowered into the
      // platform's hidden result register (x8 on aarch64), not an ordinary arg register.
      unsigned sretKind = LLVMGetEnumAttributeKindForName("sret", 4);
      auto sretAttr = LLVMCreateTypeAttribute(globalState->context, sretKind, slotLT);
      LLVMAddCallSiteAttribute(resultLE, 1u, sretAttr);
    }
    assert(LLVMTypeOf(resultLE) == LLVMVoidTypeInContext(globalState->context));
    hostReturnLE = LLVMBuildLoad2(builder, slotLT, localPtrLE, "hostReturn");
    buildFlare(FL(), globalState, functionState, builder, "Loaded the return! ",
        LLVMABISizeOfType(globalState->dataLayout, LLVMTypeOf(hostReturnLE)));
  } else {
    hostReturnLE =
        buildMaybeNeverCall(globalState, builder, externFuncL, hostArgsLE);
  }

  buildFlare(FL(), globalState, functionState, builder, "Done calling function ", prototype->name->name);
  buildFlare(FL(), globalState, functionState, builder, "Resuming function ", functionState->containingFuncName);

  buildFlare(FL(), globalState, functionState, builder);

  auto valeReturnRefMT = prototype->returnType;

  if (hasAbi
      && (abi->ret.kind == CoercionKind::DirectInt || abi->ret.kind == CoercionKind::Cast)
      && dynamic_cast<StructKind*>(returnKind)) {
    // rustc returned this small struct in a single register integer (DirectInt for a scalar-repr struct,
    // Cast for a memory-repr one). hostReturnLE is that iN; reinterpret its bytes back into the Vale
    // struct through an integer-typed slot, whose alloca carries the integer's alignment (which the
    // struct's natural alignment could underprovide). Its type now matches translateType(returnKind).
    auto valeStructLT = globalState->getRegion(returnKind)->translateType(returnKind);
    auto slot = makeBackendLocal(functionState, builder, LLVMTypeOf(hostReturnLE), "retCoerceSlot", hostReturnLE);
    hostReturnLE = LLVMBuildLoad2(
        builder, valeStructLT,
        LLVMBuildBitCast(builder, slot, LLVMPointerType(valeStructLT, 0), "retCoercePtr"), "retStruct");
  }

  auto valeReturnRef =
      receiveHostObjectIntoVale(
          globalState, functionState, builder, valeReturnRefMT, hostReturnLE);

  // dont we have to free here too

  return valeReturnRef;
}

Ref buildExternCall(
    GlobalState* globalState,
    FunctionState* functionState,
    LLVMBuilderRef builder,
    Prototype* prototype,
    const std::vector<Ref>& args) {
  if (prototype->name->name == "__vbi_addI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildAdd(builder, leftLE, rightLE,"add");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_multiplyI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto resultIntLE = LLVMBuildMul(builder, leftLE, rightLE, "mul");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultIntLE);
  } else if (prototype->name->name == "__vbi_subtractI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto resultIntLE = LLVMBuildSub(builder, leftLE, rightLE, "diff");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultIntLE);
  } else if (prototype->name->name == "__vbi_lessThanI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSLT, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_greaterThanI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSGT, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_lessThanFloat") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildFCmp(builder, LLVMRealOLT, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_greaterThanFloat") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildFCmp(builder, LLVMRealOGT, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_greaterThanOrEqI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSGE, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_lessThanOrEqI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSLE, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_eqI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntEQ, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_modI32") {
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    assert(args.size() == 2);
    auto result = LLVMBuildSRem( builder, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_divideI32") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildSDiv(builder, leftLE, rightLE,"add");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_addI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildAdd(builder, leftLE, rightLE,"add");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_multiplyI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto resultIntLE = LLVMBuildMul(builder, leftLE, rightLE, "mul");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultIntLE);
  } else if (prototype->name->name == "__vbi_subtractI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto resultIntLE = LLVMBuildSub(builder, leftLE, rightLE, "diff");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultIntLE);
  } else if (prototype->name->name == "__vbi_lessThanI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSLT, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_greaterThanI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSGT, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_greaterThanOrEqI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSGE, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_lessThanOrEqI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntSLE, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_eqI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntEQ, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_modI64") {
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    assert(args.size() == 2);
    auto result = LLVMBuildSRem( builder, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_divideI64") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildSDiv(builder, leftLE, rightLE,"add");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_divideFloatFloat") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildFDiv(builder, leftLE, rightLE,"divided");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_multiplyFloatFloat") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildFMul(builder, leftLE, rightLE,"multiplied");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_subtractFloatFloat") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildFSub(builder, leftLE, rightLE,"subtracted");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_negateFloat") {
    assert(args.size() == 1);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto result = LLVMBuildFNeg(builder, leftLE, "negated");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_strLength") {
    assert(args.size() == 1);

    assert(dynamic_cast<ShareRef*>(prototype->params[0]) != nullptr);
    auto expectedType = globalState->metalCache->str;

    auto strLiveRef =
        globalState->getRegion(expectedType)
        ->checkRefLive(FL(), functionState, builder, expectedType, args[0]);

    auto resultLenLE =
        globalState->getRegion(expectedType)
        ->getStringLen(
            functionState, builder, expectedType, strLiveRef);
    globalState->getRegion(expectedType)
        ->dealias(FL(), functionState, builder, expectedType, args[0]);
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultLenLE);
  }
  // ─── String intrinsics ────────────────────────────────────────────────
  // VCOORD: TEMPORARY: this whole section exists only until str is a proper Vale
  // class (a struct in stdlib source with its methods written in Vale).
  // Once that lands, these operators become ordinary Vale code and the
  // backend keeps at most a few tiny leaf intrinsics; the __vale_rt_
  // helpers stay as their runtime support.
  //
  // These implement Vale's string operators (`+`, `str(int)`, `streq`, ...).
  // Each intrinsic:
  //  1. Picks between mutStrRef/immStrRef based on arg ownership (like __vbi_strLength).
  //  2. Uses checkRefLive → getStringBytesPtr/getStringLen to read source data.
  //  3. Calls mallocStr (RCImm's) or a C helper for allocation of new strs.
  //  4. Dealiases every share-typed arg exactly once before returning.
  else if (prototype->name->name == "__vbi_addStr" ||
           prototype->name->name == "__vbi_streq" ||
           prototype->name->name == "__vbi_strcmp" ||
           prototype->name->name == "__vbi_strindexof") {
    // Signature: (a str, aBegin i32, aEnd i32, b str, bBegin i32, bEnd i32) -> str|bool|i32
    assert(args.size() == 6);
    auto strTypeA = globalState->metalCache->str;
    auto strTypeB = globalState->metalCache->str;

    auto aLiveRef = globalState->getRegion(strTypeA)
        ->checkRefLive(FL(), functionState, builder, strTypeA, args[0]);
    auto bLiveRef = globalState->getRegion(strTypeB)
        ->checkRefLive(FL(), functionState, builder, strTypeB, args[3]);

    auto aBaseLE = globalState->getRegion(strTypeA)
        ->getStringBytesPtr(functionState, builder, strTypeA, aLiveRef);
    auto bBaseLE = globalState->getRegion(strTypeB)
        ->getStringBytesPtr(functionState, builder, strTypeB, bLiveRef);

    auto int8LT = LLVMInt8TypeInContext(globalState->context);
    auto int32LT = LLVMInt32TypeInContext(globalState->context);
    auto aBeginLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto aEndLE   = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[2], args[2]);
    auto bBeginLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[4], args[4]);
    auto bEndLE   = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[5], args[5]);
    auto aLenLE = LLVMBuildSub(builder, aEndLE, aBeginLE, "aLen");
    auto bLenLE = LLVMBuildSub(builder, bEndLE, bBeginLE, "bLen");
    auto aStartLE = LLVMBuildInBoundsGEP2(builder, int8LT, aBaseLE, &aBeginLE, 1, "aStart");
    auto bStartLE = LLVMBuildInBoundsGEP2(builder, int8LT, bBaseLE, &bBeginLE, 1, "bStart");

    auto dealiasBothAndReturn =
        [&](Ref resultRef) {
          globalState->getRegion(strTypeA)->dealias(FL(), functionState, builder, strTypeA, args[0]);
          globalState->getRegion(strTypeB)->dealias(FL(), functionState, builder, strTypeB, args[3]);
          return resultRef;
        };
    if (prototype->name->name == "__vbi_addStr") {
      // Allocate result = concat(a[aBegin..aEnd], b[bBegin..bEnd]).
      // mallocStr allocates totalLen bytes and does one strncpy from
      // aStart — that strncpy stops at the first null in a's chars (Vale
      // strings are null-terminated at their length, so we don't over-read
      // the allocation), padding the rest of totalLen with zeros. Its
      // output would be wrong when a or b contain internal nulls, so we
      // overwrite both halves with explicit memcpys below.
      auto totalLenLE = LLVMBuildAdd(builder, aLenLE, bLenLE, "totalLen");
      auto strResultRef = globalState->getRegion(globalState->metalCache->str)
          ->mallocStr(functionState, builder, totalLenLE, aStartLE);
      auto strResultLiveRef = globalState->getRegion(globalState->metalCache->str)
          ->checkRefLive(FL(), functionState, builder,
              globalState->metalCache->str, strResultRef);
      auto resultBaseLE = globalState->getRegion(globalState->metalCache->str)
          ->getStringBytesPtr(functionState, builder, globalState->metalCache->str, strResultLiveRef);
      // memcpy(resultBase, aStart, aLen)
      auto aLenI64LE = LLVMBuildZExt(builder, aLenLE, LLVMInt64TypeInContext(globalState->context), "aLenI64");
      buildCallWith64BitSExt(globalState, builder, globalState->externs->memcpy, {resultBaseLE, aStartLE, aLenI64LE});
      // memcpy(resultBase + aLen, bStart, bLen)
      auto resultBOffsetLE = LLVMBuildInBoundsGEP2(builder, int8LT, resultBaseLE, &aLenLE, 1, "resultBOffset");
      auto bLenI64LE = LLVMBuildZExt(builder, bLenLE, LLVMInt64TypeInContext(globalState->context), "bLenI64");
      buildCallWith64BitSExt(globalState, builder, globalState->externs->memcpy, {resultBOffsetLE, bStartLE, bLenI64LE});
      return dealiasBothAndReturn(strResultRef);
    } else if (prototype->name->name == "__vbi_streq") {
      // Return (aLen == bLen) && memcmp(aStart, bStart, aLen) == 0.
      auto lenEqLE = LLVMBuildICmp(builder, LLVMIntEQ, aLenLE, bLenLE, "lenEq");
      // Even if lengths differ, we still call strncmp with the shorter length
      // to keep the IR straight-line; then AND with lenEq.
      auto minLenLE = LLVMBuildSelect(builder,
          LLVMBuildICmp(builder, LLVMIntSLT, aLenLE, bLenLE, "aShorter"),
          aLenLE, bLenLE, "minLen");
      auto minLenI64LE = LLVMBuildZExt(builder, minLenLE, LLVMInt64TypeInContext(globalState->context), "minLenSizeT");
      // buildCallWith64BitSExt sign-extends strncmp's i32 return to i64;
      // truncate back to i32 for the icmp so we compare like-typed values.
      auto cmpI64LE = buildCallWith64BitSExt(globalState, builder, globalState->externs->strncmp, {aStartLE, bStartLE, minLenI64LE});
      auto cmpLE = LLVMBuildTrunc(builder, cmpI64LE, int32LT, "cmpI32");
      auto cmpZeroLE = LLVMBuildICmp(builder, LLVMIntEQ, cmpLE, LLVMConstInt(int32LT, 0, false), "cmpZero");
      auto eqLE = LLVMBuildAnd(builder, lenEqLE, cmpZeroLE, "eq");
      return dealiasBothAndReturn(
          toRef(globalState->getRegion(prototype->returnType), prototype->returnType, eqLE));
    } else if (prototype->name->name == "__vbi_strcmp") {
      // Compare by memcmp up to min(aLen, bLen); if equal but lengths differ,
      // shorter compares less.
      auto minLenLE = LLVMBuildSelect(builder,
          LLVMBuildICmp(builder, LLVMIntSLT, aLenLE, bLenLE, "aShorter"),
          aLenLE, bLenLE, "minLen");
      auto minLenI64LE = LLVMBuildZExt(builder, minLenLE, LLVMInt64TypeInContext(globalState->context), "minLenSizeT");
      auto cmpI64LE = buildCallWith64BitSExt(globalState, builder, globalState->externs->strncmp, {aStartLE, bStartLE, minLenI64LE});
      auto cmpLE = LLVMBuildTrunc(builder, cmpI64LE, int32LT, "cmpI32");
      auto cmpZeroLE = LLVMBuildICmp(builder, LLVMIntEQ, cmpLE, LLVMConstInt(int32LT, 0, false), "cmpZero");
      // If cmp != 0, return sign of cmp. If cmp == 0, return sign of (aLen - bLen).
      auto lenDiffLE = LLVMBuildSub(builder, aLenLE, bLenLE, "lenDiff");
      auto resultLE = LLVMBuildSelect(builder, cmpZeroLE, lenDiffLE, cmpLE, "cmpResult");
      return dealiasBothAndReturn(
          toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultLE));
    } else if (prototype->name->name == "__vbi_strindexof") {
      // __vbi_strindexof — search for b in a. Delegate to C helper.
      // Helper returns i32; buildCallWith64BitSExt sign-extends to i64. Trunc
      // back so the return type matches Vale int (i32).
      auto findI64LE = buildCallWith64BitSExt(globalState, builder,
          globalState->externs->valeRtBytesFindLF,
          {aStartLE, aLenLE, bStartLE, bLenLE});
      auto findResultLE = LLVMBuildTrunc(builder, findI64LE, int32LT, "findI32");
      return dealiasBothAndReturn(
          toRef(globalState->getRegion(prototype->returnType), prototype->returnType, findResultLE));
    } else {
      // A new two-string __vbi_ op must add its own case above, not fall here.
      assert(false);
      throw 1337;
    }
  } else if (prototype->name->name == "__vbi_substring") {
    // Signature: (s str, begin i32, end i32) -> str
    assert(args.size() == 3);
    auto strType = globalState->metalCache->str;
    auto liveRef = globalState->getRegion(strType)
        ->checkRefLive(FL(), functionState, builder, strType, args[0]);
    auto baseLE = globalState->getRegion(strType)
        ->getStringBytesPtr(functionState, builder, strType, liveRef);
    auto int8LT = LLVMInt8TypeInContext(globalState->context);
    auto beginLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto endLE   = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[2], args[2]);
    auto lenLE = LLVMBuildSub(builder, endLE, beginLE, "len");
    auto startLE = LLVMBuildInBoundsGEP2(builder, int8LT, baseLE, &beginLE, 1, "start");

    auto strResultRef = globalState->getRegion(globalState->metalCache->str)
        ->mallocStr(functionState, builder, lenLE, startLE);
    // Overwrite via memcpy to guard against internal nulls in source.
    auto strResultLiveRef = globalState->getRegion(globalState->metalCache->str)
        ->checkRefLive(FL(), functionState, builder,
            globalState->metalCache->str, strResultRef);
    auto resultBaseLE = globalState->getRegion(globalState->metalCache->str)
        ->getStringBytesPtr(functionState, builder, globalState->metalCache->str, strResultLiveRef);
    auto lenI64LE = LLVMBuildZExt(builder, lenLE, LLVMInt64TypeInContext(globalState->context), "lenI64");
    buildCallWith64BitSExt(globalState, builder, globalState->externs->memcpy, {resultBaseLE, startLE, lenI64LE});

    globalState->getRegion(strType)->dealias(FL(), functionState, builder, strType, args[0]);
    return strResultRef;
  } else if (prototype->name->name == "__vbi_strtoascii") {
    // Signature: (s str, begin i32, end i32) -> i32
    // Loads byte at s.chars[begin] and returns it zero-extended.
    assert(args.size() == 3);
    auto strType = globalState->metalCache->str;
    auto liveRef = globalState->getRegion(strType)
        ->checkRefLive(FL(), functionState, builder, strType, args[0]);
    auto baseLE = globalState->getRegion(strType)
        ->getStringBytesPtr(functionState, builder, strType, liveRef);
    auto int8LT = LLVMInt8TypeInContext(globalState->context);
    auto int32LT = LLVMInt32TypeInContext(globalState->context);
    auto beginLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto charPtrLE = LLVMBuildInBoundsGEP2(builder, int8LT, baseLE, &beginLE, 1, "charPtr");
    auto byteLE = LLVMBuildLoad2(builder, int8LT, charPtrLE, "byte");
    auto byteI32LE = LLVMBuildZExt(builder, byteLE, int32LT, "byteI32");

    globalState->getRegion(strType)->dealias(FL(), functionState, builder, strType, args[0]);
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, byteI32LE);
  } else if (prototype->name->name == "__vbi_strfromascii") {
    // Signature: (code i32) -> str
    // Builds a 1-char string from the given ASCII code.
    assert(args.size() == 1);
    auto codeLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto int8LT = LLVMInt8TypeInContext(globalState->context);
    auto int32LT = LLVMInt32TypeInContext(globalState->context);
    // Stash the low byte in a stack local so we can hand mallocStr a pointer.
    auto byteLE = LLVMBuildTrunc(builder, codeLE, int8LT, "byteLE");
    auto bufLE = makeBackendLocal(functionState, builder, int8LT, "asciiByte", byteLE);
    return globalState->getRegion(globalState->metalCache->str)
        ->mallocStr(functionState, builder,
            LLVMConstInt(int32LT, 1, false), bufLE);
  } else if (prototype->name->name == "__vbi_printstr") {
    // Signature: (s str, start i32, length i32) -> void
    assert(args.size() == 3);
    auto strType = globalState->metalCache->str;
    auto liveRef = globalState->getRegion(strType)
        ->checkRefLive(FL(), functionState, builder, strType, args[0]);
    auto baseLE = globalState->getRegion(strType)
        ->getStringBytesPtr(functionState, builder, strType, liveRef);
    auto int8LT = LLVMInt8TypeInContext(globalState->context);
    auto startLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto lengthLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[2], args[2]);
    auto ptrLE = LLVMBuildInBoundsGEP2(builder, int8LT, baseLE, &startLE, 1, "printPtr");
    buildCallWith64BitSExt(globalState, builder, globalState->externs->valeRtWriteStdoutLF, {ptrLE, lengthLE});
    globalState->getRegion(strType)->dealias(FL(), functionState, builder, strType, args[0]);
    return makeVoidRef(globalState);
  } else if (prototype->name->name == "__vbi_getMainArg") {
    // Signature: (i int) -> str
    // Pulls argv[i] from the process's stashed __main_args pointer and hands
    // its (length, byte pointer) to mallocStr, which does its own memcpy into
    // a fresh Vale-owned share str. Mirrors the __vbi_castI64Str shape but
    // uses argv storage directly instead of a formatted stack buffer.
    assert(args.size() == 1);
    auto int32LT = LLVMInt32TypeInContext(globalState->context);
    auto int64LT = LLVMInt64TypeInContext(globalState->context);
    auto iLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto iI64LE = LLVMBuildSExt(builder, iLE, int64LT, "iI64");
    // buildCallWith64BitSExt sign-extends i32 return to i64; truncate back
    // because mallocStr asserts i32 length.
    auto lenI64LE = buildCallWith64BitSExt(globalState, builder, globalState->externs->valeRtGetMainArgLenLF, {iI64LE});
    auto lenI32LE = LLVMBuildTrunc(builder, lenI64LE, int32LT, "lenI32");
    // Pointer return passes through unmodified (buildCallWith64BitSExt only
    // sign-extends integer returns narrower than 64 bits).
    auto bytesPtrLE = buildCallWith64BitSExt(globalState, builder, globalState->externs->valeRtGetMainArgPtrLF, {iI64LE});
    return globalState->getRegion(globalState->metalCache->str)
        ->mallocStr(functionState, builder, lenI32LE, bytesPtrLE);
  } else if (prototype->name->name == "__vbi_castI32Str" ||
             prototype->name->name == "__vbi_castI64Str" ||
             prototype->name->name == "__vbi_castFloatStr") {
    // Signature: (x <primitive>) -> str
    // Format the primitive into a stack buffer via a C helper, then mallocStr.
    assert(args.size() == 1);
    auto int8LT = LLVMInt8TypeInContext(globalState->context);
    auto int32LT = LLVMInt32TypeInContext(globalState->context);
    // 32-byte buffer covers i64 and double representations comfortably.
    const int bufSize = 32;
    auto bufTypeLT = LLVMArrayType(int8LT, bufSize);
    auto bufLE = makeBackendLocal(functionState, builder, bufTypeLT, "asciiBuf", LLVMGetUndef(bufTypeLT));
    // Bitcast [32 x i8] alloca → i8*
    auto bufPtrLE = LLVMBuildBitCast(builder, bufLE, LLVMPointerType(int8LT, 0), "asciiBufPtr");
    auto bufSizeLE = LLVMConstInt(int32LT, bufSize, false);
    auto xLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    LLVMValueRef writtenLE = nullptr;
    if (prototype->name->name == "__vbi_castI32Str") {
      // Widen to i64 and call the same helper.
      auto xI64LE = LLVMBuildSExt(builder, xLE, LLVMInt64TypeInContext(globalState->context), "xI64");
      writtenLE = buildCallWith64BitSExt(globalState, builder, globalState->externs->valeRtI64ToAsciiLF, {xI64LE, bufPtrLE, bufSizeLE});
    } else if (prototype->name->name == "__vbi_castI64Str") {
      writtenLE = buildCallWith64BitSExt(globalState, builder, globalState->externs->valeRtI64ToAsciiLF, {xLE, bufPtrLE, bufSizeLE});
    } else {
      writtenLE = buildCallWith64BitSExt(globalState, builder, globalState->externs->valeRtFloatToAsciiLF, {xLE, bufPtrLE, bufSizeLE});
    }
    // buildCallWith64BitSExt sign-extends the i32 return to i64; truncate
    // back before handing to mallocStr, which asserts i32 length.
    auto writtenI32LE = LLVMBuildTrunc(builder, writtenLE, int32LT, "writtenI32");
    return globalState->getRegion(globalState->metalCache->str)
        ->mallocStr(functionState, builder, writtenI32LE, bufPtrLE);
  } else if (prototype->name->name == "__vbi_addFloatFloat") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildFAdd(builder, leftLE, rightLE, "add");
    return toRef(globalState->getRegion(globalState->metalCache->floatType), globalState->metalCache->floatType, result);
  } else if (prototype->name->name == "__vbi_panic") {
    buildPrintToStderr(globalState, builder, "(panic)\n");
    // See MPESC for status codes
    auto exitCodeLE = makeConstIntExpr(functionState, builder, LLVMInt64TypeInContext(globalState->context), 1);
    buildCallWith64BitSExt(globalState, builder, globalState->externs->exit, {exitCodeLE});
    LLVMBuildRet(builder, LLVMGetUndef(functionState->returnTypeL));
    return toRef(globalState->getRegion(globalState->metalCache->neverType), globalState->metalCache->neverType, globalState->neverLE);
  } else if (prototype->name->name == "__vbi_getch") {
    auto resultIntLE = buildCallWith64BitSExt(globalState, builder, globalState->externs->getch, {});
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, resultIntLE);
  } else if (prototype->name->name == "__vbi_eqFloatFloat") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildFCmp(builder, LLVMRealOEQ, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_eqBoolBool") {
    assert(args.size() == 2);
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    auto result = LLVMBuildICmp(builder, LLVMIntEQ, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_not") {
    assert(args.size() == 1);
    auto result = LLVMBuildNot(
        builder,
        checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]),
        "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_and") {
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    assert(args.size() == 2);
    auto result = LLVMBuildAnd( builder, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_or") {
    auto leftLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    auto rightLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[1], args[1]);
    assert(args.size() == 2);
    auto result = LLVMBuildOr( builder, leftLE, rightLE, "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else if (prototype->name->name == "__vbi_ExtendI32ToI64") {
    auto intLE = checkValidInternalReference(FL(), globalState, functionState, builder, true, prototype->params[0], args[0]);
    assert(args.size() == 1);
    auto result = LLVMBuildSExt(builder, intLE, LLVMInt64TypeInContext(globalState->context), "");
    return toRef(globalState->getRegion(prototype->returnType), prototype->returnType, result);
  } else {
    auto valeReturnRef = buildCallOrSideCall(globalState, functionState, builder, prototype, args);
    return buildResultOrEarlyReturnOfNever(globalState, functionState, builder, prototype, valeReturnRef);
  }
  { assert(false); throw 1337; }
}

Ref translateExternCall(
    GlobalState* globalState,
    FunctionState* functionState,
    BlockState* blockState,
    LLVMBuilderRef builder,
    ExternCall* call) {
  auto name = call->prototype->name->name;
  auto params = call->prototype->params;
  std::vector<Ref> args;
  assert(call->args.size() == call->prototype->params.size());
  for (int i = 0; i < call->args.size(); i++) {
    args.emplace_back(
        translateExpression(globalState, functionState, blockState, builder, call->args[i]));
  }
  return buildExternCall(globalState, functionState, builder, call->prototype, args);
}
