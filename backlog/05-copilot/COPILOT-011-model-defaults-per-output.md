---
id: COPILOT-011
title: Select default models per output type
tags: [copilot, feature, ui, ts]
epic: COPILOT
priority: P1
depends_on: [COPILOT-003, POLICY-007]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Settings › Copilot & models lists output-type groups (App/Image/Deck, Architecture/Data model, Document/Diagram/Motion) each with a default model. The composer's model picker starts from that default. Only models allowed by policy and available to the user appear; 'Auto' lets Copilot choose.

## Acceptance criteria

- [ ] Changing the Architecture default changes the model used for new runs in the Architecture workspace
- [ ] The composer picker can override per message and shows the effective model on the run
- [ ] Models removed by a policy update fall back to Auto with a one-time notice
- [ ] Defaults persist across restarts in user config
