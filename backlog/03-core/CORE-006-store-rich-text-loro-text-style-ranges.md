---
id: CORE-006
title: Store rich text with Loro text and style ranges
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-004]
---

## Description

Text nodes use a Loro rich-text container with marks for font, size, weight, color token, link and paragraph style. Concurrent edits from the user and Copilot merge character by character. Paragraph styles reference text-style tokens from the design system.

## Acceptance criteria

- [ ] Concurrent inserts in the same paragraph merge without loss
- [ ] Marks survive splits and joins of paragraphs
- [ ] Paragraph styles resolve through tokens
- [ ] Text round-trips to the JSON export with marks preserved
