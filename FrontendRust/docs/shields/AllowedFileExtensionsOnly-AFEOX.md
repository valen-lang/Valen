---
description: Only .rs, .md, .cpp, .c, .h, and .vale files may be edited by an AI agent in this project, plus .py files under tmp/scripts/ (the safe-script-runner staging directory).
g_model: SimpleSmall
g_primary: rust
g_program: AllowedFileExtensionsOnly-AFEOX
g_context: diff
g_filter_file: "*"
g_read_when: Read when an AI agent is about to edit a file with an unfamiliar extension.
---

# Allowed File Extensions Only (AFEOX)

This project restricts AI-driven edits to a fixed set of file extensions: `.rs`, `.md`, `.cpp`, `.c`, `.h`, and `.vale`. Any edit to a file whose path doesn't end in one of these extensions is a violation, regardless of what the edit contains.

One exception: ALLOW a `.py` file when its path is under a `tmp/scripts/` directory (whether the path is relative like `tmp/scripts/foo.py` or absolute like `/…/worktree/tmp/scripts/foo.py`). That directory is the staging area for safe-script-runner bulk-edit transforms (see the scripting skill), so writing Python scripts there is sanctioned. This exception is narrow: DENY a `.py` file anywhere else, and DENY any other disallowed extension even inside `tmp/scripts/`.

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

**DENY:**
```
FILE: tmp/notes.py

+print("not in tmp/scripts/")
```

**DENY:**
```
FILE: tmp/scripts/deploy.sh

+rm -rf build/
```

**ALLOW:**
```
FILE: tmp/scripts/migrate-corpus-imports.py

+import sys, re
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
