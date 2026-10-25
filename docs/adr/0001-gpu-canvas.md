# 0001 · Native GPU canvas under a transparent webview

**Status:** accepted · 2026-10-25

**Context.** CADE has to match Figma-level performance with thousands of layers on macOS, Windows and Linux. A DOM-based canvas is quick to build but stalls on large files. WebGPU inside the webview isn't reliable on every platform (WebKitGTK in particular).

**Decision.** The canvas is a native wgpu surface owned by Rust and rendered with Vello (text: Parley, layout: Taffy). The Tauri webview sits on top, transparent where the canvas shows through, and carries all chrome. Generated apps preview in a separate sandboxed webview.

**Consequences.** Input over the canvas is routed from the webview to Rust (CANVAS backlog). Window compositing differs per OS and needs per-platform work in FOUND. Rendering is shared by the canvas, exports and Copilot's visual self-check.
