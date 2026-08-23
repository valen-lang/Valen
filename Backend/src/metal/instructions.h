
#ifndef VALE_INSTRUCTIONS_H_
#define VALE_INSTRUCTIONS_H_

class Expression;
class IRegister;
class ReferenceRegister;
class AddressRegister;
class Local;
class VariableId;
class StackHeight;

enum class RefCountCategory {
    VARIABLE_REF_COUNT,
    MEMBER_REF_COUNT,
    REGISTER_REF_COUNT
};

class Expression {
public:
    virtual ~Expression() {}

//    virtual Kind* getResultType() const = 0;
};

class ConstantVoid : public Expression {
public:
  ConstantVoid() {}
};

class ConstantInt : public Expression {
public:
  int64_t value;
  int bits;

  ConstantInt(
      int64_t value_,
      int bits_)
      : value(value_),
        bits(bits_) {}
};

class ConstantBool : public Expression {
public:
  bool value;

  ConstantBool(
      bool value_)
      : value(value_) {}
};



class ConstantStr : public Expression {
public:
  std::string value;
  Kind* result;

  ConstantStr(
      const std::string &value_,
      Kind* result_) :
      value(value_),
      result(result_) {}
};


class ConstantF64 : public Expression {
public:
  double value;

  ConstantF64(
      const double &value_) :
      value(value_) {}
};


class Argument : public Expression {
public:
  int paramIndex;
  Kind* tyype;

  Argument(
      int paramIndex_,
      Kind* tyype_) :
    paramIndex(paramIndex_),
    tyype(tyype_) {}
};


class Stackify : public Expression {
public:
  Local* variable;
  Expression* expr;
  Kind* result;

  Stackify(
      Local* variable_,
      Expression* expr_,
      Kind* result_) :
    variable(variable_),
    expr(expr_),
    result(result_) {}
};

class Restackify : public Expression {
public:
  Local* variable;
  Expression* sourceExpr;
  Kind* result;

  Restackify(
      Local* variable_,
      Expression* sourceExpr_,
      Kind* result_) :
      variable(variable_),
      sourceExpr(sourceExpr_),
      result(result_) {}
};


class Unstackify : public Expression {
public:
  Local* variable;
  Kind* result;

  Unstackify(Local* variable_, Kind* result_) :
    variable(variable_),
    result(result_) {}
};


class Destroy : public Expression {
public:
  Expression* structExpr;
  StructKind* structType;
  std::vector<Local*> destinationLocals;

  Destroy(
      Expression* expr_,
      StructKind* structKind_,
      std::vector<Local*> destinationLocals_) :
      structExpr(expr_),
      structType(structKind_),
      destinationLocals(destinationLocals_) {}
};


class StructToInterfaceUpcast : public Expression {
public:
  Expression* innerExpr;
  Kind* sourceType;
  InterfaceKind* targetInterface;
  Name* implName;
  Kind* result;

  StructToInterfaceUpcast(
      Expression* innerExpr_,
      Kind* sourceType_,
      InterfaceKind* targetInterface_,
      Name* implName_,
      Kind* result_) :
      innerExpr(innerExpr_),
      sourceType(sourceType_),
      targetInterface(targetInterface_),
      implName(implName_),
      result(result_) {}
};

class InterfaceToInterfaceUpcast : public Expression {
public:
  Expression* innerExpr;
  InterfaceKind* targetInterface;
  Kind* result;

  InterfaceToInterfaceUpcast(
      Expression* innerExpr_,
      InterfaceKind* targetInterface_,
      Kind* result_) :
      innerExpr(innerExpr_),
      targetInterface(targetInterface_),
      result(result_) {}
};

class IsSameInstance : public Expression {
public:
  Expression* left;
  Kind* leftType;
  Expression* right;
  Kind* rightType;

  IsSameInstance(
      Expression* left_,
      Kind* leftType_,
      Expression* right_,
      Kind* rightType_) :
    left(left_),
    leftType(leftType_),
    right(right_),
    rightType(rightType_) {}
};

class LocalStore : public Expression {
public:
  Local* local;
  Expression* sourceExpr;
  std::string localName;

  LocalStore(
      Local* local_,
      Expression* sourceExpr_,
      std::string localName_) :
      local(local_),
      sourceExpr(sourceExpr_),
      localName(localName_) {}
};

