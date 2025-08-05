# Listens

Every track has its own collection of listens associated to it. 
The modules of this page allows modifying this collection to refine the interval of data wanted

Listens cannot be duplicated. So if a module tries to add listen data already present, it won't add anything

## Clear listens

Clear the associated listens

### Inputs

(No inputs)

### Example

```json
{
    "step_type": "clear_listens",
    "id": "clear_listens",
}
```

## Listen Interval

Removes all the listens not in the interval

### Inputs

- `period: Period` (Default: `Last90Days`): The [period](../create/input_types/stat_period.md) to keep the listens from
- `min_ts: Integer` (Default: `0`): The minimum timestamp of the listen (including)
- `max_ts: Integer` (Default: `\[Forever\]`): The maximum timestamp of the listen (including)

### Example

```json
{
    "step_type": "listen_interval",
    "id": "listen_interval",
    "inputs": {
        "period": "Last90Days"
    }
}
```

## Latest listens

This module add / remove the latest X listens of the recording

### Inputs

- `user: String` (required): The user to pull listens from
- `count: Integer` (Default: `3`): The number of latest listens to add to each recording
- `action: ListenAction` (Default: `Add`): Either add or remove listens. Accept: `"Add"`, `"Remove"`, or `"Replace"`
- `buffer: Integer` (Default: `8`): Buffer size for the listens. See [buffering](../create/performance.md#buffering)

### Example

```json
{
    "step_type": "latest_listens",
    "id": "latest_listens",
    "inputs": {
        "user": "RustyNova"
    }
}
```