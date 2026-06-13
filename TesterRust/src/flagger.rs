// Emulation of Vale's `stdlib.flagger` — the Flag/parse_all_flags/get_*_flag
// API that main.vale imports as `import stdlib.flagger.*;`. None of this has a
// direct counterpart in Tester/src/main.vale (which just uses the stdlib);
// it's reproduced here so the call sites in main.rs read 1:1 with main.vale.

use std::collections::HashMap;

#[allow(non_camel_case_types)]
pub enum FlagType {
    Str,
    Int,
    Bool,
}
// no main.vale counterpart (stdlib.flagger emulation).

#[allow(non_snake_case)]
pub fn FLAG_STR() -> FlagType { FlagType::Str }
// no main.vale counterpart (stdlib.flagger emulation).

#[allow(non_snake_case)]
pub fn FLAG_INT() -> FlagType { FlagType::Int }
// no main.vale counterpart (stdlib.flagger emulation).

#[allow(non_snake_case)]
pub fn FLAG_BOOL() -> FlagType { FlagType::Bool }
// no main.vale counterpart (stdlib.flagger emulation).

pub struct Flag {
    pub name: String,
    #[allow(dead_code)]
    pub kind: FlagType,
    #[allow(dead_code)]
    pub summary: String,
    #[allow(dead_code)]
    pub default: String,
    #[allow(dead_code)]
    pub description: String,
}
// no main.vale counterpart (stdlib.flagger emulation).

#[allow(non_snake_case)]
pub fn Flag(
    name: &str,
    kind: FlagType,
    summary: &str,
    default: &str,
    description: &str,
) -> Flag {
    Flag {
        name: name.to_string(),
        kind,
        summary: summary.to_string(),
        default: default.to_string(),
        description: description.to_string(),
    }
}
// no main.vale counterpart (stdlib.flagger emulation).

pub struct ParsedFlags {
    values: HashMap<String, String>,
    pub unrecognized_inputs: Vec<String>,
}
// no main.vale counterpart (stdlib.flagger emulation).

impl ParsedFlags {
    pub fn get_bool_flag(&self, name: &str, default: bool) -> bool {
        match self.values.get(name) {
            Some(v) => match v.as_str() {
                "true" | "1" => true,
                "false" | "0" => false,
                _ => default,
            },
            None => default,
        }
    }
    // no main.vale counterpart (stdlib.flagger emulation).

    pub fn get_int_flag(&self, name: &str, default: i64) -> i64 {
        match self.values.get(name) {
            Some(v) => v.parse::<i64>().unwrap_or(default),
            None => default,
        }
    }
    // no main.vale counterpart (stdlib.flagger emulation).

    pub fn get_string_flag(&self, name: &str) -> Option<String> {
        self.values.get(name).cloned()
    }
    // no main.vale counterpart (stdlib.flagger emulation).

    pub fn expect_string_flag(&self, _flags: &[Flag], name: &str) -> String {
        self.values
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("missing required flag {}", name))
    }
    // no main.vale counterpart (stdlib.flagger emulation).
}

pub fn parse_all_flags(flags: &[Flag], all_args: &[String]) -> ParsedFlags {
    let mut values: HashMap<String, String> = HashMap::new();
    let mut unrecognized_inputs: Vec<String> = Vec::new();

    let known: HashMap<&str, ()> = flags.iter().map(|f| (f.name.as_str(), ())).collect();

    let mut i = 1;
    while i < all_args.len() {
        let arg = &all_args[i];
        if known.contains_key(arg.as_str()) {
            if i + 1 >= all_args.len() {
                panic!("{} requires a value", arg);
            }
            values.insert(arg.clone(), all_args[i + 1].clone());
            i += 2;
        } else {
            unrecognized_inputs.push(arg.clone());
            i += 1;
        }
    }

    ParsedFlags {
        values,
        unrecognized_inputs,
    }
}
// no main.vale counterpart (stdlib.flagger emulation).
