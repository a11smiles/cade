---
id: IMPORT-011
title: Keep GitHub auth and report Spark-hosted deploys
tags: [import, feature, rust]
epic: IMPORT
priority: P1
depends_on: [IMPORT-006]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Map Spark's GitHub authentication to standard GitHub OAuth in the generated app (configurable for org
SSO). Spark-hosted deployments are not migrated; the report points to 'Deploy app' to choose an
architecture.

## Acceptance criteria

- [ ] Generated app signs in with GitHub using configured client settings
- [ ] The report notes that Spark-hosted deploys were not moved
- [ ] The success state links to the System view to plan deployment
