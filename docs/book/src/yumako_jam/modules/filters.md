## And

This filter takes in another radio, and only keep tracks that are both found in the current radio and the other one

### Inputs

- `radio_schema`: The schema of another radio

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

- `duration`: The ammount of time that should pass before allowing the track to pass.

### Example

```json
{
    "step_type": "and_filter",
    "id": "and_filter",
    "inputs": {
        "duration": "1 day 12 hours 24 minutes"
    }
}
```

## Minimum listens

This filter only let tracks that have a minimum (inclusive) of listens

### Inputs

- `minimum`: The minimum amount of listens

### Example

```json
{
    "step_type": "minimum_listen_filter",
    "id": "minimum_listen_filter",
    "inputs": {
        "minimum": 3 
    }
}
```