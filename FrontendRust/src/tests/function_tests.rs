/// Function parsing tests
/// Mirrors Frontend/ParsingPass/test/dev/vale/parsing/functions/FunctionTests.scala

use crate::tests::test_parse_utils::*;
use crate::parsing::ast::*;
use crate::lexing::errors::ParseError;
use crate::{should_have};

// Mirrors FunctionTests.scala line 12
#[test]
fn test_simple_function() {
    let file = compile_file_expect("func main() { }");
    assert_eq!(file.denizens.len(), 1);
    
    should_have!(&file.denizens[0], IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: None, .. },
            ..
        },
        body: Some(box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        }),
        ..
    }) if str.str == "main" && params.is_empty() => {});
}

// Mirrors FunctionTests.scala line 24
#[test]
fn test_functions_with_weird_names() {
    let file1 = compile_file_expect("func !=() { }");
    assert_eq!(file1.denizens.len(), 1);
    
    let file2 = compile_file_expect("func <=() { }");
    assert_eq!(file2.denizens.len(), 1);
    
    let file3 = compile_file_expect("func >=() { }");
    assert_eq!(file3.denizens.len(), 1);
    
    let file4 = compile_file_expect("func <() { }");
    assert_eq!(file4.denizens.len(), 1);
    
    let file5 = compile_file_expect("func >() { }");
    assert_eq!(file5.denizens.len(), 1);
    
    let file6 = compile_file_expect("func ==() { }");
    assert_eq!(file6.denizens.len(), 1);
}

// Mirrors FunctionTests.scala line 33
#[test]
fn test_function_then_struct() {
    let file = compile_file_expect(
        r#"
        exported func main() int {}
        
        struct mork { }
        "#
    );
    
    assert_eq!(file.denizens.len(), 2);
    assert!(matches!(file.denizens[0], IDenizenP::TopLevelFunction(_)));
    assert!(matches!(file.denizens[1], IDenizenP::TopLevelStruct(_)));
}

// Mirrors FunctionTests.scala line 45
#[test]
fn test_simple_function_with_return() {
    let denizen = compile_denizen_expect("func sum() int {3}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: Some(_), .. },
            ..
        },
        body: Some(box BlockPE {
            inner: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            ..
        }),
        ..
    }) if str.str == "sum" && params.is_empty() => {});
}

// Mirrors FunctionTests.scala line 54
#[test]
fn test_pure_function() {
    let denizen = compile_denizen_expect("pure func sum() {3}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: None, .. },
            ..
        },
        body: Some(box BlockPE {
            inner: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            ..
        }),
        ..
    }) if str.str == "sum" && params.is_empty() && attrs.iter().any(|a| matches!(a, IAttributeP::PureAttribute(_))) => {});
}
// Mirrors FunctionTests.scala line 63
#[test]
fn test_extern_function() {
    let file = compile_file_expect("extern func sum();");
    assert_eq!(file.denizens.len(), 1);
    
    should_have!(&file.denizens[0], IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: None, .. },
            ..
        },
        body: None,
        ..
    }) if str.str == "sum" && params.is_empty() && attrs.iter().any(|a| matches!(a, IAttributeP::ExternAttribute(_))) => {});
}

// Mirrors FunctionTests.scala line 72
#[test]
fn test_function_ending_with_set() {
    compile_denizen_expect(
        r#"
        func moo() {
          set bork = value
        }
        "#
    );
}

// Mirrors FunctionTests.scala line 81
#[test]
fn test_extern_function_generated() {
    let file = compile_file_expect(r#"extern("bork") func sum();"#);
    assert_eq!(file.denizens.len(), 1);
    
    should_have!(&file.denizens[0], IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str: ref func_str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: None, .. },
            ..
        },
        body: None,
        ..
    }) if func_str.str == "sum" && params.is_empty() && attrs.iter().any(|a| {
        matches!(a, IAttributeP::BuiltinAttribute { generator_name: NameP { str, .. }, .. } if str.str == "bork")
    }) => {});
}

