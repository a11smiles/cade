---
id: COPILOT-028
title: Prompt before shell commands, scoped to the project
tags: [copilot, feature, security, ui, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-003, POLICY-008]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

When the agent wants to run a command, show the permission card: the exact command, where it runs (project folder only), what policy allows, and Allow once / Always allow in this project / Deny. 'Always allow' is stored per project and command prefix. Pre-approved commands from policy run without asking; network stays off unless policy allows it.

## Acceptance criteria

- [ ] Commands run with the project folder as working directory and no access above it (sandbox test)
- [ ] Allow once runs the command a single time; the next identical request asks again
- [ ] Always allow in this project persists in `.cade/permissions.json` and can be revoked in Settings
- [ ] Commands not on the allowlist when policy is `shell: deny` are blocked with the policy-blocked state
- [ ] Pre-approved commands (e.g. `cade-render`, `bicep`) run without a prompt and are logged
