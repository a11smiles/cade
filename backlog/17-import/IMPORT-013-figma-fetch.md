---
id: IMPORT-013
title: Fetch Figma files, nodes and images via the REST API
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-012]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Client for Figma's REST API: file document, nodes by id, image fills and rendered images, components,
styles and variables. Handles rate limits with backoff, large files by fetching pages separately, and caches
responses in the staging folder.

## Acceptance criteria

- [ ] A 212-frame reference file fetches completely without hitting rate-limit failures
- [ ] Large files are fetched page by page with progress
- [ ] Responses are cached so re-running an import does not refetch
- [ ] Network errors resume from the last completed page
