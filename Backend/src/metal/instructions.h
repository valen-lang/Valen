
#ifndef VALE_INSTRUCTIONS_H_
#define VALE_INSTRUCTIONS_H_

class Expression;
class IRegister;
class ReferenceRegister;
class AddressRegister;
class Local;
class VariableId;
class StackHeight;
class MetalCache;

enum class RefCountCategory {
    VARIABLE_REF_COUNT,
    MEMBER_REF_COUNT,
    REGISTER_REF_COUNT
};

class Expression {
public:
    virtual ~Expression() {}

    // The onion kind this expression evaluates to (mirrors the instantiated IR's ExpressionIE::result).
    // Defined in instructions.cpp. Needs the cache for nodes whose result is a singleton kind
    // (constants, void, never, etc.) rather than a stored field.
    virtual Kind* resultKind(MetalCache* cache) const = 0;
};

class ConstantVoid : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  ConstantVoid() {}
};

class ConstantInt : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  bool value;

  ConstantBool(
      bool value_)
      : value(value_) {}
};



class ConstantStr : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  double value;

  ConstantF64(
      const double &value_) :
      value(value_) {}
};


class Argument : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  Local* variable;
  Kind* result;

  Unstackify(Local* variable_, Kind* result_) :
    variable(variable_),
    result(result_) {}
};


class Destroy : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* expr;
  StructKind* structKind;
  std::vector<Local*> destinationLocals;

  Destroy(
      Expression* expr_,
      StructKind* structKind_,
      std::vector<Local*> destinationLocals_) :
      expr(expr_),
      structKind(structKind_),
      destinationLocals(destinationLocals_) {}
};


class StructToInterfaceUpcast : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* innerExpr;
  InterfaceKind* targetInterface;
  Name* implName;
  Kind* result;

  StructToInterfaceUpcast(
      Expression* innerExpr_,
      InterfaceKind* targetInterface_,
      Name* implName_,
      Kind* result_) :
      innerExpr(innerExpr_),
      targetInterface(targetInterface_),
      implName(implName_),
      result(result_) {}
};

class InterfaceToInterfaceUpcast : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  Expression* left;
  Expression* right;

  IsSameInstance(
      Expression* left_,
      Expression* right_) :
    left(left_),
    right(right_) {}
};

class WeakAlias : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* innerExpr;
  Kind* result;

  WeakAlias(
      Expression* innerExpr_,
      Kind* result_) :
    innerExpr(innerExpr_),
    result(result_) {}
};


class NewArrayFromValues : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  Prototype* superFunctionPrototype;
  int virtualParamIndex;
  std::vector<Expression*> args;
  Kind* result;

  InterfaceCall(
      Prototype* superFunctionPrototype_,
      int virtualParamIndex_,
      std::vector<Expression*> args_,
      Kind* result_) :
    superFunctionPrototype(superFunctionPrototype_),
    virtualParamIndex(virtualParamIndex_),
    args(args_),
    result(result_) {
  }
};


class If : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* condition;
  Expression* thenCall;
  Expression* elseCall;
  Kind* result;

  If(
      Expression* condition_,
      Expression* thenCall_,
      Expression* elseCall_,
      Kind* result_) :
    condition(condition_),
    thenCall(thenCall_),
    elseCall(elseCall_),
    result(result_) {}
};

class While : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* block;
  Kind* result;

  While(Expression* block_, Kind* result_) :
    block(block_),
    result(result_) {}
};

class Consecutor : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  Expression* inner;
  Kind* result;

  Block(Expression* inner_, Kind* result_) :
  inner(inner_),
  result(result_) {}
};

class Break : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
};

class Return : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression *sourceExpr;

  Return(
    Expression *sourceExpr_)
    : sourceExpr(sourceExpr_) {}
};


class NewRuntimeSizedArray : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  Expression* arrayExpr;

  DestroyRuntimeSizedArray(
      Expression* arrayExpr_) :
    arrayExpr(arrayExpr_) {}
};

class NewStruct : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
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
  Kind* resultKind(MetalCache* cache) const override;
  Expression* arrayExpr;

  ArrayLength(
      Expression* arrayExpr_) :
      arrayExpr(arrayExpr_) {}
};

class ArrayCapacity : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* arrayExpr;

  ArrayCapacity(
      Expression* arrayExpr_) :
      arrayExpr(arrayExpr_) {}
};

