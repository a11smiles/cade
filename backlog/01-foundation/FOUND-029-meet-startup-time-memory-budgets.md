---
id: FOUND-029
title: Meet startup time and memory budgets
tags: [found, test, perf]
epic: FOUND
priority: P1
depends_on: [FOUND-003]
---

## Description

Cold start to interactive Start screen under 1.5 s and idle memory under 250 MB on a reference machine per OS. Measure in CI with a startup benchmark and fail on regressions over 15%.

## Acceptance criteria

- [ ] Startup benchmark runs nightly on all three OSes
- [ ] Regressions above 15% fail the nightly job and open an issue
- [ ] Reference machines and methodology are in `docs/perf.md`
