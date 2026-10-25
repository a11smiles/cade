---
id: REL-013
title: Build the documentation site
tags: [rel, docs]
epic: REL
priority: P1
depends_on: [REL-004]
---

## Description

A docs site (Astro Starlight or mdBook) published to GitHub Pages: getting started per OS, a guide for each output type, Copilot and BYOK setup, policy reference for admins, the .cade format spec, CLI reference, troubleshooting and the component catalog. Versioned per minor release.

## Acceptance criteria

- [ ] Site builds in CI and deploys on release
- [ ] Each output type has a getting-started guide
- [ ] Docs are versioned and the version picker works
- [ ] Search works offline in the built site
