---
id: POLICY-005
title: Refresh policy and notify users of changes
tags: [policy, feature, rust, ui]
epic: POLICY
priority: P1
depends_on: [POLICY-004]
---

## Description

Re-fetch policy on sign-in, every 6 hours while running, and on demand. When the version changes, show a summary of what changed ('Shell commands now ask each time'). Running agent runs finish under the old policy; new runs use the new one.

## Acceptance criteria

- [ ] New policy version is picked up within 6 hours without restart
- [ ] Change summary lists added, removed and changed rules
- [ ] In-flight runs are not interrupted by a policy change
- [ ] 'Check for policy updates' in Settings forces a refresh
