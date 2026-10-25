---
id: POLICY-001
title: Define the policy.yml v1 JSON Schema
tags: [policy, feature, rust]
epic: POLICY
priority: P0
depends_on: [FOUND-001]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

Write the JSON Schema for `cade/policy.yml` and the Rust types in `cade-policy`. Sections: `version`, `applies_to` (teams), `models` (allow list, byok, byok_endpoints), `tools` (file_edits, shell: allow|ask|deny, preapproved, network, mcp allowlist), `outputs` (design_systems, ship_via, iac_lint, pii_classification, visio_macros), `budgets` (tokens per user, per cost center, reset day), `telemetry`, `reporting`, `admin_contact`. Publish the schema in the repo so editors can validate it.

## Acceptance criteria

- [ ] Schema file `schemas/policy.v1.json` validates the sample from Admin.dc.html
- [ ] Rust types round-trip the sample YAML without loss
- [ ] Unknown keys produce a warning, not a failure, for forward compatibility
- [ ] Schema is documented in `docs/policy.md` with every key, default and example
