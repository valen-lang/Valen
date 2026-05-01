#!/usr/bin/env python3
"""
Check that Scala definitions (classes, traits, objects, functions) have
corresponding snake_case identifiers in the Rust codebase.

Usage:
  python3 zen/check_scala_rust_mapping.py [--scala-dir PATH] [--rust-dir PATH]

Scans Scala files for:
  - class Name
  - trait Name
  - object Name
  - def name(...)

Converts each to snake_case and searches Rust files for a matching identifier.
Reports missing mappings.
"""

import argparse
import os
import re
import sys
from pathlib import Path
from typing import Optional


def camel_to_snake(name: str) -> str:
    """Convert CamelCase/camelCase (including acronyms) to snake_case."""
    if not name or not name[0].isalpha():
        return name
    # Normalize separators first.
    normalized = re.sub(r"[-\s]+", "_", name)
    # Split boundaries like "parseLet" -> "parse_Let" and "HTTPServer" -> "HTTP_Server".
    step1 = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", normalized)
    # Split lower/digit-to-upper transitions, including acronym tails:
    # "OwnershipPT" -> "Ownership_PT", "TypedPR" -> "Typed_PR".
    step2 = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", step1)
    return step2.lower()


def _normalize_identifier(name: str) -> str:
    """Normalize identifier for fuzzy matching."""
    return re.sub(r"[^A-Za-z0-9]+", "", name).lower()


def _normalize_fuzzy(name: str) -> str:
    """Normalize for fuzzy match: lowercase, strip underscores and other separators."""
    return re.sub(r"[^a-z0-9]+", "", name.lower())


def _fuzzy_matches(rust_ident: str, candidates: list[str]) -> Optional[str]:
    """
    Check if rust_ident fuzzy-matches any candidate (ignore case, underscores).
    Returns the matching candidate, or None.
    Only exact normalized equality. Renames (e.g. merge -> merge_uses) use MIGALLOW.
    """
    rust_norm = _normalize_fuzzy(rust_ident)
    if not rust_norm:
        return None
    for c in candidates:
        cand_norm = _normalize_fuzzy(c)
        if not cand_norm:
            continue
        if rust_norm == cand_norm:
            return c
    return None


def normalize_identifier(name: str) -> str:
    """Public wrapper for identifier normalization."""
    return _normalize_identifier(name)


def _strip_rust_comments(text: str) -> str:
    """Remove Rust line/block comments while preserving newlines."""
    out: list[str] = []
    i = 0
    n = len(text)
    block_depth = 0
    in_line_comment = False
    in_string = False
    in_raw_string = False
    raw_hashes = 0

    while i < n:
        c = text[i]

        if in_line_comment:
            if c == "\n":
                in_line_comment = False
                out.append("\n")
            i += 1
            continue

        if block_depth > 0:
            if i + 1 < n and text[i : i + 2] == "/*":
                block_depth += 1
                i += 2
                continue
            if i + 1 < n and text[i : i + 2] == "*/":
                block_depth -= 1
                i += 2
                continue
            if c == "\n":
                out.append("\n")
            i += 1
            continue

        if in_raw_string:
            out.append(c)
            if c == '"':
                j = i + 1
                hashes_seen = 0
                while j < n and text[j] == "#":
                    hashes_seen += 1
                    j += 1
                if hashes_seen == raw_hashes:
                    out.extend("#" * hashes_seen)
                    i = j
                    in_raw_string = False
                    continue
            i += 1
            continue

        if in_string:
            out.append(c)
            if c == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if c == '"':
                in_string = False
            i += 1
            continue

        # Normal code state
        if i + 1 < n and text[i : i + 2] == "//":
            in_line_comment = True
            i += 2
            continue
        if i + 1 < n and text[i : i + 2] == "/*":
            block_depth = 1
            i += 2
            continue

        # Raw string starts with r"...", r#"... "#, etc.
        if c == "r":
            j = i + 1
            hashes = 0
            while j < n and text[j] == "#":
                hashes += 1
                j += 1
            if j < n and text[j] == '"':
                out.append("r")
                out.extend("#" * hashes)
                out.append('"')
                i = j + 1
                in_raw_string = True
                raw_hashes = hashes
                continue

        if c == '"':
            in_string = True
            out.append(c)
            i += 1
            continue

        out.append(c)
        i += 1

    return "".join(out)


