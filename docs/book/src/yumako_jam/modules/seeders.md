# Seeders

Seeders provide the initial tracks for your radios. 

## Artist Seeder

This seeder provide all the recording's of an artist

### Inputs

- `artist_mbid: String`: The mbid of the artist

### Stream Info

- The stream is [finite](../create/consuming_stream.md)
- The scores are set to 0
- No listens are added

### Example

```json
{
    "step_type": "artist_seeder",
    "id": "artist_seeder",
    "inputs": {
        "artist_mbid": "1f8ef6a0-6d01-4ea2-92d4-693bc565fb61"
    }
}
```

## Listen Seeder

This seeder provide all the tracks listened by an user

### Inputs

- `user: String`: The username of the user to pull the listening history from

### Stream Info

- The stream is [finite](../create/consuming_stream.md)
- The scores are set to 0
- Adds all time listens

### Example

```json
{
    "step_type": "listen_seeder",
    "id": "listen_seeder",
    "inputs": {
        "user": "RustyNova"
    }
}
```

