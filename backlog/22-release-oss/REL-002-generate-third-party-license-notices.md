---
id: REL-002
title: Generate third-party license notices
tags: [rel, chore]
epic: REL
priority: P0
depends_on: [REL-001, FOUND-022]
---

## Description

Use `cargo-about` and a JS license tool to produce a combined `THIRD_PARTY_NOTICES` file per release, bundled in the app and shown in About. Include font licenses (Geist, OFL) and any bundled runtime.

## Acceptance criteria

- [ ] Notices are generated in the release workflow
- [ ] About screen links to the bundled notices
- [ ] Notices include fonts and the bundled Copilot CLI's license terms
