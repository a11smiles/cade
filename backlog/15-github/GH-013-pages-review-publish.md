---
id: GH-013
title: Publish review links to private GitHub Pages
tags: [gh, feature, rust, security]
epic: GH
priority: P0
depends_on: [GH-010, HIST-010]
design: design/exports/v0001-2026-10-25/Review.dc.html
---

## Description

Render the selected frames and prototype to static files and publish them to the repo's private GitHub Pages under `/review/<pr>`, via a branch or Actions artifact. Only repo readers can open the link.

## Acceptance criteria

- [ ] Publishing creates a review page reachable at the PR's review path
- [ ] Pages visibility is private; CADE refuses to publish if the repo's Pages are public, unless policy allows
- [ ] Each new version updates the page and keeps older versions selectable
- [ ] Unpublishing removes the review path

## Notes

Private Pages requires GitHub Enterprise Cloud; on GHES or other plans fall back to a PR artifact download.

Absorbs HIST-011, which duplicated this item and was removed during backlog reconciliation.
