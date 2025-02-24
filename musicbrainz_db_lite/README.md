# Musicbrainz DB Lite

⚠️ Alpha crate! Don't expect anything just yet! (Maybe dragons though...) ⚠️

A rewrite of Musicbrainz's and Listenbrainz's databases in SQLite, with Rust bindings

# FAQ
## But why?

Musicbrainz is arguably the best music database out there, and the API is quite powerful, but extremely slow (1 request per seconds...).
So either you follow into Picard's footsteps and make the user wait up to minutes for their data to finish loading, or you use a caching system.

That's what this crate is the best for. It use SQLite to store data, allowing to embed a miniature version of the database without forcing the user to download postgres.

## Do I need it?

It all depends on your use case. If you need to cache Musicbrainz data on the user's computer, and don't mind working with sqlite's limitations, go for it.
But if you need caching on a server, or loave sqlite, the official database is fine enough.

## It isn't the same schema!

It is close enough™️. The official schema as been used as base, but has been simplified a bit to fit in the constraint of SQLite, and remove fields that are impossible to fetch through the API. Fields are also based on the API's name, as they are more commonly known than the official ones.