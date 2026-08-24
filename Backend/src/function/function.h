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
  std::unordered_map<VarNameM, LLVMValueRef> localAddrByLocal;
  std::unordered_set<VarNameM> unstackifiedLocals;

public:
//  LLVMBuilderRef builder;

  BlockState(const BlockState&) = delete;

  BlockState(AddressNumberer* addressNumberer_, BlockState* maybeParentBlockState_, std::optional<LLVMBasicBlockRef> maybeAfterLoop_) :
      addressNumberer(addressNumberer_),
      maybeParentBlockState(maybeParentBlockState_),
      maybeAfterLoop(maybeAfterLoop_) {
  }

  LLVMValueRef getLocalAddr(const VarNameM& localId, bool expectValid) const {
    if (expectValid) {
      assert(unstackifiedLocals.count(localId) == 0);
    }
    auto localAddrIter = localAddrByLocal.find(localId);
    if (localAddrIter != localAddrByLocal.end()) {
      return localAddrIter->second;
    }
    if (maybeParentBlockState) {
      return maybeParentBlockState->getLocalAddr(localId, expectValid);
    } else {
      { assert(false); throw 1337; }
    }
  }

  bool localExists(const VarNameM& localId, bool considerParentsToo) const {
    if (localAddrByLocal.find(localId) != localAddrByLocal.end()) {
      return true;
    }
    if (considerParentsToo && maybeParentBlockState && maybeParentBlockState->localExists(localId, true)) {
      return true;
    }
    return false;
  }

  void addLocal(const VarNameM& localId, LLVMValueRef localL) {
    assert(!localExists(localId, true));
    localAddrByLocal.emplace(localId, localL);
  }

  std::unordered_set<VarNameM> getAllLocals(bool considerParentsToo) const {
    std::unordered_set<VarNameM> result;
    if (considerParentsToo && maybeParentBlockState) {
      result = maybeParentBlockState->getAllLocals(true);
    }
    for (auto p : localAddrByLocal) {
      result.insert(p.first);
    }
    return result;
  }

  bool localWasUnstackified(const VarNameM& localId, bool considerParentsToo) const {
    if (unstackifiedLocals.count(localId)) {
      return true;
    }
    if (considerParentsToo && maybeParentBlockState && maybeParentBlockState->localWasUnstackified(localId, true)) {
      return true;
    }
    return false;
  }

  void markLocalUnstackified(const VarNameM& localId) {
    assert(!localWasUnstackified(localId, true));
    unstackifiedLocals.insert(localId);
  }


  void restackify(const VarNameM& localId) {
    assert(localWasUnstackified(localId, true));
    unstackifiedLocals.erase(localId);
  }

  void checkAllIntroducedLocalsWereUnstackified() {
    for (auto localAndLocalAddr : localAddrByLocal) {
      auto localId = localAndLocalAddr.first;
      // Ignore those that were made in the parent.
      if (maybeParentBlockState &&
          maybeParentBlockState->localAddrByLocal.count(localId))
        continue;
      // local came from the child block. Make sure the child unstackified it.
      if (unstackifiedLocals.count(localId) == 0) {
        std::cerr << "Un-unstackified local: " << localId.name << std::endl;
        { assert(false); throw 1337; }
      }
    }
  }

  // Get parent locals that the child unstackified.
  std::unordered_set<VarNameM> getParentLocalsThatSelfUnstackified() {
    assert(maybeParentBlockState);
    std::unordered_set<VarNameM> childUnstackifiedParentLocals;
    for (const VarNameM& unstackifiedLocalId : unstackifiedLocals) {
      // Ignore any that were made by the child block
      if (localAddrByLocal.count(unstackifiedLocalId))
        continue;
      // Ignore any that were already unstackified by the parent
      if (maybeParentBlockState->localWasUnstackified(unstackifiedLocalId, true))
        continue;
      childUnstackifiedParentLocals.insert(unstackifiedLocalId);
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

//LLVMTypeRef translateExternType(GlobalState* globalState, Kind* reference);


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

#endif