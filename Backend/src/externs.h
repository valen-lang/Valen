#ifndef EXTERNS_H_
#define EXTERNS_H_

class Externs {
public:
  RawFuncPtrLE malloc;
  RawFuncPtrLE free;
  RawFuncPtrLE assert;
  RawFuncPtrLE exit;
  RawFuncPtrLE perror;
  RawFuncPtrLE assertI64Eq;
  RawFuncPtrLE printCStr;
  RawFuncPtrLE printCStrToStderr;
  RawFuncPtrLE getch;
  RawFuncPtrLE printInt;
  RawFuncPtrLE printIntToStderr;
  RawFuncPtrLE strlen;
  RawFuncPtrLE memset;
  RawFuncPtrLE strncpy;
  RawFuncPtrLE strncmp;
  RawFuncPtrLE memcpy;


  RawFuncPtrLE fopen;
  RawFuncPtrLE fclose;
  RawFuncPtrLE fread;
  RawFuncPtrLE fwrite;

//  RawFuncPtrLE initTwinPages;
  RawFuncPtrLE censusContains;
  RawFuncPtrLE censusAdd;
  RawFuncPtrLE censusRemove;

  RawFuncPtrLE strHasherCallLF;
  RawFuncPtrLE strEquatorCallLF;
  RawFuncPtrLE int256HasherCallLF;
  RawFuncPtrLE int256EquatorCallLF;

  // `ptrSizeBits` matches GlobalState::ptrSize and determines the LLVM
  // integer width used for `size_t`-typed libc args (i64 on native 64-bit,
  // i32 on wasm32).
  Externs(LLVMModuleRef mod, LLVMContextRef context, int ptrSizeBits);
};

bool includeSizeParam(GlobalState* globalState, Prototype* prototype, int paramIndex);

#endif
