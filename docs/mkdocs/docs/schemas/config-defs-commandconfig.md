# Untitled object in Config Schema

```txt
path#/$defs/CommandConfig
```

Configuration for the commands

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## CommandConfig Type

`object` ([Details](config-defs-commandconfig.md))

# CommandConfig Properties

| Property        | Type   | Required | Nullable       | Defined by                                                                                           |
| :-------------- | :----- | :------- | :------------- | :--------------------------------------------------------------------------------------------------- |
| [daily](#daily) | Merged | Optional | cannot be null | [Config](config-defs-commandconfig-properties-daily.md "path#/$defs/CommandConfig/properties/daily") |

## daily



`daily`

* is optional

* Type: merged type ([Details](config-defs-commandconfig-properties-daily.md))

* cannot be null

* defined in: [Config](config-defs-commandconfig-properties-daily.md "path#/$defs/CommandConfig/properties/daily")

### daily Type

merged type ([Details](config-defs-commandconfig-properties-daily.md))

any of

* [Untitled object in Config](config-defs-dailycommandconfig.md "check type definition")

* [Untitled null in Config](config-defs-commandconfig-properties-daily-anyof-1.md "check type definition")
