---
id: COPILOT-041
title: Connect MCP servers from the org allowlist
tags: [copilot, feature, rust, security, policy]
epic: COPILOT
priority: P1
depends_on: [COPILOT-003, COPILOT-010]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

Let Copilot use Model Context Protocol servers as extra tools, limited to the servers the org's policy allows. In `cade-copilot`, read per-project MCP config from `.cade/mcp.json` and user config from settings. Start stdio servers as child processes in the project folder and connect HTTP servers through the network policy. Pass their tools to the Copilot SDK session (or CADE's own agent loop for BYOK) with the same permission prompts as built-in tools.

## Acceptance criteria

- [ ] Servers listed in project or user config start on demand and their tools appear in the context inspector with the server name
- [ ] A server not on the effective policy allowlist is not started, and the panel shows which rule blocked it
- [ ] MCP tool calls that write files or reach the network go through the same approval prompts as built-in tools
- [ ] Stdio servers run with the project folder as their working directory and are stopped when the project closes
- [ ] A crashing or slow server times out without failing the whole run, and the run log records the error
- [ ] Integration test with a fixture MCP server covers list-tools, call-tool and allowlist denial

## Notes

POLICY-009 enforces the allowlist; this item does the connection and tool plumbing.
