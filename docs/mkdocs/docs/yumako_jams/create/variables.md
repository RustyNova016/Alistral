## Radio inputs

Radio inputs allows mapping ugly variable paths to pretty variable names. 
They also serve to designate inputs where the user might want to tweak things, or is required to fill.

They are located in the `"input"` field of the radio schema:

```json
{
    "name": "See my inputs!",
    "stack": [
        "..."
    ],
    "inputs": {
        "username": {
            "targets": ["listen_seeder.user"],
            "title": "Username",
            "description": "Your listenbrainz username",
            "required": true,
        }
    }
}
```

### targets

This is a list of all the variable paths this variable fill in

### title

The name of the variable to display to the user.

### description

A description to tell the user what this variable does

### required

Force this variable to be provided by the user

### default

Add a default for this variable

### hidden

Hide this variable from the user. They can still overwrite it

## Provided variables

Those variables are automatically filled at the radio creation. However you need to manually map them to your module inputs using `radio.inputs`

- current_time
- timeouts

```json
{
    "name": "See my inputs!",
    "stack": [
        "..."
    ],
    "inputs": {
        "timeouts": {
            "targets": ["timeout_filter.timeouts"],
            "hidden": true
        }
    }
}
```