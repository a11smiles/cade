---
id: REL-001
title: License the project under Apache-2.0 with a NOTICE file
tags: [rel, chore]
epic: REL
priority: P0
depends_on: []
---

## Description

Add `LICENSE` (Apache-2.0), a `NOTICE` naming Joshua Davis as copyright holder, and SPDX headers (`SPDX-License-Identifier: Apache-2.0`) on source files. A CI check rejects new source files without a header.

## Acceptance criteria

- [ ] LICENSE and NOTICE are at the repo root
- [ ] Every Rust and TS source file has an SPDX header
- [ ] CI fails on a missing header
- [ ] Crate and package manifests declare `Apache-2.0`
