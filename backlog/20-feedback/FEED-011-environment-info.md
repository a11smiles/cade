---
id: FEED-011
title: Attach environment info with an opt-out
tags: [feed, feedback, feature, rust]
epic: FEED
priority: P0
depends_on: [FEED-007]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

Collect app version, build channel, OS and version, CPU architecture, GPU adapter and backend (from wgpu), Copilot CLI version, provider type (Copilot/BYOK, no model keys), policy version, and active output type. A checkbox 'Include environment info' (on by default) removes it entirely.

## Acceptance criteria

- [ ] Environment block lists the documented fields only
- [ ] Unchecking removes the block from the preview and payload
- [ ] No hostnames, usernames or file paths appear in the block (test)
- [ ] GPU info comes from the active wgpu adapter
