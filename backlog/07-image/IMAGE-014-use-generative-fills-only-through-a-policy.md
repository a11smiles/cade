---
id: IMAGE-014
title: Use generative fills only through a policy-gated layer type
tags: [image, rust, feature, policy]
epic: IMAGE
priority: P2
depends_on: [POLICY-001, COPILOT-019]
---

## Description

Keep Image vector-first. If generative image fills are added, they are a separate layer type that's off by default, gated by policy, labeled as generated, and never mixed into vector layers.

## Acceptance criteria

- [ ] Generative fill is hidden unless policy enables it
- [ ] Generated layers are labeled and listed in the export manifest
- [ ] Turning policy off disables creating new generative layers but keeps existing ones visible