def _identifier_candidates(snake_name: str, scala_name: Optional[str]) -> list[str]:
    candidates = [snake_name]
    pascal_name = "".join(w.capitalize() for w in snake_name.split("_"))
    candidates.append(pascal_name)
    if scala_name:
        candidates.append(scala_name)
    # Keep order while removing duplicates.
    return list(dict.fromkeys(candidates))


def _declaration_pattern(kind: Optional[str], ident: str) -> Optional[re.Pattern]:
    escaped = re.escape(ident)
    if kind == "def":
        return re.compile(rf"\bfn\s+{escaped}\b")
    if kind == "trait":
        return re.compile(rf"\btrait\s+{escaped}\b")
    if kind in ("class", "object"):
        # Scala class/object often maps to Rust struct/enum/type/mod/impl names.
        return re.compile(rf"\b(struct|enum|type|mod|impl)\s+{escaped}\b")
    return None


def _first_identifier_match(pattern: re.Pattern, text: str) -> Optional[str]:
    m = pattern.search(text)
    if not m:
        return None
    if m.lastindex:
        # Prefer the named identifier group when present.
        return m.group("ident")
    return m.group(0)


# Scala name: alphanumeric or operators (++, --, +, etc.)
MIGALLOW_PATTERN = re.compile(
    r"//\s*MIGALLOW:\s*([A-Za-z0-9+*\/=\-!]+)\s*->\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?://|$)"
)


def find_migallow_mappings(
    rust_dir: Path, exclude_test: bool = True
) -> tuple[dict[tuple[str, str], tuple[Path, str]], set[str]]:
    """
    Scan Rust files for MIGALLOW comments: // MIGALLOW: ScalaName -> rust_ident
    Returns:
      - migallow_by_rust_file: (rust_file_stem, scala_name) -> (rust_path, rust_ident)
      - migallow_rust_idents: set of normalized rust idents that have MIGALLOW (to exclude from extras)
    """
    migallow_by_rust_file: dict[tuple[str, str], tuple[Path, str]] = {}
    migallow_rust_idents: set[str] = set()

    for root, _dirs, files in os.walk(rust_dir):
        if exclude_test and any("test" in part.lower() for part in Path(root).parts):
            continue
        for f in files:
            if not f.endswith(".rs"):
                continue
            path = Path(root) / f
            rust_stem = path.stem
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except Exception:
                continue
            for line in text.splitlines():
                m = MIGALLOW_PATTERN.search(line)
                if m:
                    scala_name, rust_ident = m.group(1), m.group(2)
                    migallow_by_rust_file[(rust_stem, scala_name)] = (path, rust_ident)
                    migallow_rust_idents.add(_normalize_identifier(rust_ident))

    return migallow_by_rust_file, migallow_rust_idents


def find_rust_function_declarations(
    rust_dir: Path, exclude_test: bool = True
) -> list[tuple[str, Path, int]]:
    """
    Return Rust function declarations as (name, path, line_number).
    Only scans implementation files when exclude_test is True.
    """
    fn_decl = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\b")
    results: list[tuple[str, Path, int]] = []

    for root, _dirs, files in os.walk(rust_dir):
        if exclude_test and any("test" in part.lower() for part in Path(root).parts):
            continue
        for f in files:
            if not f.endswith(".rs"):
                continue
            path = Path(root) / f
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except Exception:
                continue

            stripped = _strip_rust_comments(text)
            for i, line in enumerate(stripped.splitlines(), 1):
                m = fn_decl.search(line)
                if not m:
                    continue
                name = m.group(1)
                results.append((name, path, i))

    return results


def find_extra_rust_functions(
    rust_dir: Path,
    mapped_rust_identifiers: set[str],
    exclude_test: bool = True,
) -> list[tuple[str, Path, int]]:
    """
    Find Rust function declarations whose normalized names aren't in mapped identifiers.
    """
    extras: list[tuple[str, Path, int]] = []
    seen: set[tuple[str, str]] = set()

    for name, path, line in find_rust_function_declarations(
        rust_dir, exclude_test=exclude_test
    ):
        if _normalize_identifier(name) in mapped_rust_identifiers:
            continue
        key = (str(path), name)
        if key in seen:
            continue
        seen.add(key)
        extras.append((name, path, line))

    extras.sort(key=lambda x: (str(x[1]), x[2], x[0]))
    return extras


