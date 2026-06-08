# zed-text-marker

A private Zed extension that toggles marks on hotkey and underlines every matching occurrence in the open documents.

It reimplements VSCode's text-marker as a Rust LSP plus a thin Zed wasm wrapper. Zed has no arbitrary-highlight API (issue #49438), so it reuses `publishDiagnostics` underline rendering. Marks show up as underlines (not background fills), in a single severity (Error).

## Layout

- `server/` — native `text-marker` binary (LSP server plus the toggle/clear/install CLI)
- `extension/` — Zed wasm wrapper extension
- `assets/` — sample keymap / tasks

## Install

1. Put the binary on a directory that is on your `PATH` so Zed can resolve it:

   ```bash
   cargo install --path server --root ~/.local
   ```

   This assumes `~/.local/bin` is on your `PATH`. If `~/.cargo/bin` is on your `PATH`, plain `cargo install --path server` works too.

2. Set up the Zed tasks and the marks directory:

   ```bash
   text-marker install
   ```

   This idempotently merges two tasks into `~/.config/zed/tasks.json` (existing tasks are preserved) and creates the marks directory. It also prints the remaining steps (3 and 4 below).

3. Register the dev extension in Zed: run `zed: install dev extension` from the command palette and select this repo's `extension/` directory. This is a GUI action and cannot be done from the CLI.

4. Add the bindings to your Zed keymap by merging `assets/keymap.json`. Two contexts are used:

   ```json
   // Editor && vim_mode == normal
   "m h": ["workspace::SendKeystrokes", "v i w m h"],
   "m shift-h": ["task::Spawn", { "task_name": "text-marker: clear" }]

   // Editor && vim_mode == visual
   "m h": ["task::Spawn", { "task_name": "text-marker: toggle" }]
   ```

   The normal-mode `m h` selects the inner word and then fires the visual-mode `m h`. Zed tasks have no variable for "the word under the cursor" and can only pass `$ZED_SELECTED_TEXT` (the selection), so this binding toggles the word under the cursor. A trailing `escape` cannot be added to return to normal mode: the task reads `$ZED_SELECTED_TEXT` after the keystrokes finish, so an `escape` would clear the selection before the task sees it. The editor stays in visual mode after `m h`.

## Usage

- normal mode `m h` — toggle the mark on the word under the cursor (every occurrence of the word lights up)
- visual mode `m h` — toggle the mark on the selected text
- normal mode `m H` — clear all marks

Marks are persisted globally in `~/.config/zed/text-marker/marks.json`. After `m h` the editor stays in visual mode; press `escape` or just keep moving to leave it.
