# CADE backlog

Every requirement needed to take CADE from the design canvas to a working 1.0 on macOS, Windows and Linux. Each item is one Markdown file in an epic folder. The format, epic list and fixed technical decisions are in [CONVENTIONS.md](CONVENTIONS.md).

## Reading an item

- **Frontmatter:** `id` (never reused), `title`, `tags` (the epic tag, one type tag — feature, chore, spike, bug, docs or test — plus surface tags), `epic`, `priority`, `depends_on`, and optionally `design`, the exported screen it implements.
- **Priority:** P0 is needed for any usable build, P1 for 1.0, P2 after 1.0.
- **Body:** a Description, 3–7 testable Acceptance criteria, and optional Notes.
- **Checking:** run `python3 scripts/backlog_check.py` after any edit. It fails on dangling ids, dependency cycles and format errors.

## Epics

| Folder | Epic | Items | P0 | P1 | P2 |
|---|---|---|---|---|---|
| [01-foundation](01-foundation/) | Foundation | 29 | 18 | 11 | 0 |
| [02-design-system](02-design-system/) | Design system | 22 | 16 | 5 | 1 |
| [03-core](03-core/) | Core document model | 29 | 22 | 7 | 0 |
| [04-canvas](04-canvas/) | Canvas | 41 | 31 | 10 | 0 |
| [05-copilot](05-copilot/) | Copilot | 40 | 21 | 19 | 0 |
| [06-app](06-app/) | App | 27 | 18 | 8 | 1 |
| [07-image](07-image/) | Image | 14 | 9 | 4 | 1 |
| [08-deck](08-deck/) | Deck | 24 | 17 | 6 | 1 |
| [09-document](09-document/) | Document | 21 | 14 | 5 | 2 |
| [10-motion](10-motion/) | Motion | 20 | 11 | 8 | 1 |
| [11-diagram](11-diagram/) | Diagram | 27 | 18 | 8 | 1 |
| [12-architecture](12-architecture/) | Architecture (Azure) | 34 | 16 | 16 | 2 |
| [13-data-model](13-data-model/) | Data model | 24 | 15 | 7 | 2 |
| [14-system](14-system/) | System and deploy | 17 | 10 | 6 | 1 |
| [15-github](15-github/) | GitHub | 21 | 13 | 7 | 1 |
| [16-policy](16-policy/) | Policy and admin | 25 | 9 | 14 | 2 |
| [17-import](17-import/) | Import | 25 | 19 | 6 | 0 |
| [18-history](18-history/) | History and review | 13 | 6 | 5 | 2 |
| [19-onboarding-settings](19-onboarding-settings/) | Onboarding and settings | 19 | 11 | 8 | 0 |
| [20-feedback](20-feedback/) | Feedback | 18 | 11 | 6 | 1 |
| [21-quality](21-quality/) | Quality | 25 | 18 | 7 | 0 |
| [22-release-oss](22-release-oss/) | Release and OSS | 20 | 12 | 8 | 0 |
| | **Total** | **535** | **335** | **181** | **19** |

## Milestone M1: open, edit and save a project with Copilot

The smallest build where someone can create an App project, draw and edit frames on the GPU canvas, ask Copilot for a change, review the staged changes, apply them, and have it all saved and committed. These are the 53 items in dependency order, with P0 first wherever the order allows.

1. FOUND-001 — Create the Cargo workspace with all CADE crates (P0)
2. COPILOT-001 — Create cade-copilot crate wrapping github-copilot-sdk (P0)
3. COPILOT-002 — Define the ModelProvider trait and streaming event types (P0)
4. CORE-001 — "Spike: validate Loro for a 10k-layer design document" (P0)
5. CORE-002 — Wrap Loro behind the cade-crdt API (P0)
6. CORE-003 — Define the document schema and node kinds (P0)
7. CORE-004 — Add typed property accessors with forward-compatible storage (P0)
8. CORE-007 — Bind properties to design tokens with modes (P0)
9. CORE-011 — Route all edits through a validated operations API (P0)
10. COPILOT-018 — Expose read-only document tools to the agent (P0)
11. CORE-012 — Implement undo and redo grouped by origin (P0)
12. CORE-014 — Compute semantic diffs between document versions (P0)
13. CORE-013 — Stage Copilot changes in a forked document and apply selectively (P0)
14. COPILOT-021 — Stage agent edits as change sets on a Loro fork (P0)
15. COPILOT-019 — Expose write tools that produce staged operations (P0)
16. COPILOT-024 — Undo or restore a whole Copilot run as one step (P0)
17. CORE-015 — Define the .cade on-disk project format (P0)
18. CORE-018 — Write a canonical JSON export for diffs and review (P0)
19. FOUND-002 — Scaffold app/ui with Vite, React and strict TypeScript (P0)
20. DS-001 — Make design/system/tokens.json the single source of UI tokens (P0)
21. DS-002 — Define dark and light color tokens with enforced contrast (P0)
22. DS-005 — Define spacing, radius, elevation and motion tokens (P0)
23. DS-006 — Build CADE's chrome icon set (P0)
24. DS-007 — Build Button and IconButton (P0)
25. DS-008 — Build form controls (P0)
26. DS-010 — Build resizable panel layout primitives (P0)
27. DS-011 — Build the contextual properties strip (P0)
28. DS-012 — Build the Copilot chat component kit (P0)
29. COPILOT-014 — Build the Copilot panel message list and composer (P0)
30. COPILOT-022 — Review staged changes one by one (P0)
31. DS-016 — Build tree view, list rows and data table (P0)
32. CANVAS-025 — Build the Layers panel (P0)
33. FOUND-003 — Boot the Tauri 2 shell with a locked-down capability set (P0)
34. APP-001 — Create an App project from Start (P0)
35. APP-002 — Build the App workspace shell and mode switch (P0)
36. FOUND-004 — Generate typed IPC bindings between Rust and TypeScript (P0)
37. FOUND-006 — "Spike: composite a native wgpu surface under a transparent webview" (P0)
38. CANVAS-001 — Embed a native wgpu surface under the Tauri webview (P0)
39. CANVAS-003 — Build Vello scenes from the document tree (P0)
40. CANVAS-006 — Hit-test layers precisely and fast (P0)
41. CANVAS-007 — Show selection, resize and rotate handles (P0)
42. CANVAS-008 — Move, resize, rotate and flip layers (P0)
43. CANVAS-013 — Implement the Frame tool and nested frames (P0)
44. CANVAS-014 — Implement shape tools: rectangle, ellipse, line, arrow, polygon, star (P0)
45. CANVAS-028 — Build the contextual properties strip (P0)
46. FOUND-011 — Add a background job runtime with progress and cancellation (P0)
47. CORE-017 — Autosave through an append-only journal with crash recovery (P0)
48. GH-001 — Sign in with GitHub using the OAuth device flow (P0)
49. COPILOT-003 — Implement the Copilot provider on the SDK (P0)
50. GH-005 — Wrap git operations with gix (P0)
51. CORE-019 — Commit project changes to the local git repo (P0)
52. ONB-001 — Build the first-run wizard shell (P0)
53. ONB-002 — Sign in with GitHub using the device flow (P0)

## Foundation (`01-foundation`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [FOUND-001](01-foundation/FOUND-001-create-cargo-workspace-all-cade-crates.md) | Create the Cargo workspace with all CADE crates | P0 | — |
| [FOUND-002](01-foundation/FOUND-002-scaffold-app-ui-vite-react-strict-typescript.md) | Scaffold app/ui with Vite, React and strict TypeScript | P0 | — |
| [FOUND-003](01-foundation/FOUND-003-boot-tauri-2-shell-locked-down-capability.md) | Boot the Tauri 2 shell with a locked-down capability set | P0 | FOUND-001, FOUND-002 |
| [FOUND-004](01-foundation/FOUND-004-generate-typed-ipc-bindings-between-rust-typescript.md) | Generate typed IPC bindings between Rust and TypeScript | P0 | FOUND-003 |
| [FOUND-005](01-foundation/FOUND-005-define-cross-layer-error-model-user-facing.md) | Define the cross-layer error model and user-facing error mapping | P0 | FOUND-004 |
| [FOUND-006](01-foundation/FOUND-006-spike-composite-native-wgpu-surface-under-transparent.md) | Spike: composite a native wgpu surface under a transparent webview | P0 | FOUND-003 |
| [FOUND-007](01-foundation/FOUND-007-support-multiple-project-windows-persist-window-state.md) | Support multiple project windows and persist window state | P1 | FOUND-003 |
| [FOUND-008](01-foundation/FOUND-008-build-native-menus-shortcut-registry.md) | Build native menus and a shortcut registry | P1 | FOUND-003 |
| [FOUND-009](01-foundation/FOUND-009-define-per-os-app-data-directories-settings.md) | Define per-OS app data directories and settings migrations | P0 | FOUND-001 |
| [FOUND-010](01-foundation/FOUND-010-store-secrets-os-keychain.md) | Store secrets in the OS keychain | P0 | FOUND-001 |
| [FOUND-011](01-foundation/FOUND-011-add-background-job-runtime-progress-cancellation.md) | Add a background job runtime with progress and cancellation | P0 | FOUND-004 |
| [FOUND-012](01-foundation/FOUND-012-confine-file-access-project-folders-watch-changes.md) | Confine file access to project folders and watch for changes | P0 | FOUND-001 |
| [FOUND-013](01-foundation/FOUND-013-provide-network-status-service.md) | Provide a network status service | P1 | FOUND-004 |
| [FOUND-014](01-foundation/FOUND-014-add-structured-local-logging-export.md) | Add structured local logging with export | P0 | FOUND-009 |
| [FOUND-015](01-foundation/FOUND-015-capture-crashes-locally-offer-report-them.md) | Capture crashes locally and offer to report them | P1 | FOUND-014 |
| [FOUND-016](01-foundation/FOUND-016-enforce-single-instance-route-file-opens.md) | Enforce single instance and route file opens | P1 | FOUND-007 |
| [FOUND-017](01-foundation/FOUND-017-register-file-associations-cade-deep-link.md) | Register file associations and the cade:// deep link | P1 | FOUND-016 |
| [FOUND-018](01-foundation/FOUND-018-externalize-ui-strings-set-up-localization.md) | Externalize UI strings and set up localization | P1 | FOUND-002 |
| [FOUND-019](01-foundation/FOUND-019-add-feature-flags-unfinished-surfaces.md) | Add feature flags for unfinished surfaces | P1 | FOUND-009 |
| [FOUND-020](01-foundation/FOUND-020-provide-one-command-developer-setup.md) | Provide one-command developer setup | P0 | FOUND-001, FOUND-002 |
| [FOUND-021](01-foundation/FOUND-021-run-ci-macos-windows-linux-every-pr.md) | Run CI on macOS, Windows and Linux for every PR | P0 | FOUND-001, FOUND-002 |
| [FOUND-022](01-foundation/FOUND-022-lock-down-dependency-supply-chain.md) | Lock down the dependency supply chain | P0 | FOUND-021 |
| [FOUND-023](01-foundation/FOUND-023-package-sign-notarize-macos-build.md) | Package, sign and notarize the macOS build | P0 | FOUND-021 |
| [FOUND-024](01-foundation/FOUND-024-package-sign-windows-build-as-msi-exe.md) | Package and sign the Windows build as MSI and EXE | P0 | FOUND-021 |
| [FOUND-025](01-foundation/FOUND-025-package-linux-build-as-appimage-deb-rpm.md) | Package the Linux build as AppImage, deb, rpm and Flatpak | P0 | FOUND-021 |
| [FOUND-026](01-foundation/FOUND-026-ship-signed-auto-updates-stable-beta-channels.md) | Ship signed auto-updates with stable and beta channels | P1 | FOUND-023, FOUND-024, FOUND-025 |
| [FOUND-027](01-foundation/FOUND-027-support-managed-enterprise-configuration.md) | Support managed enterprise configuration | P1 | FOUND-009 |
| [FOUND-028](01-foundation/FOUND-028-automate-tagged-releases-provenance.md) | Automate tagged releases with provenance | P0 | FOUND-023, FOUND-024, FOUND-025, FOUND-022 |
| [FOUND-029](01-foundation/FOUND-029-meet-startup-time-memory-budgets.md) | Meet startup time and memory budgets | P1 | FOUND-003 |

