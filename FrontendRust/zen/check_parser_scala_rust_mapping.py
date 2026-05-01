#!/usr/bin/env python3
"""
Generate implementation-only Scala->Rust mapping for the parser migration.

This is a parser-specific wrapper around check_scala_rust_mapping.py that:
  - scans Scala parser sources
  - skips Scala test files/paths
  - searches Rust parser implementation (excluding Rust tests by default)
  - writes a Cursor rule .mdc file with frontmatter and mapping lines:
      scala/path.scala:line kind Name -> snake_name in rust/path.rs
"""

import argparse
from pathlib import Path

from check_scala_rust_mapping import (
    camel_to_snake,
    find_extra_rust_functions,
    find_migallow_mappings,
    find_rust_identifier_path,
    find_scala_definitions,
    normalize_identifier,
)


def is_impl_scala_path(path: Path) -> bool:
    low_parts = [p.lower() for p in path.parts]
    stem = path.stem.lower()
    name = path.name.lower()
    if "test" in low_parts or "tests" in low_parts:
        return False
    if "test" in stem or "test" in name:
        return False
    return True


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Check parser Scala->Rust mappings (implementation only)"
    )
    parser.add_argument(
        "--scala-dir",
        default=Path(__file__).resolve().parent.parent.parent
        / "Frontend"
        / "ParsingPass"
        / "src"
        / "dev"
        / "vale"
        / "parsing",
        type=Path,
        help="Root of Scala parser source",
    )
    parser.add_argument(
        "--rust-dir",
        default=Path(__file__).resolve().parent.parent / "src" / "parsing",
        type=Path,
        help="Root of Rust parser source",
    )
    parser.add_argument(
        "--output",
        default=Path(__file__).resolve().parent.parent.parent
        / ".cursor"
        / "rules"
        / "parser_impl_scala_rust_mapping.mdc",
        type=Path,
        help="Output report path",
    )
    parser.add_argument(
        "--include-rust-test",
        action="store_true",
        help="Include Rust test directories in search",
    )
    args = parser.parse_args()

    scala_dir = args.scala_dir
    rust_dir = args.rust_dir
    output_path = args.output
    exclude_rust_test = not args.include_rust_test

    if not scala_dir.is_dir():
        raise SystemExit(f"Error: Scala dir not found: {scala_dir}")
    if not rust_dir.is_dir():
        raise SystemExit(f"Error: Rust dir not found: {rust_dir}")

    definitions = [
        d for d in find_scala_definitions(scala_dir) if is_impl_scala_path(d[2])
    ]

    migallow_by_rust_file, migallow_rust_idents = find_migallow_mappings(
        rust_dir, exclude_test=exclude_rust_test
    )

    seen = set()
    rows = []
    mapped_rust_identifiers: set[str] = set(migallow_rust_idents)
    for name, kind, path, line in definitions:
        key = (name, kind, path)
        if key in seen:
            continue
        seen.add(key)

        snake = camel_to_snake(name)
        rel_scala = path.relative_to(scala_dir)
        rust_match = find_rust_identifier_path(
            rust_dir,
            snake,
            exclude_test=exclude_rust_test,
            scala_name=name,
            scala_kind=kind,
            scala_path=path,
            migallow_by_rust_file=migallow_by_rust_file,
        )

        if rust_match is not None:
            rust_match_path, matched_ident = rust_match
            mapped_rust_identifiers.add(normalize_identifier(matched_ident))
            try:
                rel_rust = rust_match_path.relative_to(rust_dir.parent)
            except ValueError:
                rel_rust = rust_match_path
            line_text = (
                f"{rel_scala}:{line} {kind} {name} -> {matched_ident} in {rel_rust}"
            )
            rows.append((0, kind, name.lower(), line_text))
        else:
            line_text = f"{rel_scala}:{line} {kind} {name} in <missing>"
            rows.append((1, kind, name.lower(), line_text))

    rows.sort()
    missing_count = sum(1 for r in rows if r[0] == 1)
    found_count = len(rows) - missing_count
    extra_rust_functions = find_extra_rust_functions(
        rust_dir,
        mapped_rust_identifiers,
        exclude_test=exclude_rust_test,
    )

    output_path.parent.mkdir(parents=True, exist_ok=True)
    with output_path.open("w", encoding="utf-8") as f:
        f.write("---\n")
        f.write("description: Parser Scala-to-Rust implementation mapping\n")
        f.write("globs: FrontendRust/src/parsing/**/*.rs\n")
        f.write("alwaysApply: false\n")
        f.write("---\n\n")
        f.write("# Parser Implementation Mapping (Scala -> Rust)\n\n")
        f.write(f"- Scala dir: `{scala_dir}`\n")
        f.write(f"- Rust dir: `{rust_dir}`\n\n")
        for _, _, _, line in rows:
            f.write(f"- {line}\n")
        for fn_name, fn_path, _ in extra_rust_functions:
            try:
                rel_fn_path = fn_path.relative_to(rust_dir.parent)
            except ValueError:
                rel_fn_path = fn_path
            f.write(f"- <missing> -> {fn_name} in {rel_fn_path}\n")
        f.write("\n")
        f.write(f"- Total found: `{found_count}`\n")
        f.write(f"- Total missing: `{missing_count}`\n")
        f.write(f"- Total checked: `{len(rows)}`\n")
        f.write(f"- Extra rust functions: `{len(extra_rust_functions)}`\n")

    print(
        f"Wrote {output_path} "
        f"(found={found_count}, missing={missing_count}, checked={len(rows)}, "
        f"extra_functions={len(extra_rust_functions)})"
    )


if __name__ == "__main__":
    main()