class Mutate : public Expression {
public:
  Expression* destinationExpr;
  BorrowRef* destinationType;
  Expression* sourceExpr;
  Kind* sourceType;
  Kind* result;

  Mutate(Expression* destinationExpr_, BorrowRef* destinationType_, Expression* sourceExpr_, Kind* sourceType_, Kind* result_) :
      destinationExpr(destinationExpr_), destinationType(destinationType_), sourceExpr(sourceExpr_), sourceType(sourceType_), result(result_) {}
};


// TODO: replace LocalLoad with this perhaps?
class LocalLookup : public Expression {
public:
  Local* localVariable;
  Kind* result;

  LocalLookup(Local* localVariable_, Kind* result_) :
      localVariable(localVariable_), result(result_) {}
};

class WeakAlias : public Expression {
public:
  Expression* innerExpr;
  Kind* sourceType;
  Kind* result;

  WeakAlias(
      Expression* innerExpr_,
      Kind* sourceType_,
      Kind* result_) :
    innerExpr(innerExpr_),
    sourceType(sourceType_),
    result(result_) {}
};
//
// class MemberLoad : public Expression {
// public:
//   Expression* structExpr;
//   StructKind* structId;
//   Kind* structType;
//   int memberIndex;
//   Kind* expectedMemberType;
//   Kind* expectedResultType;
//   std::string memberName;
//
//   MemberLoad(
//       Expression* structExpr_,
//       StructKind* structId_,
//       Kind* structType_,
//       int memberIndex_,
//       Kind* expectedMemberType_,
//       Kind* expectedResultType_,
//       std::string memberName_) :
//     structExpr(structExpr_),
//     structId(structId_),
//     structType(structType_),
//     memberIndex(memberIndex_),
//     expectedMemberType(expectedMemberType_),
//     expectedResultType(expectedResultType_),
//     memberName(memberName_) {}
// };

class MemberLookup : public Expression {
public:
  Expression* structExpr;
  BorrowRef* structType;
  int memberIndex;
  std::string memberName;
  Kind* memberType;
  Kind* result;

  MemberLookup(Expression* structExpr_, BorrowRef* structType_, int memberIndex_, std::string memberName_, Kind* memberType_, Kind* result_) :
      structExpr(structExpr_), structType(structType_), memberIndex(memberIndex_), memberName(memberName_), memberType(memberType_), result(result_) {}
};


class NewArrayFromValues : public Expression {
public:
  std::vector<Expression*> elements;
  Kind* result;
  StaticSizedArrayT* arrayType;

  NewArrayFromValues(
      std::vector<Expression*> elements_,
      Kind* result_,
      StaticSizedArrayT* arrayType_) :
      elements(elements_),
      result(result_),
      arrayType(arrayType_) {}
};


class Call : public Expression {
public:
  Prototype* callable;
  std::vector<Expression *> args;
  Kind* result;

  Call(
      Prototype* callable_,
      std::vector<Expression *> args_,
      Kind* result_)
      : callable(callable_),
        args(args_),
        result(result_) {}
};

class ExternCall : public Expression {
public:
    Prototype* prototype;
    std::vector<Expression *> args;
    Kind* result;

    ExternCall(
        Prototype* prototype_,
        std::vector<Expression *> args_,
        Kind* result_)
        : prototype(prototype_),
        args(args_),
        result(result_) {}
};


class InterfaceCall : public Expression {
public:
  Prototype* superFunctionPrototype;
  int virtualParamIndex;
  int indexInEdge;
  std::vector<Expression*> args;
  Kind* result;

  InterfaceCall(
      Prototype* superFunctionPrototype_,
      int virtualParamIndex_,
      int indexInEdge_,
      std::vector<Expression*> args_,
      Kind* result_) :
    superFunctionPrototype(superFunctionPrototype_),
    virtualParamIndex(virtualParamIndex_),
    indexInEdge(indexInEdge_),
    args(args_),
    result(result_) {
  }
};


class If : public Expression {
public:
  Expression* condition;
  Expression* thenCall;
  Expression* elseCall;
  Kind* thenResultType;
  Kind* elseResultType;
  Kind* result;

  If(
      Expression* condition_,
      Expression* thenCall_,
      Expression* elseCall_,
      Kind* thenResultType_,
      Kind* elseResultType_,
      Kind* result_) :
    condition(condition_),
    thenCall(thenCall_),
    elseCall(elseCall_),
    thenResultType(thenResultType_),
    elseResultType(elseResultType_),
    result(result_) {}
};

