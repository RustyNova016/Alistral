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

### Inputs

- `direction: String`: Whether to sort by ascending or descending order. By default, it sort by descending order. Accept: `"Asc"` or `"Desc"`

### Stream Info

- ⚠️ This [consume](../create/consuming_stream.md) the stream

### Example

```json
{
    "step_type": "sort_module",
    "id": "sort_module",
    "inputs": {
        "direction": "Desc"
    }
}
```

## Bump score

This multiply the score by the bump value for each recording.

A bump is a temporary increase / decrease an user can set to make a track more or less frequent.

Bumps usually are automatically provided by the app using the `bumps` variable. 
However they still require linking the path of the step in the variable's targets

### Inputs

- `bumps`

### Example

```json
{
    "step_type": "bump_score",
    "id": "bump_score",
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

## Overdue count scorer

Set the score to the number of times the user should have listened to the track in between the latest listen and now

### Inputs

- `merge`

### Example

```json
{
    "step_type": "overdue_count_scorer",
    "id": "overdue_count_scorer",
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
    "step_type": "overdue_duration_scorer",
    "id": "overdue_duration_scorer",
    "inputs": {
        "merge": "Add"
    }
}
```