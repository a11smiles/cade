# Azure architecture icons — drop them here

CADE draws Architecture projects with Microsoft's official **Azure Architecture Icons**. They aren't committed to this repo; each user downloads them.

1. Download the icon pack from Microsoft Learn: search "Azure architecture icons" on learn.microsoft.com, then use the download link on that page.
2. Unzip it.
3. Copy the `Icons` folder from the zip into this folder, so you have `assets/icons/azure/Icons/<category>/<file>.svg`, for example `assets/icons/azure/Icons/web/10035-icon-service-App-Services.svg`.

Keep the folder structure and file names exactly as Microsoft ships them. CADE finds icons by matching the service name at the end of each file name (see `icon-map.json`), so a newer pack version works without code changes.

Microsoft's terms allow using these icons in architecture diagrams. Don't modify, recolor or crop them. `.gitignore` keeps the files out of commits.

If an icon is missing, CADE draws a labeled tile for that resource and lists it in Settings › Design systems › Azure icons.
