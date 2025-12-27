// CLI for FrontendRust - Vale parser
// Produces VPST (Vale Parsed Syntax Tree) files from Vale source
// Mirrors LexAndExplore.scala import-driven parsing logic

use frontend_rust::interner::Interner;
use frontend_rust::keywords::Keywords;
use frontend_rust::lexing::iterator::LexingIterator;
use frontend_rust::lexing::lexer::Lexer;
use frontend_rust::parsing::parser::Parser;
use frontend_rust::parsing::vonifier::ParserVonifier;
use frontend_rust::von::printer::VonPrinter;
use frontend_rust::error_humanizer::ParseErrorHumanizer;
use frontend_rust::parsing::ast::{FileP, FileCoordinate, PackageCoordinate, IDenizenP};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{Arc, Mutex};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} --output_dir <dir> [project_name=path ...]", args[0]);
        process::exit(1);
    }

    let mut output_dir = None;
    let mut inputs = Vec::new();
    
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--output_dir" && i + 1 < args.len() {
            output_dir = Some(PathBuf::from(&args[i + 1]));
            i += 2;
        } else if let Some((project, path_str)) = args[i].split_once('=') {
            inputs.push((project.to_string(), PathBuf::from(path_str)));
            i += 1;
        } else {
            i += 1;
        }
    }

    let output_dir = output_dir.expect("--output_dir required");
    
    // Create output directories
    let vpst_dir = output_dir.join("vpst");
    fs::create_dir_all(&vpst_dir).expect("Failed to create vpst directory");

    println!("FrontendRust: Parsing Vale source files...");
    
    // Initialize interner and keywords
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);

    // From PassManager.scala lines 136-143: Build inputs
    // DirectFilePathInput for files, ModulePathInput for directories
    let mut module_roots: HashMap<String, PathBuf> = HashMap::new();
    let mut direct_file_inputs: HashMap<PackageCoordinate, PathBuf> = HashMap::new();
    let mut initial_packages: Vec<PackageCoordinate> = Vec::new();
    
    for (project_name, input_path) in inputs {
        let module_str = {
            let mut interner_lock = interner.lock().unwrap();
            interner_lock.intern(&project_name)
        };
        
        if input_path.is_dir() {
            // From PassManager.scala line 142: ModulePathInput
            module_roots.insert(project_name.clone(), input_path);
        } else if input_path.extension().and_then(|s| s.to_str()) == Some("vale") {
            // From PassManager.scala line 137: DirectFilePathInput
            let package_coord = PackageCoordinate {
                module: Arc::new(module_str),
                packages: vec![],
            };
            direct_file_inputs.insert(package_coord.clone(), input_path.clone());
            initial_packages.push(package_coord);
            
            // Also register parent directory as module root for resolving imports
            if let Some(parent) = input_path.parent() {
                module_roots.entry(project_name).or_insert_with(|| parent.to_path_buf());
            }
        }
    }
    
    // From PassManager.scala line 238: Always add PackageCoordinate.BUILTIN to packages
    // From CodeHierarchy.scala line 50: BUILTIN is the root package (empty module, empty packages)
    // This ensures builtin files (str.vale, print.vale, etc.) are always parsed
    if module_roots.contains_key("stdlib") {
        let stdlib_root_package = PackageCoordinate {
            module: Arc::new({
                let mut interner_lock = interner.lock().unwrap();
                interner_lock.intern("stdlib")
            }),
            packages: vec![],
        };
        initial_packages.push(stdlib_root_package);
    }

    // From LexAndExplore.scala lines 58-146: Import-driven parsing loop
    lex_and_explore(
        interner,
        keywords,
        initial_packages,
        module_roots,
        direct_file_inputs,
        &vpst_dir,
    );

    println!("FrontendRust: Parsing complete!");
}

// From LexAndExplore.scala lines 43-150: Main import-driven parsing loop
fn lex_and_explore(
    interner: Arc<Mutex<Interner>>,
    keywords: Arc<Keywords>,
    packages: Vec<PackageCoordinate>,
    module_roots: HashMap<String, PathBuf>,
    direct_file_inputs: HashMap<PackageCoordinate, PathBuf>,
    vpst_dir: &Path,
) {
    // From LexAndExplore.scala lines 52-53
    let mut unexplored_packages: HashSet<PackageCoordinate> = packages.into_iter().collect();
    let mut started_packages: HashSet<PackageCoordinate> = HashSet::new();
    
    // From LexAndExplore.scala lines 58-146: Main loop
    while !unexplored_packages.is_empty() {
        // From LexAndExplore.scala lines 59-61
        let needed_package_coord = unexplored_packages.iter().next().cloned().unwrap();
        unexplored_packages.remove(&needed_package_coord);
        started_packages.insert(needed_package_coord.clone());
        
        // From LexAndExplore.scala lines 65-79: Resolve package to file paths
        let filepaths_and_contents = resolve_package(&needed_package_coord, &module_roots, &direct_file_inputs);
        
        if filepaths_and_contents.is_empty() {
            eprintln!("Error: Couldn't find package: {}.{}", 
                needed_package_coord.module.str,
                needed_package_coord.packages.iter().map(|p| p.str.as_str()).collect::<Vec<_>>().join("."));
            process::exit(1);
        }
        
        // From LexAndExplore.scala lines 82-145: Process each file in the package
        for (file_coord, code) in filepaths_and_contents {
            // Parse the file and extract imports
            let file_p = parse_file_and_extract_imports(
                &file_coord,
                &code,
                interner.clone(),
                keywords.clone(),
            );
            
            // From LexAndExplore.scala lines 105-119, 137-140: Extract imports and add to unexplored
            let imports = extract_imports(&file_p);
            for import_coord in imports {
                if !started_packages.contains(&import_coord) {
                    unexplored_packages.insert(import_coord);
                }
            }
            
            // Generate .vpst file
            write_vpst(&file_p, vpst_dir);
        }
    }
}