// Mirrors FunctionTests.scala line 90
#[test]
fn test_extern_function_with_return() {
    let file = compile_file_expect("extern func sum() int;");
    assert_eq!(file.denizens.len(), 1);
    
    should_have!(&file.denizens[0], IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: Some(ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. })), .. },
            ..
        },
        body: None,
        ..
    }) if str.str == "sum" && params.is_empty() && ret_str.str == "int" && attrs.iter().any(|a| matches!(a, IAttributeP::ExternAttribute(_))) => {});
}

// Mirrors FunctionTests.scala line 99
#[test]
fn test_abstract_function() {
    let denizen = compile_denizen_expect("abstract func sum();");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: None, .. },
            ..
        },
        body: None,
        ..
    }) if str.str == "sum" && params.is_empty() && attrs.iter().any(|a| matches!(a, IAttributeP::AbstractAttribute(_))) => {});
}
// Mirrors FunctionTests.scala line 108
#[test]
fn test_pure_and_default_region() {
    let denizen = compile_denizen_expect(r#"pure func findNearbyUnits() i'int i'{ }"#);
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { 
                ret_type: Some(ITemplexPT::Interpreted { 
                    maybe_region: Some(box RegionRunePT { name: Some(NameP { str: ref region_str, .. }), .. }),
                    inner: box ITemplexPT::NameOrRune(NameP { str: ref type_str, .. }),
                    ..
                }), 
                .. 
            },
            ..
        },
        body: Some(box BlockPE {
            maybe_default_region: Some(RegionRunePT { name: Some(NameP { str: ref body_region_str, .. }), .. }),
            inner: box IExpressionPE::Void(_),
            ..
        }),
        ..
    }) if str.str == "findNearbyUnits" && params.is_empty() && 
         attrs.iter().any(|a| matches!(a, IAttributeP::PureAttribute(_))) &&
         region_str.str == "i" && type_str.str == "int" && body_region_str.str == "i" => {});
}

// Mirrors FunctionTests.scala line 121
#[test]
fn test_return_isolate() {
    let denizen = compile_denizen_expect(r#"func findNearbyUnits() 'int { }"#);
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { 
                ret_type: Some(ITemplexPT::Interpreted { 
                    maybe_region: Some(box RegionRunePT { name: None, .. }),
                    inner: box ITemplexPT::NameOrRune(NameP { str: ref type_str, .. }),
                    ..
                }), 
                .. 
            },
            ..
        },
        ..
    }) if str.str == "findNearbyUnits" && params.is_empty() && type_str.str == "int" => {});
}

// Mirrors FunctionTests.scala line 134
#[test]
fn test_coord_generic_with_associated_region() {
    let denizen = compile_denizen_expect(r#"func findNearbyUnits<t', t'T>(x T) { }"#);
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if str.str == "findNearbyUnits" && gen_params.len() == 2 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str: ref name0, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             ..
         } if name0.str == "t") &&
         matches!(&gen_params[1], GenericParameterP { 
             name: NameP { str: ref name1, .. }, 
             coord_region: Some(RegionRunePT { name: Some(NameP { str: ref region_str, .. }), .. }),
             ..
         } if name1.str == "T" && region_str.str == "t") => {});
}

// Mirrors FunctionTests.scala line 151
#[test]
fn test_attribute_after_return() {
    let denizen = compile_denizen_expect("abstract func sum() int;");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: Some(ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. })), .. },
            ..
        },
        body: None,
        ..
    }) if str.str == "sum" && params.is_empty() && ret_str.str == "int" && 
         attrs.iter().any(|a| matches!(a, IAttributeP::AbstractAttribute(_))) => {});
}

