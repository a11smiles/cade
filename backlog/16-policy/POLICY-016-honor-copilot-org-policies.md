---
id: POLICY-016
title: Honor Copilot org policies exposed by the SDK
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [COPILOT-003, POLICY-006]
---

## Description

Read whatever Copilot policy the SDK exposes for the signed-in user (enabled models, content exclusions, feature enablement) and merge it with CADE policy, strictest wins. Content-excluded files are never attached as context or read by agent tools.

## Acceptance criteria

- [ ] A model disabled in Copilot org settings is unavailable in CADE
- [ ] Files matching content exclusions are excluded from context with a visible reason
- [ ] If the SDK exposes no policy data, CADE policy alone applies and Settings says so
- [ ] Merged rules show their source (Copilot or CADE policy)
