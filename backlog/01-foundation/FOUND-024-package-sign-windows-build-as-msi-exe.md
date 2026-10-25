---
id: FOUND-024
title: Package and sign the Windows build as MSI and EXE
tags: [found, feature, windows, security]
epic: FOUND
priority: P0
depends_on: [FOUND-021]
---

## Description

Produce an MSI (WiX) for enterprise deployment and an NSIS EXE for individuals, both Authenticode-signed (Azure Trusted Signing or an EV certificate). Bootstrap or embed WebView2 for offline installs. MSI supports silent install and per-machine scope for Intune and SCCM.

## Acceptance criteria

- [ ] Installers are signed and show the publisher with no SmartScreen block once reputation is established
- [ ] `msiexec /i cade.msi /qn` installs silently per-machine
- [ ] Offline install works without internet using the fixed WebView2 runtime option
- [ ] Uninstall removes binaries but keeps user projects and settings
