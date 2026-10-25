---
id: FOUND-025
title: Package the Linux build as AppImage, deb, rpm and Flatpak
tags: [found, feature, linux]
epic: FOUND
priority: P0
depends_on: [FOUND-021]
---

## Description

Ship AppImage (portable), `.deb` (Ubuntu/Debian), `.rpm` (Fedora/RHEL) and a Flatpak manifest for Flathub. Declare WebKitGTK 4.1 and libsoup3 dependencies, desktop entry, MIME types and icons. Sign repository metadata and publish checksums.

## Acceptance criteria

- [ ] Packages install and launch on Ubuntu LTS, Fedora and one Arch-based distro
- [ ] Flatpak builds in CI with only the portals it needs
- [ ] Desktop entry registers the `.cade` MIME type and `cade://` scheme
- [ ] SHA-256 checksums and GPG signatures are published with each release
