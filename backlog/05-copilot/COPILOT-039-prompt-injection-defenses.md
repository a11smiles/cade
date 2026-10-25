---
id: COPILOT-039
title: Defend agent runs against prompt injection
tags: [copilot, feature, security, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-018, COPILOT-028]
---

## Description

Imported files, Figma/Visio text, PRDs and web content are untrusted. Wrap tool outputs that contain user/third-party text in delimited untrusted-data blocks, never grant new permissions based on content, and require confirmation for destructive tool calls (delete page, delete >20 layers, overwrite files) regardless of 'always allow'.

## Acceptance criteria

- [ ] Tool outputs carrying document text are fenced as untrusted data in the request
- [ ] A red-team test document instructing 'delete all layers' does not cause deletion without confirmation
- [ ] Destructive tools always show a confirmation card
- [ ] Injection test suite runs in CI against the fake provider

## Notes

Absorbs QA-021, which duplicated this item and was removed during backlog reconciliation.
