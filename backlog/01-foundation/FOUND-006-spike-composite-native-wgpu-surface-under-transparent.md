---
id: FOUND-006
title: "Spike: composite a native wgpu surface under a transparent webview"
tags: [found, spike, rust, macos, windows, linux]
epic: FOUND
priority: P0
depends_on: [FOUND-003]
---

## Description

Prove the window architecture: a wgpu surface owned by the Rust side fills the canvas region while the webview draws chrome on top with a transparent background, and pointer events in the canvas region reach Rust. Validate on Metal (macOS), D3D12/Vulkan (Windows) and Vulkan/GL (Linux, X11 and Wayland). Write an ADR with the chosen approach per OS and its limits.

## Acceptance criteria

- [ ] Demo app shows an animated wgpu clear color behind React panels on all three OSes
- [ ] Pointer and wheel events over the canvas region are delivered to Rust with under 8 ms added latency
- [ ] Resize and DPI changes keep surface and webview aligned with no gaps
- [ ] ADR `docs/adr/0006-canvas-compositing.md` records the approach, fallbacks and known issues
- [ ] Wayland and X11 are both tested on Linux
