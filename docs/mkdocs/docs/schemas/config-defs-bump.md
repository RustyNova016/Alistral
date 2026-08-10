# Untitled object in Config Schema

```txt
path#/$defs/BumpList/items
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## items Type

`object` ([Details](config-defs-bump.md))

# items Properties

| Property                             | Type     | Required | Nullable       | Defined by                                                                                             |
| :----------------------------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------- |
| [expiration\_date](#expiration_date) | `string` | Required | cannot be null | [Config](config-defs-bump-properties-expiration_date.md "path#/$defs/Bump/properties/expiration_date") |
| [recording](#recording)              | `string` | Required | cannot be null | [Config](config-defs-bump-properties-recording.md "path#/$defs/Bump/properties/recording")             |
| [username](#username)                | `string` | Required | cannot be null | [Config](config-defs-bump-properties-username.md "path#/$defs/Bump/properties/username")               |
| [value](#value)                      | Multiple | Required | cannot be null | [Config](config-defs-bump-properties-value.md "path#/$defs/Bump/properties/value")                     |

## expiration\_date



`expiration_date`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-bump-properties-expiration_date.md "path#/$defs/Bump/properties/expiration_date")

### expiration\_date Type

`string`

### expiration\_date Constraints

**date time**: the string must be a date time string, according to [RFC 3339, section 5.6](https://tools.ietf.org/html/rfc3339 "check the specification")

## recording



`recording`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-bump-properties-recording.md "path#/$defs/Bump/properties/recording")

### recording Type

`string`

## username



`username`

* is required

* Type: `string`

* cannot be null

* defined in: [Config](config-defs-bump-properties-username.md "path#/$defs/Bump/properties/username")

### username Type

`string`

## value



`value`

* is required

* Type: any of the following: `string` or `number` ([Details](config-defs-bump-properties-value.md))

* cannot be null

* defined in: [Config](config-defs-bump-properties-value.md "path#/$defs/Bump/properties/value")

### value Type

any of the following: `string` or `number` ([Details](config-defs-bump-properties-value.md))

### value Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^-?\d+(\.\d+)?([eE]\d+)?$
```

[try pattern](https://regexr.com/?expression=%5E-%3F%5Cd%2B\(%5C.%5Cd%2B\)%3F\(%5BeE%5D%5Cd%2B\)%3F%24 "try regular expression with regexr.com")
