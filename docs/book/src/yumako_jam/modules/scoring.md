# Scoring

Having a flux of tracks is nice and all, but having them randomly arranged doesn't make a good radio.
That's where "scoring" comes in play. Each track can get assigned a score, then sorted accordingly. This score is a decimal number

## Merging scores

By default, scorers replace the score of the track with their computed value. You can change this by setting the `merge` variable
present on all the scorers.

Accepted values are: 
- Replace (default)
- Add
- Sub
- Multiply
- Divide

## Sort

This is the main module of the scoring system. It sorts the tracks depending on their score.
This [consume](../consuming_stream.md) the entire stream.

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

## Listen rate scorer

Set the score to the number of listens estimated to happen in a year

### Inputs

- `merge`

### Example

```json
{
    "step_type": "listenrate_scorer",
    "id": "listenrate_scorer",
    "inputs": {
        "merge": "Add"
    }
}
```

## Overdue duration scorer

Set the score to the number of seconds elapsed since the user should have listened to the track again.


### Inputs

- `merge`

### Example

```json
{
    "step_type": "overdue_scorer",
    "id": "overdue_scorer",
    "inputs": {
        "merge": "Add"
    }
}
```