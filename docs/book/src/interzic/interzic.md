# Interzic

Interzic is the translation layer between different music providers, services, or even apps (ex: Youtube, Spotify, listenbrainz, Tauon, etc...).
It is responsible for uploading playlists to different services.

Those commands allows to querry the inner database for data about the mappings, and configure them

## `get-mapping`

Retrieve the mapped id from a recording.

[Usage](../CommandLineHelp.md#alistral-interzic-get-mapping)

## `reload`

Fetch musicbrainz for recording urls and save them

[Usage](../CommandLineHelp.md#alistral-interzic-reload)

## `reverse-mapping`

Retrieve the recording data from an id.

[Usage](../CommandLineHelp.md#alistral-interzic-reverse-mapping)

## `overwrite`

Overwrite the mapping for an user. You can use this in case the auto mapping mapped to the wrong id, or you prefer another one

[Usage](../CommandLineHelp.md#alistral-interzic-overwrite)

