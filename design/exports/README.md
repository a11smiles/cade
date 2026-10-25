# Design exports

Every time the UI design changes, the export goes into a **new, numbered folder**. Old exports are never edited or overwritten, so backlog items and pull requests can always point at the exact design they were built from.

```
design/exports/
  INDEX.md                      every version, newest first
  LATEST                        name of the newest folder
  v0001-2026-10-25/             canvas source snapshot (referenced by the backlog)
  v0002-YYYY-MM-DD/             your next export
    MANIFEST.json               files with sha256, source, note
    ...exported files...
```

## Add an export

1. Export from the design canvas (Share › Export) and unzip it anywhere.
2. Run:

   ```sh
   python3 scripts/design_export.py path/to/unzipped-export --note "Feedback dialog and Azure-only architecture"
   ```

   This creates the next `vNNNN-<today>` folder, copies the files, writes `MANIFEST.json`, updates `INDEX.md` and `LATEST`, and lists which screens were added, changed or removed since the previous version.

3. Commit the new folder. When a backlog item should follow the new design, update its `design:` path to the new version in the same pull request.
