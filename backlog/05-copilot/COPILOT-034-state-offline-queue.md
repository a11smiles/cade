---
id: COPILOT-034
title: Queue prompts while offline
tags: [copilot, feature, ui, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-014]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

Detect loss of connectivity to the Copilot endpoint. Show the gray offline banner, queue prompts locally, send them in order on reconnect, and stage results for review as usual. Users can retry now or cancel a queued prompt.

## Acceptance criteria

- [ ] Offline is detected within 5 s of the endpoint becoming unreachable
- [ ] Queued prompts persist across app restarts
- [ ] On reconnect, queued prompts send in order and each produces its own run
- [ ] Cancelling removes the prompt from the queue
