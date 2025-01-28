# The configuration file

The main configuration file is a JSON document located at:
- Windows: C:\Users\\{user}\AppData\Roaming\alistral\config.json
- Linux: /home/{user}/.config/alistral/config.json
- MacOs: /Users/{user}/Library/Application Support/alistral/config.json


## Values

### Default user

Set a default user for all the commands (gets overrided if set in the command arguments)

```
{
    ...
    "default_user": "spanish_inquisition"
    ...
}
```

### Listenbrainz URL

Allow setting a custom url for listenbrainz

```
{
    ...
    "listenbrainz_url": "https://api.listenbrainz.org/1/"
    ...
}
```

### Musicbrainz URL

Allow setting a custom url for musicbrainz

```
{
    ...
    "musicbrainz_url": "http://musicbrainz.org/ws/2"
    ...
}
```