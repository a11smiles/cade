---
id: FOUND-026
title: Ship signed auto-updates with stable and beta channels
tags: [found, feature, security]
epic: FOUND
priority: P1
depends_on: [FOUND-023, FOUND-024, FOUND-025]
---

## Description

Use the Tauri updater plugin with a signing key held only in the release environment. Offer stable and beta channels. Updates download in the background and apply on restart; the user is never interrupted mid-run. Org policy can disable updates or pin a version range.

## Acceptance criteria

- [ ] Update manifests are signed and unsigned updates are rejected
- [ ] Users can switch channel in Settings
- [ ] Policy `updates: off` or a version pin is honored and shown in Settings
- [ ] A failed update rolls back to the previous version
