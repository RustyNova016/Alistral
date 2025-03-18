# Mappers

Those modules turn a track into one or more other tracks.

## Artist discography

This module collect all the tracks of the stream, and return all the discography of the artists credited on the original tracks.
They are returned in a random order

### Inputs

(No inputs)

### Stream Info

- ⚠️ This [consume](../create/consuming_stream.md) the stream
- ⚠️ This turns the stream [infinite](../create/consuming_stream.md)
- ⚠️ The scores are reset to 0
- ⚠️ Clear the listens


### Example

```json
{
    "step_type": "artist_discography_mapper",
    "id": "artist_discography_mapper"
}
```