# Contributing to CADE

Thanks for helping. CADE is a personal open-source project maintained by Joshua Davis under Apache-2.0.

## Find something to work on

The whole plan lives in [GitHub issues](https://github.com/a11smiles/cade/issues): about 535 items in 22 epics, each titled with its id (for example `[CORE-004]`) and carrying acceptance criteria and dependencies. Start with the issues labeled `milestone:M1`. Comment on the issue before starting anything large.

## Build

```sh
# Rust crates (no system webview needed)
cargo test

# UI
pnpm --dir app/ui install
pnpm --dir app/ui build
pnpm --dir app/ui test

# Desktop app (needs the platform webview: WebView2 on Windows, WKWebView on macOS,
# webkit2gtk-4.1 on Linux)
cargo run -p cade-app
```

## Rules of the road

- One backlog item per pull request where possible. Put the id in the PR title.
- Every change keeps `cargo fmt`, `cargo clippy -D warnings`, `cargo test` and `pnpm --dir app/ui test` green.
- UI values come from design tokens only. Edit `design/system/tokens.json`, never `tokens.css`.
- Check dark and light themes.
- Don't commit Microsoft's Azure icon files (see `assets/icons/azure/README.md`).
- No telemetry, no network calls outside the ones a backlog item names.

## Feedback without code

Use the issue forms (bug, feature request, comment), or Help › Send feedback inside CADE.
