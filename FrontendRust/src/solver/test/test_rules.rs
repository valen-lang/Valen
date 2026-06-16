
// mig: trait IRule
pub trait IRule {
    fn all_runes(&self) -> Vec<i64>;
    fn all_puzzles(&self) -> Vec<Vec<i64>>;
}
#[derive(Clone, Debug)]
pub enum TestRule {
    Lookup(Lookup),
    Literal(Literal),
    Equals(Equals),
    CoordComponents(CoordComponents),
    OneOf(OneOf),
    Call(Call),
    Send(Send),
    Implements(Implements),
    Pack(Pack),
}

impl TestRule {
    pub fn all_runes(&self) -> Vec<i64> {
        match self {
            TestRule::Lookup(x) => x.all_runes(),
            TestRule::Literal(x) => x.all_runes(),
            TestRule::Equals(x) => x.all_runes(),
            TestRule::CoordComponents(x) => x.all_runes(),
            TestRule::OneOf(x) => x.all_runes(),
            TestRule::Call(x) => x.all_runes(),
            TestRule::Send(x) => x.all_runes(),
            TestRule::Implements(x) => x.all_runes(),
            TestRule::Pack(x) => x.all_runes(),
        }
    }

    pub fn all_puzzles(&self) -> Vec<Vec<i64>> {
        match self {
            TestRule::Lookup(x) => x.all_puzzles(),
            TestRule::Literal(x) => x.all_puzzles(),
            TestRule::Equals(x) => x.all_puzzles(),
            TestRule::CoordComponents(x) => x.all_puzzles(),
            TestRule::OneOf(x) => x.all_puzzles(),
            TestRule::Call(x) => x.all_puzzles(),
            TestRule::Send(x) => x.all_puzzles(),
            TestRule::Implements(x) => x.all_puzzles(),
            TestRule::Pack(x) => x.all_puzzles(),
        }
    }
}

// mig: struct Lookup
#[derive(Clone, Debug)]
pub struct Lookup {
    pub rune: i64,
    pub name: String,
}
// mig: impl Lookup
impl IRule for Lookup {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![vec![]]
    }
}

// mig: struct Literal
#[derive(Clone, Debug)]
pub struct Literal {
    pub rune: i64,
    pub value: String,
}
// mig: impl Literal
impl IRule for Literal {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![vec![]]
    }
}

// mig: struct Equals
#[derive(Clone, Debug)]
pub struct Equals {
    pub left_rune: i64,
    pub right_rune: i64,
}
// mig: impl Equals
impl IRule for Equals {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.left_rune, self.right_rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![vec![self.left_rune], vec![self.right_rune]]
    }
}

// mig: struct CoordComponents
#[derive(Clone, Debug)]
pub struct CoordComponents {
    pub coord_rune: i64,
    pub ownership_rune: i64,
    pub kind_rune: i64,
}
// mig: impl CoordComponents
impl IRule for CoordComponents {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.coord_rune, self.ownership_rune, self.kind_rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![
            vec![self.coord_rune],
            vec![self.ownership_rune, self.kind_rune],
        ]
    }
}

// mig: struct OneOf
#[derive(Clone, Debug)]
pub struct OneOf {
    pub coord_rune: i64,
    pub possible_values: Vec<String>,
}
// mig: impl OneOf
impl IRule for OneOf {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.coord_rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![vec![self.coord_rune]]
    }
}

// mig: struct Call
#[derive(Clone, Debug)]
pub struct Call {
    pub result_rune: i64,
    pub name_rune: i64,
    pub arg_rune: i64,
}
// mig: impl Call
impl IRule for Call {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.result_rune, self.name_rune, self.arg_rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![
            vec![self.result_rune, self.name_rune],
            vec![self.name_rune, self.arg_rune],
        ]
    }
}

// mig: struct Send
#[derive(Clone, Debug)]
pub struct Send {
    pub sender_rune: i64,
    pub receiver_rune: i64,
}
// mig: impl Send
impl IRule for Send {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.receiver_rune, self.sender_rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![vec![self.receiver_rune]]
    }
}

// mig: struct Implements
#[derive(Clone, Debug)]
pub struct Implements {
    pub sub_rune: i64,
    pub super_rune: i64,
}
// mig: impl Implements
impl IRule for Implements {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.sub_rune, self.super_rune]
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        vec![vec![self.sub_rune, self.super_rune]]
    }
}

// mig: struct Pack
#[derive(Clone, Debug)]
pub struct Pack {
    pub result_rune: i64,
    pub member_runes: Vec<i64>,
}
// mig: impl Pack
impl IRule for Pack {
    fn all_runes(&self) -> Vec<i64> {
        vec![self.result_rune]
            .into_iter()
            .chain(self.member_runes.iter().cloned())
            .collect()
    }
    fn all_puzzles(&self) -> Vec<Vec<i64>> {
        if self.member_runes.is_empty() {
            vec![vec![self.result_rune]]
        } else {
            vec![
                vec![self.result_rune],
                self.member_runes.clone(),
            ]
        }
    }
}

