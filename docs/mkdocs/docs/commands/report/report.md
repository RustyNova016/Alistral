# Reports
Reports are a powerful dashboard to have a view of your statistics over a given period. This takes in a start date, and en end date:

```bash
# Yearly report for 2025
alistral report 2025-01-01 2026-01-01
```

## Speedup
Reports require a lot of data from musicbrainz, and alistral is limited to 1 request per second. All the data fetched is saved locally so it won't have to get fetched again, so the app can be closed and it will continue from where it stopped (The progressbar may not reflect this). 

If you don't care waiting between sections, you can add `--lazy-fetching` to only fetch what's needed for displaying the next section

## Configuration

Reports are highly customisable. Customisations are bundled in presets, and are selected in the cli. Add a preset in the command settings of your config.json:

```json
{
  "commands": {
    "reports": {
      "presets": {
        "quick_tops": {
          "sections": [
            {
              "type": "AllTimeChangesRecordings",
              "table_settings": {
                "row_count": 10
              }
            },
            {
              "type": "AllTimeChangesArtists",
              "table_settings": {
                "row_count": 10
              }
            },
            {
              "type": "AllTimeChangesReleaseGroups",
              "table_settings": {
                "row_count": 10
              }
            }
          ]
        }
      }
    }
  }
}
```

Then run with the selected preset

```bash
# Yearly report for 2025
alistral report 2025-01-01 2026-01-01 --preset quick_tops
```

You can find the list of sections [here](./sections.md). To add a section to your preset, add a json object with the type being the name of the section:
```json
{
    "type": "AllTimeChangesReleaseGroups",
}
```