// Mirrors FunctionTests.scala line 164
#[test]
fn test_attribute_before_return() {
    let denizen = compile_denizen_expect("abstract func sum() Int;");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            attributes: ref attrs,
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: Some(ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. })), .. },
            ..
        },
        body: None,
        ..
    }) if str.str == "sum" && params.is_empty() && ret_str.str == "Int" && 
         attrs.iter().any(|a| matches!(a, IAttributeP::AbstractAttribute(_))) => {});
}
// Mirrors FunctionTests.scala line 177
#[test]
fn test_simple_function_with_identifying_rune() {
    let denizen = compile_denizen_expect("func sum<A>(a A){a}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: None,
             coord_region: None,
             ..
         } if str.str == "A") => {});
}

// Mirrors FunctionTests.scala line 185
#[test]
fn test_simple_function_with_coord_typed_identifying_rune() {
    let denizen = compile_denizen_expect("func sum<A Ref>(a A){a}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::CoordType, .. }),
             ..
         } if str.str == "A") => {});
}

// Mirrors FunctionTests.scala line 193
#[test]
fn test_simple_function_with_region_typed_identifying_rune() {
    let denizen = compile_denizen_expect("func sum<a'>(){}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             ..
         } if str.str == "a") => {});
}

// Mirrors FunctionTests.scala line 201
#[test]
fn test_readonly_region_rune() {
    let denizen = compile_denizen_expect("func sum<r' ro>(){}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             attributes: ref attrs,
             ..
         } if str.str == "r" && attrs.iter().any(|a| matches!(a, IRuneAttributeP::ReadOnlyRegionRuneAttribute(_)))) => {});
}

// Mirrors FunctionTests.scala line 209
#[test]
fn test_simple_function_with_apostrophe_region_typed_identifying_rune() {
    let denizen = compile_denizen_expect("func sum<r'>(a &r'Marine){a}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             ..
         } if str.str == "r") => {});
}
// Mirrors FunctionTests.scala line 217
#[test]
fn test_pool_region() {
    let denizen = compile_denizen_expect("func sum<r' = pool>(a &r'Marine){a}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             maybe_default: Some(ITemplexPT::NameOrRune(NameP { str: ref default_str, .. })),
             ..
         } if str.str == "r" && default_str.str == "pool") => {});
}

// Mirrors FunctionTests.scala line 230
#[test]
fn test_pool_readonly_region() {
    let denizen = compile_denizen_expect("func sum<r' ro = pool>(a &r'Marine){a}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             attributes: ref attrs,
             maybe_default: Some(ITemplexPT::NameOrRune(NameP { str: ref default_str, .. })),
             ..
         } if str.str == "r" && default_str.str == "pool" && 
              attrs.iter().any(|a| matches!(a, IRuneAttributeP::ReadOnlyRegionRuneAttribute(_)))) => {});
}

// Mirrors FunctionTests.scala line 243
#[test]
fn test_arena_region() {
    let denizen = compile_denizen_expect("func sum<x' = arena>(a &x'Marine){a}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             maybe_default: Some(ITemplexPT::NameOrRune(NameP { str: ref default_str, .. })),
             ..
         } if str.str == "x" && default_str.str == "arena") => {});
}

// Mirrors FunctionTests.scala line 257
#[test]
fn test_readonly_region() {
    let denizen = compile_denizen_expect("func sum<x'>(a &x'Marine){a}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str, .. }, 
             maybe_type: Some(GenericParameterTypeP { tyype: ITypePR::RegionType, .. }),
             maybe_default: None,
             ..
         } if str.str == "x") => {});
}

// Mirrors FunctionTests.scala line 270
#[test]
fn test_virtual_function() {
    let denizen = compile_denizen_expect("func doCivicDance(virtual this Car) int;");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: Some(ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. })), .. },
            ..
        },
        body: None,
        ..
    }) if str.str == "doCivicDance" && ret_str.str == "int" && params.len() == 1 &&
         matches!(&params[0], ParameterP {
             virtuality: Some(AbstractP { .. }),
             pattern: Some(PatternPP {
                 destination: Some(DestinationLocalP { decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref param_name, .. }), .. }),
                 templex: Some(ITemplexPT::NameOrRune(NameP { str: ref type_name, .. })),
                 ..
             }),
             ..
         } if param_name.str == "this" && type_name.str == "Car") => {});
}