## Design system (`02-design-system`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [DS-001](02-design-system/DS-001-make-design-system-tokens-json-single-source.md) | Make design/system/tokens.json the single source of UI tokens | P0 | FOUND-002 |
| [DS-002](02-design-system/DS-002-define-dark-light-color-tokens-enforced-contrast.md) | Define dark and light color tokens with enforced contrast | P0 | DS-001 |
| [DS-003](02-design-system/DS-003-switch-between-dark-light-match-system-themes.md) | Switch between Dark, Light and Match system themes | P0 | DS-002 |
| [DS-004](02-design-system/DS-004-bundle-geist-fonts-define-type-scale.md) | Bundle Geist fonts and define the type scale | P0 | DS-001 |
| [DS-005](02-design-system/DS-005-define-spacing-radius-elevation-motion-tokens.md) | Define spacing, radius, elevation and motion tokens | P0 | DS-001 |
| [DS-006](02-design-system/DS-006-build-cade-s-chrome-icon-set.md) | Build CADE's chrome icon set | P0 | DS-001 |
| [DS-007](02-design-system/DS-007-build-button-iconbutton.md) | Build Button and IconButton | P0 | DS-002, DS-005, DS-006 |
| [DS-008](02-design-system/DS-008-build-form-controls.md) | Build form controls | P0 | DS-002, DS-005 |
| [DS-009](02-design-system/DS-009-build-segmented-control-tabs.md) | Build Segmented control and Tabs | P0 | DS-007 |
| [DS-010](02-design-system/DS-010-build-resizable-panel-layout-primitives.md) | Build resizable panel layout primitives | P0 | DS-005 |
| [DS-011](02-design-system/DS-011-build-contextual-properties-strip.md) | Build the contextual properties strip | P0 | DS-008, DS-007 |
| [DS-012](02-design-system/DS-012-build-copilot-chat-component-kit.md) | Build the Copilot chat component kit | P0 | DS-007, DS-008 |
| [DS-013](02-design-system/DS-013-build-pills-badges-type-tags.md) | Build pills, badges and type tags | P0 | DS-002 |
| [DS-014](02-design-system/DS-014-build-menus-context-menus-command-palette.md) | Build menus, context menus and the command palette | P1 | DS-007, FOUND-008 |
| [DS-015](02-design-system/DS-015-build-dialogs-banners-toasts.md) | Build dialogs, banners and toasts | P0 | DS-007 |
| [DS-016](02-design-system/DS-016-build-tree-view-list-rows-data-table.md) | Build tree view, list rows and data table | P0 | DS-008 |
| [DS-017](02-design-system/DS-017-theme-code-editor-diff-views.md) | Theme the code editor and diff views | P0 | DS-002, DS-004 |
| [DS-018](02-design-system/DS-018-support-high-contrast-forced-colors.md) | Support high contrast and forced colors | P1 | DS-003 |
| [DS-019](02-design-system/DS-019-honor-os-text-scaling-add-ui-zoom.md) | Honor OS text scaling and add UI zoom | P1 | DS-010 |
| [DS-020](02-design-system/DS-020-publish-component-catalog-a11y-checks.md) | Publish a component catalog with a11y checks | P1 | DS-007 |
| [DS-021](02-design-system/DS-021-add-visual-regression-tests-components-both-themes.md) | Add visual regression tests for components in both themes | P1 | DS-020 |
| [DS-022](02-design-system/DS-022-keep-design-system-artifact-tokens-json-sync.md) | Keep the design-system artifact and tokens.json in sync | P2 | DS-001 |

## Core document model (`03-core`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [CORE-001](03-core/CORE-001-spike-validate-loro-10k-layer-design-document.md) | Spike: validate Loro for a 10k-layer design document | P0 | FOUND-001 |
| [CORE-002](03-core/CORE-002-wrap-loro-behind-cade-crdt-api.md) | Wrap Loro behind the cade-crdt API | P0 | CORE-001 |
| [CORE-003](03-core/CORE-003-define-document-schema-node-kinds.md) | Define the document schema and node kinds | P0 | CORE-002 |
| [CORE-004](03-core/CORE-004-add-typed-property-accessors-forward-compatible-storage.md) | Add typed property accessors with forward-compatible storage | P0 | CORE-003 |
| [CORE-005](03-core/CORE-005-model-transforms-bounds-units.md) | Model transforms, bounds and units | P0 | CORE-004 |
| [CORE-006](03-core/CORE-006-store-rich-text-loro-text-style-ranges.md) | Store rich text with Loro text and style ranges | P0 | CORE-004 |
| [CORE-007](03-core/CORE-007-bind-properties-design-tokens-modes.md) | Bind properties to design tokens with modes | P0 | CORE-004 |
| [CORE-008](03-core/CORE-008-add-project-design-system-libraries.md) | Add project design-system libraries | P0 | CORE-007 |
| [CORE-009](03-core/CORE-009-implement-components-variants-instances-overrides.md) | Implement components, variants and instances with overrides | P0 | CORE-003, CORE-007 |
| [CORE-010](03-core/CORE-010-share-components-across-projects-through-libraries.md) | Share components across projects through libraries | P1 | CORE-009, CORE-008 |
| [CORE-011](03-core/CORE-011-route-all-edits-through-validated-operations-api.md) | Route all edits through a validated operations API | P0 | CORE-003, CORE-004 |
| [CORE-012](03-core/CORE-012-implement-undo-redo-grouped-by-origin.md) | Implement undo and redo grouped by origin | P0 | CORE-002, CORE-011 |
| [CORE-013](03-core/CORE-013-stage-copilot-changes-forked-document-apply-selectively.md) | Stage Copilot changes in a forked document and apply selectively | P0 | CORE-002, CORE-011, CORE-014 |
| [CORE-014](03-core/CORE-014-compute-semantic-diffs-between-document-versions.md) | Compute semantic diffs between document versions | P0 | CORE-003 |
| [CORE-015](03-core/CORE-015-define-cade-disk-project-format.md) | Define the .cade on-disk project format | P0 | CORE-003 |
| [CORE-016](03-core/CORE-016-version-format-migrate-old-projects.md) | Version the format and migrate old projects | P0 | CORE-015 |
| [CORE-017](03-core/CORE-017-autosave-through-append-only-journal-crash-recovery.md) | Autosave through an append-only journal with crash recovery | P0 | CORE-015, FOUND-011 |
| [CORE-018](03-core/CORE-018-write-canonical-json-export-diffs-review.md) | Write a canonical JSON export for diffs and review | P0 | CORE-015 |
| [CORE-019](03-core/CORE-019-commit-project-changes-local-git-repo.md) | Commit project changes to the local git repo | P0 | CORE-015, CORE-018, GH-005 |
| [CORE-020](03-core/CORE-020-provide-git-merge-driver-cade-projects.md) | Provide a git merge driver for .cade projects | P1 | CORE-019, CORE-002 |
| [CORE-021](03-core/CORE-021-store-assets-content-addressed-garbage-collection.md) | Store assets content-addressed with garbage collection | P0 | CORE-015 |
| [CORE-022](03-core/CORE-022-support-cross-artifact-references-between-output-types.md) | Support cross-artifact references between output types | P0 | CORE-003 |
| [CORE-023](03-core/CORE-023-track-ephemeral-presence-copilot-cursor-selections.md) | Track ephemeral presence for the Copilot cursor and selections | P1 | CORE-002 |
| [CORE-024](03-core/CORE-024-implement-selection-model.md) | Implement the selection model | P0 | CORE-003 |
| [CORE-025](03-core/CORE-025-reconcile-external-changes-open-project.md) | Reconcile external changes to an open project | P1 | CORE-020, FOUND-012 |
| [CORE-026](03-core/CORE-026-index-large-documents-load-pages-lazily.md) | Index large documents and load pages lazily | P1 | CORE-003, CORE-005 |
| [CORE-027](03-core/CORE-027-add-project-validation-repair.md) | Add project validation and repair | P1 | CORE-015, CORE-022, CORE-021 |
| [CORE-028](03-core/CORE-028-ship-headless-cade-cli.md) | Ship a headless cade CLI | P1 | CORE-015, CORE-018 |
| [CORE-029](03-core/CORE-029-track-project-metadata-manifest.md) | Track project metadata in the manifest | P0 | CORE-015 |

