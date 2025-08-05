## And

This filter takes in another radio, and only keep tracks that are both found in the current radio and the other one

### Inputs

- `radio_schema: Radio` (required): The schema of another radio. This will consume it entirely

### Example

```json
{
    "step_type": "and_filter",
    "id": "and_filter",
    "inputs": {
        "radio_schema": {
            "name": "Inner Radio!",
            "stack": [
                "..."
            ],
        }
    }
}
```

## Cooldown

Removes all the tracks that have been recently listened.

### Inputs

- `duration: String` (required): The amount of time that should pass since the last listen before allowing the track to pass.

### Example

```json
{
    "step_type": "cooldown_filter",
    "id": "cooldown_filter",
    "inputs": {
        "duration": "1 day 12 hours 24 minutes"
    }
}
```

## listens

This filter only let tracks that have a minimum (inclusive) or maximum (inclusive) of listens..

Setting a maximum of 0 only give unlistened tracks

⚠️ Make sure the listens are loaded before this step

### Inputs

- `minimum: Int` (Default: `0`): The minimum amount of listens
- `maximum: Int` (Default: `infinite`): The minimum amount of listens

### Example

```json
{
    "step_type": "listen_filter",
    "id": "listen_filter",
    "inputs": {
        "minimum": 3 
    }
}
```

## Timeouts

Removes all the tracks that are "in timeout". 

### Inputs

- `timeouts: Timeout`: The list of track timeouts.

### Example

```json
{
    "step_type": "timeout_filter",
    "id": "timeout_filter",
    "inputs": {
        "timeouts": {
            "1119dec1-2ed9-49ff-a059-2bd5b048af3c": "2025-09-20T14:26:57.455793591Z",
            "7cd8da60-3255-4708-9cc0-6bfdd196fd5f": "2025-09-20T14:28:19.264683649Z",
        }
    }
}
```