---
id: REL-014
title: Generate reference docs for format, policy and CLI
tags: [rel, docs]
epic: REL
priority: P1
depends_on: [REL-013, CORE-015, CORE-028]
---

## Description

Generate reference pages from source: JSON Schemas for the manifest, project JSON and `policy.yml`; CLI help output; error codes; and rustdoc for public crates. CI fails if generated reference is stale.

## Acceptance criteria

- [ ] Reference pages are generated, not hand-written
- [ ] Stale generated docs fail CI
- [ ] Each schema page includes an example
