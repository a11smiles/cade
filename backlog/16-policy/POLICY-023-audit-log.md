---
id: POLICY-023
title: Record a local audit log of policy-relevant events
tags: [policy, feature, rust, security]
epic: POLICY
priority: P1
depends_on: [POLICY-008]
---

## Description

Append-only local log of policy events: blocked actions, permission grants and revocations, exception requests, ships, deploys, policy version changes. Users can view and export it; when reporting is on, only counts by event type are reported.

## Acceptance criteria

- [ ] Each event has timestamp, event type, rule, project id hash and outcome
- [ ] Log is append-only with a hash chain so edits are detectable
- [ ] Settings › Privacy › Audit log shows and exports events as JSON
- [ ] Reporting includes only per-type counts
