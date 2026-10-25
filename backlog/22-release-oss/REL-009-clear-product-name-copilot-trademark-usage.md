---
id: REL-009
title: Clear the product name and Copilot trademark usage
tags: [rel, chore]
epic: REL
priority: P0
depends_on: []
---

## Description

"GitHub" and "Copilot" are trademarks of GitHub/Microsoft. Before public release: search for conflicts on the name CADE in software classes, review GitHub's trademark and logo guidelines, and decide whether the "Copilot ADE" expansion can be used. Default plan: brand as "CADE", describe it as "works with GitHub Copilot", use no GitHub or Copilot logos, and keep a fallback name ready.

## Acceptance criteria

- [ ] Name search results and decision are recorded in `docs/naming.md`
- [ ] No GitHub or Copilot logos appear in the app, icon, site or packages
- [ ] All references to GitHub Copilot use nominative wording ("works with")
- [ ] A fallback name is chosen in case the expansion can't be used
