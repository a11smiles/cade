---
id: IMPORT-005
title: Clone Spark repos read-only for import
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-004]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Shallow-clone selected repos into the staging folder with `cade-git` using the user's credentials. The
source repo is never pushed to. The new CADE project records the source repo and commit SHA.

## Acceptance criteria

- [ ] Clones use the signed-in identity and work with SSO-protected orgs
- [ ] No push or write API call is ever made to the source repo
- [ ] The new project records source repo and SHA in its metadata
