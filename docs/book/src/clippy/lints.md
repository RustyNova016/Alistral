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



