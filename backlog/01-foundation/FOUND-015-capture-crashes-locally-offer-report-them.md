---
id: FOUND-015
title: Capture crashes locally and offer to report them
tags: [found, feature, rust, security]
epic: FOUND
priority: P1
depends_on: [FOUND-014]
---

## Description

Install a panic hook and native crash handler (minidumps via `minidumper`/`crash-handler`) that writes reports to the local crash folder. Nothing is uploaded automatically. On next launch the app offers to file a bug through the Feedback flow, showing exactly what will be attached.

## Acceptance criteria

- [ ] A forced panic and a forced native crash both produce a local report
- [ ] Next launch shows a dismissible prompt to report the crash
- [ ] Reports contain no document content or secrets
- [ ] Unsaved document state is recovered from the CRDT journal after a crash

## Notes

Recovery depends on CORE autosave. Reporting depends on FEED in-app submission.
