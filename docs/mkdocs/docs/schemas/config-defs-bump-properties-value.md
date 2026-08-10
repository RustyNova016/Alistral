# Untitled undefined type in Config Schema

```txt
path#/$defs/Bump/properties/value
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                        |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [config.schema.json\*](../../../../out/config.schema.json "open original schema") |

## value Type

any of the following: `string` or `number` ([Details](config-defs-bump-properties-value.md))

## value Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^-?\d+(\.\d+)?([eE]\d+)?$
```

[try pattern](https://regexr.com/?expression=%5E-%3F%5Cd%2B\(%5C.%5Cd%2B\)%3F\(%5BeE%5D%5Cd%2B\)%3F%24 "try regular expression with regexr.com")
