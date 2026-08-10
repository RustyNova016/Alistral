# Untitled object in Config Schema

```txt
path#/$defs/MapperConfig
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## MapperConfig Type

`object` ([Details](config-defs-mapperconfig.md))

# MapperConfig Properties

| Property                  | Type    | Required | Nullable       | Defined by                                                                                                   |
| :------------------------ | :------ | :------- | :------------- | :----------------------------------------------------------------------------------------------------------- |
| [backlisted](#backlisted) | `array` | Required | cannot be null | [Config](config-defs-mapperconfig-properties-backlisted.md "path#/$defs/MapperConfig/properties/backlisted") |

## backlisted

List of recordings that shouldn't be proposed to be mapped

`backlisted`

* is required

* Type: `string[]`

* cannot be null

* defined in: [Config](config-defs-mapperconfig-properties-backlisted.md "path#/$defs/MapperConfig/properties/backlisted")

### backlisted Type

`string[]`
