# Seeders

Seeders provide new radio items for your radios. While it's commonly found as the first thing on a radio, more can be added afterward, and will add radio items after the previous(es) seeders finished

## Artist Seeder

This seeder provide all the recording's of an artist. The discography tracks are in a random order, but the artists are in the order they have been declared

### Inputs

- `artist_mbids: String[]`: The mbids of artists to seed from

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
        "artist_mbids": ["1f8ef6a0-6d01-4ea2-92d4-693bc565fb61"]
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

## Release Seeder

This seeder provide all the recording's of an artist

### Inputs

- `release_mbids: String[]`: The mbids of releases to seed from

### Stream Info

- The stream is [finite](../create/consuming_stream.md)
- The scores are set to 0
- No listens are added

### Example

```json
{
    "step_type": "release_seeder",
    "id": "release_seeder",
    "inputs": {
        "release_mbids": ["d96154ed-2403-428e-ae8e-39ca2cc01430"]
    }
}
```