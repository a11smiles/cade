# Bundled Geist font provenance

Issue: #609. Recorded October 27, 2026.

**Status:** matching upstream license and font-source metadata verified;
cache-to-source byte equality and literal font digests verified by the
implementation handoff and independently reviewed in QA preflight on
October 27, 2026. Canonical public notices repair the source-side QA findings;
their inclusion in a rebuilt production artifact still requires build and
independent QA evidence. This record is not packaging or runtime acceptance.

## Font identity and cached source

These four assets are selected from the existing `next@15.5.21` pnpm cache,
package repository `vercel/next.js`. No Next dependency is being added.
The package version is not the font version.

The October 27, 2026 offline-font diagnosis reports that all four embedded
font versions are `1.401`, with Regular style and variable `wght` range
100–900 (default 400). Families are Geist and Geist Mono. Both Latin and
Latin Extended subsets are retained. The Latin files each map 223 code
points; the extended files map 164 (Geist) and 162 (Geist Mono).

All source paths below are relative to the cached package root. The four
destination files remain at `app/ui/src/assets/fonts/<basename>`, with names
identical to the source basenames. Only the notices live in
`app/ui/public/fonts/`; no font binaries are duplicated there.

| Source path | Bytes | SHA-256 |
| --- | ---: | --- |
| `dist/next-devtools/server/font/geist-latin.woff2` | 28356 | `1b5ebfb3a01a97343ac96873e6d59a8cb285c66012b6a1ac509cb2765e995ba8` |
| `dist/next-devtools/server/font/geist-latin-ext.woff2` | 15292 | `f7604a53a00250f66db4b47dd44327ce2a43f6edec850d5217bb80238819efdd` |
| `dist/next-devtools/server/font/geist-mono-latin.woff2` | 31288 | `b7ac144b394cbd81052d6397ec0c33397977b1d7e9bc095e744e652a378c6fb3` |
| `dist/next-devtools/server/font/geist-mono-latin-ext.woff2` | 13036 | `7d5609538a79545ec945e019771ed562e2c65a6c1c3c4ccc4ea34372e277c396` |

The source index is
`store/v10/index/fd/3b1d06d9162e1925f8d54bdd4ab0b3652984d77a20fcceb734a4d5f1da7fed-next@15.5.21.json`
under the pnpm data directory. Each source path identifies an entry in
`files`; its `integrity` field contains the SHA-512 SRI digest. Decoding
that digest to hexadecimal identifies the cache blob at
`store/v10/files/<first two hex characters>/<remaining hex characters>`.

The implementation handoff records fresh cache-integrity checks and exact
cache-to-destination byte equality for all four assets. Independent QA
preflight also verified cache, source and the then-existing emitted font
bytes. The literal destination SHA-512 digests below equal the decoded
pnpm index integrity values; they are verified handoff data, not a new
writer hash run.

| Destination basename | SHA-512 |
| --- | --- |
| `geist-latin.woff2` | `5a51527bb1bd390523b22a8c1af81690b756e3eda90a1fdaec695cef4df0b8a52356acbc0ee1e5b7d61b81e54bb52d5503daa249fdc1756ab35930cf8b1f1b22` |
| `geist-latin-ext.woff2` | `fc0a288d2c12226509bac4d5ab63dec2b16f2edc928b6fb54c4b3b44f8431561a2a639576bd0fb8d2678dca9a37490bb82cf12568873982238d792a2570dae3a` |
| `geist-mono-latin.woff2` | `c8b02f1655b30d6fd6e0b3ee92280d175fb0410938a71f625354fe79e009fb2928ed55b45d0000db9e527a388e8f3eb42875c6965c80eb4b03046c50f49ed187` |
| `geist-mono-latin-ext.woff2` | `4d7c70e88f8249bcef5ec0414321c45f2e21d69d86e6ea69743206af8ffaddce167bfdda877d99177589f8280bda58d5f7342c628df75b16c366ad6419d5849c` |

The approved integration preserves cached bytes unchanged: no font upgrade,
remote font download, conversion, subsetting, renaming, or font-table edits.
These cache subsets are not claimed byte-identical to upstream release
WOFF2 files.

## Official license and version evidence

Official repository: `https://github.com/vercel/geist-font`.
GitHub API retrieval on October 27, 2026 verified tag `1.4.01` resolves to
commit `b193ef74010119759bfb7f71ddf81a3dee238535`.
All upstream paths below are pinned to that commit:

