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
pnpm --dir app/ui exec playwright install chromium   # once, for end-to-end tests
pnpm --dir app/ui test:e2e

# Desktop app (needs the platform webview: WebView2 on Windows, WKWebView on macOS,
# webkit2gtk-4.1 on Linux)
cargo run -p cade-app
```

## Rules of the road

- Participation is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).
- One backlog item per pull request where possible. Put the id in the PR title.
- Every change keeps `cargo fmt`, `cargo clippy -D warnings`, `cargo test` and `pnpm --dir app/ui test` green.
- UI values come from design tokens only. Edit `design/system/tokens.json`, never `tokens.css`.
- Check dark and light themes.
- Don't commit Microsoft's Azure icon files (see `assets/icons/azure/README.md`).
- No telemetry, no network calls outside the ones a backlog item names.

## Feedback without code

Choose the issue form that best fits your feedback:

- **Bug:** Something is broken or wrong.
- **Feature request:** A capability or change CADE should add.
- **Comment or question:** Praise, questions, or general feedback. Comment submissions are triaged weekly and may be converted to Discussions.

You can also use Help › Send feedback inside CADE.