def find_scala_definitions(scala_dir: Path) -> list[tuple[str, str, Path, int]]:
    """
    Scan Scala files for class, trait, object, and def declarations.
    Returns list of (identifier, kind, file_path, line_number).
    """
    results = []
    # Match: class/trait/object Name, def name
    class_trait_object = re.compile(
        r"^\s*(class|trait|object)\s+([A-Za-z0-9_]+)\s*[({\s]"
    )
    # Match def name(...), def name[...], def name: Type (property-style)
    # Also match private[parsing] def / protected[foo] def
    def_match = re.compile(
        r"^\s*(?:private(?:\[[^\]]+\])?|protected(?:\[[^\]]+\])?)?\s*(?:override\s+)?def\s+([A-Za-z0-9_]+)\s*[(\[:]"
    )

    for root, _dirs, files in os.walk(scala_dir):
        for f in files:
            if not f.endswith(".scala"):
                continue
            path = Path(root) / f
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except Exception as e:
                print(f"Warning: could not read {path}: {e}", file=sys.stderr)
                continue
            for i, line in enumerate(text.splitlines(), 1):
                m = class_trait_object.search(line)
                if m:
                    kind, name = m.group(1), m.group(2)
                    results.append((name, kind, path, i))
                    continue
                m = def_match.search(line)
                if m:
                    name = m.group(1)
                    # Skip operators and special names
                    if name in ("++", "--", "+", "-", "*", "/", "==", "!=", "unapply"):
                        continue
                    results.append((name, "def", path, i))
    return results