| Path | Git blob SHA-1 | Evidence |
| --- | --- | --- |
| `OFL.txt` | `61f6f4b7853081236aec6407f2dd3c7170997f84` | Full SIL Open Font License 1.1; upstream size 4387 bytes |
| `sources/Geist.glyphspackage/fontinfo.plist` | `733b6d4b58edfff9a86d80277bc41db99ba15bb1` | `versionMajor = 1`, `versionMinor = 401`; matching copyright and OFL 1.1 |
| `sources/GeistMono.glyphspackage/fontinfo.plist` | `a5c41d194e456e27984589c945870a710d884d3f` | `versionMajor = 1`, `versionMinor = 401`; matching copyright and OFL 1.1 |

Pinned license API source:
`https://api.github.com/repos/vercel/geist-font/contents/OFL.txt?ref=b193ef74010119759bfb7f71ddf81a3dee238535`.

Both pinned source font metadata files and the full OFL notice identify
`Copyright 2024 The Geist Project Authors (https://github.com/vercel/geist-font.git)`.
The source metadata identifies SIL Open Font License 1.1 and
`https://openfontlicense.org`, matching the embedded authors, version, and
license URL reported by the cached-font diagnosis. This is the literal
copyright attribution, not a substituted or abbreviated author list.

The canonical `app/ui/public/fonts/OFL.txt` preserves that pinned copyright
and complete license text, including every condition and the disclaimer.
There are exactly two formatting differences from the 4387-byte upstream
blob: remove the single ASCII space after `embedded,` on line 21, and add
one terminal LF. LF line separators otherwise remain unchanged. Neither
change alters a license word, punctuation or attribution. These changes
balance to an expected local length of 4387 bytes; equal lengths do not
mean equal bytes or equal hashes. The upstream Git blob SHA-1 is not the
local file's SHA-256. A fresh local digest belongs in the remediation
verification receipt; none is inferred from the upstream identity.

The same upstream commit also contains `LICENSE.txt` with a different,
2023 Vercel/basement.studio notice. Its `packages/next/package.json` reports
package version `1.3.1`, not font version `1.401`. Neither is substituted
for the matching 2024 `OFL.txt` and source-font metadata above. The cached
Next package's MIT license likewise is not the license for these fonts.

## Canonical notices and production delivery

The canonical source notices are `app/ui/public/fonts/OFL.txt` and
`app/ui/public/fonts/PROVENANCE.md` (this file). Root `NOTICE` attributes
Geist and Geist Mono, their SIL OFL 1.1 license, official source and pinned
provenance. Actual #609 AC3 requires notices in both NOTICE and the shipped
bundle; source-only notices did not satisfy that criterion.

The assigned correction uses the existing Vite public-asset copy behavior:
the next build is expected to emit `app/ui/dist/fonts/OFL.txt` and
`app/ui/dist/fonts/PROVENANCE.md`, respectively `fonts/OFL.txt` and
`fonts/PROVENANCE.md` relative to the built frontend root. No workflow,
signing or Tauri configuration change is part of this correction.
The existing artifact inspected by QA contained neither notice. Moving
these source files does not establish that the production artifact now
contains them; rebuilding, artifact assertions and independent QA remain
required.

## Runtime boundary

The approved #609 integration bundles all four local WOFF2 files through
relative CSS asset URLs. Font loading must use bundled assets only, with
no Google Fonts stylesheet, external preconnect, external font request,
or installed-font `local()` fallback. Upstream URLs here are attribution
and source-retrieval references, not runtime dependencies.

That is the integration contract, not a claim that runtime checks passed.
HTML, CSS, font copying, and regression tests are owned by the separate
implementation handoff. Independent license/packaging and runtime QA remain
pending.

## Change history

- October 27, 2026, initial notice: source notices lived beside the font
  binaries. The local full OFL was 4388 bytes, SHA-256
  `6a873c900f584109b13ae0aaf81d6e3cf0a68751a216b03f7b6c68d547057bb4`;
  QA verified it equaled the pinned upstream bytes plus exactly one LF.
  This is the historical hash, not the normalized public notice's hash.
  The initial writer lacked literal font SHA-512 and destination equality
  evidence; the later implementation handoff supplied both, superseding
  that earlier evidence gap.
- October 27, 2026, QA remediation: architect assigned the canonical notice
  moves to `app/ui/public/fonts/`, root NOTICE attribution for actual AC3,
  and removal of the upstream line-21 trailing space while retaining the
  terminal LF. This supersedes the source-only placement and the earlier
  suggestion that no required NOTICE entry had been identified. QA
  findings F609-QA-1 and F609-QA-2 remain subject to built-artifact and
  indexed-whitespace rechecks; no acceptance is implied by these edits.
