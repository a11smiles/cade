---
id: DECK-016
title: Embed live linked frames from App and other outputs
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-001, CORE-022, APP-005]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Insert a frame from another CADE output (e.g. 'Step 1 · Mobile' from the App) as a live reference. The
slide renders the source frame's current state and shows a 'Live link' badge in edit mode. Users can pin to
a version or break the link to make a static copy. Links resolve within the same project or linked projects.

## Acceptance criteria

- [ ] Editing the source frame updates the slide within 1 second
- [ ] Pinning to a version stops updates; unpinning resumes
- [ ] Break link creates an editable local copy and removes the badge
- [ ] A missing source shows a clear broken-link placeholder rather than an empty area
- [ ] Export check lists every live frame and how it will export
