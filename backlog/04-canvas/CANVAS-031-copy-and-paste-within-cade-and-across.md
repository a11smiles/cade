---
id: CANVAS-031
title: Copy and paste within CADE and across apps
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-004]
---

## Description

Copy/paste layers within and between CADE projects with full fidelity (CRDT subtree + assets). Paste from other apps: SVG markup, images, plain and rich text, and Figma clipboard where present. Copy out as SVG, PNG and CSS for the selection.

## Acceptance criteria

- [ ] Copy-paste between two CADE windows keeps components, tokens and images
- [ ] Pasting SVG creates editable vector layers
- [ ] Pasting an image or text from the browser or Office creates the right layer type
- [ ] Copy as SVG, PNG and CSS put correct content on the clipboard on all three OSes
