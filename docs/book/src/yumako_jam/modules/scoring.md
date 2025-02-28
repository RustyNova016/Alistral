# Scoring

Having a flux of tracks is nice and all, but having them randomly arranged doesn't make a good radio.
That's where "scoring" comes in play. Each track can get assigned a score, then sorted accordingly.

## Sort

This is the main module of the scoring system. It sorts the tracks depending on their score.
This consume the entire stream.

### Inputs

- `direction: String`: Whether to sort by ascending or descending order. By default, it sort by descending order. Accept: `"asc"` or `"desc`

### Example

```json
{
    "step_type": "sort_module",
    "id": "sort_module",
    "inputs": {
        "direction": "desc"
    }
}
```