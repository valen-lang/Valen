use crate::interner::StrI;
use crate::postparsing::names::IRuneS;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::itemplatatype::{
  BooleanTemplataType, ITemplataType, IntegerTemplataType, StringTemplataType,
};
use crate::utils::range::RangeS;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneUsage<'s> {
  pub range: RangeS<'s>,
  pub rune: IRuneS<'s>,
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IRulexSR<'s> {
  Equals(EqualsSR<'s>),
  Literal(LiteralSR<'s>),
  Lookup(LookupSR<'s>),
  Call(CallSR<'s>),
  RuneParentEnvLookup(RuneParentEnvLookupSR<'s>),
  KindList(KindListSR<'s>),
  CallSiteFunc(CallSiteFuncSR<'s>),
  DefinitionFunc(DefinitionFuncSR<'s>),
  Resolve(ResolveSR<'s>),
  BorrowRef(BorrowRefSR<'s>),
  HeapOwnRef(HeapOwnRefSR<'s>),
  ShareRef(ShareRefSR<'s>),
  WeakRef(WeakRefSR<'s>),
}

impl<'s> IRulexSR<'s> {
  pub fn range<'r>(&'r self) -> &'r RangeS<'s> {
    match self {
      IRulexSR::Equals(x) => &x.range,
      IRulexSR::Literal(x) => &x.range,
      IRulexSR::Lookup(x) => &x.range,
      IRulexSR::Call(x) => &x.range,
      IRulexSR::RuneParentEnvLookup(x) => &x.range,
      IRulexSR::KindList(x) => &x.range,
      IRulexSR::CallSiteFunc(x) => &x.range,
      IRulexSR::DefinitionFunc(x) => &x.range,
      IRulexSR::Resolve(x) => &x.range,
      IRulexSR::BorrowRef(x) => &x.range,
      IRulexSR::HeapOwnRef(x) => &x.range,
      IRulexSR::ShareRef(x) => &x.range,
      IRulexSR::WeakRef(x) => &x.range,
    }
  }


  pub fn rune_usages<'r>(&'r self) -> Vec<RuneUsage<'s>> {
    match self {
      IRulexSR::Equals(x) => vec![x.left.clone(), x.right.clone()],
      IRulexSR::Literal(x) => vec![x.rune.clone()],
      IRulexSR::Lookup(x) => vec![x.rune.clone()],
      IRulexSR::Call(x) => {
        let mut usages = vec![x.result_rune.clone(), x.template_rune.clone()];
        usages.extend(x.args.iter().cloned());
        usages
      }
      IRulexSR::RuneParentEnvLookup(x) => vec![x.rune.clone()],
      IRulexSR::KindList(x) => {
        let mut usages = vec![x.result_rune.clone()];
        usages.extend(x.members.iter().cloned());
        usages
      }
      IRulexSR::CallSiteFunc(x) => vec![x.prototype_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()],
      IRulexSR::DefinitionFunc(x) => vec![x.result_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()],
      IRulexSR::Resolve(x) => vec![x.result_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()],
      IRulexSR::BorrowRef(x) => {
        let mut usages = vec![x.result_rune.clone(), x.inner_rune.clone()];
        if let Some(region_rune) = &x.region_rune {
          usages.push(region_rune.clone());
        }
        usages
      }
      IRulexSR::HeapOwnRef(x) => vec![x.result_rune.clone(), x.inner_rune.clone()],
      IRulexSR::ShareRef(x) => vec![x.result_rune.clone(), x.inner_rune.clone()],
      IRulexSR::WeakRef(x) => vec![x.result_rune.clone(), x.inner_rune.clone()],
    }
  }

}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EqualsSR<'s> {
  pub range: RangeS<'s>,
  pub left: RuneUsage<'s>,
  pub right: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResolveSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub name: StrI<'s>,
  pub params_list_rune: RuneUsage<'s>,
  pub return_rune: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CallSiteFuncSR<'s> {
  pub range: RangeS<'s>,
  pub prototype_rune: RuneUsage<'s>,
  pub name: StrI<'s>,
  pub params_list_rune: RuneUsage<'s>,
  pub return_rune: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DefinitionFuncSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub name: StrI<'s>,
  pub params_list_rune: RuneUsage<'s>,
  pub return_rune: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiteralSR<'s> {
  pub range: RangeS<'s>,
  pub rune: RuneUsage<'s>,
  pub literal: ILiteralSL<'s>,
}


// A rule that looks up something by name in the enclosing environment.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LookupSR<'s> {
  pub range: RangeS<'s>,
  pub rune: RuneUsage<'s>,
  pub name: IImpreciseNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CallSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub template_rune: RuneUsage<'s>,
  pub args: &'s [RuneUsage<'s>],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RuneParentEnvLookupSR<'s> {
  pub range: RangeS<'s>,
  pub rune: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowRefSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub inner_rune: RuneUsage<'s>,
  pub region_rune: Option<RuneUsage<'s>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HeapOwnRefSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub inner_rune: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShareRefSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub inner_rune: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeakRefSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub inner_rune: RuneUsage<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KindListSR<'s> {
  pub range: RangeS<'s>,
  pub result_rune: RuneUsage<'s>,
  pub members: &'s [RuneUsage<'s>],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ILiteralSL<'s> {
  IntLiteral(IntLiteralSL),
  StringLiteral(StringLiteralSL<'s>),
  BoolLiteral(BoolLiteralSL),
}

impl<'s> ILiteralSL<'s> {
  pub fn get_type<'a>(&self) -> ITemplataType<'a> {
    match self {
      ILiteralSL::IntLiteral(x) => x.get_type(),
      ILiteralSL::StringLiteral(x) => x.get_type(),
      ILiteralSL::BoolLiteral(x) => x.get_type(),
    }
  }

}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntLiteralSL {
  pub value: i64,
}


impl IntLiteralSL {
  pub fn get_type<'a>(&self) -> ITemplataType<'a> {
    ITemplataType::IntegerTemplataType(IntegerTemplataType {})
  }

}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StringLiteralSL<'s> {
  pub value: StrI<'s>,
}


impl<'s> StringLiteralSL<'s> {
  pub fn get_type<'a>(&self) -> ITemplataType<'a> {
    ITemplataType::StringTemplataType(StringTemplataType {})
  }

}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoolLiteralSL {
  pub value: bool,
}


impl BoolLiteralSL {
  pub fn get_type<'a>(&self) -> ITemplataType<'a> {
    ITemplataType::BooleanTemplataType(BooleanTemplataType {})
  }

}
