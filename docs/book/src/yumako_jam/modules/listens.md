# Listens

Every track has its own collection of listens associated to it. 
The modules of this page allows modifying this collection to refine the interval of data wanted

## Lastest listens

This module add / remove the latest X listens of the recording

### Inputs

- `user: String` (required): The user to pull listens from
- `action: ListenAction` (Default: Add): Either add or remove listens. Accept: `"Add"` or `"Remove"`
- `buffer: Integer` (Default: 8): Buffer size for the listens. See [buffering](../create/performance.md#buffering)

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