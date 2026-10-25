---
id: FOUND-017
title: "Register file associations and the cade:// deep link"
tags: [found, feature, rust, macos, windows, linux]
epic: FOUND
priority: P1
depends_on: [FOUND-016]
---

## Description

Register `.cade` as the native project type and offer CADE for `.vsdx`, `.vsd`, `.vssx`, `.vstx` and `.pptx` (open-with, not default). Register a `cade://` scheme for review links and OAuth callbacks, validating every deep link before acting on it.

## Acceptance criteria

- [ ] `.cade` files show the CADE icon and open the app
- [ ] Visio and PowerPoint files list CADE under Open With without becoming the default
- [ ] `cade://open?repo=…` validates host and parameters and asks before cloning
- [ ] Malformed or unexpected deep links are logged and ignored