class While : public Expression {
public:
  Expression* block;
  Kind* result;

  While(Expression* block_, Kind* result_) :
    block(block_),
    result(result_) {}
};

class Consecutor : public Expression {
public:
  std::vector<Expression *> exprs;
  Kind* result;

  Consecutor(
      std::vector<Expression *> exprs_,
      Kind* result_) :
      exprs(exprs_),
      result(result_) {}
};

class Block : public Expression {
public:
  Expression* inner;
  Kind* result;

  Block(Expression* inner_, Kind* result_) :
  inner(inner_),
  result(result_) {}
};

class Break : public Expression {
public:
};

class Return : public Expression {
public:
  Expression *sourceExpr;
  Kind* sourceType;

  Return(
    Expression *sourceExpr_,
    Kind* sourceType_)
    : sourceExpr(sourceExpr_), sourceType(sourceType_) {}
};


class NewRuntimeSizedArray : public Expression {
public:
  RuntimeSizedArrayT* arrayType;
  Expression* capacityExpr;
  Kind* result;

  NewRuntimeSizedArray(
      RuntimeSizedArrayT* arrayType_,
      Expression* capacityExpr_,
      Kind* result_) :
      arrayType(arrayType_),
      capacityExpr(capacityExpr_),
      result(result_) {}
};

class StaticArrayFromCallable : public Expression {
public:
  StaticSizedArrayT* arrayType;
  Expression* generator;
  Prototype* generatorMethod;
  Kind* result;

  StaticArrayFromCallable(
      StaticSizedArrayT* arrayType_,
      Expression* generator_,
      Prototype* generatorMethod_,
      Kind* result_) :
      arrayType(arrayType_),
      generator(generator_),
      generatorMethod(generatorMethod_),
      result(result_) {}
};

class DestroyStaticSizedArrayIntoFunction : public Expression {
public:
  Expression* arrayExpr;
  StaticSizedArrayT* arrayType;
  Expression* consumer;
  Prototype* consumerMethod;

  DestroyStaticSizedArrayIntoFunction(
      Expression* arrayExpr_,
      StaticSizedArrayT* arrayType_,
      Expression* consumer_,
      Prototype* consumerMethod_) :
    arrayExpr(arrayExpr_),
    arrayType(arrayType_),
    consumer(consumer_),
    consumerMethod(consumerMethod_) {}
};

class DestroyStaticSizedArrayIntoLocals : public Expression {
public:
  Expression* expr;
  StaticSizedArrayT* staticSizedArray;
  std::vector<Local*> destinationLocals;

  DestroyStaticSizedArrayIntoLocals(
    Expression* expr_,
    StaticSizedArrayT* staticSizedArray_,
    std::vector<Local*> destinationLocals_) :
      expr(expr_),
      staticSizedArray(staticSizedArray_),
      destinationLocals(destinationLocals_) {}
};

class DestroyRuntimeSizedArray : public Expression {
public:
  Expression* arrayExpr;
  Kind* arrayType;

  DestroyRuntimeSizedArray(
      Expression* arrayExpr_) :
    arrayExpr(arrayExpr_) {}
};

class NewStruct : public Expression {
public:
  StructKind* structKind;
  Kind* result;
  std::vector<Expression*> args;

  NewStruct(
      StructKind* structKind_,
      Kind* result_,
      std::vector<Expression*> args_) :
      structKind(structKind_),
      result(result_),
      args(args_) {}
};

class ArrayLength : public Expression {
public:
  Expression* arrayExpr;
  BorrowRef* arrayType;

  ArrayLength(
      Expression* arrayExpr_,
      BorrowRef* arrayType_) :
      arrayExpr(arrayExpr_),
      arrayType(arrayType_) {}
};

class ArrayCapacity : public Expression {
public:
  Expression* arrayExpr;
  BorrowRef* arrayType;

  ArrayCapacity(
      Expression* arrayExpr_,
      BorrowRef* arrayType_) :
      arrayExpr(arrayExpr_),
      arrayType(arrayType_) {}
};

class PushRuntimeSizedArray : public Expression {
public:
  Expression* arrayExpr;
  BorrowRef* arrayType;
  Expression* newElementExpr;
  Kind* elementType;

