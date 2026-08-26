# Untitled object in Config Schema

```txt
path#/$defs/DailyCommandConfig
```

Configuration of the daily command

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## DailyCommandConfig Type

`object` ([Details](config-defs-dailycommandconfig.md))

# DailyCommandConfig Properties

| Property                             | Type      | Required | Nullable    | Defined by                                                                                                                         |
| :----------------------------------- | :-------- | :------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| [minimum\_listens](#minimum_listens) | `integer` | Optional | can be null | [Config](config-defs-dailycommandconfig-properties-minimum_listens.md "path#/$defs/DailyCommandConfig/properties/minimum_listens") |

## minimum\_listens

The minimum listens needed to display a track's anniversary / first discovery

`minimum_listens`

* is optional

* Type: `integer`

* can be null

* defined in: [Config](config-defs-dailycommandconfig-properties-minimum_listens.md "path#/$defs/DailyCommandConfig/properties/minimum_listens")

### minimum\_listens Type

`integer`

### minimum\_listens Constraints

**minimum**: the value of this number must greater than or equal to: `0`

**unknown format**: the value of this string must follow the format: `uint`
