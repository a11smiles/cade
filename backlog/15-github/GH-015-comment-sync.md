---
id: GH-015
title: Sync review comments with the PR both ways
tags: [gh, feature, rust]
epic: GH
priority: P0
depends_on: [GH-014]
design: design/exports/v0001-2026-10-25/Review.dc.html
---

## Description

Comments placed on the review page post as PR review comments with frame and position metadata; replies on GitHub appear on the page and in CADE as pins.

## Acceptance criteria

- [ ] A comment on the page appears on the PR within 10 seconds
- [ ] A reply on GitHub shows on the page and in CADE
- [ ] Resolving on either side resolves on the other
- [ ] Pin positions survive design changes where the layer still exists

## Notes

Absorbs HIST-012, which duplicated this item and was removed during backlog reconciliation.
