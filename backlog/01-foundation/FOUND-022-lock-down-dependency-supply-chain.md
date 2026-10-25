---
id: FOUND-022
title: Lock down the dependency supply chain
tags: [found, chore, security]
epic: FOUND
priority: P0
depends_on: [FOUND-021]
---

## Description

Add `cargo-deny` (licenses, advisories, bans, sources), `cargo-audit`, `pnpm audit`, Dependabot for cargo/npm/actions, and SBOM generation (CycloneDX) per release. Allowed licenses are compatible with Apache-2.0.

## Acceptance criteria

- [ ] CI fails on a known-vulnerable crate or npm package
- [ ] CI fails on a dependency with a disallowed license
- [ ] Each release attaches CycloneDX SBOMs for Rust and JS
- [ ] Dependabot PRs are grouped weekly