// Mirrors FunctionTests.scala line 290
#[test]
fn test_bad_thing_for_body() {
    let result = compile_denizen(
        r#"
        func doCivicDance(virtual this Car) moo blork
        "#
    );
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(matches!(err, ParseError::BadFunctionBodyError(_)));
    }
}
// Mirrors FunctionTests.scala line 300
#[test]
fn test_function_with_parameter_and_return() {
    let file = compile_file_expect("func main(moo T) T { }");
    assert_eq!(file.denizens.len(), 1);
    
    should_have!(&file.denizens[0], IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: Some(ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. })), .. },
            ..
        },
        body: Some(box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        }),
        ..
    }) if str.str == "main" && ret_str.str == "T" && params.len() == 1 &&
         matches!(&params[0].pattern, Some(PatternPP {
             destination: Some(DestinationLocalP { decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref param_name, .. }), .. }),
             templex: Some(ITemplexPT::NameOrRune(NameP { str: ref param_type, .. })),
             ..
         }) if param_name.str == "moo" && param_type.str == "T") => {});
}

// Mirrors FunctionTests.scala line 312
#[test]
fn test_function_with_generics() {
    let file = compile_file_expect("func main<T>() { }");
    assert_eq!(file.denizens.len(), 1);
    
    should_have!(&file.denizens[0], IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            ..
        },
        ..
    }) if str.str == "main" && gen_params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { 
             name: NameP { str: ref gen_name, .. },
             maybe_type: None,
             coord_region: None,
             ..
         } if gen_name.str == "T") => {});
}

// Mirrors FunctionTests.scala line 327
#[test]
fn test_impl_function() {
    let denizen = compile_denizen_expect("func maxHp(virtual this Marine) { return 5; }");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: None, .. },
            ..
        },
        body: Some(box BlockPE { .. }),
        ..
    }) if str.str == "maxHp" && params.len() == 1 &&
         matches!(&params[0], ParameterP {
             virtuality: Some(AbstractP { .. }),
             pattern: Some(PatternPP {
                 destination: Some(DestinationLocalP { decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref param_name, .. }), .. }),
                 templex: Some(ITemplexPT::NameOrRune(NameP { str: ref type_name, .. })),
                 ..
             }),
             ..
         } if param_name.str == "this" && type_name.str == "Marine") => {});
}

// Mirrors FunctionTests.scala line 351
#[test]
fn test_param() {
    let denizen = compile_denizen_expect("func call(f F){f()}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            params: Some(ParamsP { params: ref params, .. }),
            ..
        },
        ..
    }) if params.len() == 1 &&
         matches!(&params[0].pattern, Some(PatternPP {
             destination: Some(DestinationLocalP { decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref param_name, .. }), .. }),
             templex: Some(ITemplexPT::NameOrRune(NameP { str: ref type_name, .. })),
             ..
         }) if param_name.str == "f" && type_name.str == "F") => {});
}

// Mirrors FunctionTests.scala line 358
#[test]
fn test_func_with_rules() {
    let denizen = compile_denizen_expect("func sum () where X Int {3}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            template_rules: Some(_),
            ..
        },
        body: Some(box BlockPE {
            inner: box IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            ..
        }),
        ..
    }) if str.str == "sum" => {});
}

