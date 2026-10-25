---
id: DATA-008
title: Classify fields and tables (PII, Confidential)
tags: [data, feature, security, policy]
epic: DATA
priority: P0
depends_on: [DATA-001]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Fields carry classifications such as PII, Confidential and Public; tables inherit the highest. Badges show on cards and classifications are emitted as column comments or extended properties.

## Acceptance criteria

- [ ] Fields can be tagged with one or more classifications
- [ ] Tables show the highest classification of their fields
- [ ] Generated DDL includes classifications as comments or extended properties
- [ ] Classification list is configurable by policy
