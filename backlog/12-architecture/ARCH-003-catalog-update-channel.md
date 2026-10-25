---
id: ARCH-003
title: Refresh the Azure catalog without an app release
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-002]
---

## Description

Let CADE download a newer signed catalog from the project's GitHub releases so new Azure services appear without waiting for an app update. Respect policy: if agent network access is off, the refresh is a user action, never automatic.

## Acceptance criteria

- [ ] Settings shows the catalog version and date and a Check for update action
- [ ] Downloaded catalogs are verified against a signature before use
- [ ] A failed or tampered download keeps the bundled catalog and shows why
- [ ] Org policy can pin a catalog version
