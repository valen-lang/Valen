## 1: Remove cmd-K from builtin terminal

Press Ctrl+Shift+P and type "Preferences: Open Keyboard Shortcuts (JSON)"
Add a keybinding like this:

```
{
   "key": "cmd+k",
   "command": "workbench.action.terminal.clear",
},
```

Might need to somehow remove all the conflicting cmd+k keybindings...

## 2: Configure building

 * cmd-shift-p
 * Tasks: Configure Task
    * rust: cargo build
 * In the file that pops up, add `"options": { "cwd": "${workspaceFolder}/FrontendRust" }`

Restart editor.

## 3: Set build hotkey

Press Ctrl+Shift+P and type "Preferences: Open Keyboard Shortcuts (JSON)"
Add a keybinding like this:

```
{
   "key": "cmd+shift+b",
   "command": "workbench.action.tasks.runTask",
   "args": "rust: cargo build"
},
```