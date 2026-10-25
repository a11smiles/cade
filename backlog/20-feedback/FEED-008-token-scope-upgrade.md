---
id: FEED-008
title: Handle token scope for issue creation and prompt to upgrade
tags: [feed, feedback, feature, rust, security]
epic: FEED
priority: P0
depends_on: [FEED-007, GH-001]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

The token used for Copilot may not include permission to create issues on the CADE repo. Check the token's capabilities before submit; if missing, show 'Allow CADE to post feedback as you' and run an incremental authorization (GitHub App user token with Issues: write on the CADE repo, or OAuth `public_repo` for OAuth-app builds). Never request broader scopes than needed.

## Acceptance criteria

- [ ] Missing permission is detected before the user fills the form
- [ ] Upgrade prompt explains exactly what permission is requested and why
- [ ] After authorization, submit succeeds without re-entering the form
- [ ] Declining keeps the draft and offers 'Open on GitHub instead' with the form prefilled via URL
