---
id: FOUND-005
title: Define the cross-layer error model and user-facing error mapping
tags: [found, feature, rust, ts]
epic: FOUND
priority: P0
depends_on: [FOUND-004]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

Create `CadeError` in `cade-core` with stable machine codes (e.g. `policy.blocked`, `budget.exhausted`, `auth.expired`, `net.offline`, `io.permission`), a human message, an optional remediation list and a correlation id. The UI maps codes to the failure-state patterns in the States screen: say what happened, show what still works, offer at most three next steps.

## Acceptance criteria

- [ ] Every error crossing IPC carries a code from a documented enum
- [ ] Unknown codes render a generic but non-empty message with a copyable correlation id
- [ ] Error codes are listed in `docs/errors.md` with the UI treatment for each
- [ ] Unit tests cover serialization of every error variant
