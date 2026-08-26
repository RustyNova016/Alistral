# Untitled object in Config Schema

```txt
path#/$defs/SubsonicConfig
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## SubsonicConfig Type

`object` ([Details](config-defs-subsonicconfig.md))

# SubsonicConfig Properties

| Property                     | Type      | Required | Nullable       | Defined by                                                                                                         |
| :--------------------------- | :-------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------- |
| [name](#name)                | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-name.md "path#/$defs/SubsonicConfig/properties/name")               |
| [password](#password)        | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-password.md "path#/$defs/SubsonicConfig/properties/password")       |
| [strict\_mbid](#strict_mbid) | `boolean` | Optional | can be null    | [Config](config-defs-subsonicconfig-properties-strict_mbid.md "path#/$defs/SubsonicConfig/properties/strict_mbid") |
| [url](#url)                  | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-url.md "path#/$defs/SubsonicConfig/properties/url")                 |
| [username](#username)        | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-username.md "path#/$defs/SubsonicConfig/properties/username")       |

## name

The name of the subsonic instance

`name`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-name.md "path#/$defs/SubsonicConfig/properties/name")

### name Type

`string`

## password

The password of the user

`password`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-password.md "path#/$defs/SubsonicConfig/properties/password")

### password Type

`string`

## strict\_mbid

If set to true, only subsonic tracks that match the MBID of the requested recording are mapped

Only set this if your subsonic server is capable of searching by MBID (Like navidrome),
and your collection has been passed through Musicbrainz Picard

`strict_mbid`

* is optional

* Type: `boolean`

* can be null

* defined in: [Config](config-defs-subsonicconfig-properties-strict_mbid.md "path#/$defs/SubsonicConfig/properties/strict_mbid")

### strict\_mbid Type

`boolean`

## url

The url of the subsonic instance

`url`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-url.md "path#/$defs/SubsonicConfig/properties/url")

### url Type

`string`

## username

The username of the user

`username`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-username.md "path#/$defs/SubsonicConfig/properties/username")

### username Type

`string`
