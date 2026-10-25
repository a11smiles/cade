# 0003 · Copilot SDK for both Copilot and BYOK

**Status:** accepted · 2026-10-25

**Decision.** All model access goes through the Rust `github-copilot-sdk` (pinned `=1.0.26`). With a Copilot seat, sessions use the user's GitHub identity. With BYOK, the same SDK session gets a `ProviderConfig` (Anthropic, OpenAI, Azure OpenAI or OpenAI-compatible), with the key read from the OS keychain at session start. CADE gets one agent loop, one tool API, one permission handler and one token meter.

Release builds enable the SDK's `bundled-cli` feature so the Copilot runtime ships inside the app. Development builds use `runtime`.

**Consequences.** BYOK multi-provider is marked experimental in the SDK. CADE wraps it in `cade-copilot::ModelSource` so an API change is contained. Usage is metered in tokens.
