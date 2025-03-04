# Seeders

Seeders provide the initial tracks for your radios. 

## Listen Seeder

This seeder provide all the tracks listened by an user

### Inputs

- `user: String`: The username of the user to pull the listening history from

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