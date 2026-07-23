---
description: Only .rs, .md, .cpp, .c, .h, and .vale files may be edited by an AI agent in this project.
g_model: SimpleSmall
g_primary: rust
g_program: AllowedFileExtensionsOnly-AFEOX
g_context: diff
g_filter_file: "*"
g_read_when: Read when an AI agent is about to edit a file with an unfamiliar extension.
---

# Allowed File Extensions Only (AFEOX)

This project restricts AI-driven edits to a fixed set of file extensions: `.rs`, `.md`, `.cpp`, `.c`, `.h`, and `.vale`. Any edit to a file whose path doesn't end in one of these extensions is a violation, regardless of what the edit contains.

## Examples

**DENY:**
```
FILE: scripts/build.py

+import subprocess
```

**DENY:**
```
FILE: Cargo.toml

+[workspace]
```

**DENY:**
```
FILE: Backend/src/foo.hpp

+void foo();
```

**ALLOW:**
```
FILE: Backend/src/externs.cpp

+  fwrite = addExtern(mod, "fwrite", sizeTLT, {int8PtrLT, sizeTLT, sizeTLT, int8PtrLT});
```

**ALLOW:**
```
FILE: FrontendRust/src/typing/mod.rs

+pub fn check_types() {}
```

**ALLOW:**
```
FILE: docs/architecture/arenas.md

+Arenas are immutable after construction.
```

**ALLOW:**
```
FILE: FrontendRust/src/tests/programs/virtuals/interfaceimm.vale

+My
```
