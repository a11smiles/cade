---
id: CORE-015
title: Define the .cade on-disk project format
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-003]
---

## Description

A project is a folder: `project.cade/manifest.json` (format version, app version, output types, libraries, linked repos), `doc.loro` (compacted snapshot), `journal/` (append-only update files), `assets/` (content-addressed blobs) and `export/project.json` (canonical JSON). Document this format publicly so other tools can read it.

## Acceptance criteria

- [ ] Format spec is published in `docs/format/README.md`
- [ ] A minimal valid project can be created by hand from the spec
- [ ] Loading validates the manifest and reports precise errors
- [ ] Format version is a semantic version independent of the app version
