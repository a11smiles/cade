---
id: COPILOT-037
title: Handle an expired GitHub session without losing work
tags: [copilot, feature, ui]
epic: COPILOT
priority: P0
depends_on: [GH-001]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

When the GitHub token expires or is revoked, keep the prompt, show 'Your GitHub session expired', and offer Sign in with GitHub (device flow in the browser with org SSO) or Not now (keep working offline). After sign-in, the pending prompt sends automatically if the user confirms.

## Acceptance criteria

- [ ] 401 from Copilot triggers the state instead of an error toast
- [ ] Pending prompt survives sign-in and is offered for sending
- [ ] SSO-required orgs prompt SSO authorization in the browser
- [ ] Ship and Deploy show the same sign-in requirement
