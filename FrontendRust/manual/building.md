Build:

`cd /Volumes/V/Sylvan && cargo run --manifest-path CoordinatorRust/Cargo.toml -- build --output_dir build --run_backend false --frontend_path_override /Volumes/V/Sylvan/Frontend/Frontend.jar --builtins_dir_override /Volumes/V/Sylvan/Frontend/Builtins stdlib=/Volumes/V/Sylvan/stdlib/src --backend_path_override /Volumes/V/Sylvan/Backend/build/backend test=/Volumes/V/Sylvan/Frontend/Tests/test/main/resources/programs/roguelike.vale`

Tests:

`cd /Volumes/V/Sylvan/FrontendRust && cargo test --lib`