// Main entry point for Vale compiler coordinator

mod build;
mod midas;
mod valestrom;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() {
    let all_args: Vec<String> = env::args().collect();

    if all_args.is_empty() {
        eprintln!("Error: No arguments provided");
        process::exit(1);
    }

    let program_path = PathBuf::from(&all_args[0]);
    let program_path = if program_path.exists() {
        program_path
    } else {
        // Try to resolve it
        match fs::canonicalize(&program_path) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Path {} does not exist!", program_path.display());
                process::exit(1);
            }
        }
    };

    let compiler_dir = program_path
        .parent()
        .expect("Could not determine compiler directory")
        .to_path_buf();

    if all_args.len() < 2 {
        eprintln!("Must specify a command (build or help).");
        process::exit(1);
    }

    let command = &all_args[1];

    match command.as_str() {
        "version" | "--version" => {
            let version_file = compiler_dir.join("valec-version.txt");
            match fs::read_to_string(&version_file) {
                Ok(content) => println!("{}", content),
                Err(e) => {
                    eprintln!("Error reading version file: {}", e);
                    process::exit(1);
                }
            }
        }
        "help" | "--help" => {
            if all_args.len() >= 3 && all_args[2] == "build" {
                let help_file = compiler_dir.join("valec-help-build.txt");
                match fs::read_to_string(&help_file) {
                    Ok(content) => println!("{}", content),
                    Err(e) => {
                        eprintln!("Error reading help file: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                let help_file = compiler_dir.join("valec-help.txt");
                match fs::read_to_string(&help_file) {
                    Ok(content) => println!("{}", content),
                    Err(e) => {
                        eprintln!("Error reading help file: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
        "build" => {
            build::build_stuff(&compiler_dir, &all_args);
        }
        _ => {
            eprintln!("Unknown command: {}.", command);
            eprintln!("");
            let help_file = compiler_dir.join("valec-help.txt");
            match fs::read_to_string(&help_file) {
                Ok(content) => println!("{}", content),
                Err(_) => {}
            }
            process::exit(1);
        }
    }
}
