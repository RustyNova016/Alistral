# Dealing with inputs and variables

So far, we have put our inputs directly in the radio file. This is useful to set defaults that users don't need to touch, but not really practical. So let's improve that

## The inputs json

Internally, all the inputs of a radio are assembled into a giant JSON, with each layer id having it's inputs:

```json
{
    "listen_seeder": {
        "user": "RustyNova"
    },
    "sort_module": {
        "direction": "Desc"
    }
}
```
*Note: It's a simplification. It's not really how it works but it's pretty close*

When a user puts arguments when running the radio, they creates own input json, which overide the inputs set in the radio file:

```json
{
    "listen_seeder": {
        "user": "Alephria"
    }
}
```

*Is added and becomes:*

```json
{
    "listen_seeder": {
        "user": "Alephria"
    },
    "sort_module": {
        "direction": "Desc"
    }
}
```

So **any** input on **any** step is overridable. This is really powerful for testing stuff

## Variables

... But not practical for the users. That's where variables comes in. Those are aliases on the inputs json.

```json
{
    "username": {
        "targets": [
            "listen_seeder.user"
        ]
    },
}
```

Here we create an alias called `username`. The target is the `listen_seeder`'s `user` input. This means that if the user input this json:

```json
{
    "username": "RustyNova"
}
```

it will get turned into:


```json
{
    "listen_seeder": {
        "user": "RustyNova"
    }
}
```

Then added to the radio's defaults

## Other variables usages

Variables can target multiple layers at once:

```json
{
    "username": {
        "targets": [
            "listen_seeder.user",
            "and_filter.radio.username",
            "latest_listens.user"
        ]
    },
}
```

Variables can have a default value:

```json
{
   "minimum_listens": {
        "description": "The minimum all time listens the tracks need to have before getting suggested",
        "targets": [
            "listen_filter.minimum"
        ],
        "default": 3
   },
}
```

## Special variables

Some variables are automatically provided and the user doesn't need to provide them. However you probably want to use them:

- `username`: The name of the default listenbrainz user
- `timeouts`: The recordings in timeout set by the user. To be used in a `timeout_filter`