// Mirrors FunctionTests.scala line 368
#[test]
fn test_func_with_func_bound() {
    let denizen = compile_denizen_expect("func sum<T>() where func moo(&T)void {3}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            template_rules: Some(TemplateRulesP { rules: ref rules, .. }),
            ..
        },
        ..
    }) if rules.len() == 1 &&
         matches!(&rules[0], IRulexPR::Templex(ITemplexPT::Func {
             name: NameP { str: ref func_name, .. },
             parameters: ref params,
             return_type: box ITemplexPT::NameOrRune(NameP { str: ref ret_str, .. }),
             ..
         }) if func_name.str == "moo" && ret_str.str == "void" && params.len() == 1) => {});
}
// Mirrors FunctionTests.scala line 389
#[test]
fn test_identifying_runes() {
    let denizen = compile_denizen_expect("func wrap<A, F>(a A) { }");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            name: Some(NameP { str, .. }),
            generic_parameters: Some(GenericParametersP { params: ref gen_params, .. }),
            params: Some(ParamsP { params: ref params, .. }),
            ret: FunctionReturnP { ret_type: None, .. },
            ..
        },
        body: Some(box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        }),
        ..
    }) if str.str == "wrap" && gen_params.len() == 2 && params.len() == 1 &&
         matches!(&gen_params[0], GenericParameterP { name: NameP { str: ref name0, .. }, .. } if name0.str == "A") &&
         matches!(&gen_params[1], GenericParameterP { name: NameP { str: ref name1, .. }, .. } if name1.str == "F") &&
         matches!(&params[0].pattern, Some(PatternPP {
             destination: Some(DestinationLocalP { decl: INameDeclarationP::LocalNameDeclaration(NameP { str: ref param_name, .. }), .. }),
             templex: Some(ITemplexPT::NameOrRune(NameP { str: ref param_type, .. })),
             ..
         }) if param_name.str == "a" && param_type.str == "A") => {});
}

// Mirrors FunctionTests.scala line 407
#[test]
fn test_never_signature() {
    // This test is here because we were parsing the first _ of __Never as an anonymous
    // rune then stopping.
    let denizen = compile_denizen_expect("func __vbi_panic() __Never {}");
    
    should_have!(denizen, IDenizenP::TopLevelFunction(FunctionP {
        header: FunctionHeaderP {
            ret: FunctionReturnP { 
                ret_type: Some(ITemplexPT::NameOrRune(NameP { str, .. })), 
                .. 
            },
            ..
        },
        ..
    }) if str.str == "__Never" => {});
}

// Mirrors FunctionTests.scala line 416
#[test]
fn test_should_require_identifying_runes() {
    let result = compile_denizen(
        r#"
        func do(callable) int {callable()}
        "#
    );
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(matches!(err, ParseError::LightFunctionMustHaveParamTypes { param_index: 0, .. }));
    }
}

// Mirrors FunctionTests.scala line 427
#[test]
fn test_short_self() {
    let denizen = compile_denizen_expect(
        r#"
        interface IMoo {
          func moo(&self) {}
        }
        "#
    );
    
    should_have!(denizen, IDenizenP::TopLevelInterface(InterfaceP {
        name: NameP { str: ref interface_name, .. },
        members: ref members,
        ..
    }) if interface_name.str == "IMoo" && members.len() == 1 &&
         matches!(&members[0], FunctionP {
             header: FunctionHeaderP {
                 name: Some(NameP { str: ref func_name, .. }),
                 params: Some(ParamsP { params: ref params, .. }),
                 ..
             },
             ..
         } if func_name.str == "moo" && params.len() == 1 &&
              matches!(&params[0], ParameterP {
                  virtuality: None,
                  self_borrow: Some(_),
                  ..
              })) => {});
}

// === AfterRegionsFunctionTests.scala ===
// Mirrors AfterRegionsFunctionTests.scala line 212
// NOTE: This test is also not passing in Scala (marked with comment there).
#[test]
#[ignore] // Known issue: parser doesn't validate that func bounds must be in where clause
fn test_func_with_func_bound_with_missing_where() {
    // It parses that func moo as a templex, and apparently a return can be a templex
    let result = compile_denizen("func sum<T>() func moo(&T)void {3}");
    
    assert!(result.is_err());
}

