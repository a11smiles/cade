---
id: POLICY-008
title: Enforce agent tool rules: files, shell, network
tags: [policy, feature, rust, security]
epic: POLICY
priority: P0
depends_on: [POLICY-002, COPILOT-029, CORE-011]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

Implement the enforcement points for `tools.file_edits`, `tools.shell` (allow/ask/deny), `tools.preapproved`, and `tools.network`. Network off means the agent's shell commands run without network (OS-level where possible: sandbox-exec on macOS, Job Objects + firewall rule or AppContainer on Windows, network namespace on Linux) and agent tools that fetch URLs are disabled.

## Acceptance criteria

- [ ] `shell: deny` blocks all non-preapproved commands
- [ ] `shell: ask` always prompts, even for commands previously 'always allowed' if policy changed
- [ ] With `network: off`, `curl` in an agent command fails on all three OSes
- [ ] Preapproved commands match by exact program and argument prefix, not substring
- [ ] Each enforcement decision is logged to the audit log

## Notes

OS-level network isolation differs per platform; document which guarantees hold on each OS.
