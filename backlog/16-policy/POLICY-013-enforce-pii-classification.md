---
id: POLICY-013
title: Require PII classification in data models
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-002, DATA-008]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

With `outputs.pii_classification: required`, every column in a data model must have a classification (None, PII, Confidential…) before migration ship. Columns with names matching common PII patterns (email, phone, ssn, dob, name) are pre-flagged for review.

## Acceptance criteria

- [ ] Unclassified columns block 'Ship migration to repo'
- [ ] Heuristics pre-flag likely PII columns
- [ ] Classification is written into the migration as a comment or extended property
- [ ] Rule badge shows in the Data model header

## Notes

Absorbs DATA-009, which duplicated this item and was removed during backlog reconciliation.
