---
id: ONB-007
title: Offer 'Use my own API key instead' during onboarding
tags: [onb, feature, ui]
epic: ONB
priority: P1
depends_on: [COPILOT-005, COPILOT-009, POLICY-007]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

The link opens a BYOK setup sheet: pick provider, paste key or configure endpoint, test connection, save to keychain. Hidden when policy disables BYOK. GitHub sign-in is still required for Ship, Deploy, history sharing and feedback.

## Acceptance criteria

- [ ] Link is hidden when BYOK is disabled by policy
- [ ] Test connection must pass before saving
- [ ] Explains that GitHub sign-in is still needed for GitHub features
- [ ] Completing BYOK setup advances to the projects folder step
