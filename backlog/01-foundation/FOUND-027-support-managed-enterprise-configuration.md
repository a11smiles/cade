---
id: FOUND-027
title: Support managed enterprise configuration
tags: [found, feature, policy, windows, macos, linux]
epic: FOUND
priority: P1
depends_on: [FOUND-009]
---

## Description

Read admin-managed defaults from the OS: macOS managed preferences (configuration profile), Windows registry policy keys (HKLM\Software\Policies\CADE) and `/etc/cade/managed.toml` on Linux. Settings include the policy repo location, GitHub host, update channel/pin and proxy/CA. Managed values are locked in the Settings UI.

## Acceptance criteria

- [ ] Each OS mechanism is read and documented with an example
- [ ] Managed values show a lock icon and can't be changed by the user
- [ ] Managed GitHub host routes all auth to GHES or GHE.com
- [ ] Missing or malformed managed config logs a warning and falls back to defaults

## Notes

Policy content itself comes from POLICY; this item only covers locating it and machine-level defaults.
