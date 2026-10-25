---
id: DATA-006
title: Lay out and route ER diagrams
tags: [data, feature, rust]
epic: DATA
priority: P1
depends_on: [DATA-005, CANVAS-041]
---

## Description

Auto layout for ER diagrams minimizing crossings and orthogonal routing that keeps lines attached near the related fields. Shares the router with Architecture and Diagram.

## Acceptance criteria

- [ ] Auto layout of a 30-entity model has no overlapping cards
- [ ] Lines attach next to the FK field row
- [ ] Manual positions persist

## Notes

Router shared with ARCH-009.
