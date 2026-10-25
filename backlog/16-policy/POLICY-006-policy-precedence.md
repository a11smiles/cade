---
id: POLICY-006
title: Merge policies from multiple orgs, strictest wins
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-003]
---

## Description

Users can belong to several orgs with policies. Merge them so the strictest value wins per rule (intersection of model allowlists, `deny` beats `ask` beats `allow`, `false` beats `true` for permissions, lowest budget). Per-project override: when a project's repo belongs to an org, that org's policy applies to the project in addition to the merged baseline.

## Acceptance criteria

- [ ] Merge function is property-tested: result is never looser than any input
- [ ] Settings shows which org set each effective rule
- [ ] Opening a project owned by org B applies org B's rules on top of the baseline
- [ ] Conflicting design-system requirements across orgs are reported, not silently dropped
