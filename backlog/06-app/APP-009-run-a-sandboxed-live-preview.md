---
id: APP-009
title: Run a sandboxed live preview
tags: [app, rust, feature, security]
epic: APP
priority: P0
depends_on: [APP-005, FOUND-003]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

Run the project's Vite dev server as a child process inside the project folder with no network egress except localhost, and show it in a separate webview. Process lifetime is tied to the project window. Port shows in the header (e.g. localhost:5173).

## Acceptance criteria

- [ ] Preview starts within 3 s of opening Code mode for a scaffolded project
- [ ] Hot reload reflects canvas and code edits
- [ ] The preview process can't read outside the project folder or reach the network (policy enforced)
- [ ] Closing the project stops the preview process on all three OSes