// From PassManager.scala lines 153-201: Resolve package coordinate to file paths
fn resolve_package(
    package_coord: &PackageCoordinate,
    module_roots: &HashMap<String, PathBuf>,
    direct_file_inputs: &HashMap<PackageCoordinate, PathBuf>,
) -> Vec<(FileCoordinate, String)> {
    // From PassManager.scala line 190-196: Check for DirectFilePathInput first
    if let Some(file_path) = direct_file_inputs.get(package_coord) {
        if let Ok(code) = fs::read_to_string(file_path) {
            let file_coord = FileCoordinate {
                package_coord: package_coord.clone(),
                filepath: file_path.to_string_lossy().to_string(),
            };
            return vec![(file_coord, code)];
        }
    }
    
    // From PassManager.scala line 168-189: ModulePathInput - find all files in directory
    let module_name = &package_coord.module.str;
    let module_root = match module_roots.get(module_name.as_str()) {
        Some(root) => root,
        None => {
            eprintln!("Error: No module root registered for module '{}'", module_name);
            process::exit(1);
        }
    };
    
    // Build path: module_root/package1/package2/...
    let mut dir_path = module_root.clone();
    for package_step in &package_coord.packages {
        dir_path.push(&package_step.str);
    }
    
    // Find all .vale files in this directory
    let mut results = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("vale") {
                if let Ok(code) = fs::read_to_string(&path) {
                    let file_coord = FileCoordinate {
                        package_coord: package_coord.clone(),
                        filepath: path.to_string_lossy().to_string(),
                    };
                    results.push((file_coord, code));
                }
            }
        }
    }
    
    results
}

// From LexAndExplore.scala lines 82-135: Parse a file and return FileP
fn parse_file_and_extract_imports(
    file_coord: &FileCoordinate,
    code: &str,
    interner: Arc<Mutex<Interner>>,
    keywords: Arc<Keywords>,
) -> FileP {
    println!("  Parsing: {}", file_coord.filepath);
    
    // From LexAndExplore.scala lines 85-102: Lex the file
    let mut lexer = Lexer::new(interner.clone(), keywords.clone());
    let mut lex_iter = LexingIterator::new(code.to_string());
    
    let mut denizens = Vec::new();
    while !lex_iter.at_end() {
        lex_iter.consume_comments_and_whitespace();
        if lex_iter.at_end() {
            break;
        }
        
        match lexer.lex_denizen(&mut lex_iter) {
            Ok(denizen) => denizens.push(denizen),
            Err(e) => {
                let error_msg = ParseErrorHumanizer::humanize(Path::new(&file_coord.filepath), code, &e);
                eprint!("{}", error_msg);
                process::exit(22);
            }
        }
    }

    // Parse the denizens
    let mut parser = Parser::new(interner.clone(), keywords.clone());
    let mut parsed_denizens = Vec::new();
    
    for denizen in denizens {
        match parser.parse_denizen(denizen) {
            Ok(parsed) => parsed_denizens.push(parsed),
            Err(e) => {
                let error_msg = ParseErrorHumanizer::humanize(Path::new(&file_coord.filepath), code, &e);
                eprint!("{}", error_msg);
                process::exit(22);
            }
        }
    }

    FileP {
        file_coord: file_coord.clone(),
        comments_ranges: vec![],
        denizens: parsed_denizens,
    }
}

// From LexAndExplore.scala lines 105-119, 137-140: Extract imports from parsed file
fn extract_imports(file_p: &FileP) -> Vec<PackageCoordinate> {
    let mut imports = Vec::new();
    
    for denizen in &file_p.denizens {
        if let IDenizenP::TopLevelImport(import) = denizen {
            // Build PackageCoordinate from import
            let package_coord = PackageCoordinate {
                module: import.module_name.str.clone(),
                packages: import.package_steps.iter().map(|name| name.str.clone()).collect(),
            };
            imports.push(package_coord);
        }
    }
    
    imports
}

// Write FileP as .vpst file
fn write_vpst(file_p: &FileP, vpst_dir: &Path) {
    // Vonify (convert to JSON-serializable format)
    let von_data = ParserVonifier::vonify_file(file_p);

    // Serialize to JSON
    let von_printer = VonPrinter::new();
    let json_str = von_printer.print(&von_data);

    // Create output filename: module.package1.package2.filename.vpst
    let file_name = Path::new(&file_p.file_coord.filepath)
        .file_stem()
        .unwrap()
        .to_string_lossy();
    
    let mut output_filename = file_p.file_coord.package_coord.module.str.clone();
    for package_step in &file_p.file_coord.package_coord.packages {
        output_filename.push('.');
        output_filename.push_str(&package_step.str);
    }
    if !file_p.file_coord.package_coord.packages.is_empty() {
        output_filename.push('.');
    }
    output_filename.push_str(&file_name);
    output_filename.push_str(".vpst");

    let output_path = vpst_dir.join(&output_filename);
    
    // Write VPST file
    if let Err(e) = fs::write(&output_path, json_str) {
        eprintln!("Error writing {}: {}", output_path.display(), e);
        process::exit(1);
    }

    println!("    -> {}", output_path.display());
}

