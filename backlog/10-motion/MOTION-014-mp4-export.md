---
id: MOTION-014
title: Export MP4 (H.264) video
tags: [motion, feature, rust]
epic: MOTION
priority: P1
depends_on: [MOTION-013]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Encode offscreen frames to H.264 MP4 using the platform encoder (VideoToolbox on macOS, Media Foundation on
Windows, VA-API or a bundled encoder on Linux) to avoid shipping a patent-encumbered software encoder.
Options: resolution, frame rate, quality preset.

## Acceptance criteria

- [ ] MP4 output plays in QuickTime, Windows Media Player and Chrome
- [ ] Uses the OS encoder where available and reports when it is not
- [ ] Encoding uses a fast preset by default, with a best-quality option
- [ ] Export can be cancelled and leaves no partial file

## Notes

Spike the Linux encoder path first; licensing of bundled H.264 encoders must be reviewed (see REL epic).
