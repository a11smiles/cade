---
id: QA-013
title: Test with VoiceOver, NVDA and Orca
tags: [qa, test, a11y, macos, windows, linux]
epic: QA
priority: P1
depends_on: [QA-012]
---

## Description

Manual and scripted screen reader passes on macOS (VoiceOver), Windows (NVDA, Narrator) and Linux (Orca) for first run, a Copilot run with review and apply, ship, settings and failure states.

## Acceptance criteria

- [ ] Each flow is completed with each screen reader and results recorded
- [ ] Copilot streaming output is announced without flooding
- [ ] Staged changes and failure states are announced