## Canvas (`04-canvas`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [CANVAS-001](04-canvas/CANVAS-001-embed-a-native-wgpu-surface-under-the.md) | Embed a native wgpu surface under the Tauri webview | P0 | FOUND-003, FOUND-006 |
| [CANVAS-002](04-canvas/CANVAS-002-route-input-from-the-webview-and-os.md) | Route input from the webview and OS to the canvas | P0 | CANVAS-001 |
| [CANVAS-003](04-canvas/CANVAS-003-build-vello-scenes-from-the-document-tree.md) | Build Vello scenes from the document tree | P0 | CORE-003, CANVAS-001 |
| [CANVAS-004](04-canvas/CANVAS-004-render-text-with-parley-including-fallback-bidi.md) | Render text with Parley, including fallback, bidi and variable fonts | P0 | CANVAS-003 |
| [CANVAS-005](04-canvas/CANVAS-005-edit-text-in-place-with-ime-selection.md) | Edit text in place with IME, selection and caret | P0 | CANVAS-004, CANVAS-002 |
| [CANVAS-006](04-canvas/CANVAS-006-hit-test-layers-precisely-and-fast.md) | Hit-test layers precisely and fast | P0 | CANVAS-003 |
| [CANVAS-007](04-canvas/CANVAS-007-show-selection-resize-and-rotate-handles.md) | Show selection, resize and rotate handles | P0 | CANVAS-006 |
| [CANVAS-008](04-canvas/CANVAS-008-move-resize-rotate-and-flip-layers.md) | Move, resize, rotate and flip layers | P0 | CANVAS-007, CORE-012 |
| [CANVAS-009](04-canvas/CANVAS-009-select-multiple-layers-by-marquee-shift-click.md) | Select multiple layers by marquee, shift-click and select-all | P0 | CANVAS-006 |
| [CANVAS-010](04-canvas/CANVAS-010-pan-and-zoom-the-infinite-canvas.md) | Pan and zoom the infinite canvas | P0 | CANVAS-002 |
| [CANVAS-011](04-canvas/CANVAS-011-snap-to-objects-pixel-grid-and-spacing.md) | Snap to objects, pixel grid and spacing | P0 | CANVAS-008 |
| [CANVAS-012](04-canvas/CANVAS-012-add-rulers-guides-and-layout-grids.md) | Add rulers, guides and layout grids | P1 | CANVAS-011 |
| [CANVAS-013](04-canvas/CANVAS-013-implement-the-frame-tool-and-nested-frames.md) | Implement the Frame tool and nested frames | P0 | CANVAS-008 |
| [CANVAS-014](04-canvas/CANVAS-014-implement-shape-tools-rectangle-ellipse-line-arrow.md) | Implement shape tools: rectangle, ellipse, line, arrow, polygon, star | P0 | CANVAS-008 |
| [CANVAS-015](04-canvas/CANVAS-015-implement-the-pen-tool-and-vector-network.md) | Implement the pen tool and vector network editing | P0 | CANVAS-014 |
| [CANVAS-016](04-canvas/CANVAS-016-add-boolean-operations-and-flatten.md) | Add boolean operations and flatten | P1 | CANVAS-015 |
| [CANVAS-017](04-canvas/CANVAS-017-place-crop-and-mask-images.md) | Place, crop and mask images | P0 | CANVAS-003 |
| [CANVAS-018](04-canvas/CANVAS-018-add-the-eyedropper.md) | Add the eyedropper | P1 | CANVAS-003 |
| [CANVAS-019](04-canvas/CANVAS-019-add-fills-strokes-and-effects.md) | Add fills, strokes and effects | P0 | CANVAS-003 |
| [CANVAS-020](04-canvas/CANVAS-020-implement-blend-modes-and-opacity-groups.md) | Implement blend modes and opacity groups | P1 | CANVAS-019 |
| [CANVAS-021](04-canvas/CANVAS-021-lay-out-frames-with-taffy-auto-layout.md) | Lay out frames with Taffy auto layout | P0 | CANVAS-013, CORE-004 |
| [CANVAS-022](04-canvas/CANVAS-022-add-constraints-for-non-auto-layout-children.md) | Add constraints for non-auto-layout children | P0 | CANVAS-008 |
| [CANVAS-023](04-canvas/CANVAS-023-support-responsive-breakpoints-for-one-design.md) | Support responsive breakpoints for one design | P1 | CANVAS-021 |
| [CANVAS-024](04-canvas/CANVAS-024-group-ungroup-reorder-and-align-layers.md) | Group, ungroup, reorder and align layers | P0 | CANVAS-009 |
| [CANVAS-025](04-canvas/CANVAS-025-build-the-layers-panel.md) | Build the Layers panel | P0 | CORE-004, DS-010, DS-016 |
| [CANVAS-026](04-canvas/CANVAS-026-build-the-assets-panel.md) | Build the Assets panel | P0 | CORE-009, CORE-008, DS-010 |
| [CANVAS-027](04-canvas/CANVAS-027-build-the-tokens-panel.md) | Build the Tokens panel | P1 | CORE-007, DS-010 |
| [CANVAS-028](04-canvas/CANVAS-028-build-the-contextual-properties-strip.md) | Build the contextual properties strip | P0 | CANVAS-007, CORE-007, DS-011 |
| [CANVAS-029](04-canvas/CANVAS-029-bind-layer-properties-to-design-tokens.md) | Bind layer properties to design tokens | P0 | CORE-007, CANVAS-028 |
| [CANVAS-030](04-canvas/CANVAS-030-create-components-variants-and-instance-overrides.md) | Create components, variants and instance overrides | P0 | CORE-009, CANVAS-028 |
| [CANVAS-031](04-canvas/CANVAS-031-copy-and-paste-within-cade-and-across.md) | Copy and paste within CADE and across apps | P0 | CORE-004 |
| [CANVAS-032](04-canvas/CANVAS-032-export-selections-and-frames-to-png-svg.md) | Export selections and frames to PNG, SVG and PDF | P0 | CANVAS-003 |
| [CANVAS-033](04-canvas/CANVAS-033-add-comment-pins-on-the-canvas.md) | Add comment pins on the canvas | P1 | CANVAS-006 |
| [CANVAS-034](04-canvas/CANVAS-034-show-copilot-s-live-cursor-and-pending.md) | Show Copilot's live cursor and pending regions | P0 | CORE-023, CANVAS-003 |
| [CANVAS-035](04-canvas/CANVAS-035-render-staged-change-overlays-for-copilot-runs.md) | Render staged change overlays for Copilot runs | P0 | CORE-013, CANVAS-003 |
| [CANVAS-036](04-canvas/CANVAS-036-navigate-pages.md) | Navigate pages | P0 | CORE-004 |
| [CANVAS-037](04-canvas/CANVAS-037-render-accessibility-checks-on-the-canvas.md) | Render accessibility checks on the canvas | P1 | CANVAS-028 |
| [CANVAS-038](04-canvas/CANVAS-038-set-and-meet-canvas-performance-budgets.md) | Set and meet canvas performance budgets | P0 | CANVAS-003, CANVAS-010 |
| [CANVAS-039](04-canvas/CANVAS-039-add-canvas-keyboard-shortcuts-and-the-tool.md) | Add canvas keyboard shortcuts and the tool rail | P0 | CANVAS-002, DS-007, FOUND-008 |
| [CANVAS-040](04-canvas/CANVAS-040-keep-the-canvas-usable-when-the-gpu.md) | Keep the canvas usable when the GPU is lost | P1 | CANVAS-001 |
| [CANVAS-041](04-canvas/CANVAS-041-build-shared-orthogonal-connector-router.md) | Build a shared orthogonal connector router in cade-layout | P1 | CORE-005 |

## Copilot (`05-copilot`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [COPILOT-001](05-copilot/COPILOT-001-cade-copilot-crate-sdk-client.md) | Create cade-copilot crate wrapping github-copilot-sdk | P0 | FOUND-001 |
| [COPILOT-002](05-copilot/COPILOT-002-model-provider-trait.md) | Define the ModelProvider trait and streaming event types | P0 | COPILOT-001 |
| [COPILOT-003](05-copilot/COPILOT-003-copilot-provider-impl.md) | Implement the Copilot provider on the SDK | P0 | COPILOT-002, GH-001 |
| [COPILOT-004](05-copilot/COPILOT-004-bundled-cli-lifecycle.md) | Manage the bundled Copilot CLI process lifecycle | P0 | COPILOT-001, FOUND-003 |
| [COPILOT-005](05-copilot/COPILOT-005-byok-anthropic.md) | Add the Anthropic BYOK provider | P1 | COPILOT-002, COPILOT-009, COPILOT-010 |
| [COPILOT-006](05-copilot/COPILOT-006-byok-openai.md) | Add the OpenAI BYOK provider | P1 | COPILOT-002, COPILOT-009, COPILOT-010 |
| [COPILOT-007](05-copilot/COPILOT-007-byok-azure-openai.md) | Add the Azure OpenAI BYOK provider | P1 | COPILOT-006 |
| [COPILOT-008](05-copilot/COPILOT-008-byok-openai-compatible.md) | Add the OpenAI-compatible endpoint provider | P1 | COPILOT-006 |
| [COPILOT-009](05-copilot/COPILOT-009-keychain-secrets.md) | Store provider secrets in the OS keychain | P0 | FOUND-010 |
| [COPILOT-010](05-copilot/COPILOT-010-byok-agent-loop.md) | Build CADE's own agent loop for BYOK providers | P1 | COPILOT-002, COPILOT-018, COPILOT-019 |
| [COPILOT-011](05-copilot/COPILOT-011-model-defaults-per-output.md) | Select default models per output type | P1 | COPILOT-003, POLICY-007 |
| [COPILOT-012](05-copilot/COPILOT-012-project-threads.md) | Persist Copilot threads per project and artifact | P0 | COPILOT-002, CORE-015 |
| [COPILOT-013](05-copilot/COPILOT-013-thread-history-search.md) | Add thread list, rename, archive and search | P1 | COPILOT-012 |
| [COPILOT-014](05-copilot/COPILOT-014-copilot-panel-ui.md) | Build the Copilot panel message list and composer | P0 | COPILOT-002, DS-012 |
| [COPILOT-015](05-copilot/COPILOT-015-context-chips.md) | Attach context with chips and @-mentions | P0 | COPILOT-014, CORE-024 |
| [COPILOT-016](05-copilot/COPILOT-016-context-inspector.md) | Show exactly what context is sent to the model | P1 | COPILOT-015 |
| [COPILOT-017](05-copilot/COPILOT-017-project-instructions-skills.md) | Load project instructions and skills | P1 | COPILOT-012 |
| [COPILOT-018](05-copilot/COPILOT-018-agent-tools-read.md) | Expose read-only document tools to the agent | P0 | CORE-011, COPILOT-002 |
| [COPILOT-019](05-copilot/COPILOT-019-agent-tools-write.md) | Expose write tools that produce staged operations | P0 | COPILOT-018, COPILOT-021, CORE-011 |
| [COPILOT-020](05-copilot/COPILOT-020-agent-tool-packs.md) | Register output-type tool packs for the agent | P1 | COPILOT-019 |
| [COPILOT-021](05-copilot/COPILOT-021-staged-change-sets.md) | Stage agent edits as change sets on a Loro fork | P0 | CORE-013, COPILOT-002 |
| [COPILOT-022](05-copilot/COPILOT-022-change-set-review-ui.md) | Review staged changes one by one | P0 | COPILOT-021, COPILOT-014 |
| [COPILOT-023](05-copilot/COPILOT-023-before-after-overlay.md) | Show Before, After and Overlay views for a run | P1 | COPILOT-021, CANVAS-035 |
| [COPILOT-024](05-copilot/COPILOT-024-undo-whole-run.md) | Undo or restore a whole Copilot run as one step | P0 | COPILOT-021, CORE-012 |
| [COPILOT-025](05-copilot/COPILOT-025-plan-and-run-control.md) | Show the agent plan and allow stopping a run | P0 | COPILOT-002, COPILOT-014 |
| [COPILOT-027](05-copilot/COPILOT-027-inline-questions.md) | Ask clarifying questions inline with options | P1 | COPILOT-014, COPILOT-018 |
| [COPILOT-028](05-copilot/COPILOT-028-shell-permission-prompts.md) | Prompt before shell commands, scoped to the project | P0 | COPILOT-003, POLICY-008 |
| [COPILOT-029](05-copilot/COPILOT-029-filesystem-scope.md) | Restrict agent file access to the project folder | P0 | COPILOT-018 |
| [COPILOT-030](05-copilot/COPILOT-030-visual-self-check.md) | Let the agent render frames and check its own work | P1 | CANVAS-032, COPILOT-019 |
| [COPILOT-031](05-copilot/COPILOT-031-token-metering.md) | Meter tokens per run, thread, output type and month | P0 | COPILOT-002 |
| [COPILOT-032](05-copilot/COPILOT-032-state-budget-reached.md) | Handle the token budget reached state | P0 | COPILOT-031, POLICY-021 |
| [COPILOT-033](05-copilot/COPILOT-033-state-policy-blocked.md) | Handle a policy-blocked step during a run | P0 | POLICY-008, COPILOT-025 |
| [COPILOT-034](05-copilot/COPILOT-034-state-offline-queue.md) | Queue prompts while offline | P1 | COPILOT-014 |
| [COPILOT-035](05-copilot/COPILOT-035-state-run-failed.md) | Recover from a run that fails mid-way | P0 | COPILOT-021, COPILOT-025 |
| [COPILOT-036](05-copilot/COPILOT-036-state-model-unavailable.md) | Fall back when a model is unavailable | P1 | COPILOT-011 |
| [COPILOT-037](05-copilot/COPILOT-037-state-signin-expired.md) | Handle an expired GitHub session without losing work | P0 | GH-001 |
| [COPILOT-038](05-copilot/COPILOT-038-run-log-local.md) | Keep a local, redacted run log for debugging | P1 | COPILOT-002 |
| [COPILOT-039](05-copilot/COPILOT-039-prompt-injection-defenses.md) | Defend agent runs against prompt injection | P1 | COPILOT-018, COPILOT-028 |
| [COPILOT-040](05-copilot/COPILOT-040-copilot-test-harness.md) | Build a recorded-session test harness for Copilot flows | P1 | COPILOT-002, COPILOT-021 |
| [COPILOT-041](05-copilot/COPILOT-041-connect-mcp-servers-from-org-allowlist.md) | Connect MCP servers from the org allowlist | P1 | COPILOT-003, COPILOT-010 |

