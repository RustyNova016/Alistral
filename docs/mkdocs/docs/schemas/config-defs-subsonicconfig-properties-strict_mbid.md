# Untitled undefined type in Config Schema

```txt
path#/$defs/SubsonicConfig/properties/strict_mbid
```

If set to true, only subsonic tracks that match the MBID of the requested recording are mapped

Only set this if your subsonic server is capable of searching by MBID (Like navidrome),
and your collection has been passed through Musicbrainz Picard

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## strict\_mbid Type

`boolean`
