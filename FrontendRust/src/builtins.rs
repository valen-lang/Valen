// From Frontend/Builtins/src/dev/vale/Builtins.scala

use crate::parsing::ast::{FileCoordinate, PackageCoordinate};
use crate::file_coordinate_map::FileCoordinateMap;
use crate::interner::Interner;
use crate::keywords::Keywords;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::Path;

// From Builtins.scala lines 9-39: moduleToFilename
pub const MODULE_TO_FILENAME: &[(&str, &str)] = &[
    ("arith", "arith.vale"),
    ("functor1", "functor1.vale"),
    ("logic", "logic.vale"),
    ("migrate", "migrate.vale"),
    ("str", "str.vale"),
    ("drop", "drop.vale"),
    ("clone", "clone.vale"),
    ("arrays", "arrays.vale"),
    ("runtime_sized_array_mut_new", "runtime_sized_array_mut_new.vale"),
    ("runtime_sized_array_push", "runtime_sized_array_push.vale"),
    ("runtime_sized_array_pop", "runtime_sized_array_pop.vale"),
    ("runtime_sized_array_len", "runtime_sized_array_len.vale"),
    ("runtime_sized_array_capacity", "runtime_sized_array_capacity.vale"),
    ("runtime_sized_array_mut_drop", "runtime_sized_array_mut_drop.vale"),
    ("static_sized_array_mut_drop", "static_sized_array_mut_drop.vale"),
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

// From Builtins.scala lines 41-70: load
// Note: In Scala this loads from embedded resources. In Rust CLI, we load from filesystem.
pub fn load(builtins_dir: &str, resource_filename: &str) -> Result<String, String> {
    let path = Path::new(builtins_dir).join(resource_filename);
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to load builtin file {}: {}", path.display(), e))
}

// From Builtins.scala lines 78-90: getModulizedCodeMap
// Modulized is a made up word, it means we're pretending the builtins are in different modules.
// This lets tests import only certain kinds of builtins.
// The more basic foundational tests will choose not to import any builtins, so they can test the
// bare minimum. For example, the most basic test is `func main() int { return 42; }`, and we don't want it
// to fail just because the builtin-yet-unused `func as<T, X>(x X) Opt<T> { ... }` doesn't want to
// work right now.
pub fn get_modulized_code_map(
    interner: Arc<Mutex<Interner>>,
    keywords: Arc<Keywords>,
    builtins_dir: &str,
) -> Result<FileCoordinateMap<String>, String> {
    let mut result = FileCoordinateMap::new();
    
    for (module_name, filename) in MODULE_TO_FILENAME {
        let module_name_stri = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern(module_name)
        };
        
        let package_coord = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern_package_coordinate(PackageCoordinate {
                module: Arc::new(keywords.v.clone()),
                packages: vec![Arc::new(keywords.builtins.clone()), Arc::new(module_name_stri)],
            })
        };
        
        let file_coord = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern_file_coordinate(FileCoordinate {
                package_coord: package_coord,
                filepath: filename.to_string(),
            })
        };
        
        let code = load(builtins_dir, filename)?;
        result.put(file_coord, code);
    }
    
    Ok(result)
}

// From Builtins.scala lines 94-111: getCodeMap
// Add an empty v.builtins.whatever so that the aforementioned imports still work.
// But load the actual files all inside the root package.
pub fn get_code_map(
    interner: Arc<Mutex<Interner>>,
    keywords: Arc<Keywords>,
    builtins_dir: &str,
) -> Result<FileCoordinateMap<String>, String> {
    let builtin_namespace_coord = {
        let mut interner_lock = interner.lock().unwrap();
        interner_lock.intern_package_coordinate(PackageCoordinate {
            module: Arc::new(keywords.empty_string.clone()),
            packages: vec![],
        })
    };
    
    let mut result = FileCoordinateMap::new();
    
    for (module_name, filename) in MODULE_TO_FILENAME {
        let module_name_stri = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern(module_name)
        };
        
        // Put empty string for v.builtins.moduleName
        let modulized_package_coord = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern_package_coordinate(PackageCoordinate {
                module: Arc::new(keywords.v.clone()),
                packages: vec![Arc::new(keywords.builtins.clone()), Arc::new(module_name_stri)],
            })
        };
        
        let modulized_file_coord = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern_file_coordinate(FileCoordinate {
                package_coord: modulized_package_coord,
                filepath: filename.to_string(),
            })
        };
        
        result.put(modulized_file_coord, String::new());
        
        // Put actual code for root package
        let root_file_coord = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern_file_coordinate(FileCoordinate {
                package_coord: builtin_namespace_coord.clone(),
                filepath: filename.to_string(),
            })
        };
        
        let code = load(builtins_dir, filename)?;
        result.put(root_file_coord, code);
    }
    
    Ok(result)
}