  PushRuntimeSizedArray(
      Expression* arrayExpr_,
      BorrowRef* arrayType_,
      Expression* newElementExpr_,
      Kind* elementType_) :
      arrayExpr(arrayExpr_),
      arrayType(arrayType_),
      newElementExpr(newElementExpr_),
      elementType(elementType_) {}
};

class PopRuntimeSizedArray : public Expression {
public:
  Expression* arrayExpr;
  BorrowRef* arrayType;
  Kind* result;

  PopRuntimeSizedArray(
      Expression* arrayExpr_,
      BorrowRef* arrayType_,
      Kind* result_) :
      arrayExpr(arrayExpr_),
      arrayType(arrayType_),
      result(result_) {}
};


class Discard : public Expression {
public:
  Expression* expr;
  Kind* sourceType;

  Discard(Expression* expr_, Kind* sourceType_) :
      expr(expr_), sourceType(sourceType_) {}
};

class LockWeak : public Expression {
public:
  Expression* innerExpr;
  Kind* sourceType;
  Prototype* someConstructor;
  Prototype* noneConstructor;
  Name* someImplName;
  Name* noneImplName;
  Kind* result;

  LockWeak(
      Expression* innerExpr_,
      Kind* sourceType_,
      Prototype* someConstructor_,
      Prototype* noneConstructor_,
      Name* someImplName_,
      Name* noneImplName_,
      Kind* result_) :
    innerExpr(innerExpr_),
    sourceType(sourceType_),
    someConstructor(someConstructor_),
    noneConstructor(noneConstructor_),
    someImplName(someImplName_),
    noneImplName(noneImplName_),
    result(result_) {}
};


class AsSubtype : public Expression {
public:
  Expression* sourceExpr;
  Kind* sourceType;
  Kind* targetType;
  Prototype* okConstructor;
  Prototype* errConstructor;
  Name* implName;
  Name* okImplName;
  Name* errImplName;
  Kind* result;

  AsSubtype(
      Expression* sourceExpr_,
      Kind* sourceType_,
      Kind* targetType_,
      Prototype* okConstructor_,
      Prototype* errConstructor_,
      Name* implName_,
      Name* okImplName_,
      Name* errImplName_,
      Kind* result_) :
    sourceExpr(sourceExpr_),
    sourceType(sourceType_),
    targetType(targetType_),
    okConstructor(okConstructor_),
    errConstructor(errConstructor_),
    implName(implName_),
    okImplName(okImplName_),
    errImplName(errImplName_),
    result(result_) {}
};

class CopyPrim : public Expression {
public:
    Expression* inner;
    Kind* result;

    CopyPrim(Expression* inner_, Kind* result_) :
        inner(inner_), result(result_) {}
};


class LetAndLend : public Expression {
public:
  Local* variable;
  Expression* expr;
  Kind* result;

  LetAndLend(Local* variable_, Expression* expr_, Kind* result_) :
      variable(variable_), expr(expr_), result(result_) {}
};

class Deref : public Expression {
public:
  Expression* inner;
  Kind* sourceType;
  Kind* result;

  Deref(Expression* inner_, Kind* sourceType_, Kind* result_) :
      inner(inner_), sourceType(sourceType_), result(result_) {}
};

class StaticSizedArrayLookup : public Expression {
public:
  Expression* arrayExpr;
  BorrowRef* arrayType;
  Expression* indexExpr;
  Kind* indexType;
  Kind* result;

  StaticSizedArrayLookup(Expression* arrayExpr_, BorrowRef* arrayType_, Expression* indexExpr_, Kind* indexType_, Kind* result_) :
      arrayExpr(arrayExpr_), arrayType(arrayType_), indexExpr(indexExpr_), indexType(indexType_), result(result_) {}
};

class RuntimeSizedArrayLookup : public Expression {
public:
  Expression* arrayExpr;
  BorrowRef* arrayType;
  Expression* indexExpr;
  Kind* indexType;
  Kind* result;

  RuntimeSizedArrayLookup(Expression* arrayExpr_, BorrowRef* arrayType_, Expression* indexExpr_, Kind* indexType_, Kind* result_) :
      arrayExpr(arrayExpr_), arrayType(arrayType_), indexExpr(indexExpr_), indexType(indexType_), result(result_) {}
};

class ArraySize : public Expression {
public:
  Expression* array;
  Kind* result;

  ArraySize(Expression* array_, Kind* result_) :
      array(array_), result(result_) {}
};



#endif
