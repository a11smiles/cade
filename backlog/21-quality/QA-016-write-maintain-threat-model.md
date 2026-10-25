---
id: QA-016
title: Write and maintain the threat model
tags: [qa, docs, security]
epic: QA
priority: P0
depends_on: [FOUND-003]
---

## Description

STRIDE threat model covering: the Copilot agent's tools (file edits, shell, network), generated-app preview servers, importers parsing untrusted files, deep links, review pages, policy tampering, token and key theft, update channel compromise, and prompt injection via imported content. Each threat maps to a mitigation and a test or backlog item.

## Acceptance criteria

- [ ] `docs/security/threat-model.md` covers each component listed
- [ ] Every high-risk threat has a linked mitigation item
- [ ] The model is reviewed before each minor release
