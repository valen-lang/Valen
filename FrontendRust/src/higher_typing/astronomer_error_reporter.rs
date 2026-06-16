use crate::utils::range::RangeS;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::rune_type_solver::RuneTypeSolveError;
use std::any::Any;
use std::collections::HashSet;
// VISTODO: rename


// mig: struct CompileErrorExceptionA
pub struct CompileErrorExceptionA<'s> {
    pub err: ICompileErrorA<'s>,
}

// mig: impl CompileErrorExceptionA
impl<'s> CompileErrorExceptionA<'s> {
// mig: fn equals

// mig: fn hash_code
}

// mig: trait ICompileErrorA
pub enum ICompileErrorA<'s> {
    
    CouldntFindType(CouldntFindTypeA<'s>),
    TooManyMatchingTypes(TooManyMatchingTypesA<'s>),
    CouldntSolveRules(CouldntSolveRulesA<'s>),
    CircularModuleDependency(CircularModuleDependency<'s>),
    WrongNumArgsForTemplate(WrongNumArgsForTemplateA<'s>),
    RangedInternalError(RangedInternalErrorA<'s>),
    
}
impl<'s> ICompileErrorA<'s> {
    pub fn range(&self) -> RangeS<'s> {
        match self {
            ICompileErrorA::CouldntFindType(x) => x.range.clone(),
            ICompileErrorA::TooManyMatchingTypes(x) => x.range.clone(),
            ICompileErrorA::CouldntSolveRules(x) => x.range.clone(),
            ICompileErrorA::CircularModuleDependency(x) => x.range.clone(),
            ICompileErrorA::WrongNumArgsForTemplate(x) => x.range.clone(),
            ICompileErrorA::RangedInternalError(x) => x.range.clone(),
        }
    }
}

// mig: trait ILookupFailedErrorA
pub enum ILookupFailedErrorA<'s> {
    CouldntFindType(CouldntFindTypeA<'s>),
    TooManyMatchingTypes(TooManyMatchingTypesA<'s>),
}
impl<'s> From<ILookupFailedErrorA<'s>> for ICompileErrorA<'s> {
    fn from(e: ILookupFailedErrorA<'s>) -> Self {
        match e {
            ILookupFailedErrorA::CouldntFindType(x) => ICompileErrorA::CouldntFindType(x),
            ILookupFailedErrorA::TooManyMatchingTypes(x) => ICompileErrorA::TooManyMatchingTypes(x),
        }
    }
}

// mig: struct TooManyMatchingTypesA
pub struct TooManyMatchingTypesA<'s> {
    pub range: RangeS<'s>,
    pub name: IImpreciseNameS<'s>,
}

// mig: impl TooManyMatchingTypesA
// mig: fn equals
// mig: fn hash_code


// mig: struct CouldntFindTypeA
pub struct CouldntFindTypeA<'s> {
    pub range: RangeS<'s>,
    pub name: IImpreciseNameS<'s>,
}

// mig: impl CouldntFindTypeA
// mig: fn equals
// mig: fn hash_code


// mig: struct CouldntSolveRulesA
pub struct CouldntSolveRulesA<'s> {
    pub range: RangeS<'s>,
    pub error: RuneTypeSolveError<'s>,
}

// mig: impl CouldntSolveRulesA
// mig: fn equals
// mig: fn hash_code


// mig: struct CircularModuleDependency
pub struct CircularModuleDependency<'s> {
    pub range: RangeS<'s>,
    pub modules: HashSet<String>,
}

// mig: impl CircularModuleDependency
impl<'s> CircularModuleDependency<'s> {}
// mig: struct WrongNumArgsForTemplateA
pub struct WrongNumArgsForTemplateA<'s> {
    pub range: RangeS<'s>,
    pub expected_num_args: i32,
    pub actual_num_args: i32,
}

// mig: impl WrongNumArgsForTemplateA
impl<'s> WrongNumArgsForTemplateA<'s> {}
// mig: struct RangedInternalErrorA
pub struct RangedInternalErrorA<'s> {
    pub range: RangeS<'s>,
    pub message: String,
}

// mig: impl RangedInternalErrorA
impl<'s> RangedInternalErrorA<'s> {}

// mig: fn report
pub fn report<'s>(_err: ICompileErrorA<'s>) -> ! {
    panic!("Unimplemented: report");
}
