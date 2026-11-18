# Report sections
Here are all the available sections in the report

## `GeneralStats`
Generic stats for the period

## `GroupByWeekDay`
Group the listen durations per day of the week they were listened on

## `GroupByMonth`
Group the listen durations per month they were listened on


## `TopArtists`, `Toplabels`, `TopRecordings`, `TopReleases`, `TopReleaseGroups`
A simple list of the most listened entities

Settings:
```json
{
    "table_settings": <Table Settings>
}
```

## `DiscoveredArtists`, `Discoveredlabels`, `DiscoveredRecordings`, `DiscoveredReleases`, `DiscoveredReleaseGroups`
A list of the most listened entities that you discovered in this period

Settings:
```json
{
    "table_settings": <Table Settings>
}
```

## `AllTimeChangesArtists`, `AllTimeChangeslabels`, `AllTimeChangesRecordings`, `AllTimeChangesReleases`, `AllTimeChangesReleaseGroups`
Show the top listen of all time (until the end date) of the entity.

Settings:
```json
{
    "table_settings": <Table Settings>
}
```

## `DebutsRecordings`, `DebutsReleases`
A list of the most listened entities that debuted in this period

Settings:
```json
{
    "table_settings": <Table Settings>
}
```

## `ComebackArtists`, `Comebacklabels`, `ComebackRecordings`, `ComebackReleases`, `ComebackReleaseGroups`
A list of the most listened entities that weren't listened in the last period, but were listened before (a track listened in 2024 and not in 2025 will be displayed for a 2026 report)

Settings:
```json
{
    "table_settings": <Table Settings>
}
```

## Extra types

### Table Settings
Settings on how the table is displayed

```json5
{
    "row_count": 10
}
```