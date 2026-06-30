
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use std::fs;
use std::path::Path;

pub const MODULE_TO_FILENAME: &[(&str, &str)] = &[
    ("arith", "arith.vale"),
    ("logic", "logic.vale"),
    ("migrate", "migrate.vale"),
    ("str", "str.vale"),
    ("drop", "drop.vale"),
    ("clone", "clone.vale"),
    ("implicit_clone", "implicit_clone.vale"),
    ("arrays", "arrays.vale"),
    ("runtime_sized_array_new", "runtime_sized_array_new.vale"),
    ("runtime_sized_array_push", "runtime_sized_array_push.vale"),
    ("runtime_sized_array_pop", "runtime_sized_array_pop.vale"),
    ("runtime_sized_array_len", "runtime_sized_array_len.vale"),
    ("runtime_sized_array_capacity", "runtime_sized_array_capacity.vale"),
    ("runtime_sized_array_drop", "runtime_sized_array_drop.vale"),
    ("static_sized_array_drop", "static_sized_array_drop.vale"),
    ("mainargs", "mainargs.vale"),
    ("as", "as.vale"),
    ("print", "print.vale"),
    ("tup0", "tup0.vale"),
    ("tup1", "tup1.vale"),
    ("tup2", "tup2.vale"),
    ("tupN", "tupN.vale"),
    ("streq", "streq.vale"),
    ("panic", "panic.vale"),
    ("panicutils", "panicutils.vale"),
    ("opt", "opt.vale"),
    ("result", "result.vale"),
    ("sameinstance", "sameinstance.vale"),
    ("weak", "weak.vale"),
];

const BUILTINS_DIR: &str = "src/builtins/resources";

pub fn load(builtins_dir: &str, resource_filename: &str) -> Result<String, String> {
    let path = Path::new(builtins_dir).join(resource_filename);
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to load builtin file {}: {}", path.display(), e))
}

// Modulized is a made up word, it means we're pretending the builtins are in different modules.
// This lets tests import only certain kinds of builtins.
// The more basic foundational tests will choose not to import any builtins, so they can test the
// bare minimum. For example, the most basic test is `func main() int { return 42; }`, and we don't want it
// to fail just because the builtin-yet-unused `func as<T, X>(x X) Opt<T> { ... }` doesn't want to
// work right now.
pub fn get_modulized_code_map<'a>(
    parse_arena: &ParseArena<'a>,
    keywords: &Keywords<'a>,
) -> Result<FileCoordinateMap<'a, String>, String> {
    let mut result = FileCoordinateMap::new();

    for (module_name, filename) in MODULE_TO_FILENAME {
        let module_name_stri = parse_arena.intern_str(module_name);
        let package_coord = parse_arena.intern_package_coordinate(keywords.v, &[keywords.builtins, module_name_stri]);
        let file_coord = parse_arena.intern_file_coordinate(package_coord, filename);
        let code = load(BUILTINS_DIR, filename)?;
        result.put(file_coord, code);
    }

    Ok(result)
}

pub fn get_embedded_modulized_code_map<'a>(
    parse_arena: &ParseArena<'a>,
    keywords: &Keywords<'a>,
) -> FileCoordinateMap<'a, String> {
    let entries: &[(&str, &str, &str)] = &[
        ("arith",                          "arith.vale",                          include_str!("resources/arith.vale")),
        ("logic",                          "logic.vale",                          include_str!("resources/logic.vale")),
        ("migrate",                        "migrate.vale",                        include_str!("resources/migrate.vale")),
        ("str",                            "str.vale",                            include_str!("resources/str.vale")),
        ("drop",                           "drop.vale",                           include_str!("resources/drop.vale")),
        ("clone",                          "clone.vale",                          include_str!("resources/clone.vale")),
        ("implicit_clone",                 "implicit_clone.vale",                 include_str!("resources/implicit_clone.vale")),
        ("arrays",                         "arrays.vale",                         include_str!("resources/arrays.vale")),
        ("runtime_sized_array_new",    "runtime_sized_array_new.vale",    include_str!("resources/runtime_sized_array_new.vale")),
        ("runtime_sized_array_push",       "runtime_sized_array_push.vale",       include_str!("resources/runtime_sized_array_push.vale")),
        ("runtime_sized_array_pop",        "runtime_sized_array_pop.vale",        include_str!("resources/runtime_sized_array_pop.vale")),
        ("runtime_sized_array_len",        "runtime_sized_array_len.vale",        include_str!("resources/runtime_sized_array_len.vale")),
        ("runtime_sized_array_capacity",   "runtime_sized_array_capacity.vale",   include_str!("resources/runtime_sized_array_capacity.vale")),
        ("runtime_sized_array_drop",   "runtime_sized_array_drop.vale",   include_str!("resources/runtime_sized_array_drop.vale")),
        ("static_sized_array_drop",    "static_sized_array_drop.vale",    include_str!("resources/static_sized_array_drop.vale")),
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
    let mut result = FileCoordinateMap::new();
    for (module_name, filename, contents) in entries {
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
    let entries: &[(&str, &str, &str)] = &[
        ("arith",                          "arith.vale",                          include_str!("resources/arith.vale")),
        ("logic",                          "logic.vale",                          include_str!("resources/logic.vale")),
        ("migrate",                        "migrate.vale",                        include_str!("resources/migrate.vale")),
        ("str",                            "str.vale",                            include_str!("resources/str.vale")),
        ("drop",                           "drop.vale",                           include_str!("resources/drop.vale")),
        ("clone",                          "clone.vale",                          include_str!("resources/clone.vale")),
        ("implicit_clone",                 "implicit_clone.vale",                 include_str!("resources/implicit_clone.vale")),
        ("arrays",                         "arrays.vale",                         include_str!("resources/arrays.vale")),
        ("runtime_sized_array_new",    "runtime_sized_array_new.vale",    include_str!("resources/runtime_sized_array_new.vale")),
        ("runtime_sized_array_push",       "runtime_sized_array_push.vale",       include_str!("resources/runtime_sized_array_push.vale")),
        ("runtime_sized_array_pop",        "runtime_sized_array_pop.vale",        include_str!("resources/runtime_sized_array_pop.vale")),
        ("runtime_sized_array_len",        "runtime_sized_array_len.vale",        include_str!("resources/runtime_sized_array_len.vale")),
        ("runtime_sized_array_capacity",   "runtime_sized_array_capacity.vale",   include_str!("resources/runtime_sized_array_capacity.vale")),
        ("runtime_sized_array_drop",   "runtime_sized_array_drop.vale",   include_str!("resources/runtime_sized_array_drop.vale")),
        ("static_sized_array_drop",    "static_sized_array_drop.vale",    include_str!("resources/static_sized_array_drop.vale")),
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
    let builtin_namespace_coord = parse_arena.intern_package_coordinate(keywords.empty_string, &[]);
    let mut result = FileCoordinateMap::new();

    for (module_name, filename, contents) in entries {
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
