---
id: ONB-004
title: Detect the user's Copilot seat and handle no seat
tags: [onb, feature, rust, ui]
epic: ONB
priority: P0
depends_on: [COPILOT-003]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Determine whether the user has Copilot access (Free, Pro, Business, Enterprise as exposed by the SDK/API) and show it ('Copilot Enterprise seat found · Token usage counts toward your org's Copilot budget'). Without a seat: offer Copilot Free signup link, or BYOK if policy allows, or continue without AI.

## Acceptance criteria

- [ ] Seat type is shown when available
- [ ] No-seat users see the three options
- [ ] Continue without AI opens Start with Copilot features disabled and explained
- [ ] Seat check failure due to network shows retry, not 'no seat'
