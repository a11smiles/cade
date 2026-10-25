---
id: GH-023
title: Fallback review via PR artifacts when Pages isn't available
tags: [gh, feature, rust]
epic: GH
priority: P2
depends_on: [GH-013]
design: design/exports/v0001-2026-10-25/Review.dc.html
---

## Description

Where private Pages isn't available, attach the review build as a workflow artifact and post a PR comment with instructions.

## Acceptance criteria

- [ ] Fallback triggers automatically when Pages is unavailable
- [ ] PR comment explains how to view the artifact
- [ ] Artifact contains the same static review build as the Pages version
