---
id: IMPORT-024
title: Import by drag-and-drop and OS 'Open with'
tags: [import, feature, ui]
epic: IMPORT
priority: P1
depends_on: [IMPORT-003]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Dropping a supported file on the app window or the Start screen, or opening it with CADE from the OS,
starts the matching import with the dry run shown first. Register file associations on all three OSes.

## Acceptance criteria

- [ ] Dropping a .vsdx, .pptx or .fig link starts the right importer
- [ ] File associations are registered by the installers on macOS, Windows and Linux
- [ ] The dry run is always shown before writing a project