class PushRuntimeSizedArray : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* arrayExpr;
  Expression* newElementExpr;

  PushRuntimeSizedArray(
      Expression* arrayExpr_,
      Expression* newElementExpr_) :
      arrayExpr(arrayExpr_),
      newElementExpr(newElementExpr_) {}
};

class PopRuntimeSizedArray : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* arrayExpr;
  Kind* result;

  PopRuntimeSizedArray(
      Expression* arrayExpr_,
      Kind* result_) :
      arrayExpr(arrayExpr_),
      result(result_) {}
};


class Discard : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* expr;

  Discard(Expression* expr_) :
      expr(expr_) {}
};

class LockWeak : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* innerExpr;
  Prototype* someConstructor;
  Prototype* noneConstructor;
  Name* someImplName;
  Name* noneImplName;
  Kind* result;

  LockWeak(
      Expression* innerExpr_,
      Prototype* someConstructor_,
      Prototype* noneConstructor_,
      Name* someImplName_,
      Name* noneImplName_,
      Kind* result_) :
    innerExpr(innerExpr_),
    someConstructor(someConstructor_),
    noneConstructor(noneConstructor_),
    someImplName(someImplName_),
    noneImplName(noneImplName_),
    result(result_) {}
};


class AsSubtype : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* sourceExpr;
  Kind* targetType;
  Prototype* okConstructor;
  Prototype* errConstructor;
  Name* implName;
  Name* okImplName;
  Name* errImplName;
  Kind* result;

  AsSubtype(
      Expression* sourceExpr_,
      Kind* targetType_,
      Prototype* okConstructor_,
      Prototype* errConstructor_,
      Name* implName_,
      Name* okImplName_,
      Name* errImplName_,
      Kind* result_) :
    sourceExpr(sourceExpr_),
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
  Kind* resultKind(MetalCache* cache) const override;
    Expression* inner;
    Kind* result;

    CopyPrim(Expression* inner_, Kind* result_) :
        inner(inner_), result(result_) {}
};


class LetAndLend : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Local* variable;
  Expression* expr;
  Kind* result;

  LetAndLend(Local* variable_, Expression* expr_, Kind* result_) :
      variable(variable_), expr(expr_), result(result_) {}
};

class LocalLookup : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Local* localVariable;
  Kind* result;

  LocalLookup(Local* localVariable_, Kind* result_) :
      localVariable(localVariable_), result(result_) {}
};

class Deref : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* inner;
  Kind* result;

  Deref(Expression* inner_, Kind* result_) :
      inner(inner_), result(result_) {}
};

class MemberLookup : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* structExpr;
  std::string memberName;
  Kind* result;

  MemberLookup(Expression* structExpr_, std::string memberName_, Kind* result_) :
      structExpr(structExpr_), memberName(memberName_), result(result_) {}
};

class StaticSizedArrayLookup : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* arrayExpr;
  StaticSizedArrayT* arrayType;
  Expression* indexExpr;
  Kind* result;

  StaticSizedArrayLookup(Expression* arrayExpr_, StaticSizedArrayT* arrayType_, Expression* indexExpr_, Kind* result_) :
      arrayExpr(arrayExpr_), arrayType(arrayType_), indexExpr(indexExpr_), result(result_) {}
};

class RuntimeSizedArrayLookup : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* arrayExpr;
  RuntimeSizedArrayT* arrayType;
  Expression* indexExpr;
  Kind* result;

  RuntimeSizedArrayLookup(Expression* arrayExpr_, RuntimeSizedArrayT* arrayType_, Expression* indexExpr_, Kind* result_) :
      arrayExpr(arrayExpr_), arrayType(arrayType_), indexExpr(indexExpr_), result(result_) {}
};

class Mutate : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* destinationExpr;
  Expression* sourceExpr;
  Kind* result;

  Mutate(Expression* destinationExpr_, Expression* sourceExpr_, Kind* result_) :
      destinationExpr(destinationExpr_), sourceExpr(sourceExpr_), result(result_) {}
};

class ArraySize : public Expression {
public:
  Kind* resultKind(MetalCache* cache) const override;
  Expression* array;
  Kind* result;

  ArraySize(Expression* array_, Kind* result_) :
      array(array_), result(result_) {}
};



#endif
