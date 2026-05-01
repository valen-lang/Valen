Build:

`cargo build --manifest-path CoordinatorRust/Cargo.toml && cargo build --manifest-path FrontendRust/Cargo.toml && cargo run --manifest-path CoordinatorRust/Cargo.toml -- build --output_dir build --run_backend false --frontend_path_override /Volumes/V/Sylvan/Frontend/Frontend.jar --builtins_dir_override /Volumes/V/Sylvan/Frontend/Builtins stdlib=/Volumes/V/Sylvan/stdlib/src --backend_path_override /Volumes/V/Sylvan/Backend/build/backend test=/Volumes/V/Sylvan/Frontend/Tests/test/main/resources/programs/roguelike.vale`

Tests:

`cargo test --manifest-path FrontendRust/Cargo.toml --lib`
