# Untitled object in Config Schema

```txt
path#/$defs/RateLimitConfig
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## RateLimitConfig Type

`object` ([Details](config-defs-ratelimitconfig.md))

# RateLimitConfig Properties

| Property                 | Type      | Required | Nullable       | Defined by                                                                                                       |
| :----------------------- | :-------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------- |
| [each](#each)            | `integer` | Required | cannot be null | [Config](config-defs-ratelimitconfig-properties-each.md "path#/$defs/RateLimitConfig/properties/each")           |
| [max\_burst](#max_burst) | `integer` | Optional | can be null    | [Config](config-defs-ratelimitconfig-properties-max_burst.md "path#/$defs/RateLimitConfig/properties/max_burst") |
| [tokens](#tokens)        | `integer` | Required | cannot be null | [Config](config-defs-ratelimitconfig-properties-tokens.md "path#/$defs/RateLimitConfig/properties/tokens")       |

## each



`each`

* is required

* Type: `integer`

* cannot be null

* defined in: [Config](config-defs-ratelimitconfig-properties-each.md "path#/$defs/RateLimitConfig/properties/each")

### each Type

`integer`

### each Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## max\_burst



`max_burst`

* is optional

* Type: `integer`

* can be null

* defined in: [Config](config-defs-ratelimitconfig-properties-max_burst.md "path#/$defs/RateLimitConfig/properties/max_burst")

### max\_burst Type

`integer`

### max\_burst Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`

## tokens



`tokens`

* is required

* Type: `integer`

* cannot be null

* defined in: [Config](config-defs-ratelimitconfig-properties-tokens.md "path#/$defs/RateLimitConfig/properties/tokens")

### tokens Type

`integer`

### tokens Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint32`