## App (`06-app`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [APP-001](06-app/APP-001-create-an-app-project-from-start.md) | Create an App project from Start | P0 | CORE-003, FOUND-003 |
| [APP-002](06-app/APP-002-build-the-app-workspace-shell-and-mode.md) | Build the App workspace shell and mode switch | P0 | APP-001, DS-010 |
| [APP-003](06-app/APP-003-link-and-update-a-design-system-library.md) | Link and update a design system library | P0 | CORE-008, CORE-007 |
| [APP-004](06-app/APP-004-define-the-component-to-code-mapping-for.md) | Define the component-to-code mapping for a design system | P0 | APP-003 |
| [APP-005](06-app/APP-005-generate-react-typescript-from-frames.md) | Generate React + TypeScript from frames | P0 | APP-004, CANVAS-021 |
| [APP-006](06-app/APP-006-sync-design-and-code-both-ways.md) | Sync design and code both ways | P0 | APP-005 |
| [APP-007](06-app/APP-007-embed-a-code-editor-in-code-mode.md) | Embed a code editor in Code mode | P0 | APP-006 |
| [APP-008](06-app/APP-008-generate-responsive-css-from-breakpoints.md) | Generate responsive CSS from breakpoints | P1 | CANVAS-023, APP-005 |
| [APP-009](06-app/APP-009-run-a-sandboxed-live-preview.md) | Run a sandboxed live preview | P0 | APP-005, FOUND-003 |
| [APP-010](06-app/APP-010-scaffold-the-project-s-runtime-on-first.md) | Scaffold the project's runtime on first codegen | P0 | APP-005 |
| [APP-011](06-app/APP-011-lint-generated-and-edited-code-against-the.md) | Lint generated and edited code against the design system | P0 | APP-005 |
| [APP-012](06-app/APP-012-generate-validation-from-the-data-model.md) | Generate validation from the data model | P0 | DATA-020, APP-005 |
| [APP-013](06-app/APP-013-generate-tests-from-validation-rules.md) | Generate tests from validation rules | P1 | APP-012, COPILOT-019 |
| [APP-014](06-app/APP-014-add-error-empty-and-loading-states-as.md) | Add error, empty and loading states as variants | P1 | CANVAS-030 |
| [APP-015](06-app/APP-015-build-prototype-mode-flows.md) | Build Prototype mode: flows | P0 | APP-002 |
| [APP-016](06-app/APP-016-define-interactions-triggers-conditions-else-and-actions.md) | Define interactions: triggers, conditions, else and actions | P0 | APP-015 |
| [APP-017](06-app/APP-017-add-transitions-using-motion-tokens.md) | Add transitions using motion tokens | P1 | APP-016, DS-005 |
| [APP-018](06-app/APP-018-add-prototype-variables-and-sample-data.md) | Add prototype variables and sample data | P1 | APP-016, DATA-019 |
| [APP-019](06-app/APP-019-play-a-flow-in-the-app.md) | Play a flow in the app | P0 | APP-016 |
| [APP-020](06-app/APP-020-preview-on-a-phone-over-the-local.md) | Preview on a phone over the local network | P1 | APP-019, APP-009 |
| [APP-021](06-app/APP-021-record-a-prototype-walkthrough.md) | Record a prototype walkthrough | P2 | APP-019, MOTION-014 |
| [APP-022](06-app/APP-022-expose-app-canvas-operations-as-copilot-tools.md) | Expose app canvas operations as Copilot tools | P0 | COPILOT-020, APP-005 |
| [APP-023](06-app/APP-023-start-an-agent-run-from-a-selection.md) | Start an agent run from a selection | P0 | APP-022, COPILOT-015 |
| [APP-024](06-app/APP-024-ship-an-app-project-to-a-repo.md) | Ship an App project to a repo | P0 | GH-010, APP-011 |
| [APP-025](06-app/APP-025-ship-a-design-spec-without-code.md) | Ship a design spec without code | P1 | APP-024 |
| [APP-026](06-app/APP-026-show-what-admins-can-see-before-shipping.md) | Show what admins can see before shipping | P0 | APP-024, POLICY-022 |
| [APP-028](06-app/APP-028-persist-app-workspace-state-per-project.md) | Persist App workspace state per project | P1 | APP-002 |

## Image (`07-image`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [IMAGE-001](07-image/IMAGE-001-create-an-image-project-with-size-presets.md) | Create an Image project with size presets | P0 | CORE-003, FOUND-003 |
| [IMAGE-002](07-image/IMAGE-002-build-the-image-workspace-layout.md) | Build the Image workspace layout | P0 | IMAGE-001, DS-010 |
| [IMAGE-003](07-image/IMAGE-003-keep-multiple-sizes-in-sync.md) | Keep multiple sizes in sync | P0 | IMAGE-001, CANVAS-022 |
| [IMAGE-004](07-image/IMAGE-004-enforce-a-brand-locked-palette.md) | Enforce a brand-locked palette | P0 | CANVAS-029 |
| [IMAGE-005](07-image/IMAGE-005-switch-a-shape-s-form-without-losing.md) | Switch a shape's form without losing its styling | P1 | CANVAS-014, CANVAS-015 |
| [IMAGE-006](07-image/IMAGE-006-apply-effects-with-presets.md) | Apply effects with presets | P0 | CANVAS-019 |
| [IMAGE-007](07-image/IMAGE-007-edit-opacity-and-layer-order-for-illustration.md) | Edit opacity and layer order for illustration work | P0 | CANVAS-020, CANVAS-024 |
| [IMAGE-008](07-image/IMAGE-008-place-brand-assets.md) | Place brand assets | P0 | CANVAS-026 |
| [IMAGE-009](07-image/IMAGE-009-generate-variations-side-by-side.md) | Generate variations side by side | P1 | COPILOT-020, IMAGE-003 |
| [IMAGE-010](07-image/IMAGE-010-scope-copilot-edits-to-the-selected-shape.md) | Scope Copilot edits to the selected shape | P0 | COPILOT-015, IMAGE-003 |
| [IMAGE-011](07-image/IMAGE-011-export-every-size-in-one-action.md) | Export every size in one action | P0 | CANVAS-032, IMAGE-003 |
| [IMAGE-012](07-image/IMAGE-012-check-text-legibility-across-sizes.md) | Check text legibility across sizes | P1 | IMAGE-003, CANVAS-037 |
| [IMAGE-013](07-image/IMAGE-013-add-alt-text-to-image-exports.md) | Add alt text to image exports | P1 | IMAGE-011 |
| [IMAGE-014](07-image/IMAGE-014-use-generative-fills-only-through-a-policy.md) | Use generative fills only through a policy-gated layer type | P2 | POLICY-001, COPILOT-019 |

## Deck (`08-deck`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [DECK-001](08-deck/DECK-001-deck-document-model.md) | Define the deck and slide model in the Loro document | P0 | CORE-003 |
| [DECK-002](08-deck/DECK-002-slide-strip-panel.md) | Build the slide strip with reorder, duplicate and sections | P0 | DECK-001, DS-010 |
| [DECK-003](08-deck/DECK-003-masters-and-layouts.md) | Implement slide masters and layouts with inheritance | P0 | DECK-001, CORE-003 |
| [DECK-004](08-deck/DECK-004-placeholders-autofit.md) | Add title, body and media placeholders with text autofit | P0 | DECK-003, CANVAS-004 |
| [DECK-005](08-deck/DECK-005-aspect-ratio-resize.md) | Support 16:9, 4:3 and custom slide sizes with rescale | P1 | DECK-001 |
| [DECK-006](08-deck/DECK-006-deck-workspace-shell.md) | Build the Deck workspace header and layout | P0 | DECK-002, DS-010, COPILOT-014 |
| [DECK-007](08-deck/DECK-007-chart-object-model.md) | Define a native chart object with series and data table | P0 | CORE-003 |
| [DECK-008](08-deck/DECK-008-chart-rendering.md) | Render charts on the canvas with axes, labels and legends | P0 | DECK-007, CANVAS-003 |
| [DECK-009](08-deck/DECK-009-chart-properties-strip.md) | Build the chart properties strip | P0 | DECK-008, DS-011 |
| [DECK-010](08-deck/DECK-010-chart-data-editor.md) | Add a spreadsheet-style chart data editor with paste and CSV | P0 | DECK-007 |
| [DECK-011](08-deck/DECK-011-chart-sub-selection.md) | Make series, data points and labels individually selectable | P0 | DECK-008, CORE-024 |
| [DECK-012](08-deck/DECK-012-slide-tables.md) | Add tables to slides | P1 | DECK-001, DOC-008 |
| [DECK-013](08-deck/DECK-013-speaker-notes.md) | Add per-slide speaker notes with Copilot drafting | P0 | DECK-001 |
| [DECK-014](08-deck/DECK-014-present-mode.md) | Implement full-screen present mode | P0 | DECK-008, CANVAS-003 |
| [DECK-015](08-deck/DECK-015-presenter-view.md) | Add a presenter view with notes, next slide and timer | P1 | DECK-014, DECK-013 |
| [DECK-016](08-deck/DECK-016-linked-live-frames.md) | Embed live linked frames from App and other outputs | P0 | DECK-001, CORE-022, APP-005 |
| [DECK-017](08-deck/DECK-017-pptx-export.md) | Export decks to .pptx with editable text, shapes and charts | P0 | DECK-003, DECK-008, DECK-013 |
| [DECK-018](08-deck/DECK-018-pptx-export-check.md) | Add the pre-export fidelity check for .pptx | P0 | DECK-017 |
| [DECK-019](08-deck/DECK-019-font-embedding-mapping.md) | Embed or map fonts for .pptx and PDF export | P1 | DECK-017 |
| [DECK-020](08-deck/DECK-020-deck-pdf-png-export.md) | Export decks to PDF, handouts and PNG | P1 | DECK-014, CANVAS-032 |
| [DECK-021](08-deck/DECK-021-pptx-potx-reader.md) | Read .pptx and .potx into the deck model | P0 | DECK-003, DECK-007 |
| [DECK-022](08-deck/DECK-022-copilot-deck-tools.md) | Expose deck operations as Copilot agent tools | P0 | COPILOT-020, DECK-007, DECK-016 |
| [DECK-023](08-deck/DECK-023-deck-quick-actions.md) | Add deck quick actions in the Copilot column | P2 | DECK-022 |
| [DECK-024](08-deck/DECK-024-deck-accessibility.md) | Check slide reading order, alt text and contrast | P1 | DECK-017 |

## Document (`09-document`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [DOC-001](09-document/DOC-001-document-page-model.md) | Define the document model: flow, pages and page settings | P0 | CORE-003 |
| [DOC-002](09-document/DOC-002-pagination-engine.md) | Build the incremental pagination engine | P0 | DOC-001, CANVAS-004 |
| [DOC-003](09-document/DOC-003-page-canvas-and-thumbnails.md) | Show pages on the canvas with a pages panel | P0 | DOC-002, CANVAS-003 |
| [DOC-004](09-document/DOC-004-text-styles-from-ds.md) | Bind document text styles to design-system tokens | P0 | DOC-001, CORE-007 |
| [DOC-005](09-document/DOC-005-rich-text-editing.md) | Implement rich text editing with IME and spellcheck | P0 | DOC-001, CANVAS-005 |
| [DOC-006](09-document/DOC-006-callout-block.md) | Add the callout block with tone tokens | P0 | DOC-004 |
| [DOC-007](09-document/DOC-007-stat-row-block.md) | Add the stat row block with placeholder detection | P1 | DOC-004 |
| [DOC-008](09-document/DOC-008-table-block.md) | Add the table block shared by documents and decks | P0 | DOC-002 |
| [DOC-009](09-document/DOC-009-linked-frame-block.md) | Embed live linked frames in documents | P0 | DOC-001, DECK-016 |
| [DOC-010](09-document/DOC-010-images-figures.md) | Add images and figures with captions | P1 | DOC-002 |
| [DOC-011](09-document/DOC-011-headers-footers.md) | Add headers, footers, page numbers and classification labels | P1 | DOC-002 |
| [DOC-012](09-document/DOC-012-section-columns.md) | Support multi-column sections | P2 | DOC-002 |
| [DOC-013](09-document/DOC-013-block-properties-strip.md) | Build the document block properties strip | P0 | DOC-006, DS-011 |
| [DOC-014](09-document/DOC-014-print-check-rules.md) | Implement the print check rules engine | P0 | DOC-002, DOC-007, DOC-010 |
| [DOC-015](09-document/DOC-015-print-check-panel.md) | Show print check results in the header and Copilot column | P0 | DOC-014 |
| [DOC-016](09-document/DOC-016-pdf-export.md) | Export documents to tagged PDF | P0 | DOC-002, DOC-010 |
| [DOC-017](09-document/DOC-017-docx-export.md) | Export documents to editable .docx | P0 | DOC-004, DOC-008, DOC-011 |
| [DOC-018](09-document/DOC-018-markdown-export.md) | Export documents to Markdown | P1 | DOC-001 |
| [DOC-019](09-document/DOC-019-copilot-doc-tools.md) | Expose document editing as Copilot tools | P0 | COPILOT-020, DOC-006 |
| [DOC-020](09-document/DOC-020-document-templates.md) | Provide document templates from the design system | P1 | DOC-004, CORE-008 |
| [DOC-021](09-document/DOC-021-docx-import.md) | Import .docx into a document | P2 | DOC-017 |

