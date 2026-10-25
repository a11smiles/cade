---
id: ARCH-032
title: Show an estimated monthly cost per environment
tags: [arch, feature, rust]
epic: ARCH
priority: P2
depends_on: [ARCH-014]
---

## Description

Estimate monthly cost from SKUs and scale using the public Azure Retail Prices API (user-triggered), shown per resource and per environment, clearly labeled as an estimate.

## Acceptance criteria

- [ ] Estimates show per resource and total
- [ ] Prices are fetched only on user action and cached with a date
- [ ] Resources without a price are listed as unknown
