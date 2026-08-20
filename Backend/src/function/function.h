#ifndef FUNCTION_H_
#define FUNCTION_H_

#include <llvm-c/Core.h>

#include <unordered_map>
#include <unordered_set>
#include <iostream>
#include "../region/iregion.h"

#include "../metal/ast.h"
#include "../metal/instructions.h"
#include "../globalstate.h"

class BlockState {
public:
  AddressNumberer* const addressNumberer;
private:
  BlockState* maybeParentBlockState;
  std::optional<LLVMBasicBlockRef> maybeAfterLoop;
  // Locals are keyed by Local* pointer identity: the lowerer builds one Local per variable and
  // reuses the handle, so the pointer is a stable per-variable key (no VariableId anymore).
  std::unordered_map<Local*, LLVMValueRef, AddressHasher<Local*>> localAddrByLocal;
  std::unordered_set<Local*, AddressHasher<Local*>> unstackifiedLocals;

public:
//  LLVMBuilderRef builder;

  BlockState(const BlockState&) = delete;

  BlockState(AddressNumberer* addressNumberer_, BlockState* maybeParentBlockState_, std::optional<LLVMBasicBlockRef> maybeAfterLoop_) :
      addressNumberer(addressNumberer_),
      maybeParentBlockState(maybeParentBlockState_),
      maybeAfterLoop(maybeAfterLoop_),
      localAddrByLocal(0, addressNumberer_->makeHasher<Local*>()),
      unstackifiedLocals(0, addressNumberer_->makeHasher<Local*>()) {
  }

  LLVMValueRef getLocalAddr(Local* local, bool expectValid) const {
    if (expectValid) {
      assert(unstackifiedLocals.count(local) == 0);
    }
    auto localAddrIter = localAddrByLocal.find(local);
    if (localAddrIter != localAddrByLocal.end()) {
      return localAddrIter->second;
    }
    if (maybeParentBlockState) {
      return maybeParentBlockState->getLocalAddr(local, expectValid);
    } else {
      { assert(false); throw 1337; }
    }
  }

  bool localExists(Local* local, bool considerParentsToo) const {
    if (localAddrByLocal.find(local) != localAddrByLocal.end()) {
      return true;
    }
    if (considerParentsToo && maybeParentBlockState && maybeParentBlockState->localExists(local, true)) {
      return true;
    }
    return false;
  }

  void addLocal(Local* local, LLVMValueRef localL) {
    assert(!localExists(local, true));
    localAddrByLocal.emplace(local, localL);
  }

  std::unordered_set<Local*> getAllLocals(bool considerParentsToo) const {
    std::unordered_set<Local*> result;
    if (considerParentsToo && maybeParentBlockState) {
      result = maybeParentBlockState->getAllLocals(true);
    }
    for (auto p : localAddrByLocal) {
      result.insert(p.first);
    }
    return result;
  }

  bool localWasUnstackified(Local* local, bool considerParentsToo) const {
    if (unstackifiedLocals.count(local)) {
      return true;
    }
    if (considerParentsToo && maybeParentBlockState && maybeParentBlockState->localWasUnstackified(local, true)) {
      return true;
    }
    return false;
  }

  void markLocalUnstackified(Local* local) {
    assert(!localWasUnstackified(local, true));
    unstackifiedLocals.insert(local);
  }


  void restackify(Local* local) {
    assert(localWasUnstackified(local, true));
    unstackifiedLocals.erase(local);
  }

  void checkAllIntroducedLocalsWereUnstackified() {
    for (auto localAndLocalAddr : localAddrByLocal) {
      auto local = localAndLocalAddr.first;
      // Ignore those that were made in the parent.
      if (maybeParentBlockState &&
          maybeParentBlockState->localAddrByLocal.count(local))
        continue;
      // local came from the child block. Make sure the child unstackified it.
      if (unstackifiedLocals.count(local) == 0) {
        std::cerr << "Un-unstackified local: " << local->name << std::endl;
        { assert(false); throw 1337; }
      }
    }
  }

  // Get parent locals that the child unstackified.
  std::unordered_set<Local*> getParentLocalsThatSelfUnstackified() {
    assert(maybeParentBlockState);
    std::unordered_set<Local*> childUnstackifiedParentLocals;
    for (Local* unstackifiedLocal : unstackifiedLocals) {
      // Ignore any that were made by the child block
      if (localAddrByLocal.count(unstackifiedLocal))
        continue;
      // Ignore any that were already unstackified by the parent
      if (maybeParentBlockState->localWasUnstackified(unstackifiedLocal, true))
        continue;
      childUnstackifiedParentLocals.insert(unstackifiedLocal);
    }
    return childUnstackifiedParentLocals;
  }

  std::optional<std::tuple<BlockState*, LLVMBasicBlockRef>> getNearestLoopEnd() {
    if (maybeAfterLoop) {
      return std::optional(std::make_tuple(this, *maybeAfterLoop));
    } else {
      if (maybeParentBlockState) {
        return maybeParentBlockState->getNearestLoopEnd();
      } else {
        return std::nullopt;
      }
    }
  }
};

// Alias for an integer, to keep straight the difference between an LLVM arg and a user arg.
struct UserArgIndex { int userArgIndex; };

//// Alias for an integer, to keep straight the difference between an LLVM arg and a user arg.
//struct LlvmArgIndex { int llvmArgIndex; };

class FunctionState {
public:
  std::string containingFuncName;
  LLVMValueRef containingFuncL;
  // This is here so we can return an Undef of this when we realize we just
  // called into a Never-returning function.
  LLVMTypeRef returnTypeL;
  LLVMBuilderRef localsBuilder;
  int nextBlockNumber = 1;
  int instructionDepthInAst = 0;

  FunctionState(
      std::string containingFuncName_,
      LLVMValueRef containingFuncL_,
      LLVMTypeRef returnTypeL_,
      LLVMBuilderRef localsBuilder_) :
    containingFuncName(containingFuncName_),
    containingFuncL(containingFuncL_),
    returnTypeL(returnTypeL_),
    localsBuilder(localsBuilder_) {}

  std::string nextBlockName() {
    return std::string("block") + std::to_string(nextBlockNumber++);
  }

  LLVMValueRef getParam(UserArgIndex userArgIndex);
};

void translateFunction(
    GlobalState* globalState,
    Function* functionM);

ValeFuncPtrLE declareFunction(
    GlobalState* globalState,
    Function* functionM);

void exportFunction(GlobalState* globalState, Package* package, const std::string& exportName, Prototype* prototypeM);

RawFuncPtrLE declareExternFunction(
    GlobalState* globalState,
    Package* package,
    Prototype* prototypeM);

//LLVMTypeRef translateExternType(GlobalState* globalState, Reference* reference);


void declareExtraFunction(
    GlobalState* globalState,
    Prototype* prototype,
    std::string llvmName);

void defineFunctionBodyV(
    GlobalState* globalState,
    Prototype* prototype,
    std::function<void(FunctionState*, LLVMBuilderRef)> definer);

void declareAndDefineExtraFunction(
    GlobalState* globalState,
    Prototype* prototype,
    std::string llvmName,
    std::function<void(FunctionState*, LLVMBuilderRef)> definer);

bool typeNeedsPointerParameter(GlobalState* globalState, Reference* returnMT);
bool translatesToCVoid(GlobalState* globalState, Reference* returnMT);
LLVMTypeRef translateExternReturnType(GlobalState* globalState, Reference* returnMT);

#endif