## Motion (`10-motion`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [MOTION-001](10-motion/MOTION-001-motion-model.md) | Define the motion model: timelines, tracks and keyframes | P0 | CORE-003 |
| [MOTION-002](10-motion/MOTION-002-animatable-properties.md) | Build the animatable property registry | P0 | MOTION-001, CANVAS-003 |
| [MOTION-003](10-motion/MOTION-003-interpolation-engine.md) | Implement interpolation for numbers, colors and paths | P0 | MOTION-002 |
| [MOTION-004](10-motion/MOTION-004-easing-and-motion-tokens.md) | Support cubic-bezier and spring easing bound to tokens | P0 | MOTION-003, DS-005 |
| [MOTION-005](10-motion/MOTION-005-timeline-panel.md) | Build the timeline panel with tracks, ruler and bars | P0 | MOTION-001, DS-010 |
| [MOTION-006](10-motion/MOTION-006-playback-controls.md) | Add playhead, play/pause, loop and frame stepping | P0 | MOTION-003, MOTION-005 |
| [MOTION-007](10-motion/MOTION-007-keyframe-editing.md) | Implement keyframe add, move, delete and token snapping | P0 | MOTION-005 |
| [MOTION-008](10-motion/MOTION-008-easing-curve-editor.md) | Build the easing curve editor | P1 | MOTION-004 |
| [MOTION-009](10-motion/MOTION-009-keyframe-properties-strip.md) | Build the keyframe properties strip | P0 | MOTION-007, DS-011 |
| [MOTION-010](10-motion/MOTION-010-state-stage-view.md) | Show key states side by side on the stage | P1 | MOTION-006 |
| [MOTION-011](10-motion/MOTION-011-reduced-motion-fallback.md) | Require and preview a reduced-motion fallback | P0 | MOTION-001 |
| [MOTION-012](10-motion/MOTION-012-motion-in-app-prototype.md) | Use motions in App prototype interactions | P0 | MOTION-001, APP-016 |
| [MOTION-013](10-motion/MOTION-013-offscreen-frame-renderer.md) | Render deterministic offscreen frames for export | P0 | MOTION-003, CANVAS-003 |
| [MOTION-014](10-motion/MOTION-014-mp4-export.md) | Export MP4 (H.264) video | P1 | MOTION-013 |
| [MOTION-015](10-motion/MOTION-015-gif-export.md) | Export GIF with palette quantization | P1 | MOTION-013 |
| [MOTION-016](10-motion/MOTION-016-lottie-export.md) | Export Lottie JSON | P1 | MOTION-002, MOTION-004 |
| [MOTION-017](10-motion/MOTION-017-css-export.md) | Export CSS keyframes that reference design tokens | P1 | MOTION-004, MOTION-011 |
| [MOTION-018](10-motion/MOTION-018-copilot-motion-tools.md) | Expose motion editing as Copilot tools | P1 | COPILOT-020, MOTION-007 |
| [MOTION-019](10-motion/MOTION-019-compare-variants.md) | Compare motion variants side by side | P2 | MOTION-006 |
| [MOTION-020](10-motion/MOTION-020-motion-perf-budget.md) | Hold playback performance budgets | P1 | MOTION-006 |

## Diagram (`11-diagram`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [DIAGRAM-001](11-diagram/DIAGRAM-001-diagram-model.md) | Define the diagram model: shapes, connectors, pages and layers | P0 | CORE-003 |
| [DIAGRAM-002](11-diagram/DIAGRAM-002-basic-shape-library.md) | Ship built-in flowchart and process shape libraries | P0 | DIAGRAM-001 |
| [DIAGRAM-003](11-diagram/DIAGRAM-003-stencil-panel.md) | Build the stencil panel with search and drag-to-canvas | P0 | DIAGRAM-002, DS-016 |
| [DIAGRAM-004](11-diagram/DIAGRAM-004-connector-tool-and-glue.md) | Implement the connector tool with point and dynamic glue | P0 | DIAGRAM-001, CANVAS-006 |
| [DIAGRAM-005](11-diagram/DIAGRAM-005-orthogonal-routing.md) | Build orthogonal connector routing with obstacle avoidance | P0 | DIAGRAM-004, CANVAS-041 |
| [DIAGRAM-006](11-diagram/DIAGRAM-006-connector-labels-arrows.md) | Add connector labels and arrowheads | P0 | DIAGRAM-004 |
| [DIAGRAM-007](11-diagram/DIAGRAM-007-swimlanes.md) | Implement swimlanes with lane membership | P0 | DIAGRAM-001 |
| [DIAGRAM-008](11-diagram/DIAGRAM-008-containers.md) | Add containers with membership and auto-resize | P1 | DIAGRAM-001 |
| [DIAGRAM-009](11-diagram/DIAGRAM-009-diagram-layers.md) | Support layers with visibility, lock and print settings | P0 | DIAGRAM-001 |
| [DIAGRAM-010](11-diagram/DIAGRAM-010-shape-data.md) | Implement shape data (custom properties) | P0 | DIAGRAM-001 |
| [DIAGRAM-011](11-diagram/DIAGRAM-011-shape-text.md) | Support text on shapes with wrap and autosize | P0 | DIAGRAM-001, CANVAS-005 |
| [DIAGRAM-012](11-diagram/DIAGRAM-012-diagram-pages.md) | Support multiple pages with background pages | P1 | DIAGRAM-001 |
| [DIAGRAM-013](11-diagram/DIAGRAM-013-diagram-properties-strip.md) | Build the diagram properties strip | P0 | DIAGRAM-007, DIAGRAM-010, DS-011 |
| [DIAGRAM-014](11-diagram/DIAGRAM-014-vsdx-reader.md) | Read .vsdx files into the diagram model | P0 | DIAGRAM-001, DIAGRAM-009, DIAGRAM-010 |
| [DIAGRAM-015](11-diagram/DIAGRAM-015-shapesheet-subset.md) | Evaluate a ShapeSheet formula subset and freeze the rest | P1 | DIAGRAM-014 |
| [DIAGRAM-016](11-diagram/DIAGRAM-016-vsd-legacy-import.md) | Import legacy binary .vsd files | P1 | DIAGRAM-014 |
| [DIAGRAM-017](11-diagram/DIAGRAM-017-stencils-and-templates.md) | Import .vssx stencils and .vstx templates | P0 | DIAGRAM-014, DIAGRAM-003 |
| [DIAGRAM-018](11-diagram/DIAGRAM-018-macro-stripping.md) | Strip macros from Visio files and never execute them | P0 | DIAGRAM-014 |
| [DIAGRAM-019](11-diagram/DIAGRAM-019-visio-import-report.md) | Show the Visio import fidelity report in the Diagram workspace | P0 | DIAGRAM-014, IMPORT-002 |
| [DIAGRAM-020](11-diagram/DIAGRAM-020-vsdx-writer.md) | Export diagrams to .vsdx | P0 | DIAGRAM-014 |
| [DIAGRAM-021](11-diagram/DIAGRAM-021-visio-roundtrip-tests.md) | Build a Visio round-trip test corpus | P1 | DIAGRAM-020 |
| [DIAGRAM-022](11-diagram/DIAGRAM-022-auto-layout.md) | Add automatic diagram layout | P1 | DIAGRAM-005 |
| [DIAGRAM-023](11-diagram/DIAGRAM-023-mermaid-import.md) | Import Mermaid diagrams | P1 | DIAGRAM-022 |
| [DIAGRAM-024](11-diagram/DIAGRAM-024-mermaid-export.md) | Export diagrams to Mermaid | P1 | DIAGRAM-001 |
| [DIAGRAM-025](11-diagram/DIAGRAM-025-diagram-image-export.md) | Export diagram pages to SVG, PDF and PNG | P0 | DIAGRAM-001, CANVAS-032 |
| [DIAGRAM-026](11-diagram/DIAGRAM-026-copilot-diagram-tools.md) | Expose diagram editing as Copilot tools | P0 | COPILOT-020, DIAGRAM-005 |
| [DIAGRAM-027](11-diagram/DIAGRAM-027-diagram-app-consistency.md) | Compare a process diagram with the App prototype | P2 | DIAGRAM-026, APP-015 |

