# v6 to v7
Here are all the breaking changes of the v6 to v7 update that require migration:

## CLI

### Removed:
- `interzic reload`: Linked to youtube

## Paths
Some paths have changed, so the files requires moving. You can learn about the new paths in [config/paths](../config/paths.md).  

A few important highlights:
- The musicbrainz cache database is located in the alistral cache directory, and is now called `musicbrainz_cache.db`. While not required to migrate, it is recommended to modify it to prevent 5 hours cache refreshes
- Systemd services support. If you ran your alistral in systemd, it will now take the path from the service file (Looking at you, nix users. Don't just `nix flake update`)
- The interzic database has been moved from alistral's config directory to the interzic data directory, with the name `interzic_data.db`


## Removed Youtube support in Interzic
Youtube support has been removed from interzic. The underlying crate has been deprecated, but Google hasn't provided documentation for the new one, and in addition of painful API quotas, I cannot maintain it anymore. If anyone is intrested in reintroducing it, feel free to PR! 