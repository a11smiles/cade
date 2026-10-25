---
id: DS-006
title: Build CADE's chrome icon set
tags: [ds, feature, ui, a11y]
epic: DS
priority: P0
depends_on: [DS-001]
---

## Description

Create an original 24 px stroke icon set (1.8 px stroke, round joins) covering the tool rails, mode switches, Copilot sparkle, shield/policy, history, deploy, import, send and status glyphs seen in the screens. Ship as a React `Icon` component that takes a name, inherits `currentColor`, and is `aria-hidden` unless given a label. This set is for app chrome only; Azure icons come from the official pack (ARCH).

## Acceptance criteria

- [ ] Every icon used in the v0001 screens exists in the set
- [ ] Icons inherit text color in both themes
- [ ] Icon-only buttons fail lint without an `aria-label`
- [ ] The CADE app icon is produced at all OS-required sizes (icns, ico, PNG set)
