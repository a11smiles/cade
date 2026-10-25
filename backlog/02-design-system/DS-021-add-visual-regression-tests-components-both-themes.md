---
id: DS-021
title: Add visual regression tests for components in both themes
tags: [ds, test, ui]
epic: DS
priority: P1
depends_on: [DS-020]
---

## Description

Screenshot every story in dark and light with Playwright and compare against baselines. Run on Linux in CI with fonts bundled so results are deterministic.

## Acceptance criteria

- [ ] Baselines exist for every story in both themes
- [ ] Pixel diffs above threshold fail CI with an HTML report
- [ ] Updating baselines is one command
