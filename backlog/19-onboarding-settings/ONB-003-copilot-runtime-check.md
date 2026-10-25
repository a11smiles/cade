---
id: ONB-003
title: Check that the Copilot runtime is ready
tags: [onb, feature, rust]
epic: ONB
priority: P0
depends_on: [COPILOT-004]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Start the bundled Copilot CLI and verify a handshake; show 'Copilot agent runtime ready · Bundled with CADE, so there's nothing else to install'. On failure, show diagnostics (binary missing, blocked by antivirus, wrong architecture) and a 'Copy details' action.

## Acceptance criteria

- [ ] Healthy runtime shows the ready row within 5 s
- [ ] Missing or blocked binary shows a specific reason
- [ ] 'Copy details' copies version, path and error
- [ ] Check reruns on 'Try again'
