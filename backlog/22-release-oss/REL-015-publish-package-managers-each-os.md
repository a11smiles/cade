---
id: REL-015
title: Publish to package managers on each OS
tags: [rel, chore, macos, windows, linux]
epic: REL
priority: P1
depends_on: [FOUND-028]
---

## Description

Automate publishing to GitHub Releases, a Homebrew cask, winget, Flathub, AUR, and signed apt and rpm repositories. Each channel is updated by the release workflow or a bot PR.

## Acceptance criteria

- [ ] `brew install --cask cade` installs the latest release
- [ ] `winget install CADE` installs the latest release
- [ ] Flathub, AUR, apt and rpm channels receive updates within 24 hours of a release
- [ ] Channel status is shown on the docs install page
