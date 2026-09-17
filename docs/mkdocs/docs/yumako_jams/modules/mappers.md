# Mappers

Those modules turn a radio item into one or more other radio items.

## Artist discography

This module collect all the tracks of the stream, and return all the discography of the artists credited on the original tracks.
They are returned in a random order

### Inputs

(No inputs)

### Stream Info

- ⚠️ This [consume](../creating_radios/consuming_stream.md) the stream
- ⚠️ This turns the stream [infinite](../creating_radios/consuming_stream.md)
- ⚠️ The scores are reset to 0
- ⚠️ Clears the listens


### Example

```json
{
    "step_type": "artist_discography_mapper",
    "id": "artist_discography_mapper"
}
```