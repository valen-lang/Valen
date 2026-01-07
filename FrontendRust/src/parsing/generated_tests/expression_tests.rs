// ExpressionTests
// Mirrors ExpressionTests.scala

#[cfg(test)]
mod expression_tests {
    use crate::parsing::ast::*;
    use crate::lexing::errors::ParseError;
    use crate::parsing::tests::test_parse_utils::*;
    use crate::should_have;

    // Mirrors ExpressionTests.scala line 9
    #[test]
    fn test_simple_int() {
        let expr = compile_expression_expect("4");
        should_have!(expr, IExpressionPE::ConstantInt(ConstantIntPE { range: _, value: 4, bits: None }) => {});
    }

    // Mirrors ExpressionTests.scala line 14
    #[test]
    fn test_simple_bool() {
        let expr = compile_expression_expect("true");
        should_have!(expr, IExpressionPE::ConstantBool(ConstantBoolPE { range: _, value: true }) => {});
    }

    // Mirrors ExpressionTests.scala line 19
    #[test]
    fn test_i64() {
        let expr = compile_expression_expect("4i64");
        should_have!(expr, IExpressionPE::ConstantInt(ConstantIntPE { range: _, value: 4, bits: Some(64) }) => {});
    }

    // Mirrors ExpressionTests.scala line 24
    #[test]
    fn test_binary_operator() {
        let expr = compile_expression_expect("4 + 5");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE { 
            range: _, 
            function_name: NameP { range: _, str: ref name }, 
            left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }), 
            right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. })
        }) if name.str == "+" => {});
    }

    // Mirrors ExpressionTests.scala line 30
    #[test]
    fn test_floats() {
        let expr = compile_expression_expect("4.2");
        should_have!(expr, IExpressionPE::ConstantFloat(ConstantFloatPE { range: _, value }) if (value - 4.2).abs() < 0.001 => {});
    }

    // Mirrors ExpressionTests.scala line 35
    #[test]
    fn test_number_range() {
        let expr = compile_expression_expect("0..5");
        should_have!(expr, IExpressionPE::Range(RangePE { 
            range: _, 
            from_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 0, .. }), 
            to_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. })
        }) => {});
    }

    // Mirrors ExpressionTests.scala line 40
    #[test]
    fn test_add_as_call() {
        let expr = compile_expression_expect("+(4, 5)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref name, .. }), 
                template_args: None 
            }),
            arg_exprs
        }) if name.str == "+" && arg_exprs.len() == 2 => {});
    }

    // Mirrors ExpressionTests.scala line 45
    #[test]
    fn test_passing_equals_equals_overload_set() {
        let expr = compile_expression_expect("moo(4, ==)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: _,
            arg_exprs
        }) if arg_exprs.len() == 2 => {
            should_have!(arg_exprs[1], IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref name, .. }), 
                template_args: None 
            }) if name.str == "==" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 55
    #[test]
    fn test_call_then_binary_operator() {
        let expr = compile_expression_expect("str(i) + 5");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref op_name, .. },
            left_expr: box IExpressionPE::FunctionCall(FunctionCallPE {
                callable_expr: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref func_name, .. }), 
                    .. 
                }), 
                ..
            }),
            right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. })
        }) if op_name.str == "+" && func_name.str == "str" => {});
    }

    // Mirrors ExpressionTests.scala line 67
    #[test]
    fn test_range() {
        let expr = compile_expression_expect("a..b");
        should_have!(expr, IExpressionPE::Range(RangePE {
            range: _,
            from_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref a, .. }), 
                .. 
            }),
            to_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref b, .. }), 
                .. 
            })
        }) if a.str == "a" && b.str == "b" => {});
    }

    // Mirrors ExpressionTests.scala line 72
    #[test]
    fn test_regular_call() {
        let expr = compile_expression_expect("x(y)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                template_args: None 
            }),
            arg_exprs,
            ..
        }) if x.str == "x" && arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref y, .. }), 
                .. 
            }) if y.str == "y" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 77
    #[test]
    fn test_not() {
        let expr = compile_expression_expect("not y");
        should_have!(expr, IExpressionPE::Not(NotPE { 
            range: _, 
            inner: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref y, .. }), 
                .. 
            }) 
        }) if y.str == "y" => {});
    }

    // Mirrors ExpressionTests.scala line 82
    #[test]
    fn test_borrowing_result_of_function_call() {
        let expr = compile_expression_expect("&Muta()");
        should_have!(expr, IExpressionPE::Augment(AugmentPE {
            range: _,
            target_ownership: OwnershipP::Borrow,
            inner: box IExpressionPE::FunctionCall(FunctionCallPE {
                callable_expr: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref name, .. }), 
                    .. 
                }),
                arg_exprs,
                ..
            })
        }) if name.str == "Muta" && arg_exprs.is_empty() => {});
    }

    // Mirrors ExpressionTests.scala line 87
    #[test]
    fn test_specifying_heap() {
        let expr = compile_expression_expect("^Muta()");
        should_have!(expr, IExpressionPE::Augment(AugmentPE {
            range: _,
            target_ownership: OwnershipP::Own,
            inner: box IExpressionPE::FunctionCall(_)
        }) => {});
    }

    // Mirrors ExpressionTests.scala line 92
    // NOTE: Scala test uses Collector.collectFirst to recursively find FunctionCallPE,
    // but Rust test explicitly matches the Augment wrapper that 'inl' creates
    #[test]
    fn test_inline_call_ignored() {
        let expr = compile_expression_expect("inl Muta()");
        should_have!(expr, IExpressionPE::Augment(AugmentPE {
            target_ownership: OwnershipP::Own,
            inner: box IExpressionPE::FunctionCall(FunctionCallPE {
                callable_expr: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref name, .. }), 
                    .. 
                }),
                arg_exprs,
                ..
            }),
            ..
        }) if name.str == "Muta" && arg_exprs.is_empty() => {});
    }

    // Mirrors ExpressionTests.scala line 97
    #[test]
    fn test_method_call() {
        let expr = compile_expression_expect("x . shout ()");
        should_have!(expr, IExpressionPE::MethodCall(MethodCallPE {
            range: _,
            subject_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                .. 
            }),
            operator_range: _,
            method_lookup: box LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref shout, .. }), 
                .. 
            },
            arg_exprs
        }) if x.str == "x" && shout.str == "shout" && arg_exprs.is_empty() => {});
    }

    // Mirrors ExpressionTests.scala line 102
    #[test]
    fn test_mapping_method_call() {
        let expr = compile_expression_expect("x *. shout ()");
        should_have!(
            expr,
            IExpressionPE::MethodCall(
                MethodCallPE {
                    subject_expr: box IExpressionPE::Lookup(LookupPE { 
                        name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                        .. 
                    }),
                    method_lookup: box LookupPE { 
                        name: IImpreciseNameP::LookupName(NameP { str: ref shout, .. }), 
                        .. 
                    },
                    arg_exprs,
                    ..
                }
            ) if x.str == "x" && shout.str == "shout" && arg_exprs.is_empty() => {}
        );
    }

    // Mirrors ExpressionTests.scala line 109
    #[test]
    fn test_method_on_member() {
        let expr = compile_expression_expect("x.moo.shout()");
        should_have!(expr, IExpressionPE::MethodCall(MethodCallPE {
            range: _,
            subject_expr: box IExpressionPE::Dot(DotPE {
                range: _,
                left: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                    .. 
                }),
                operator_range: _,
                member: NameP { str: ref moo, .. }
            }),
            operator_range: _,
            method_lookup: box LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref shout, .. }), 
                .. 
            },
            arg_exprs
        }) if x.str == "x" && moo.str == "moo" && shout.str == "shout" && arg_exprs.is_empty() => {});
    }

    // Mirrors ExpressionTests.scala line 120
    #[test]
    fn test_moving_method_call() {
        let expr = compile_expression_expect("(x ).shout()");
        should_have!(expr, IExpressionPE::MethodCall(MethodCallPE {
            range: _,
            subject_expr: box IExpressionPE::SubExpression(SubExpressionPE {
                range: _,
                inner: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                    .. 
                })
            }),
            operator_range: _,
            method_lookup: box LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref shout, .. }), 
                .. 
            },
            arg_exprs
        }) if x.str == "x" && shout.str == "shout" && arg_exprs.is_empty() => {});
    }

    // Mirrors ExpressionTests.scala line 142
    #[test]
    fn test_templated_function_call() {
        let expr = compile_expression_expect("toArray<imm>( &result)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref name, .. }),
                template_args: Some(TemplateArgsP { 
                    range: _, 
                    args 
                })
            }),
            arg_exprs
        }) if name.str == "toArray" && args.len() == 1 && arg_exprs.len() == 1 => {
            should_have!(args[0], ITemplexPT::Mutability { range: _, mutability: MutabilityP::Immutable } => {});
            should_have!(arg_exprs[0], IExpressionPE::Augment(AugmentPE {
                target_ownership: OwnershipP::Borrow,
                inner: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref result, .. }), 
                    .. 
                }),
                ..
            }) if result.str == "result" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 151
    #[test]
    fn test_templated_method_call() {
        let expr = compile_expression_expect("result.toArray <imm> ()");
        should_have!(expr, IExpressionPE::MethodCall(MethodCallPE {
            range: _,
            subject_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref result, .. }), 
                .. 
            }),
            operator_range: _,
            method_lookup: box LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref to_array, .. }),
                template_args: Some(TemplateArgsP { 
                    range: _, 
                    args 
                })
            },
            arg_exprs
        }) if result.str == "result" && to_array.str == "toArray" && args.len() == 1 && arg_exprs.is_empty() => {
            should_have!(args[0], ITemplexPT::Mutability { range: _, mutability: MutabilityP::Immutable } => {});
        });
    }

    // Mirrors ExpressionTests.scala line 158
    #[test]
    fn test_custom_binaries() {
        let expr = compile_expression_expect("not y florgle not x");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref florgle, .. },
            left_expr: box IExpressionPE::Not(NotPE {
                inner: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref y, .. }), 
                    .. 
                }),
                ..
            }),
            right_expr: box IExpressionPE::Not(NotPE {
                inner: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                    .. 
                }),
                ..
            })
        }) if florgle.str == "florgle" && y.str == "y" && x.str == "x" => {});
    }

    // Mirrors ExpressionTests.scala line 163
    #[test]
    fn test_custom_with_noncustom_binaries() {
        let expr = compile_expression_expect("a + b florgle x * y");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref florgle, .. },
            left_expr: box IExpressionPE::BinaryCall(BinaryCallPE {
                function_name: NameP { str: ref plus, .. },
                left_expr: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref a, .. }), 
                    .. 
                }),
                right_expr: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref b, .. }), 
                    .. 
                }),
                ..
            }),
            right_expr: box IExpressionPE::BinaryCall(BinaryCallPE {
                function_name: NameP { str: ref times, .. },
                left_expr: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                    .. 
                }),
                right_expr: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref y, .. }), 
                    .. 
                }),
                ..
            })
        }) if florgle.str == "florgle" && plus.str == "+" && a.str == "a" && b.str == "b" && times.str == "*" && x.str == "x" && y.str == "y" => {});
    }

    // Mirrors ExpressionTests.scala line 179
    #[test]
    fn test_template_calling() {
        let expr = compile_expression_expect("MyNone< int >()");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref my_none, .. }),
                template_args: Some(TemplateArgsP { 
                    range: _, 
                    args 
                })
            }),
            arg_exprs,
            ..
        }) if my_none.str == "MyNone" && args.len() == 1 && arg_exprs.is_empty() => {
            should_have!(args[0], ITemplexPT::NameOrRune(NameP { str: ref int, .. }) if int.str == "int" => {});
        });

        let expr2 = compile_expression_expect("MySome< MyNone <int> >()");
        should_have!(expr2, IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str: ref my_some, .. }),
                template_args: Some(TemplateArgsP { 
                    range: _, 
                    args 
                })
            }),
            arg_exprs,
            ..
        }) if my_some.str == "MySome" && args.len() == 1 && arg_exprs.is_empty() => {
            should_have!(args[0], ITemplexPT::Call { 
                range: _, 
                template: box ITemplexPT::NameOrRune(NameP { str: ref my_none, .. }), 
                args: ref inner_args 
            } if my_none.str == "MyNone" && inner_args.len() == 1 => {
                should_have!(inner_args[0], ITemplexPT::NameOrRune(NameP { str: ref int, .. }) if int.str == "int" => {});
            });
        });
    }

    // Mirrors ExpressionTests.scala line 190
    #[test]
    fn test_greater_than_or_equal() {
        let expr = compile_expression_expect("9 >= 3");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref op, .. },
            left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 9, .. }),
            right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. })
        }) if op.str == ">=" => {});
    }

    // Mirrors ExpressionTests.scala line 200
    #[test]
    fn test_indexing() {
        let expr = compile_expression_expect("arr [4]");
        should_have!(expr, IExpressionPE::BraceCall(BraceCallPE {
            range: _,
            operator_range: _,
            subject_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref arr, .. }), 
                .. 
            }),
            arg_exprs,
            callable_readwrite: _
        }) if arr.str == "arr" && arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }) => {});
        });
    }

    // Mirrors ExpressionTests.scala line 205
    #[test]
    fn test_single_arg_brace_lambda() {
        let expr = compile_expression_expect("x => { x }");
        should_have!(expr, IExpressionPE::Lambda(LambdaPE {
            captures: None,
            function: FunctionP {
                header: FunctionHeaderP {
                    name: None,
                    attributes: ref attrs,
                    generic_parameters: None,
                    template_rules: None,
                    params: Some(ParamsP { 
                        range: _, 
                        params: ref params 
                    }),
                    ret: FunctionReturnP { range: _, ret_type: None },
                    ..
                },
                body: Some(box BlockPE {
                    inner: box IExpressionPE::Lookup(LookupPE { 
                        name: IImpreciseNameP::LookupName(NameP { str: ref x_inner, .. }), 
                        .. 
                    }),
                    ..
                }),
                ..
            }
        }) if attrs.is_empty() && params.len() == 1 && x_inner.str == "x" => {
            should_have!(params[0], ParameterP {
                pattern: Some(PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x, .. }),
                        ..
                    }),
                    templex: None,
                    destructure: None,
                    ..
                }),
                ..
            } if x.str == "x" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 218
    #[test]
    fn test_single_arg_no_brace_lambda() {
        let expr = compile_expression_expect("x => x");
        should_have!(expr, IExpressionPE::Lambda(LambdaPE {
            captures: None,
            function: FunctionP {
                header: FunctionHeaderP {
                    params: Some(ParamsP { 
                        range: _, 
                        params: ref params 
                    }),
                    ret: FunctionReturnP { ret_type: None, .. },
                    ..
                },
                body: Some(box BlockPE {
                    inner: box IExpressionPE::Lookup(LookupPE { 
                        name: IImpreciseNameP::LookupName(NameP { str: ref x_inner, .. }), 
                        .. 
                    }),
                    ..
                }),
                ..
            }
        }) if params.len() == 1 && x_inner.str == "x" => {
            should_have!(params[0], ParameterP {
                pattern: Some(PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x, .. }),
                        ..
                    }),
                    templex: None,
                    destructure: None,
                    ..
                }),
                ..
            } if x.str == "x" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 231
    #[test]
    fn test_single_arg_typed_brace_lambda() {
        let expr = compile_expression_expect("(x int) => { x }");
        should_have!(expr, IExpressionPE::Lambda(LambdaPE {
            captures: None,
            function: FunctionP {
                header: FunctionHeaderP {
                    params: Some(ParamsP { 
                        range: _, 
                        params: ref params 
                    }),
                    ..
                },
                ..
            }
        }) if params.len() == 1 => {
            should_have!(params[0], ParameterP {
                pattern: Some(PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x, .. }),
                        ..
                    }),
                    templex: Some(ITemplexPT::NameOrRune(NameP { str: ref int, .. })),
                    destructure: None,
                    ..
                }),
                ..
            } if x.str == "x" && int.str == "int" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 244
    #[test]
    fn test_argless_lambda() {
        let expr = compile_expression_expect("{_}");
        should_have!(expr, IExpressionPE::Lambda(LambdaPE {
            captures: None,
            function: FunctionP {
                header: FunctionHeaderP {
                    params: None,
                    ret: FunctionReturnP { ret_type: None, .. },
                    ..
                },
                body: Some(box BlockPE {
                    inner: box IExpressionPE::MagicParamLookup(_),
                    ..
                }),
                ..
            }
        }) => {});
    }

    // Mirrors ExpressionTests.scala line 256
    #[test]
    fn test_multi_arg_typed_brace_lambda() {
        let expr = compile_expression_expect("(x, y) => x");
        should_have!(expr, IExpressionPE::Lambda(LambdaPE {
            captures: None,
            function: FunctionP {
                header: FunctionHeaderP {
                    params: Some(ParamsP { 
                        range: _, 
                        params: ref params 
                    }),
                    ret: FunctionReturnP { ret_type: None, .. },
                    ..
                },
                body: Some(box BlockPE {
                    inner: box IExpressionPE::Lookup(LookupPE { 
                        name: IImpreciseNameP::LookupName(NameP { str: ref x_inner, .. }), 
                        .. 
                    }),
                    ..
                }),
                ..
            }
        }) if params.len() == 2 && x_inner.str == "x" => {
            should_have!(params[0], ParameterP {
                pattern: Some(PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x, .. }),
                        ..
                    }),
                    ..
                }),
                ..
            } if x.str == "x" => {});
            should_have!(params[1], ParameterP {
                pattern: Some(PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref y, .. }),
                        ..
                    }),
                    ..
                }),
                ..
            } if y.str == "y" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 274
    #[test]
    fn test_destructuring_lambda() {
        let expr = compile_expression_expect("([x, y]) => x");
        should_have!(expr, IExpressionPE::Lambda(LambdaPE {
            captures: None,
            function: FunctionP {
                header: FunctionHeaderP {
                    params: Some(ParamsP { 
                        range: _, 
                        params: ref params 
                    }),
                    ret: FunctionReturnP { ret_type: None, .. },
                    ..
                },
                body: Some(box BlockPE {
                    inner: box IExpressionPE::Lookup(LookupPE { 
                        name: IImpreciseNameP::LookupName(NameP { str: ref x_inner, .. }), 
                        .. 
                    }),
                    ..
                }),
                ..
            }
        }) if params.len() == 1 && x_inner.str == "x" => {
            should_have!(params[0], ParameterP {
                pattern: Some(PatternPP {
                    destination: None,
                    templex: None,
                    destructure: Some(DestructureP { range: _, patterns: ref destructure_patterns }),
                    ..
                }),
                ..
            } if destructure_patterns.len() == 2 => {
                should_have!(destructure_patterns[0], PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref x, .. }),
                        ..
                    }),
                    ..
                } if x.str == "x" => {});
                should_have!(destructure_patterns[1], PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref y, .. }),
                        ..
                    }),
                    ..
                } if y.str == "y" => {});
            });
        });
    }

    // Mirrors ExpressionTests.scala line 300
    #[test]
    fn test_dot_symbol() {
        let expr = compile_expression_expect(r#"myPath./("subdir")"#);
        should_have!(expr, IExpressionPE::MethodCall(MethodCallPE {
            range: _,
            subject_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref my_path, .. }), 
                .. 
            }),
            operator_range: _,
            method_lookup: box LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref slash, .. }), 
                .. 
            },
            arg_exprs
        }) if my_path.str == "myPath" && slash.str == "/" && arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::ConstantStr(ConstantStrPE { range: _, value: ref s }) if s == "subdir" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 311
    #[test]
    fn test_not_equal() {
        let expr = compile_expression_expect("3 != 4");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref op, .. },
            left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })
        }) if op.str == "!=" => {});
    }

    // Mirrors ExpressionTests.scala line 318
    #[test]
    fn test_set_call_isnt_interpreted_as_set_expression() {
        let expr = compile_expression_expect("set(true)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref set, .. }), 
                .. 
            }),
            ..
        }) if set.str == "set" => {});
    }

    // Mirrors ExpressionTests.scala line 324
    #[test]
    fn test_2d_array_access() {
        let expr = compile_expression_expect("arr.2.1");
        should_have!(expr, IExpressionPE::Dot(DotPE {
            range: _,
            left: box IExpressionPE::Dot(DotPE {
                range: _,
                left: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref arr, .. }), 
                    .. 
                }),
                operator_range: _,
                member: NameP { str: ref two, .. }
            }),
            operator_range: _,
            member: NameP { str: ref one, .. }
        }) if arr.str == "arr" && two.str == "2" && one.str == "1" => {});
    }

    // Mirrors ExpressionTests.scala line 337
    #[test]
    fn test_lambda_without_surrounding_parens() {
        let expr = compile_expression_expect("{ 0 }()");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: box IExpressionPE::Lambda(LambdaPE { .. }),
            arg_exprs
        }) if arg_exprs.is_empty() => {});
    }

    // Mirrors ExpressionTests.scala line 345
    #[test]
    fn test_function_call() {
        let expr = compile_expression_expect("call(sum)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref call, .. }), 
                template_args: None 
            }),
            arg_exprs
        }) if call.str == "call" && arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref sum, .. }), 
                template_args: None 
            }) if sum.str == "sum" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 355
    #[test]
    fn test_inner_expression_unlet() {
        let expr = compile_expression_expect("destroy(unlet enemy)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref destroy, .. }), 
                template_args: None 
            }),
            arg_exprs
        }) if destroy.str == "destroy" && arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::Unlet(UnletPE { 
                range: _, 
                name: IImpreciseNameP::LookupName(NameP { str: ref enemy, .. }) 
            }) if enemy.str == "enemy" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 363
    #[test]
    fn test_detect_break_in_expr() {
        let result = compile_expression_for_error("a(b, break)");
        should_have!(result, ParseError::CantUseBreakInExpression(_) => {});
    }

    // Mirrors ExpressionTests.scala line 373
    #[test]
    fn test_detect_return_in_expr() {
        let result = compile_expression_for_error("a(b, return)");
        should_have!(result, ParseError::CantUseReturnInExpression(_) => {});
    }

    // Mirrors ExpressionTests.scala line 383
    #[test]
    fn test_parens() {
        let expr = compile_expression_expect("2 * (5 - 7)");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref times, .. },
            left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
            right_expr: box IExpressionPE::SubExpression(SubExpressionPE {
                range: _,
                inner: box IExpressionPE::BinaryCall(BinaryCallPE {
                    function_name: NameP { str: ref minus, .. },
                    left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
                    right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 7, .. }),
                    ..
                })
            })
        }) if times.str == "*" && minus.str == "-" => {});
    }

    // Mirrors ExpressionTests.scala line 388
    #[test]
    fn test_precedence_1() {
        let expr = compile_expression_expect("(5 - 7) * 2");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref times, .. },
            left_expr: box IExpressionPE::SubExpression(SubExpressionPE {
                range: _,
                inner: box IExpressionPE::BinaryCall(BinaryCallPE {
                    function_name: NameP { str: ref minus, .. },
                    left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
                    right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 7, .. }),
                    ..
                })
            }),
            right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. })
        }) if times.str == "*" && minus.str == "-" => {});
    }

    // Mirrors ExpressionTests.scala line 392
    #[test]
    fn test_precedence_2() {
        let expr = compile_expression_expect("5 - 7 * 2");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref minus, .. },
            left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
            right_expr: box IExpressionPE::BinaryCall(BinaryCallPE {
                function_name: NameP { str: ref times, .. },
                left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 7, .. }),
                right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
                ..
            })
        }) if minus.str == "-" && times.str == "*" => {});
    }

    // Mirrors ExpressionTests.scala line 397
    #[test]
    fn test_static_array_from_values() {
        let expr = compile_expression_expect("[#](3, 5, 6)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Mutable }),
            variability_pt: None,
            size: IArraySizeP::StaticSized { size_pt: None },
            initializing_individual_elements: true,
            args
        }) if args.len() == 3 => {});
    }

    // Mirrors ExpressionTests.scala line 406
    #[test]
    fn test_static_array_from_values_with_newlines() {
        let expr = compile_expression_expect("[#](\n3\n)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE { .. }) => {});
    }

    // Mirrors ExpressionTests.scala line 415
    #[test]
    fn test_static_array_from_callable_with_rune() {
        let expr = compile_expression_expect("[#N]({_ * 2})");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Mutable }),
            variability_pt: None,
            size: IArraySizeP::StaticSized { size_pt: Some(ITemplexPT::NameOrRune(NameP { str: ref n, .. })) },
            initializing_individual_elements: false,
            args
        }) if n.str == "N" && args.len() == 1 => {
            should_have!(args[0], IExpressionPE::Lambda(_) => {});
        });
    }

    // Mirrors ExpressionTests.scala line 430
    #[test]
    fn test_less_than_or_equal() {
        let expr = compile_expression_expect("a <= b");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref op, .. },
            left_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref a, .. }), 
                .. 
            }),
            right_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref b, .. }), 
                .. 
            })
        }) if op.str == "<=" && a.str == "a" && b.str == "b" => {});
    }

    // Mirrors ExpressionTests.scala line 437
    #[test]
    fn test_static_array_from_callable() {
        let expr = compile_expression_expect("[#3](triple)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Mutable }),
            variability_pt: None,
            size: IArraySizeP::StaticSized { size_pt: Some(ITemplexPT::Int { range: _, value: 3 }) },
            initializing_individual_elements: false,
            args
        }) if args.len() == 1 => {});
    }

    // Mirrors ExpressionTests.scala line 450
    #[test]
    fn test_immutable_static_array_from_callable() {
        let expr = compile_expression_expect("#[#3](triple)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Immutable }),
            variability_pt: None,
            size: IArraySizeP::StaticSized { size_pt: Some(ITemplexPT::Int { range: _, value: 3 }) },
            initializing_individual_elements: false,
            args
        }) if args.len() == 1 => {});
    }

    // Mirrors ExpressionTests.scala line 463
    #[test]
    fn test_immutable_static_array_from_callable_no_size() {
        let expr = compile_expression_expect("#[#](3, 4, 5)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Immutable }),
            variability_pt: None,
            size: IArraySizeP::StaticSized { size_pt: None },
            initializing_individual_elements: true,
            args
        }) if args.len() == 3 => {});
    }

    // Mirrors ExpressionTests.scala line 476
    #[test]
    fn test_runtime_array_from_callable_with_rune() {
        let expr = compile_expression_expect("[](6, {_ * 2})");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Mutable }),
            variability_pt: None,
            size: IArraySizeP::RuntimeSized,
            initializing_individual_elements: false,
            args
        }) if args.len() == 2 => {});
    }

    // Mirrors ExpressionTests.scala line 491
    #[test]
    fn test_runtime_array_from_callable() {
        let expr = compile_expression_expect("[](6, triple)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Mutable }),
            variability_pt: None,
            size: IArraySizeP::RuntimeSized,
            initializing_individual_elements: false,
            args
        }) if args.len() == 2 => {});
    }

    // Mirrors ExpressionTests.scala line 504
    #[test]
    fn test_double_rsa_with_type() {
        let expr = compile_expression_expect("[][]bool(42)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: Some(ITemplexPT::RuntimeSizedArray {
                range: _,
                mutability: box ITemplexPT::Mutability { range: _, mutability: MutabilityP::Mutable },
                element: box ITemplexPT::NameOrRune(NameP { str: ref bool_, .. })
            }),
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Mutable }),
            variability_pt: None,
            size: IArraySizeP::RuntimeSized,
            initializing_individual_elements: false,
            args
        }) if bool_.str == "bool" && args.len() == 1 => {
            should_have!(args[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 42, .. }) => {});
        });
    }

    // Mirrors ExpressionTests.scala line 518
    #[test]
    fn test_immutable_runtime_array_from_callable() {
        let expr = compile_expression_expect("#[](6, triple)");
        should_have!(expr, IExpressionPE::ConstructArray(ConstructArrayPE {
            range: _,
            type_pt: None,
            mutability_pt: Some(ITemplexPT::Mutability { range: _, mutability: MutabilityP::Immutable }),
            variability_pt: None,
            size: IArraySizeP::RuntimeSized,
            initializing_individual_elements: false,
            args
        }) if args.len() == 2 => {});
    }

    // Mirrors ExpressionTests.scala line 532
    #[test]
    fn test_one_element_tuple() {
        let expr = compile_expression_expect("(3,)");
        should_have!(expr, IExpressionPE::Tuple(TuplePE { range: _, elements }) if elements.len() == 1 => {
            should_have!(elements[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }) => {});
        });
    }

    // Mirrors ExpressionTests.scala line 537
    #[test]
    fn test_zero_element_tuple() {
        let expr = compile_expression_expect("()");
        should_have!(expr, IExpressionPE::Tuple(TuplePE { range: _, elements }) if elements.is_empty() => {});
    }

    // Mirrors ExpressionTests.scala line 542
    #[test]
    fn test_two_element_tuple() {
        let expr = compile_expression_expect("(3,4)");
        should_have!(expr, IExpressionPE::Tuple(TuplePE { range: _, elements }) if elements.len() == 2 => {
            should_have!(elements[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }) => {});
            should_have!(elements[1], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }) => {});
        });
    }

    // Mirrors ExpressionTests.scala line 547
    #[test]
    fn test_three_element_tuple() {
        let expr = compile_expression_expect("(3,4,5)");
        should_have!(expr, IExpressionPE::Tuple(TuplePE { range: _, elements }) if elements.len() == 3 => {
            should_have!(elements[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }) => {});
            should_have!(elements[1], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }) => {});
            should_have!(elements[2], IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }) => {});
        });
    }

    // Mirrors ExpressionTests.scala line 552
    #[test]
    fn test_three_element_tuple_trailing_comma() {
        let expr = compile_expression_expect("(3,4,5,)");
        should_have!(expr, IExpressionPE::Tuple(TuplePE { range: _, elements }) if elements.len() == 3 => {
            should_have!(elements[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }) => {});
            should_have!(elements[1], IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }) => {});
            should_have!(elements[2], IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }) => {});
        });
    }

    // Mirrors ExpressionTests.scala line 557
    #[test]
    fn test_transmigrate() {
        let expr = compile_expression_expect("a'x");
        should_have!(expr, IExpressionPE::Transmigrate(TransmigratePE {
            range: _,
            target_region: NameP { str: ref a, .. },
            inner: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref x, .. }), 
                .. 
            })
        }) if a.str == "a" && x.str == "x" => {});
    }

    // Mirrors ExpressionTests.scala line 563
    #[test]
    fn test_call_callable_expr() {
        let expr = compile_expression_expect("(something.callable)(3)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: box IExpressionPE::SubExpression(SubExpressionPE {
                range: _,
                inner: box IExpressionPE::Dot(DotPE {
                    range: _,
                    left: box IExpressionPE::Lookup(LookupPE { 
                        name: IImpreciseNameP::LookupName(NameP { str: ref something, .. }), 
                        .. 
                    }),
                    operator_range: _,
                    member: NameP { str: ref callable, .. }
                })
            }),
            arg_exprs
        }) if something.str == "something" && callable.str == "callable" && arg_exprs.len() == 1 => {});
    }

    // Mirrors ExpressionTests.scala line 573
    #[test]
    fn test_array_indexing() {
        let expr = compile_expression_expect("board[i]");
        should_have!(expr, IExpressionPE::BraceCall(BraceCallPE {
            range: _,
            operator_range: _,
            subject_expr: box IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref board, .. }), 
                .. 
            }),
            arg_exprs,
            callable_readwrite: false
        }) if board.str == "board" && arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref i, .. }), 
                .. 
            }) if i.str == "i" => {});
        });

        let expr2 = compile_expression_expect("this.board[i]");
        should_have!(expr2, IExpressionPE::BraceCall(BraceCallPE {
            range: _,
            operator_range: _,
            subject_expr: box IExpressionPE::Dot(DotPE {
                range: _,
                left: box IExpressionPE::Lookup(LookupPE { 
                    name: IImpreciseNameP::LookupName(NameP { str: ref this_, .. }), 
                    .. 
                }),
                operator_range: _,
                member: NameP { str: ref board, .. }
            }),
            arg_exprs,
            callable_readwrite: false
        }) if this_.str == "this" && board.str == "board" && arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::Lookup(LookupPE { 
                name: IImpreciseNameP::LookupName(NameP { str: ref i, .. }), 
                .. 
            }) if i.str == "i" => {});
        });
    }

    // Mirrors ExpressionTests.scala line 584
    #[test]
    fn test_mod_and_equals_equals_precedence() {
        let expr = compile_expression_expect("8 mod 2 == 0");
        should_have!(expr, IExpressionPE::BinaryCall(BinaryCallPE {
            range: _,
            function_name: NameP { str: ref equals, .. },
            left_expr: box IExpressionPE::BinaryCall(BinaryCallPE {
                function_name: NameP { str: ref mod_, .. },
                left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 8, .. }),
                right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
                ..
            }),
            right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 0, .. })
        }) if equals.str == "==" && mod_.str == "mod" => {});
    }

    // Mirrors ExpressionTests.scala line 597
    #[test]
    fn test_or_and_equals_equals_precedence() {
        let expr = compile_expression_expect("2 == 0 or false");
        should_have!(expr, IExpressionPE::Or(OrPE {
            range: _,
            left: box IExpressionPE::BinaryCall(BinaryCallPE {
                function_name: NameP { str: ref equals, .. },
                left_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
                right_expr: box IExpressionPE::ConstantInt(ConstantIntPE { value: 0, .. }),
                ..
            }),
            right: box BlockPE {
                inner: box IExpressionPE::ConstantBool(ConstantBoolPE { value: false, .. }),
                ..
            }
        }) if equals.str == "==" => {});
    }

    // Mirrors ExpressionTests.scala line 609
    #[test]
    fn test_templated_lambda_param() {
        let expr = compile_expression_expect("(a => a + a)(3)");
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            range: _,
            operator_range: _,
            callable_expr: box IExpressionPE::SubExpression(SubExpressionPE {
                inner: box IExpressionPE::Lambda(_),
                ..
            }),
            arg_exprs: ref arg_exprs
        }) if arg_exprs.len() == 1 => {
            should_have!(arg_exprs[0], IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }) => {});
        });
        
        // Check pattern
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::SubExpression(SubExpressionPE {
                inner: box IExpressionPE::Lambda(LambdaPE {
                    function: FunctionP {
                        header: FunctionHeaderP {
                            params: Some(ParamsP { params: ref params, .. }),
                            ..
                        },
                        ..
                    },
                    ..
                }),
                ..
            }),
            ..
        }) if params.len() == 1 => {
            should_have!(params[0], ParameterP {
                pattern: Some(PatternPP {
                    destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref a, .. }),
                        ..
                    }),
                    ..
                }),
                ..
            } if a.str == "a" => {});
        });
        
        // Check binary call
        should_have!(expr, IExpressionPE::FunctionCall(FunctionCallPE {
            callable_expr: box IExpressionPE::SubExpression(SubExpressionPE {
                inner: box IExpressionPE::Lambda(LambdaPE {
                    function: FunctionP {
                        body: Some(box BlockPE {
                            inner: box IExpressionPE::BinaryCall(BinaryCallPE {
                                function_name: NameP { str: ref plus, .. },
                                left_expr: box IExpressionPE::Lookup(LookupPE { 
                                    name: IImpreciseNameP::LookupName(NameP { str: ref a1, .. }), 
                                    .. 
                                }),
                                right_expr: box IExpressionPE::Lookup(LookupPE { 
                                    name: IImpreciseNameP::LookupName(NameP { str: ref a2, .. }), 
                                    .. 
                                }),
                                ..
                            }),
                            ..
                        }),
                        ..
                    },
                    ..
                }),
                ..
            }),
            ..
        }) if plus.str == "+" && a1.str == "a" && a2.str == "a" => {});
    }
}
