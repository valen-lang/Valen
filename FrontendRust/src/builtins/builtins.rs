
use crate::code_source::Source;
use crate::interner::StrI;
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate};
use crate::utils::fx::HashMap;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;

/// One row per builtin `.vale` file: (module_name, filename, contents).
/// Contents are embedded at compile time via `include_str!`.
pub const ENTRIES: &[(&str, &str, &str)] = &[
    ("arith",                          "arith.vale",                          include_str!("resources/arith.vale")),
    ("logic",                          "logic.vale",                          include_str!("resources/logic.vale")),
    ("migrate",                        "migrate.vale",                        include_str!("resources/migrate.vale")),
    ("str",                            "str.vale",                            include_str!("resources/str.vale")),
    ("drop",                           "drop.vale",                           include_str!("resources/drop.vale")),
    ("clone",                          "clone.vale",                          include_str!("resources/clone.vale")),
    ("implicit_clone",                 "implicit_clone.vale",                 include_str!("resources/implicit_clone.vale")),
    ("arrays",                         "arrays.vale",                         include_str!("resources/arrays.vale")),
    ("mainargs",                       "mainargs.vale",                       include_str!("resources/mainargs.vale")),
    ("as",                             "as.vale",                             include_str!("resources/as.vale")),
    ("print",                          "print.vale",                          include_str!("resources/print.vale")),
    ("tup0",                           "tup0.vale",                           include_str!("resources/tup0.vale")),
    ("tup1",                           "tup1.vale",                           include_str!("resources/tup1.vale")),
    ("tup2",                           "tup2.vale",                           include_str!("resources/tup2.vale")),
    ("tupN",                           "tupN.vale",                           include_str!("resources/tupN.vale")),
    ("streq",                          "streq.vale",                          include_str!("resources/streq.vale")),
    ("panic",                          "panic.vale",                          include_str!("resources/panic.vale")),
    ("panicutils",                     "panicutils.vale",                     include_str!("resources/panicutils.vale")),
    ("opt",                            "opt.vale",                            include_str!("resources/opt.vale")),
    ("result",                         "result.vale",                         include_str!("resources/result.vale")),
    ("sameinstance",                   "sameinstance.vale",                   include_str!("resources/sameinstance.vale")),
    ("weak",                           "weak.vale",                           include_str!("resources/weak.vale")),
];

/// Build a `FileCoordinateMap` for the one builtin module named `name`, keyed at
/// `("v", ["builtins", name])`. Used by `Source::builtin_module`.
pub fn builtin_module_code_map<'a>(
    parse_arena: &ParseArena<'a>,
    keywords: &Keywords<'a>,
    name: &str,
) -> FileCoordinateMap<'a, String> {
    let (_, filename, contents) = ENTRIES.iter().find(|(n, _, _)| *n == name)
        .unwrap_or_else(|| panic!("Unknown builtin module: {}", name));
    let module_stri = parse_arena.intern_str(name);
    let package_coord = parse_arena.intern_package_coordinate(keywords.v, &[keywords.builtins, module_stri]);
    let file_coord = parse_arena.intern_file_coordinate(package_coord, filename);
    let mut result = FileCoordinateMap::new();
    result.put(file_coord, contents.to_string());
    result
}

/// A `Source::Fn` fallback that answers `Some(empty)` for any coord matching
/// `("v", ["builtins", _])`. Callers place this at the end of a test's CodeSource
/// vec so `lex_and_explore` can walk transitive `import v.builtins.<X>.*;` chains
/// even when the test doesn't need the actual content of every module — the empty
/// hashmap satisfies resolution without providing any exports.
pub fn empty_v_builtins_stub<'a>(coord: &PackageCoordinate<'a>) -> Option<HashMap<String, String>> {
    match (coord.module.0, coord.packages.as_slice()) {
        ("v", [StrI("builtins"), _]) => Some(HashMap::default()),
        _ => None,
    }
}

/// Build a single `Source::CodeMap` covering multiple builtin modules — one hashmap
/// entry per module, all wrapped in one Source. Cheaper at the call site than N
/// `Source::builtin_module` calls when a test's imports transitively pull in a fixed
/// cluster (e.g. `panicutils` always drags `panic`+`print`+`str`).
pub fn builtin_source_bundle<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
    names: &[&str],
) -> Source<'a>
where 'a: 'ctx,
{
    let mut result = FileCoordinateMap::new();
    for name in names {
        let (_, filename, contents) = ENTRIES.iter().find(|(n, _, _)| n == name)
            .unwrap_or_else(|| panic!("Unknown builtin module: {}", name));
        let module_stri = parse_arena.intern_str(name);
        let package_coord = parse_arena.intern_package_coordinate(keywords.v, &[keywords.builtins, module_stri]);
        let file_coord = parse_arena.intern_file_coordinate(package_coord, filename);
        result.put(file_coord, contents.to_string());
    }
    Source::from_code_map(&result)
}

