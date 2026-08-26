# Config Schema

```txt
path
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                      |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------------ |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json](../../../../out/config.schema.json "open original schema") |

## Config Type

`object` ([Config](config.md))

# Config Properties

| Property                                     | Type     | Required | Nullable       | Defined by                                                                                |
| :------------------------------------------- | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------- |
| [artist\_listened\_to](#artist_listened_to)  | Multiple | Optional | can be null    | [Config](config-properties-artist_listened_to.md "path#/properties/artist_listened_to")   |
| [bumps](#bumps)                              | `array`  | Optional | cannot be null | [Config](config-defs-bumplist.md "path#/properties/bumps")                                |
| [commands](#commands)                        | Merged   | Optional | cannot be null | [Config](config-properties-commands.md "path#/properties/commands")                       |
| [default\_user](#default_user)               | `string` | Optional | can be null    | [Config](config-properties-default_user.md "path#/properties/default_user")               |
| [interzic](#interzic)                        | `object` | Optional | cannot be null | [Config](config-defs-interzicconfig.md "path#/properties/interzic")                       |
| [listenbrainz\_domain](#listenbrainz_domain) | `string` | Optional | cannot be null | [Config](config-properties-listenbrainz_domain.md "path#/properties/listenbrainz_domain") |
| [listens](#listens)                          | `object` | Optional | cannot be null | [Config](config-defs-listenconfig.md "path#/properties/listens")                          |
| [mapper](#mapper)                            | Merged   | Optional | cannot be null | [Config](config-properties-mapper.md "path#/properties/mapper")                           |
| [musicbrainz\_server](#musicbrainz_server)   | Merged   | Optional | cannot be null | [Config](config-properties-musicbrainz_server.md "path#/properties/musicbrainz_server")   |
| [musicbrainz\_url](#musicbrainz_url)         | `string` | Optional | can be null    | [Config](config-properties-musicbrainz_url.md "path#/properties/musicbrainz_url")         |
| [tokens](#tokens)                            | `object` | Required | cannot be null | [Config](config-properties-tokens.md "path#/properties/tokens")                           |

## artist\_listened\_to



`artist_listened_to`

* is optional

* Type: any of the following: `string` or `number` ([Details](config-properties-artist_listened_to.md))

* can be null

* defined in: [Config](config-properties-artist_listened_to.md "path#/properties/artist_listened_to")

### artist\_listened\_to Type

any of the following: `string` or `number` ([Details](config-properties-artist_listened_to.md))

### artist\_listened\_to Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^-?\d+(\.\d+)?([eE]\d+)?$
```

