---
id: QA-025
title: Set up bug triage and severity rules
tags: [qa, chore]
epic: QA
priority: P0
depends_on: [QA-001, FEED-005]
---

## Description

Define severity levels (S1 data loss or security, S2 broken primary flow, S3 degraded, S4 cosmetic), response targets, and labels that work with the in-app bug reports. Weekly triage of new issues.

## Acceptance criteria

- [ ] Severity definitions and targets are in `CONTRIBUTING.md`
- [ ] Labels are created by a script from `.github/labels.yml`
- [ ] S1 issues page the maintainer

## Notes

Issue types and in-app submission are defined in FEED.