## Architecture (Azure) (`12-architecture`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [ARCH-001](12-architecture/ARCH-001-architecture-document-model.md) | Define the architecture document model | P0 | CORE-003 |
| [ARCH-002](12-architecture/ARCH-002-azure-type-schema-ingest.md) | Generate the Azure service catalog from Bicep type schemas | P0 | ARCH-001 |
| [ARCH-003](12-architecture/ARCH-003-catalog-update-channel.md) | Refresh the Azure catalog without an app release | P1 | ARCH-002 |
| [ARCH-004](12-architecture/ARCH-004-icon-pack-loader.md) | Load official Azure Architecture Icons from assets/icons/azure | P0 | — |
| [ARCH-005](12-architecture/ARCH-005-icon-type-mapping.md) | Map Azure resource types to icon files | P0 | ARCH-004, ARCH-002 |
| [ARCH-006](12-architecture/ARCH-006-service-palette.md) | Build the service palette with categories and search | P0 | ARCH-005 |
| [ARCH-007](12-architecture/ARCH-007-containment-model.md) | Model subscription, resource group, VNet and subnet containment | P0 | ARCH-001 |
| [ARCH-008](12-architecture/ARCH-008-connection-semantics.md) | Add typed connections between Azure resources | P0 | ARCH-001, ARCH-007 |
| [ARCH-009](12-architecture/ARCH-009-connection-routing.md) | Route architecture connections orthogonally around resources | P1 | ARCH-008, CANVAS-041 |
| [ARCH-010](12-architecture/ARCH-010-schema-driven-properties.md) | Generate the properties strip from the resource schema | P0 | ARCH-002, DS-011 |
| [ARCH-011](12-architecture/ARCH-011-api-version-selection.md) | Let users pick and upgrade resource API versions | P1 | ARCH-002, ARCH-010 |
| [ARCH-012](12-architecture/ARCH-012-naming-conventions.md) | Apply Azure naming conventions and abbreviations | P1 | ARCH-001 |
| [ARCH-013](12-architecture/ARCH-013-bicep-generation.md) | Generate Bicep from the architecture model | P0 | ARCH-001, ARCH-008, ARCH-010 |
| [ARCH-014](12-architecture/ARCH-014-bicepparam-environments.md) | Generate .bicepparam files per environment | P0 | ARCH-013 |
| [ARCH-015](12-architecture/ARCH-015-terraform-generation.md) | Generate Terraform (azurerm) from the architecture model | P0 | ARCH-001, ARCH-008 |
| [ARCH-016](12-architecture/ARCH-016-code-drawer.md) | Show generated IaC in a synced code drawer | P0 | ARCH-013, ARCH-015 |
| [ARCH-017](12-architecture/ARCH-017-code-to-canvas-sync.md) | Parse hand edits in Bicep back into the model | P1 | ARCH-016 |
| [ARCH-018](12-architecture/ARCH-018-terraform-to-canvas-sync.md) | Parse hand edits in Terraform back into the model | P2 | ARCH-016 |
| [ARCH-019](12-architecture/ARCH-019-diff-highlighting.md) | Highlight model and code diffs for staged changes | P0 | ARCH-016 |
| [ARCH-020](12-architecture/ARCH-020-bicep-build-lint.md) | Run bicep build and the Bicep linter locally | P0 | ARCH-013 |
| [ARCH-021](12-architecture/ARCH-021-psrule-azure.md) | Run PSRule for Azure against generated IaC | P1 | ARCH-020 |
| [ARCH-022](12-architecture/ARCH-022-tflint.md) | Run terraform validate and tflint for Terraform projects | P1 | ARCH-015 |
| [ARCH-023](12-architecture/ARCH-023-azure-signin.md) | Use the user's own Azure CLI sign-in for read operations | P0 | — |
| [ARCH-024](12-architecture/ARCH-024-what-if.md) | Run what-if against a chosen subscription | P1 | ARCH-023, ARCH-013 |
| [ARCH-025](12-architecture/ARCH-025-well-architected-review.md) | Review the design against Azure Well-Architected | P1 | ARCH-021 |
| [ARCH-026](12-architecture/ARCH-026-reverse-engineer-rg.md) | Reverse-engineer a design from an existing resource group | P1 | ARCH-023, ARCH-007 |
| [ARCH-027](12-architecture/ARCH-027-import-arm-bicep-files.md) | Import existing ARM JSON or Bicep files | P1 | ARCH-017 |
| [ARCH-028](12-architecture/ARCH-028-auto-layout.md) | Lay out architecture diagrams automatically | P1 | ARCH-007, ARCH-009 |
| [ARCH-029](12-architecture/ARCH-029-landing-zone-policy.md) | Respect landing-zone and Azure Policy constraints | P1 | ARCH-023 |
| [ARCH-030](12-architecture/ARCH-030-tags-and-diagnostics.md) | Apply standard tags and diagnostic settings | P1 | ARCH-013 |
| [ARCH-031](12-architecture/ARCH-031-copilot-iac-tools.md) | Expose architecture operations as Copilot tools | P0 | ARCH-001, ARCH-010, COPILOT-020 |
| [ARCH-032](12-architecture/ARCH-032-cost-estimate.md) | Show an estimated monthly cost per environment | P2 | ARCH-014 |
| [ARCH-033](12-architecture/ARCH-033-architecture-export.md) | Export architecture diagrams to PNG, SVG and PDF | P1 | ARCH-004, CANVAS-032 |
| [ARCH-034](12-architecture/ARCH-034-architecture-a11y.md) | Make the architecture canvas keyboard and screen-reader accessible | P1 | ARCH-006, ARCH-008 |

## Data model (`13-data-model`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [DATA-001](13-data-model/DATA-001-entity-model.md) | Define the entity, field and relationship model | P0 | CORE-003 |
| [DATA-002](13-data-model/DATA-002-logical-types.md) | Define logical types and per-dialect type mapping | P0 | DATA-001 |
| [DATA-003](13-data-model/DATA-003-dialect-selector.md) | Switch the target SQL dialect per project | P0 | DATA-002 |
| [DATA-004](13-data-model/DATA-004-er-canvas.md) | Render entities as ER cards on the canvas | P0 | DATA-001, CANVAS-003 |
| [DATA-005](13-data-model/DATA-005-relationships-cardinality.md) | Draw relationships with cardinality markers | P0 | DATA-004 |
| [DATA-006](13-data-model/DATA-006-er-routing-layout.md) | Lay out and route ER diagrams | P1 | DATA-005, CANVAS-041 |
| [DATA-007](13-data-model/DATA-007-enums.md) | Model enums and their values | P0 | DATA-001 |
| [DATA-008](13-data-model/DATA-008-classifications.md) | Classify fields and tables (PII, Confidential) | P0 | DATA-001 |
| [DATA-010](13-data-model/DATA-010-entity-properties.md) | Edit entity settings in the properties strip | P0 | DATA-004, DS-011 |
| [DATA-011](13-data-model/DATA-011-indexes-constraints.md) | Model indexes, unique and check constraints | P1 | DATA-001 |
| [DATA-012](13-data-model/DATA-012-ddl-generation.md) | Generate full schema DDL per dialect | P0 | DATA-002, DATA-007, DATA-011 |
| [DATA-013](13-data-model/DATA-013-parse-check.md) | Parse-check generated SQL per dialect | P0 | DATA-012 |
| [DATA-014](13-data-model/DATA-014-migration-generation.md) | Generate up and down migrations from model changes | P0 | DATA-012 |
| [DATA-015](13-data-model/DATA-015-migration-safety-analysis.md) | Analyze migrations for reversibility and table rewrites | P1 | DATA-014 |
| [DATA-016](13-data-model/DATA-016-migration-frameworks.md) | Export migrations for common frameworks | P2 | DATA-014 |
| [DATA-017](13-data-model/DATA-017-reverse-engineer-db.md) | Reverse-engineer a model from a live database | P1 | DATA-001, DATA-002 |
| [DATA-018](13-data-model/DATA-018-import-ddl-file.md) | Import a model from an existing DDL file | P1 | DATA-013 |
| [DATA-019](13-data-model/DATA-019-seed-data.md) | Generate and edit seed data | P1 | DATA-012 |
| [DATA-020](13-data-model/DATA-020-validation-rules.md) | Define field validation rules shared with App | P0 | DATA-001 |
| [DATA-021](13-data-model/DATA-021-app-field-binding.md) | Bind App form fields to data model fields | P0 | DATA-020, APP-005 |
| [DATA-022](13-data-model/DATA-022-ddl-drawer.md) | Show schema and migration SQL in the drawer | P0 | DATA-012, DATA-014 |
| [DATA-023](13-data-model/DATA-023-copilot-data-tools.md) | Expose data model operations as Copilot tools | P0 | DATA-001, DATA-014, COPILOT-020 |
| [DATA-024](13-data-model/DATA-024-data-export.md) | Export ER diagrams and data dictionaries | P2 | DATA-004, CANVAS-032 |
| [DATA-025](13-data-model/DATA-025-dialect-container-tests.md) | Test DDL and migrations against real databases in CI | P1 | DATA-013, DATA-014 |

## System and deploy (`14-system`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [SYS-001](14-system/SYS-001-system-graph.md) | Build the linked Data → App → Architecture graph | P0 | CORE-022, DATA-001, APP-001, ARCH-001 |
| [SYS-002](14-system/SYS-002-binding-types.md) | Define binding types between layers | P0 | SYS-001 |
| [SYS-003](14-system/SYS-003-system-view.md) | Build the System view with linked cards | P0 | SYS-001 |
| [SYS-004](14-system/SYS-004-bindings-table.md) | Show the bindings table with status and fixes | P0 | SYS-002 |
| [SYS-005](14-system/SYS-005-missing-setting-detection.md) | Detect app settings the architecture doesn't define | P0 | SYS-002, ARCH-010 |
| [SYS-006](14-system/SYS-006-impact-tracing.md) | Trace a change's impact across layers | P1 | SYS-001 |
| [SYS-007](14-system/SYS-007-environments.md) | Define environments dev, staging and prod | P0 | SYS-001 |
| [SYS-008](14-system/SYS-008-deploy-plan.md) | Compute a deploy plan per environment | P0 | SYS-007 |
| [SYS-009](14-system/SYS-009-actions-workflow-gen.md) | Generate a GitHub Actions deploy workflow with OIDC | P0 | SYS-008 |
| [SYS-010](14-system/SYS-010-oidc-setup-guide.md) | Guide setting up federated credentials for OIDC | P1 | SYS-009 |
| [SYS-011](14-system/SYS-011-environment-approvals.md) | Require approvals for protected environments | P1 | SYS-009 |
| [SYS-012](14-system/SYS-012-migration-step.md) | Run migrations as a deploy step | P0 | SYS-009, DATA-014 |
| [SYS-013](14-system/SYS-013-slot-deploy-swap.md) | Deploy to a staging slot and swap | P1 | SYS-009, ARCH-013 |
| [SYS-014](14-system/SYS-014-smoke-tests.md) | Generate smoke tests from prototype flows | P1 | SYS-013, APP-015 |
| [SYS-015](14-system/SYS-015-deploy-pr.md) | Open the deploy PR from the System view | P0 | SYS-009, GH-010 |
| [SYS-016](14-system/SYS-016-deploy-status.md) | Show deploy run status back in CADE | P1 | SYS-015, GH-020 |
| [SYS-017](14-system/SYS-017-drift-detection.md) | Detect drift between design and deployed state | P2 | SYS-008 |

## GitHub (`15-github`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [GH-001](15-github/GH-001-device-flow-auth.md) | Sign in with GitHub using the OAuth device flow | P0 | — |
| [GH-002](15-github/GH-002-token-storage.md) | Store GitHub tokens in the OS keychain | P0 | GH-001, FOUND-010 |
| [GH-003](15-github/GH-003-enterprise-endpoints.md) | Support GitHub.com, GHE.com and GitHub Enterprise Server | P0 | GH-001 |
| [GH-004](15-github/GH-004-identity-lookup.md) | Look up the user's identity, orgs and Copilot seat | P0 | GH-001 |
| [GH-005](15-github/GH-005-git-engine.md) | Wrap git operations with gix | P0 | — |
| [GH-007](15-github/GH-007-branches.md) | Create and switch branches from CADE | P1 | GH-005 |
| [GH-008](15-github/GH-008-repo-picker.md) | Choose the target repository and branch for Ship | P0 | GH-004 |
| [GH-009](15-github/GH-009-preflight-checks.md) | Run pre-flight checks before anything leaves the machine | P0 | GH-008 |
| [GH-010](15-github/GH-010-pr-creation.md) | Create the pull request with a Copilot-written description | P0 | GH-009, COPILOT-003 |
| [GH-011](15-github/GH-011-codeowners-reviewers.md) | Request reviewers from CODEOWNERS | P1 | GH-010 |
| [GH-012](15-github/GH-012-code-scanning-status.md) | Show GHAS code scanning status for the PR | P1 | GH-010 |
| [GH-013](15-github/GH-013-pages-review-publish.md) | Publish review links to private GitHub Pages | P0 | GH-010, HIST-010 |
| [GH-014](15-github/GH-014-review-page-app.md) | Build the review web page | P0 | GH-013 |
| [GH-015](15-github/GH-015-comment-sync.md) | Sync review comments with the PR both ways | P0 | GH-014 |
| [GH-016](15-github/GH-016-comments-in-cade.md) | Show PR review comments as pins in CADE | P1 | GH-015, CANVAS-033 |
| [GH-017](15-github/GH-017-issues-api.md) | Create GitHub issues with the user's identity | P0 | GH-001 |
| [GH-018](15-github/GH-018-rate-limits-retries.md) | Handle GitHub rate limits and transient errors | P0 | GH-001 |
| [GH-019](15-github/GH-019-offline-queue.md) | Queue GitHub writes while offline | P1 | GH-018 |
| [GH-020](15-github/GH-020-webhooks-polling.md) | Keep PR state fresh without webhooks | P1 | GH-018 |
| [GH-021](15-github/GH-021-repo-templates.md) | Initialize new project repos with CADE files | P1 | GH-008 |
| [GH-023](15-github/GH-023-pr-preview-artifacts.md) | Fallback review via PR artifacts when Pages isn't available | P2 | GH-013 |