[try pattern](https://regexr.com/?expression=%5E-%3F%5Cd%2B\(%5C.%5Cd%2B\)%3F\(%5BeE%5D%5Cd%2B\)%3F%24 "try regular expression with regexr.com")

## bumps



`bumps`

* is optional

* Type: `object[]` ([Details](config-defs-bump.md))

* cannot be null

* defined in: [Config](config-defs-bumplist.md "path#/properties/bumps")

### bumps Type

`object[]` ([Details](config-defs-bump.md))

### bumps Default Value

The default value is:

```json
[]
```

## commands



`commands`

* is optional

* Type: merged type ([Details](config-properties-commands.md))

* cannot be null

* defined in: [Config](config-properties-commands.md "path#/properties/commands")

### commands Type

merged type ([Details](config-properties-commands.md))

any of

* [Untitled object in Config](config-defs-commandconfig.md "check type definition")

* [Untitled null in Config](config-properties-commands-anyof-1.md "check type definition")

## default\_user



`default_user`

* is optional

* Type: `string`

* can be null

* defined in: [Config](config-properties-default_user.md "path#/properties/default_user")

### default\_user Type

`string`

## interzic



`interzic`

* is optional

* Type: `object` ([Details](config-defs-interzicconfig.md))

* cannot be null

* defined in: [Config](config-defs-interzicconfig.md "path#/properties/interzic")

### interzic Type

`object` ([Details](config-defs-interzicconfig.md))

### interzic Default Value

The default value is:

```json
{
  "subsonic_clients": []
}
```

## listenbrainz\_domain



`listenbrainz_domain`

* is optional

* Type: `string`

* cannot be null

* defined in: [Config](config-properties-listenbrainz_domain.md "path#/properties/listenbrainz_domain")

### listenbrainz\_domain Type

`string`

### listenbrainz\_domain Default Value

The default value is:

```json
"api.listenbrainz.org"
```

## listens



`listens`

* is optional

* Type: `object` ([Details](config-defs-listenconfig.md))

* cannot be null

* defined in: [Config](config-defs-listenconfig.md "path#/properties/listens")

### listens Type

`object` ([Details](config-defs-listenconfig.md))

### listens Default Value

The default value is:

```json
{
  "refresh_unmapped_listens": true
}
```

## mapper



`mapper`

* is optional

* Type: merged type ([Details](config-properties-mapper.md))

* cannot be null

* defined in: [Config](config-properties-mapper.md "path#/properties/mapper")

### mapper Type

merged type ([Details](config-properties-mapper.md))

any of

* [Untitled object in Config](config-defs-mapperconfig.md "check type definition")

* [Untitled null in Config](config-properties-mapper-anyof-1.md "check type definition")

## musicbrainz\_server



`musicbrainz_server`

* is optional

* Type: merged type ([Details](config-properties-musicbrainz_server.md))

* cannot be null

* defined in: [Config](config-properties-musicbrainz_server.md "path#/properties/musicbrainz_server")

### musicbrainz\_server Type

merged type ([Details](config-properties-musicbrainz_server.md))

any of

* [Untitled object in Config](config-defs-musicbrainzserver.md "check type definition")

* [Untitled null in Config](config-properties-musicbrainz_server-anyof-1.md "check type definition")

## musicbrainz\_url



`musicbrainz_url`

* is optional

* Type: `string`

* can be null

* defined in: [Config](config-properties-musicbrainz_url.md "path#/properties/musicbrainz_url")

### musicbrainz\_url Type

`string`

## tokens

Saved usertokens

`tokens`

* is required

* Type: `object` ([Details](config-properties-tokens.md))

* cannot be null

* defined in: [Config](config-properties-tokens.md "path#/properties/tokens")

### tokens Type

`object` ([Details](config-properties-tokens.md))

# Config Definitions

## Definitions group Bump

Reference this group by using

```json
{"$ref":"path#/$defs/Bump"}
```

| Property                             | Type     | Required | Nullable       | Defined by                                                                                             |
| :----------------------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------- |
| [expiration\_date](#expiration_date) | `string` | Required | cannot be null | [Config](config-defs-bump-properties-expiration_date.md "path#/$defs/Bump/properties/expiration_date") |
| [recording](#recording)              | `string` | Required | cannot be null | [Config](config-defs-bump-properties-recording.md "path#/$defs/Bump/properties/recording")             |
| [username](#username)                | `string` | Required | cannot be null | [Config](config-defs-bump-properties-username.md "path#/$defs/Bump/properties/username")               |
| [value](#value)                      | Multiple | Required | cannot be null | [Config](config-defs-bump-properties-value.md "path#/$defs/Bump/properties/value")                     |

### expiration\_date



`expiration_date`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-bump-properties-expiration_date.md "path#/$defs/Bump/properties/expiration_date")

#### expiration\_date Type

`string`

#### expiration\_date Constraints

**date time**: the string must be a date time string, according to [RFC 3339, section 5.6](https://tools.ietf.org/html/rfc3339 "check the specification")

### recording



`recording`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-bump-properties-recording.md "path#/$defs/Bump/properties/recording")

#### recording Type

`string`

### username



`username`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-bump-properties-username.md "path#/$defs/Bump/properties/username")

#### username Type

`string`

### value



`value`

* is required

* Type: any of the following: `string` or `number` ([Details](config-defs-bump-properties-value.md))

* cannot be null

* defined in: [Config](config-defs-bump-properties-value.md "path#/$defs/Bump/properties/value")

#### value Type

any of the following: `string` or `number` ([Details](config-defs-bump-properties-value.md))

#### value Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^-?\d+(\.\d+)?([eE]\d+)?$
```

[try pattern](https://regexr.com/?expression=%5E-%3F%5Cd%2B\(%5C.%5Cd%2B\)%3F\(%5BeE%5D%5Cd%2B\)%3F%24 "try regular expression with regexr.com")

## Definitions group BumpList

Reference this group by using

```json
{"$ref":"path#/$defs/BumpList"}
```

| Property | Type | Required | Nullable | Defined by |
| :------- | :--- | :------- | :------- | :--------- |

## Definitions group CommandConfig

Reference this group by using

```json
{"$ref":"path#/$defs/CommandConfig"}
```

| Property        | Type   | Required | Nullable       | Defined by                                                                                           |
| :-------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------- |
| [daily](#daily) | Merged | Optional | cannot be null | [Config](config-defs-commandconfig-properties-daily.md "path#/$defs/CommandConfig/properties/daily") |

### daily



`daily`

* is optional

* Type: merged type ([Details](config-defs-commandconfig-properties-daily.md))

* cannot be null

* defined in: [Config](config-defs-commandconfig-properties-daily.md "path#/$defs/CommandConfig/properties/daily")

#### daily Type

merged type ([Details](config-defs-commandconfig-properties-daily.md))

any of

* [Untitled object in Config](config-defs-dailycommandconfig.md "check type definition")

* [Untitled null in Config](config-defs-commandconfig-properties-daily-anyof-1.md "check type definition")

## Definitions group DailyCommandConfig

Reference this group by using

```json
{"$ref":"path#/$defs/DailyCommandConfig"}
```

| Property                             | Type      | Required | Nullable    | Defined by                                                                                                                         |
| :----------------------------------- | :-------- | :------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| [minimum\_listens](#minimum_listens) | `integer` | Optional | can be null | [Config](config-defs-dailycommandconfig-properties-minimum_listens.md "path#/$defs/DailyCommandConfig/properties/minimum_listens") |

### minimum\_listens

The minimum listens needed to display a track's anniversary / first discovery

`minimum_listens`

* is optional

* Type: `integer`

* can be null

* defined in: [Config](config-defs-dailycommandconfig-properties-minimum_listens.md "path#/$defs/DailyCommandConfig/properties/minimum_listens")

#### minimum\_listens Type

`integer`

#### minimum\_listens Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint`

## Definitions group InterzicConfig

Reference this group by using

```json
{"$ref":"path#/$defs/InterzicConfig"}
```

| Property                               | Type    | Required | Nullable       | Defined by                                                                                                                   |
| :------------------------------------- | :------ | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [subsonic\_clients](#subsonic_clients) | `array` | Required | cannot be null | [Config](config-defs-interzicconfig-properties-subsonic_clients.md "path#/$defs/InterzicConfig/properties/subsonic_clients") |

### subsonic\_clients



`subsonic_clients`

* is required

* Type: `object[]` ([Details](config-defs-subsonicconfig.md))

* cannot be null

* defined in: [Config](config-defs-interzicconfig-properties-subsonic_clients.md "path#/$defs/InterzicConfig/properties/subsonic_clients")

#### subsonic\_clients Type

`object[]` ([Details](config-defs-subsonicconfig.md))

## Definitions group ListenConfig

Reference this group by using

```json
{"$ref":"path#/$defs/ListenConfig"}
```

| Property                                                | Type      | Required | Nullable       | Defined by                                                                                                                               |
| :------------------------------------------------------ | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| [refresh\_unmapped\_listens](#refresh_unmapped_listens) | `boolean` | Required | cannot be null | [Config](config-defs-listenconfig-properties-refresh_unmapped_listens.md "path#/$defs/ListenConfig/properties/refresh_unmapped_listens") |

### refresh\_unmapped\_listens



`refresh_unmapped_listens`

* is required

* Type: `boolean`

* cannot be null

* defined in: [Config](config-defs-listenconfig-properties-refresh_unmapped_listens.md "path#/$defs/ListenConfig/properties/refresh_unmapped_listens")

#### refresh\_unmapped\_listens Type

`boolean`

## Definitions group MapperConfig

Reference this group by using

```json
{"$ref":"path#/$defs/MapperConfig"}
```

| Property                  | Type    | Required | Nullable       | Defined by                                                                                                   |
| :------------------------ | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------- |
| [backlisted](#backlisted) | `array` | Required | cannot be null | [Config](config-defs-mapperconfig-properties-backlisted.md "path#/$defs/MapperConfig/properties/backlisted") |

### backlisted

List of recordings that shouldn't be proposed to be mapped

`backlisted`

* is required

* Type: `string[]`

* cannot be null

* defined in: [Config](config-defs-mapperconfig-properties-backlisted.md "path#/$defs/MapperConfig/properties/backlisted")

#### backlisted Type

`string[]`

## Definitions group MusicbrainzServer

Reference this group by using

```json
{"$ref":"path#/$defs/MusicbrainzServer"}
```

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                           |
| :----------------------- | :-------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------- |
| [authority](#authority)  | `string`  | Required | cannot be null | [Config](config-defs-musicbrainzserver-properties-authority.md "path#/$defs/MusicbrainzServer/properties/authority") |
| [ratelimit](#ratelimit)  | Merged    | Optional | cannot be null | [Config](config-defs-musicbrainzserver-properties-ratelimit.md "path#/$defs/MusicbrainzServer/properties/ratelimit") |
| [use\_https](#use_https) | `boolean` | Required | cannot be null | [Config](config-defs-musicbrainzserver-properties-use_https.md "path#/$defs/MusicbrainzServer/properties/use_https") |

### authority



`authority`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-musicbrainzserver-properties-authority.md "path#/$defs/MusicbrainzServer/properties/authority")

#### authority Type

`string`

### ratelimit



`ratelimit`

* is optional

* Type: merged type ([Details](config-defs-musicbrainzserver-properties-ratelimit.md))

* cannot be null

* defined in: [Config](config-defs-musicbrainzserver-properties-ratelimit.md "path#/$defs/MusicbrainzServer/properties/ratelimit")

#### ratelimit Type

merged type ([Details](config-defs-musicbrainzserver-properties-ratelimit.md))

any of

* [Untitled object in Config](config-defs-ratelimitconfig.md "check type definition")

* [Untitled null in Config](config-defs-musicbrainzserver-properties-ratelimit-anyof-1.md "check type definition")

### use\_https



`use_https`

* is required

* Type: `boolean`

* cannot be null

* defined in: [Config](config-defs-musicbrainzserver-properties-use_https.md "path#/$defs/MusicbrainzServer/properties/use_https")

#### use\_https Type

`boolean`

## Definitions group RateLimitConfig

Reference this group by using

```json
{"$ref":"path#/$defs/RateLimitConfig"}
```

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                       |
| :----------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------- |
| [each](#each)            | `integer` | Required | cannot be null | [Config](config-defs-ratelimitconfig-properties-each.md "path#/$defs/RateLimitConfig/properties/each")           |
| [max\_burst](#max_burst) | `integer` | Optional | can be null    | [Config](config-defs-ratelimitconfig-properties-max_burst.md "path#/$defs/RateLimitConfig/properties/max_burst") |
| [tokens](#tokens-1)      | `integer` | Required | cannot be null | [Config](config-defs-ratelimitconfig-properties-tokens.md "path#/$defs/RateLimitConfig/properties/tokens")       |

### each



`each`

* is required

* Type: `integer`

* cannot be null

* defined in: [Config](config-defs-ratelimitconfig-properties-each.md "path#/$defs/RateLimitConfig/properties/each")

#### each Type

`integer`

#### each Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### max\_burst



`max_burst`

* is optional

* Type: `integer`

* can be null

* defined in: [Config](config-defs-ratelimitconfig-properties-max_burst.md "path#/$defs/RateLimitConfig/properties/max_burst")

#### max\_burst Type

`integer`

#### max\_burst Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

### tokens



`tokens`

* is required

* Type: `integer`

* cannot be null

* defined in: [Config](config-defs-ratelimitconfig-properties-tokens.md "path#/$defs/RateLimitConfig/properties/tokens")

#### tokens Type

`integer`

#### tokens Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## Definitions group SubsonicConfig

Reference this group by using

```json
{"$ref":"path#/$defs/SubsonicConfig"}
```

| Property                     | Type      | Required | Nullable       | Defined by                                                                                                         |
| :--------------------------- | :-------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------- |
| [name](#name)                | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-name.md "path#/$defs/SubsonicConfig/properties/name")               |
| [password](#password)        | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-password.md "path#/$defs/SubsonicConfig/properties/password")       |
| [strict\_mbid](#strict_mbid) | `boolean` | Optional | can be null    | [Config](config-defs-subsonicconfig-properties-strict_mbid.md "path#/$defs/SubsonicConfig/properties/strict_mbid") |
| [url](#url)                  | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-url.md "path#/$defs/SubsonicConfig/properties/url")                 |
| [username](#username-1)      | `string`  | Required | cannot be null | [Config](config-defs-subsonicconfig-properties-username.md "path#/$defs/SubsonicConfig/properties/username")       |

### name

The name of the subsonic instance

`name`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-name.md "path#/$defs/SubsonicConfig/properties/name")

#### name Type

`string`

### password

The password of the user

`password`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-password.md "path#/$defs/SubsonicConfig/properties/password")

#### password Type

`string`

### strict\_mbid

If set to true, only subsonic tracks that match the MBID of the requested recording are mapped

Only set this if your subsonic server is capable of searching by MBID (Like navidrome),
and your collection has been passed through Musicbrainz Picard

`strict_mbid`

* is optional

* Type: `boolean`

* can be null

* defined in: [Config](config-defs-subsonicconfig-properties-strict_mbid.md "path#/$defs/SubsonicConfig/properties/strict_mbid")

#### strict\_mbid Type

`boolean`

### url

The url of the subsonic instance

`url`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-url.md "path#/$defs/SubsonicConfig/properties/url")

#### url Type

`string`

### username

The username of the user

`username`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-subsonicconfig-properties-username.md "path#/$defs/SubsonicConfig/properties/username")

#### username Type

`string`
