---
id: FEED-012
title: Redact design content, prompts and paths by default
tags: [feed, feedback, feature, rust, security]
epic: FEED
priority: P0
depends_on: [FEED-010]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

Feedback never includes document content, Copilot prompts or threads, or file paths unless the user explicitly attaches them. User-typed text is scanned for home-directory paths, emails and token-like strings and the user is warned with one-click redaction.

## Acceptance criteria

- [ ] Default payload contains no `.cade` data, prompts or paths (test with a seeded project)
- [ ] Typed home-directory paths are detected and offered for redaction as `~/…`
- [ ] Token-like strings (ghp_, sk-, etc.) block submit until removed
- [ ] Attaching a run log requires an explicit checkbox and shows its contents in preview
