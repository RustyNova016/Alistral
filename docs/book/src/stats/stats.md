# The `stats` command

An extended statistic calculator that calculate fresh statistics in a blink
of an eye! (Data fetching not counted)

[Usage](../CommandLineHelp.md#alistral-stats)

## Supported entities:

- Artist
- Recording
- Release
- Release Group
- Tags
- Works

Genres will not be supported as they are included in the tags statistics

## Modifiers

### --w-recursive

Recursively add parent works to work stats. For exemple, a listen of the work ["1, 2 Oatmeal"](https://musicbrainz.org/work/319cbf84-5f94-4e2f-bd28-fba38bb4e0d6) will also count for ["Gourmet race"](https://musicbrainz.org/work/f92a43c1-b999-4bb6-b56f-86157f90b274)