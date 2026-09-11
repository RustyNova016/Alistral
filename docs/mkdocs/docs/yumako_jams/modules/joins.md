# Joins

Join modules allows joining two radios together 

## join

An sql like join. This supports the following types:
- "full": Both the current and provided radio stream are merged
- "inner": Only let items from the current stream in they exists in the other
- "left": Only let items from the current stream if they don't exist in the other
- "right": Only let items from the other stream if they aren't in the current stream
- "outer": Only let items from the current stream and other stream if they aren't in both streams 

### Inputs

- `radio_schema: Radio` (required): The schema of another radio. This will consume it entirely regardless of the mode
- `join_type: "Left" | "Right" | "Full" | "Inner" | "Outer"` (required): The join mode

### Example

```json5
{
    "step_type": "join",
    "id": "blacklist_join",
    "inputs": {
        "join_type": "Left",
        "radio_schema": {
            "name": "blacklist",
            "stack": [
                ...
            ],
            "inputs": {}
        },
        "radio": {}
    }
}
```