/// Cluster helper: `panicutils` and its transitive real-content deps (`panic`,
/// `print`, `str`). Anything that pulls in `panicutils.*` needs the full chain
/// for compilation — the individual modules are never used independently.
pub fn builtin_source_for_panicutils<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
) -> Source<'a>
where 'a: 'ctx,
{
    builtin_source_bundle(parse_arena, keywords, &["panicutils", "panic", "print", "str"])
}

/// Cluster helper: `arith` and `implicit_clone`. Any test loading arith needs
/// implicit_clone real-content for the `&int → int` borrow-passing semantics.
pub fn builtin_source_for_arith<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
) -> Source<'a>
where 'a: 'ctx,
{
    builtin_source_bundle(parse_arena, keywords, &["arith", "implicit_clone"])
}

/// Cluster helper: `arrays` and its transitive real-content deps (`arith`,
/// `drop`, `implicit_clone`). The array subsystem's various extern decls and
/// higher-level `Array<E, G>` constructor all live inside `arrays.vale` now.
pub fn builtin_source_for_arrays<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
) -> Source<'a>
where 'a: 'ctx,
{
    builtin_source_bundle(parse_arena, keywords, &["arrays", "arith", "drop", "implicit_clone"])
}

/// Cluster helper: `opt` and everything it transitively needs — its own
/// `func drop` overload requires `drop`+`implicit_clone`, and its `panic("...")`
/// calls drag in the whole `panicutils` chain.
pub fn builtin_source_for_opt<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
) -> Source<'a>
where 'a: 'ctx,
{
    builtin_source_bundle(parse_arena, keywords, &[
        "opt", "drop", "implicit_clone",
        "panicutils", "panic", "print", "str",
    ])
}

/// Cluster helper: `weak` and its full chain — `weak.vale`'s single export
/// `lock<T>(...) Opt<&T>` needs everything the `opt` cluster needs.
pub fn builtin_source_for_weak<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
) -> Source<'a>
where 'a: 'ctx,
{
    builtin_source_bundle(parse_arena, keywords, &[
        "weak", "opt", "drop", "implicit_clone",
        "panicutils", "panic", "print", "str",
    ])
}

/// Cluster helper: `as` and its full chain — `as.vale`'s `Result<...>` return
/// type drags in `result`, and typing the borrow overloads adds
/// `logic`+`drop`+`implicit_clone`+`arith` plus the `panicutils` chain.
pub fn builtin_source_for_as<'a, 'ctx>(
    parse_arena: &'ctx ParseArena<'a>,
    keywords: &'ctx Keywords<'a>,
) -> Source<'a>
where 'a: 'ctx,
{
    builtin_source_bundle(parse_arena, keywords, &[
        "as", "result", "logic",
        "drop", "implicit_clone", "arith",
        "panicutils", "panic", "print", "str",
    ])
}

// Modulized is a made up word, it means we're pretending the builtins are in different modules.
// This lets tests import only certain kinds of builtins.
// The more basic foundational tests will choose not to import any builtins, so they can test the
// bare minimum. For example, the most basic test is `func main() int { return 42; }`, and we don't want it
// to fail just because the builtin-yet-unused `func as<T, X>(x X) Opt<T> { ... }` doesn't want to
// work right now.
// This gives us a FileCoordinateMap where each file is its own module, so that we can
// pull in only files modules a certain test needs.
pub fn get_embedded_modulized_code_map<'a>(
    parse_arena: &ParseArena<'a>,
    keywords: &Keywords<'a>,
) -> FileCoordinateMap<'a, String> {
    let mut result = FileCoordinateMap::new();
    for (module_name, filename, contents) in ENTRIES {
        let module_name_stri = parse_arena.intern_str(module_name);
        let package_coord = parse_arena.intern_package_coordinate(keywords.v, &[keywords.builtins, module_name_stri]);
        let file_coord = parse_arena.intern_file_coordinate(package_coord, filename);
        result.put(file_coord, contents.to_string());
    }
    result
}


// Add an empty v.builtins.whatever so that the aforementioned imports still work.
// But load the actual files all inside the root package.
pub fn get_code_map<'a>(
    parse_arena: &ParseArena<'a>,
    keywords: &Keywords<'a>,
) -> FileCoordinateMap<'a, String> {
    let builtin_namespace_coord = parse_arena.intern_package_coordinate(keywords.empty_string, &[]);
    let mut result = FileCoordinateMap::new();

    for (module_name, filename, contents) in ENTRIES {
        let module_name_stri = parse_arena.intern_str(module_name);
        // Put empty string for v.builtins.moduleName
        let modulized_package_coord = parse_arena.intern_package_coordinate(keywords.v, &[keywords.builtins, module_name_stri]);
        let modulized_file_coord = parse_arena.intern_file_coordinate(modulized_package_coord, filename);
        result.put(modulized_file_coord, String::new());
        // Put actual code for root package
        let root_file_coord = parse_arena.intern_file_coordinate(builtin_namespace_coord, filename);
        result.put(root_file_coord, contents.to_string());
    }

    result
}
