---
id: APP-006
title: Sync design and code both ways
tags: [app, rust, feature]
epic: APP
priority: P0
depends_on: [APP-005]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

Keep a source map between layers and code ranges. Canvas edits update the code; code edits (in the editor or on disk) update the canvas when they parse to known patterns. Conflicts show a merge prompt. The header shows 'Design and code in sync'.

## Acceptance criteria

- [ ] Changing a button variant on canvas updates the JSX prop within 1 s
- [ ] Editing a JSX prop in the editor updates the canvas instance
- [ ] External edits to files on disk are detected and synced
- [ ] Unparseable code edits mark the frame 'code-owned' and stop overwriting it
- [ ] The sync status indicator reflects in-sync, syncing and conflict states
