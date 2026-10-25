---
id: CANVAS-001
title: Embed a native wgpu surface under the Tauri webview
tags: [canvas, rust, feature, windows, macos, linux]
epic: CANVAS
priority: P0
depends_on: [FOUND-003, FOUND-006]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Create the `cade-render` surface host that attaches a wgpu surface to the Tauri main window and composites it beneath a transparent webview that draws the app chrome. On macOS use a CAMetalLayer sublayer, on Windows a DirectComposition/DXGI swapchain child, on Linux a Wayland subsurface or X11 child window. The canvas region is a rectangle the webview reports, so panels can overlay or sit beside it.

## Acceptance criteria

- [ ] Canvas renders a cleared color under a transparent webview on macOS, Windows and Linux (Wayland and X11)
- [ ] Webview reports the canvas rect on layout and resize; the surface tracks it within one frame
- [ ] Pointer events over the canvas rect reach Rust; events over chrome stay in the webview
- [ ] HiDPI scale factor changes (moving between monitors) reconfigure the surface without artifacts
- [ ] Falls back to a software path (wgpu GL/WARP) with a visible notice if no adapter is found

## Notes

Spike the Linux path first; WebKitGTK transparency varies by compositor. Record results in an ADR.
