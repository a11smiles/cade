---
id: FOUND-028
title: Automate tagged releases with provenance
tags: [found, chore, security]
epic: FOUND
priority: P0
depends_on: [FOUND-023, FOUND-024, FOUND-025, FOUND-022]
---

## Description

A release workflow triggered by a signed `v*` tag builds all installers, generates release notes from merged PRs, uploads artifacts with checksums to GitHub Releases, and creates build provenance attestations so users can verify artifacts with `gh attestation verify`.

## Acceptance criteria

- [ ] Pushing a tag produces a draft release with all six installer formats
- [ ] Each artifact has a SHA-256 and a GitHub build provenance attestation
- [ ] Release notes list user-facing changes grouped by surface
- [ ] Release jobs run in a protected environment with required approval
