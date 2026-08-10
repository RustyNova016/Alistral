# Untitled object in Config Schema

```txt
path#/$defs/MusicbrainzServer
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## MusicbrainzServer Type

`object` ([Details](config-defs-musicbrainzserver.md))

# MusicbrainzServer Properties

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                           |
| :----------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------- |
| [authority](#authority)  | `string`  | Required | cannot be null | [Config](config-defs-musicbrainzserver-properties-authority.md "path#/$defs/MusicbrainzServer/properties/authority") |
| [ratelimit](#ratelimit)  | Merged    | Optional | cannot be null | [Config](config-defs-musicbrainzserver-properties-ratelimit.md "path#/$defs/MusicbrainzServer/properties/ratelimit") |
| [use\_https](#use_https) | `boolean` | Required | cannot be null | [Config](config-defs-musicbrainzserver-properties-use_https.md "path#/$defs/MusicbrainzServer/properties/use_https") |

## authority



`authority`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-musicbrainzserver-properties-authority.md "path#/$defs/MusicbrainzServer/properties/authority")

### authority Type

`string`

## ratelimit



`ratelimit`

* is optional

* Type: merged type ([Details](config-defs-musicbrainzserver-properties-ratelimit.md))

* cannot be null

* defined in: [Config](config-defs-musicbrainzserver-properties-ratelimit.md "path#/$defs/MusicbrainzServer/properties/ratelimit")

### ratelimit Type

merged type ([Details](config-defs-musicbrainzserver-properties-ratelimit.md))

any of

* [Untitled object in Config](config-defs-ratelimitconfig.md "check type definition")

* [Untitled null in Config](config-defs-musicbrainzserver-properties-ratelimit-anyof-1.md "check type definition")

## use\_https



`use_https`

* is required

* Type: `boolean`

* cannot be null

* defined in: [Config](config-defs-musicbrainzserver-properties-use_https.md "path#/$defs/MusicbrainzServer/properties/use_https")

### use\_https Type

`boolean`
