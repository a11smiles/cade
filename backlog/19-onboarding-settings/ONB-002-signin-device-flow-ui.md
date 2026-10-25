---
id: ONB-002
title: Sign in with GitHub using the device flow
tags: [onb, feature, ui]
epic: ONB
priority: P0
depends_on: [ONB-001, GH-001]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Step 1 shows 'Sign in with GitHub': display the device code, open the verification URL in the default browser, poll until authorized, then show 'Signed in with GitHub · <login> · <enterprise/org>'. Supports github.com, GHE.com and GHES host selection (ONB-017).

## Acceptance criteria

- [ ] Device code and URL are shown and copyable
- [ ] Successful authorization advances automatically
- [ ] Expired or denied codes show a retry
- [ ] Signed-in row shows login and primary enterprise or org
