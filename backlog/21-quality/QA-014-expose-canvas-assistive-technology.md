---
id: QA-014
title: Expose the canvas to assistive technology
tags: [qa, feature, a11y, rust]
epic: QA
priority: P1
depends_on: [QA-012, CORE-024]
---

## Description

The native canvas is invisible to screen readers by default. Mirror the selected node and its neighbors through an accessible layer tree in the webview with names, kinds and positions, announce selection and property changes, and allow moving and resizing with the keyboard.

## Acceptance criteria

- [ ] Screen readers announce the selected layer's name, kind, size and position
- [ ] Arrow keys move the selection; modifier+arrow resizes
- [ ] The layers tree and canvas selection stay in sync
