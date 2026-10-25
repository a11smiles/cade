---
id: MOTION-018
title: Expose motion editing as Copilot tools
tags: [motion, feature, rust]
epic: MOTION
priority: P1
depends_on: [COPILOT-020, MOTION-007]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Agent tools: change durations by token steps ('make it snappier'), swap easings, add or remove a state,
generate a reduced-motion fallback, and create a motion from a description. Changes are staged as
duration.md → duration.sm style diffs.

## Acceptance criteria

- [ ] 'Make this snappier' moves durations down one token step instead of setting raw values
- [ ] Changes are shown as a staged diff with Keep / Compare actions
- [ ] Copilot reports the new total duration
- [ ] Tools are tested with a mocked model
