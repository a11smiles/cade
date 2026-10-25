---
id: COPILOT-016
title: Show exactly what context is sent to the model
tags: [copilot, feature, ui, security]
epic: COPILOT
priority: P1
depends_on: [COPILOT-015]
---

## Description

Add a 'What Copilot can see' inspector for the next request: system and project instructions, each attached item serialized as it will be sent, tool list, and token estimates. This is the transparency control security reviewers ask for and helps users trim context.

## Acceptance criteria

- [ ] Inspector lists every item that will be sent, with its token estimate
- [ ] Totals match actual input tokens reported for the run within 10%
- [ ] Items excluded by policy content exclusions are shown as excluded with the rule name
- [ ] Inspector content can be copied for a security review