## Policy and admin (`16-policy`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [POLICY-001](16-policy/POLICY-001-policy-schema-v1.md) | Define the policy.yml v1 JSON Schema | P0 | FOUND-001 |
| [POLICY-002](16-policy/POLICY-002-policy-parser-validator.md) | Parse and validate policy files with clear errors | P0 | POLICY-001 |
| [POLICY-003](16-policy/POLICY-003-policy-fetch-on-signin.md) | Fetch org policy from the .github repo on sign-in | P0 | POLICY-002, GH-001, FOUND-027 |
| [POLICY-004](16-policy/POLICY-004-policy-cache-pinning.md) | Cache policy and pin it to a verified commit | P0 | POLICY-003 |
| [POLICY-005](16-policy/POLICY-005-policy-refresh-change-notice.md) | Refresh policy and notify users of changes | P1 | POLICY-004 |
| [POLICY-006](16-policy/POLICY-006-policy-precedence.md) | Merge policies from multiple orgs, strictest wins | P1 | POLICY-003 |
| [POLICY-007](16-policy/POLICY-007-enforce-models-byok.md) | Enforce model allowlists and the BYOK toggle | P0 | POLICY-002, COPILOT-002 |
| [POLICY-008](16-policy/POLICY-008-enforce-agent-tools.md) | Enforce agent tool rules: files, shell, network | P0 | POLICY-002, COPILOT-029, CORE-011 |
| [POLICY-009](16-policy/POLICY-009-enforce-mcp-allowlist.md) | Enforce the MCP server allowlist | P1 | POLICY-002, COPILOT-041 |
| [POLICY-010](16-policy/POLICY-010-enforce-design-systems.md) | Enforce approved design systems in outputs | P1 | POLICY-002, CORE-008 |
| [POLICY-011](16-policy/POLICY-011-enforce-ship-via-pr.md) | Require ship and deploy to go through pull requests | P0 | POLICY-002, GH-010 |
| [POLICY-012](16-policy/POLICY-012-enforce-iac-lint.md) | Require IaC lint to pass before an infrastructure PR | P1 | POLICY-002, ARCH-020, ARCH-022 |
| [POLICY-013](16-policy/POLICY-013-enforce-pii-classification.md) | Require PII classification in data models | P1 | POLICY-002, DATA-008 |
| [POLICY-014](16-policy/POLICY-014-enforce-visio-import-rules.md) | Strip Visio macros and enforce import rules | P1 | POLICY-002, DIAGRAM-018 |
| [POLICY-015](16-policy/POLICY-015-enforce-telemetry.md) | Enforce telemetry and reporting controls | P0 | POLICY-002 |
| [POLICY-016](16-policy/POLICY-016-honor-copilot-org-policies.md) | Honor Copilot org policies exposed by the SDK | P1 | COPILOT-003, POLICY-006 |
| [POLICY-017](16-policy/POLICY-017-policy-viewer-settings.md) | Show the effective policy and lock badges in Settings | P0 | POLICY-004 |
| [POLICY-018](16-policy/POLICY-018-admin-console-shell.md) | Build the admin console shell and overview | P1 | POLICY-017, GH-001 |
| [POLICY-019](16-policy/POLICY-019-admin-policy-editor.md) | Edit policy rules in the admin console | P1 | POLICY-018, POLICY-001 |
| [POLICY-020](16-policy/POLICY-020-policy-pr-flow.md) | Publish policy changes as a pull request | P1 | POLICY-019, GH-010 |
| [POLICY-021](16-policy/POLICY-021-token-budgets.md) | Enforce token budgets per user and cost center | P1 | POLICY-001, COPILOT-031 |
| [POLICY-022](16-policy/POLICY-022-install-policy-reporting.md) | Report installs and policy versions without content | P2 | POLICY-015 |
| [POLICY-023](16-policy/POLICY-023-audit-log.md) | Record a local audit log of policy-relevant events | P1 | POLICY-008 |
| [POLICY-024](16-policy/POLICY-024-policy-exception-request.md) | Let users request a policy exception | P2 | POLICY-020, COPILOT-033 |
| [POLICY-025](16-policy/POLICY-025-policy-conformance-tests.md) | Add conformance tests for every policy rule | P1 | POLICY-007, POLICY-008, POLICY-015 |

## Import (`17-import`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [IMPORT-001](17-import/IMPORT-001-import-pipeline.md) | Build the shared import pipeline | P0 | CORE-011, GH-001 |
| [IMPORT-002](17-import/IMPORT-002-fidelity-report-model.md) | Define the import fidelity report | P0 | IMPORT-001 |
| [IMPORT-003](17-import/IMPORT-003-import-screen.md) | Build the Import screen | P0 | IMPORT-001, IMPORT-002, DS-010 |
| [IMPORT-004](17-import/IMPORT-004-spark-discovery.md) | Discover exported Spark repos with the GitHub API | P0 | IMPORT-001, GH-001 |
| [IMPORT-005](17-import/IMPORT-005-spark-clone.md) | Clone Spark repos read-only for import | P0 | IMPORT-004 |
| [IMPORT-006](17-import/IMPORT-006-react-to-frames.md) | Convert React/TypeScript screens into App frames | P0 | IMPORT-005, APP-005 |
| [IMPORT-007](17-import/IMPORT-007-layout-capture.md) | Capture computed layout by running the app in the preview sandbox | P0 | IMPORT-006, APP-009 |
| [IMPORT-008](17-import/IMPORT-008-styles-to-tokens.md) | Map Tailwind and inline styles to design-system tokens | P0 | IMPORT-007, CORE-007 |
| [IMPORT-009](17-import/IMPORT-009-usekv-to-data-model.md) | Convert useKV data into Data model entities and seed data | P0 | IMPORT-006, DATA-001 |
| [IMPORT-010](17-import/IMPORT-010-llm-to-copilot-sdk.md) | Rewire Spark llm() calls to the Copilot SDK | P0 | IMPORT-006, COPILOT-001 |
| [IMPORT-011](17-import/IMPORT-011-spark-auth-and-deploy.md) | Keep GitHub auth and report Spark-hosted deploys | P1 | IMPORT-006 |
| [IMPORT-012](17-import/IMPORT-012-figma-token.md) | Manage the user's Figma access token | P0 | IMPORT-001, FOUND-010 |
| [IMPORT-013](17-import/IMPORT-013-figma-fetch.md) | Fetch Figma files, nodes and images via the REST API | P0 | IMPORT-012 |
| [IMPORT-014](17-import/IMPORT-014-figma-frames-layers.md) | Convert Figma frames, groups and vectors to native layers | P0 | IMPORT-013, CANVAS-003 |
| [IMPORT-015](17-import/IMPORT-015-figma-components-variants.md) | Convert Figma components, variants and instances | P0 | IMPORT-014, CORE-009 |
| [IMPORT-016](17-import/IMPORT-016-figma-variables-to-tokens.md) | Convert Figma variables and modes to tokens | P0 | IMPORT-013, CORE-007 |
| [IMPORT-017](17-import/IMPORT-017-figma-auto-layout.md) | Convert Figma auto layout to flex and grid layout | P0 | IMPORT-014, CANVAS-021 |
| [IMPORT-018](17-import/IMPORT-018-figma-prototype-links.md) | Convert Figma prototype connections to prototype flows | P1 | IMPORT-014, APP-016 |
| [IMPORT-019](17-import/IMPORT-019-figma-fonts.md) | Map Figma fonts to available fonts | P1 | IMPORT-014 |
| [IMPORT-020](17-import/IMPORT-020-figma-unsupported-report.md) | Report Figma features that cannot be imported | P0 | IMPORT-014, IMPORT-002 |
| [IMPORT-021](17-import/IMPORT-021-figma-team-library.md) | Import a Figma team library as a local design system | P1 | IMPORT-015, IMPORT-016 |
| [IMPORT-022](17-import/IMPORT-022-powerpoint-import-flow.md) | Import PowerPoint decks and templates through the pipeline | P0 | IMPORT-001, DECK-021 |
| [IMPORT-023](17-import/IMPORT-023-visio-import-routing.md) | Route Visio files through the pipeline into Diagram | P0 | IMPORT-001, DIAGRAM-014, DIAGRAM-017 |
| [IMPORT-024](17-import/IMPORT-024-drag-drop-and-open-with.md) | Import by drag-and-drop and OS 'Open with' | P1 | IMPORT-003 |
| [IMPORT-025](17-import/IMPORT-025-import-corpus-and-fuzzing.md) | Build importer test corpora and fuzz targets | P1 | IMPORT-014, DIAGRAM-014, DECK-021 |

## History and review (`18-history`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [HIST-001](18-history/HIST-001-structured-commits.md) | Commit every save, run, ship and import with structured trailers | P0 | CORE-019, COPILOT-024 |
| [HIST-002](18-history/HIST-002-autosave-coalescing.md) | Autosave and coalesce manual edits into versions | P0 | HIST-001, CORE-017 |
| [HIST-003](18-history/HIST-003-history-timeline-ui.md) | Build the version history timeline with filters | P0 | HIST-001 |
| [HIST-004](18-history/HIST-004-load-historical-snapshot.md) | Load any version as a read-only document | P0 | HIST-001, CORE-015 |
| [HIST-005](18-history/HIST-005-visual-compare.md) | Compare a past version with the current one side by side | P0 | HIST-004, CANVAS-003 |
| [HIST-006](18-history/HIST-006-structured-diff.md) | List what changed between two versions | P1 | HIST-004, CORE-014 |
| [HIST-007](18-history/HIST-007-restore-as-new-version.md) | Restore a past version as a new version | P0 | HIST-004 |
| [HIST-008](18-history/HIST-008-branch-from-version.md) | Branch from any version and switch branches | P1 | HIST-004, GH-007 |
| [HIST-009](18-history/HIST-009-run-detail-in-history.md) | Show Copilot run details in history | P1 | HIST-001, COPILOT-012 |
| [HIST-010](18-history/HIST-010-review-bundle-builder.md) | Build a static review bundle for a version | P1 | HIST-004, CANVAS-032 |
| [HIST-013](18-history/HIST-013-review-approve-request-changes.md) | Approve or request changes from the review page | P1 | GH-015 |
| [HIST-014](18-history/HIST-014-review-ask-copilot-fix.md) | Send a review comment to Copilot as a fix request | P2 | GH-015, COPILOT-015 |
| [HIST-015](18-history/HIST-015-history-performance.md) | Keep history fast for large and old projects | P2 | HIST-003, HIST-004 |

