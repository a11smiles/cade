---
id: POLICY-004
title: Cache policy and pin it to a verified commit
tags: [policy, feature, rust, security]
epic: POLICY
priority: P0
depends_on: [POLICY-003]
---

## Description

Cache the effective policy locally with the source commit SHA. Verify the file came from the default branch and record whether that branch is protected; show the commit in Settings. Offline launches use the cache. A user cannot loosen policy by editing the cache: the cache is integrity-checked (HMAC keyed per install, stored in the keychain) and a tampered cache falls back to the strictest defaults until refetched.

## Acceptance criteria

- [ ] Effective policy shows source repo, commit SHA and fetched time
- [ ] Offline launch uses the cached policy
- [ ] Editing the cache file on disk is detected and strict defaults apply
- [ ] Policy from an unprotected default branch shows an admin warning
