# zed-text-marker

選択した文字列をホットキーでトグルし、開いているドキュメントの一致箇所に下線ハイライトを出す Zed 用 private 拡張。

VSCode の text-marker 相当を、Rust 製 LSP + Zed wasm ラッパーで実現する。Zed には任意ハイライト API がない(issue #49438)ため、`publishDiagnostics` の下線描画を流用している。背景ベタ塗りではなく下線、色は単色(Information)。

## 構成

- `server/` — native バイナリ `text-marker`(LSP本体 + toggle/clear CLI)
- `extension/` — Zed wasm ラッパー拡張
- `assets/` — keymap / tasks のサンプル

## インストール

1. バイナリを PATH に置く。Zed がコマンドを解決できるよう、PATH 上のディレクトリにインストールする:

   ```bash
   cargo install --path server --root ~/.local
   ```

   `~/.local/bin` が PATH に含まれている前提。`~/.cargo/bin` を PATH に通している場合は `cargo install --path server` でもよい。

2. Zed に dev extension として登録: コマンドパレットで `zed: install dev extension` を実行し、この repo の `extension/` ディレクトリを選ぶ。

3. `assets/tasks.json` の2タスクを Zed の tasks(`~/.config/zed/tasks.json`)に追加する。

4. `assets/keymap.json` のバインドを Zed の keymap(`~/.config/zed/keymap.json`)に追加する。

## 使い方(vim normal mode)

- `m h` — 選択文字列のマークをトグル
- `m H` — 全マークを消す

マークは `~/.config/zed/text-marker/marks.json` にグローバルに永続化される。