def find_rust_identifier_path(
    rust_dir: Path,
    snake_name: str,
    exclude_test: bool = True,
    scala_name: Optional[str] = None,
    scala_kind: Optional[str] = None,
    scala_path: Optional[Path] = None,
    migallow_by_rust_file: Optional[dict[tuple[str, str], tuple[Path, str]]] = None,
) -> Optional[tuple[Path, str]]:
    """
    Check if the snake_case identifier appears in Rust code as a whole word.
    Matches: fn name, struct Name (PascalCase), name(, name {, etc.
    If scala_path and migallow_by_rust_file are provided, checks MIGALLOW first.
    """
    # Check MIGALLOW first when we have Scala context
    if scala_path is not None and migallow_by_rust_file is not None and scala_name is not None:
        rust_stem = camel_to_snake(scala_path.stem)
        key = (rust_stem, scala_name)
        if key in migallow_by_rust_file:
            return migallow_by_rust_file[key]

    candidates = _identifier_candidates(snake_name, scala_name)
    whole_word_patterns = [
        re.compile(r"\b(?P<ident>" + re.escape(c) + r")\b") for c in candidates
    ]
    declaration_patterns = {
        c: (
            re.compile(
                _declaration_pattern(scala_kind, c).pattern.replace(
                    re.escape(c), rf"(?P<ident>{re.escape(c)})"
                )
            )
            if _declaration_pattern(scala_kind, c)
            else None
        )
        for c in candidates
    }
    identifier_pattern = re.compile(r"\b[A-Za-z_][A-Za-z0-9_]*\b")
    best_match: Optional[tuple[Path, str]] = None
    best_match_exact_fuzzy: bool = False
    best_match_rust_norm_len: int = 0

    for root, _dirs, files in os.walk(rust_dir):
        if exclude_test and any("test" in part.lower() for part in Path(root).parts):
            continue
        for f in files:
            if not f.endswith(".rs"):
                continue
            path = Path(root) / f
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except Exception:
                continue
            searchable_text = _strip_rust_comments(text)
            # Prefer declaration sites over plain identifier use (exact match).
            for _candidate, decl_pat in declaration_patterns.items():
                if decl_pat and decl_pat.search(searchable_text):
                    ident = _first_identifier_match(decl_pat, searchable_text)
                    if ident:
                        return (path, ident)
            for pat in whole_word_patterns:
                ident = _first_identifier_match(pat, searchable_text)
                if ident:
                    if best_match is None:
                        best_match = (path, ident)
                        best_match_exact_fuzzy = _normalize_fuzzy(ident) == _normalize_fuzzy(snake_name)
                        best_match_rust_norm_len = len(_normalize_fuzzy(ident))
                    break
            # Fallback fuzzy match: ignore case and underscores.
            # Example: merge matches merge_uses, getImpreciseName matches get_imprecise_name.
            for ident in identifier_pattern.findall(searchable_text):
                if _fuzzy_matches(ident, candidates):
                    exact = _normalize_fuzzy(ident) == _normalize_fuzzy(snake_name)
                    rust_len = len(_normalize_fuzzy(ident))
                    if best_match is None:
                        best_match = (path, ident)
                        best_match_exact_fuzzy = exact
                        best_match_rust_norm_len = rust_len
                    else:
                        # Prefer exact fuzzy match; among prefix matches prefer shortest Rust ident.
                        if exact and not best_match_exact_fuzzy:
                            best_match = (path, ident)
                            best_match_exact_fuzzy = True
                            best_match_rust_norm_len = rust_len
                        elif exact == best_match_exact_fuzzy and rust_len < best_match_rust_norm_len:
                            best_match = (path, ident)
                            best_match_rust_norm_len = rust_len
    return best_match


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Check Scala definitions have corresponding Rust identifiers"
    )
    parser.add_argument(
        "--scala-dir",
        default=Path(__file__).resolve().parent.parent.parent / "Frontend" / "PostParsingPass" / "src" / "dev" / "vale" / "postparsing",
        type=Path,
        help="Root of Scala postparsing source",
    )
    parser.add_argument(
        "--rust-dir",
        default=Path(__file__).resolve().parent.parent / "src" / "postparsing",
        type=Path,
        help="Root of Rust postparsing source",
    )
    parser.add_argument(
        "--exclude-test",
        action="store_true",
        default=True,
        help="Exclude Rust test directory (default: True)",
    )
    parser.add_argument(
        "--include-test",
        action="store_false",
        dest="exclude_test",
        help="Include Rust test directory in search",
    )
    args = parser.parse_args()

    scala_dir = args.scala_dir
    rust_dir = args.rust_dir

    if not scala_dir.is_dir():
        print(f"Error: Scala dir not found: {scala_dir}", file=sys.stderr)
        sys.exit(1)
    if not rust_dir.is_dir():
        print(f"Error: Rust dir not found: {rust_dir}", file=sys.stderr)
        sys.exit(1)

    definitions = find_scala_definitions(scala_dir)
    seen = set()  # (name, kind) to avoid duplicates
    missing = []
    found = []

    for name, kind, path, line in definitions:
        key = (name, kind)
        if key in seen:
            continue
        seen.add(key)

        snake = camel_to_snake(name)
        try:
            rel_path = path.relative_to(scala_dir)
        except ValueError:
            rel_path = path

        rust_match = find_rust_identifier_path(
            rust_dir,
            snake,
            exclude_test=args.exclude_test,
            scala_name=name,
            scala_kind=kind,
        )
        if rust_match:
            rust_match_path, matched_ident = rust_match
            try:
                rust_rel = rust_match_path.relative_to(rust_dir.parent)
            except ValueError:
                rust_rel = rust_match_path
            found.append((name, kind, snake, str(rel_path), line, str(rust_rel), matched_ident))
        else:
            missing.append((name, kind, snake, str(rel_path), line))

    # Report
    print("=== Scala -> Rust mapping check ===\n")
    print(f"Scala dir: {scala_dir}")
    print(f"Rust dir:  {rust_dir}\n")

    if found:
        print("FOUND (with Rust file path):\n")
        for name, kind, snake, rel, ln, rust_rel, matched_ident in sorted(
            found, key=lambda x: (x[1], x[0])
        ):
            print(
                f"{rel}:{ln} {kind} {name} -> {snake} in {rust_rel} (matched: {matched_ident})"
            )
        print(f"\nTotal found: {len(found)}")
        print("")
    else:
        print("No Scala definitions found in Rust.\n")

    if missing:
        print("MISSING (no corresponding snake_case or PascalCase in Rust):\n")
        for name, kind, snake, rel, ln in sorted(missing, key=lambda x: (x[1], x[0])):
            print(f"{rel}:{ln} {kind} {name} -> {snake} in <missing>")
        print(f"\nTotal missing: {len(missing)}")
    else:
        print("All Scala definitions have a corresponding identifier in Rust.")

    print(f"Total checked: {len(seen)}")
    sys.exit(1 if missing else 0)


if __name__ == "__main__":
    main()
