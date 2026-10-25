---
id: FOUND-014
title: Add structured local logging with export
tags: [found, feature, rust]
epic: FOUND
priority: P0
depends_on: [FOUND-009]
---

## Description

Use `tracing` with a rolling file appender in the logs directory, JSON lines format, 7-day retention and a size cap. Logs are local only. Settings has a "Copy diagnostics" action that bundles recent logs (scrubbed) for attaching to a bug report.

## Acceptance criteria

- [ ] Logs rotate daily and are capped at 50 MB total
- [ ] Prompts, design content and secrets are never logged by default
- [ ] Diagnostics export produces a zip the user can inspect before sharing
- [ ] Log level can be raised from Settings without restart