## Onboarding and settings (`19-onboarding-settings`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [ONB-001](19-onboarding-settings/ONB-001-first-run-wizard-shell.md) | Build the first-run wizard shell | P0 | DS-010, FOUND-003 |
| [ONB-002](19-onboarding-settings/ONB-002-signin-device-flow-ui.md) | Sign in with GitHub using the device flow | P0 | ONB-001, GH-001 |
| [ONB-003](19-onboarding-settings/ONB-003-copilot-runtime-check.md) | Check that the Copilot runtime is ready | P0 | COPILOT-004 |
| [ONB-004](19-onboarding-settings/ONB-004-seat-detection.md) | Detect the user's Copilot seat and handle no seat | P0 | COPILOT-003 |
| [ONB-005](19-onboarding-settings/ONB-005-policy-load-step.md) | Load org policy during onboarding with a summary | P0 | POLICY-003, POLICY-017 |
| [ONB-006](19-onboarding-settings/ONB-006-projects-folder.md) | Choose the projects folder | P0 | ONB-001 |
| [ONB-007](19-onboarding-settings/ONB-007-byok-onboarding-path.md) | Offer 'Use my own API key instead' during onboarding | P1 | COPILOT-005, COPILOT-009, POLICY-007 |
| [ONB-008](19-onboarding-settings/ONB-008-settings-shell.md) | Build the Settings shell with navigation and search | P0 | DS-010 |
| [ONB-009](19-onboarding-settings/ONB-009-settings-general.md) | Implement Settings › General | P1 | ONB-008 |
| [ONB-010](19-onboarding-settings/ONB-010-settings-copilot-models.md) | Implement Settings › Copilot & models | P0 | ONB-008, COPILOT-011, COPILOT-031 |
| [ONB-011](19-onboarding-settings/ONB-011-settings-agent-permissions.md) | Implement Settings › Agent permissions | P0 | ONB-008, COPILOT-028 |
| [ONB-012](19-onboarding-settings/ONB-012-settings-design-systems.md) | Implement Settings › Design systems | P1 | ONB-008, DS-001, POLICY-010 |
| [ONB-013](19-onboarding-settings/ONB-013-settings-appearance.md) | Implement Appearance with dark, light and system themes | P0 | ONB-008, DS-003 |
| [ONB-014](19-onboarding-settings/ONB-014-settings-shortcuts.md) | Implement the keyboard shortcuts viewer and editor | P1 | ONB-008 |
| [ONB-015](19-onboarding-settings/ONB-015-settings-azure-subscriptions.md) | Connect Azure subscriptions without storing credentials | P1 | ONB-008, ARCH-023 |
| [ONB-016](19-onboarding-settings/ONB-016-settings-privacy-about.md) | Implement Settings › Privacy and About | P1 | ONB-008, POLICY-015, POLICY-023 |
| [ONB-017](19-onboarding-settings/ONB-017-accounts-hosts-signout.md) | Support GHE.com and GHES hosts, sign out and switching | P1 | GH-003, ONB-002 |
| [ONB-018](19-onboarding-settings/ONB-018-settings-persistence.md) | Persist settings with policy overrides applied | P0 | ONB-008, POLICY-004 |
| [ONB-019](19-onboarding-settings/ONB-019-onboarding-a11y-e2e.md) | Test onboarding and settings end to end, with accessibility | P1 | ONB-002, ONB-013 |

## Feedback (`20-feedback`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [FEED-001](20-feedback/FEED-001-issue-form-bug.md) | Add the bug report issue form | P0 | — |
| [FEED-002](20-feedback/FEED-002-issue-form-feature.md) | Add the feature request issue form | P0 | — |
| [FEED-003](20-feedback/FEED-003-issue-form-comment.md) | Add the general comment issue form | P0 | — |
| [FEED-004](20-feedback/FEED-004-issue-template-config.md) | Configure issue template chooser and contact links | P0 | FEED-001, FEED-002, FEED-003 |
| [FEED-005](20-feedback/FEED-005-labels-as-code.md) | Define labels as code and sync them | P0 | — |
| [FEED-006](20-feedback/FEED-006-native-issue-types.md) | Adopt native GitHub issue types if the repo moves to an org | P2 | FEED-005 |
| [FEED-007](20-feedback/FEED-007-cade-feedback-crate.md) | Create cade-feedback crate to submit issues as the user | P0 | GH-017 |
| [FEED-008](20-feedback/FEED-008-token-scope-upgrade.md) | Handle token scope for issue creation and prompt to upgrade | P0 | FEED-007, GH-001 |
| [FEED-009](20-feedback/FEED-009-feedback-dialog-ui.md) | Build the in-app Send feedback dialog | P0 | DS-015, FEED-007 |
| [FEED-010](20-feedback/FEED-010-payload-preview.md) | Preview exactly what will be posted | P0 | FEED-009 |
| [FEED-011](20-feedback/FEED-011-environment-info.md) | Attach environment info with an opt-out | P0 | FEED-007 |
| [FEED-012](20-feedback/FEED-012-redaction.md) | Redact design content, prompts and paths by default | P0 | FEED-010 |
| [FEED-013](20-feedback/FEED-013-screenshot-attachment.md) | Spike: attach screenshots to in-app feedback | P1 | FEED-009 |
| [FEED-014](20-feedback/FEED-014-duplicate-search.md) | Suggest similar existing issues before submitting | P1 | FEED-007 |
| [FEED-015](20-feedback/FEED-015-offline-queue.md) | Queue feedback when offline and retry | P1 | FEED-007 |
| [FEED-016](20-feedback/FEED-016-submitted-feedback-list.md) | Confirm submission and list my feedback | P1 | FEED-007 |
| [FEED-017](20-feedback/FEED-017-rate-limiting.md) | Rate-limit feedback submission client-side | P1 | FEED-007 |
| [FEED-018](20-feedback/FEED-018-report-from-failure-states.md) | Offer 'Report this' from failure states and crashes | P1 | FEED-009, COPILOT-035, FOUND-015 |

## Quality (`21-quality`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [QA-001](21-quality/QA-001-write-test-strategy-coverage-targets.md) | Write the test strategy and coverage targets | P0 | FOUND-001 |
| [QA-002](21-quality/QA-002-add-property-based-tests-document-model.md) | Add property-based tests for the document model | P0 | QA-001, CORE-011 |
| [QA-003](21-quality/QA-003-fuzz-crdt-convergence-across-forks-merges.md) | Fuzz CRDT convergence across forks and merges | P0 | QA-002, CORE-013 |
| [QA-004](21-quality/QA-004-maintain-golden-fixture-corpus-projects.md) | Maintain a golden fixture corpus of projects | P0 | CORE-015 |
| [QA-005](21-quality/QA-005-test-ipc-commands-end-end-without-gui.md) | Test IPC commands end to end without a GUI | P0 | FOUND-004 |
| [QA-006](21-quality/QA-006-test-ui-components-workspace-flows-mocked-ipc.md) | Test UI components and workspace flows with mocked IPC | P0 | FOUND-002, FOUND-004 |
| [QA-007](21-quality/QA-007-run-end-end-tests-desktop-app.md) | Run end-to-end tests of the desktop app | P1 | QA-005, QA-006 |
| [QA-008](21-quality/QA-008-add-golden-image-tests-canvas-renderer.md) | Add golden-image tests for the canvas renderer | P0 | FOUND-006, CANVAS-003 |
| [QA-009](21-quality/QA-009-generate-large-documents-performance-testing.md) | Generate large documents for performance testing | P0 | CORE-003 |
| [QA-010](21-quality/QA-010-enforce-performance-budgets-10k-layer-files.md) | Enforce performance budgets on 10k-layer files | P0 | QA-009, QA-008 |
| [QA-011](21-quality/QA-011-run-long-session-soak-tests-leaks.md) | Run long-session soak tests for leaks | P1 | QA-007 |
| [QA-012](21-quality/QA-012-make-all-app-chrome-operable-by-keyboard.md) | Make all app chrome operable by keyboard only | P0 | QA-006 |
| [QA-013](21-quality/QA-013-test-voiceover-nvda-orca.md) | Test with VoiceOver, NVDA and Orca | P1 | QA-012 |
| [QA-014](21-quality/QA-014-expose-canvas-assistive-technology.md) | Expose the canvas to assistive technology | P1 | QA-012, CORE-024 |
| [QA-015](21-quality/QA-015-run-automated-accessibility-checks-every-screen.md) | Run automated accessibility checks on every screen | P1 | QA-007 |
| [QA-016](21-quality/QA-016-write-maintain-threat-model.md) | Write and maintain the threat model | P0 | FOUND-003 |
| [QA-017](21-quality/QA-017-sandbox-generated-app-previews.md) | Sandbox generated app previews | P0 | QA-016, APP-009 |
| [QA-018](21-quality/QA-018-test-that-agent-permissions-can-t-be.md) | Test that agent permissions can't be bypassed | P0 | QA-016, CORE-011, POLICY-008, COPILOT-029 |
| [QA-019](21-quality/QA-019-fuzz-every-importer-decoder.md) | Fuzz every importer and decoder | P0 | QA-016, IMPORT-025 |
| [QA-020](21-quality/QA-020-harden-archive-xml-parsing.md) | Harden archive and XML parsing | P0 | QA-019 |
| [QA-022](21-quality/QA-022-verify-secrets-never-leak-into-logs-exports.md) | Verify secrets never leak into logs, exports or reports | P0 | FOUND-010, FOUND-014 |
| [QA-023](21-quality/QA-023-complete-security-review-before-1-0.md) | Complete a security review before 1.0 | P1 | QA-016, QA-017, QA-018, QA-020 |
| [QA-024](21-quality/QA-024-maintain-per-os-release-test-plan.md) | Maintain a per-OS release test plan | P0 | QA-001 |
| [QA-025](21-quality/QA-025-set-up-bug-triage-severity-rules.md) | Set up bug triage and severity rules | P0 | QA-001, FEED-005 |
| [QA-026](21-quality/QA-026-run-beta-program-before-1-0.md) | Run a beta program before 1.0 | P1 | QA-024, FOUND-026 |

## Release and OSS (`22-release-oss`)

| Id | Title | Priority | Depends on |
|---|---|---|---|
| [REL-001](22-release-oss/REL-001-license-project-under-apache-2-0-notice.md) | License the project under Apache-2.0 with a NOTICE file | P0 | — |
| [REL-002](22-release-oss/REL-002-generate-third-party-license-notices.md) | Generate third-party license notices | P0 | REL-001, FOUND-022 |
| [REL-003](22-release-oss/REL-003-require-dco-sign-off-contributions.md) | Require DCO sign-off on contributions | P0 | REL-001 |
| [REL-004](22-release-oss/REL-004-write-readme.md) | Write the README | P0 | — |
| [REL-005](22-release-oss/REL-005-write-contributing-md.md) | Write CONTRIBUTING.md | P0 | REL-003 |
| [REL-006](22-release-oss/REL-006-adopt-code-conduct.md) | Adopt a code of conduct | P0 | — |
| [REL-007](22-release-oss/REL-007-publish-security-policy-private-reporting.md) | Publish a security policy and private reporting | P0 | — |
| [REL-008](22-release-oss/REL-008-document-governance-rfc-process.md) | Document governance and the RFC process | P1 | REL-005 |
| [REL-009](22-release-oss/REL-009-clear-product-name-copilot-trademark-usage.md) | Clear the product name and Copilot trademark usage | P0 | — |
| [REL-010](22-release-oss/REL-010-add-non-affiliation-privacy-statement.md) | Add a non-affiliation and privacy statement | P0 | REL-009 |
| [REL-011](22-release-oss/REL-011-create-original-brand-assets.md) | Create original brand assets | P1 | REL-009 |
| [REL-012](22-release-oss/REL-012-version-design-exports-repo.md) | Version design exports in the repo | P0 | — |
| [REL-013](22-release-oss/REL-013-build-documentation-site.md) | Build the documentation site | P1 | REL-004 |
| [REL-014](22-release-oss/REL-014-generate-reference-docs-format-policy-cli.md) | Generate reference docs for format, policy and CLI | P1 | REL-013, CORE-015, CORE-028 |
| [REL-015](22-release-oss/REL-015-publish-package-managers-each-os.md) | Publish to package managers on each OS | P1 | FOUND-028 |
| [REL-016](22-release-oss/REL-016-version-releases-generate-changelogs.md) | Version releases and generate changelogs | P0 | REL-005 |
| [REL-017](22-release-oss/REL-017-define-supported-platforms-release-cadence.md) | Define supported platforms and release cadence | P0 | — |
| [REL-018](22-release-oss/REL-018-sync-backlog-markdown-github-issues.md) | Sync backlog Markdown to GitHub issues | P1 | — |
| [REL-019](22-release-oss/REL-019-open-community-channels.md) | Open community channels | P1 | REL-006 |
| [REL-020](22-release-oss/REL-020-prepare-1-0-launch.md) | Prepare the 1.0 launch | P1 | REL-015, REL-013, REL-010, QA-023, QA-026 |
