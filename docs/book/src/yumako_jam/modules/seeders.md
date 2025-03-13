# Seeders

Seeders provide the initial tracks for your radios. 

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