---
id: DOC-014
title: Implement the print check rules engine
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [DOC-002, DOC-007, DOC-010]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Rules evaluated after pagination: minimum body size (default 11 pt, configurable by policy), margins
(default 0.75 in), no widows or split callouts, no unfilled placeholders, image resolution of at least
150 ppi (warning) and 300 ppi (pass), text contrast, required classification label. Each rule returns pass,
warning or error with the locations it applies to.

## Acceptance criteria

- [ ] All rules run in under 200 ms for a 30-page document
- [ ] Each finding links to a page and block
- [ ] Thresholds can be set by org policy and are shown in the rule text
- [ ] Unit tests cover every rule with pass and fail fixtures
