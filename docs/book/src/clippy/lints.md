# `label_as_artist` (Recording, Release, Release group)

Checks if the credited artist as a `label as artist` tag. If it does, it's likely that it was meant for promotional purposes, and should instead be removed and properly set as label

## Exemples

`CHOMPO` is a label as artist used to act as a label page on Spotify.

- `HEART` by `TOKYO MACHINE` & `CHOMPO` => `HEART` by `TOKYO MACHINE`, and set the release's label to CHOMPO
- `CHOMPO II` by `Chompo` is a compilation album => `CHOMPO II` by `Various Artists`


# `missing_artist_link` (Recording)

Checks if an artist is missing an external url based on data already present in the database.
For exemple, a spotify link implies a spotify artist page link can be added to the artist.

Available websites:
- Apple music
- Spotify
- Deezer
- Bandcamp
- Tidal
- Beatport


# `missing_isrc` (Recording)

Checks if a recording is missing an ISRC and has a release that can provide it using external links

## Why is it important ?

ISRCs are a great way to identify a track. Often, two recordings with the same ISRC can be merged. 


# `missing_recording_link` (Recording)

Checks if a recording is missing an external url based on data already present in the database.
For exemple, a spotify link implies a spotify track link can be added to the recording.

Available websites:
- Apple music
- Spotify
- Deezer
- Bandcamp
- Tidal
- Beatport
- Youtube

# `missing_release_barcode` (Recording)

Checks if a release got a missing barcode, nor is set to not having one

Also warn if the barcode can be found using the external URLs

## Why is it important ?

Barcode allows for easier release identification. Two releases with the same barcode may be duplicates, but two with different barcodes aren't.

