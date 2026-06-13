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

  Externs(LLVMModuleRef mod, LLVMContextRef context);
};

bool includeSizeParam(GlobalState* globalState, Prototype* prototype, int paramIndex);

#endif
