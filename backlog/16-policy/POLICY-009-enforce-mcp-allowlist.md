---
id: POLICY-009
title: Enforce the MCP server allowlist
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-002, COPILOT-041]
---

## Description

Only MCP servers listed in `tools.mcp` (by name and command/URL) can be configured or started. Unlisted servers in project config are shown disabled with the rule.

## Acceptance criteria

- [ ] Unlisted MCP server in `.cade/mcp.json` does not start
- [ ] Allowlisted server starts and its tools appear
- [ ] Settings shows allowed count ('Org allowlist · 3')
- [ ] Allowlist matches on full command or URL, not name alone
