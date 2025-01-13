# Command-Line Help for `alistral`

This document contains the help content for the `alistral` command-line program.

**Command Overview:**

* [`alistral`↴](#alistral)
* [`alistral bump`↴](#alistral-bump)
* [`alistral bump-down`↴](#alistral-bump-down)
* [`alistral cache`↴](#alistral-cache)
* [`alistral cache copy-to-debug`↴](#alistral-cache-copy-to-debug)
* [`alistral cache clear`↴](#alistral-cache-clear)
* [`alistral cache clear-listens`↴](#alistral-cache-clear-listens)
* [`alistral cache init-database`↴](#alistral-cache-init-database)
* [`alistral cache load-dump`↴](#alistral-cache-load-dump)
* [`alistral cache refresh-data`↴](#alistral-cache-refresh-data)
* [`alistral compatibility`↴](#alistral-compatibility)
* [`alistral config`↴](#alistral-config)
* [`alistral config blacklist-mapper-msid`↴](#alistral-config-blacklist-mapper-msid)
* [`alistral config set-token`↴](#alistral-config-set-token)
* [`alistral config timeout`↴](#alistral-config-timeout)
* [`alistral config listens`↴](#alistral-config-listens)
* [`alistral config listens refresh-unmapped-listens`↴](#alistral-config-listens-refresh-unmapped-listens)
* [`alistral config default-user`↴](#alistral-config-default-user)
* [`alistral daily`↴](#alistral-daily)
* [`alistral listens`↴](#alistral-listens)
* [`alistral listens remap-msid`↴](#alistral-listens-remap-msid)
* [`alistral listens wrong-mapping`↴](#alistral-listens-wrong-mapping)
* [`alistral lookup`↴](#alistral-lookup)
* [`alistral mapping`↴](#alistral-mapping)
* [`alistral mapping list-unmapped`↴](#alistral-mapping-list-unmapped)
* [`alistral musicbrainz`↴](#alistral-musicbrainz)
* [`alistral musicbrainz clippy`↴](#alistral-musicbrainz-clippy)
* [`alistral radio`↴](#alistral-radio)
* [`alistral radio circles`↴](#alistral-radio-circles)
* [`alistral radio underrated`↴](#alistral-radio-underrated)
* [`alistral radio rate`↴](#alistral-radio-rate)
* [`alistral radio overdue`↴](#alistral-radio-overdue)
* [`alistral radio shared`↴](#alistral-radio-shared)
* [`alistral stats`↴](#alistral-stats)
* [`alistral unstable`↴](#alistral-unstable)
* [`alistral unstable best-of-mc`↴](#alistral-unstable-best-of-mc)

## `alistral`

A CLI app containing a set of useful tools for Listenbrainz

**Usage:** `alistral [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `bump` — bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target
* `bump-down` — bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target
* `cache` — Commands to deal with the local cache
* `compatibility` — 
* `config` — Commands to deal with the app's configuration
* `daily` — Daily report
* `listens` — Commands to edit listens
* `lookup` — Get detailled information about an entity
* `mapping` — Commands for interacting with listen mappings
* `musicbrainz` — Commands for musicbrainz stuff
* `radio` — Generate radio playlists for you
* `stats` — Shows top statistics for a specific target
* `unstable` — A CLI app containing a set of useful tools for Listenbrainz

###### **Options:**

* `--generate <GENERATOR>`

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`

* `--no-cleanup`

  Default value: `false`



## `alistral bump`

bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.

bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`

All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.

**Usage:** `alistral bump [OPTIONS] [RECORDING]`

###### **Arguments:**

* `<RECORDING>` — The recording to bump

###### **Options:**

* `-d`, `--duration <DURATION>` — The duration the bump last for (Default: 3 months)
* `-m`, `--multiplier <MULTIPLIER>` — The multiplier added to the score (Default: 1.1)
* `-u`, `--username <USERNAME>`



## `alistral bump-down`

bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.

bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`

All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.

**Usage:** `alistral bump-down [OPTIONS] [RECORDING]`

###### **Arguments:**

* `<RECORDING>` — The recording to bump

###### **Options:**

* `-d`, `--duration <DURATION>` — The duration the bump last for (Default: 3 months)
* `-m`, `--multiplier <MULTIPLIER>` — The multiplier added to the score (Default: 1.1)
* `-u`, `--username <USERNAME>`



## `alistral cache`

Commands to deal with the local cache

**Usage:** `alistral cache <COMMAND>`

###### **Subcommands:**

* `copy-to-debug` — Copy the release database to the debug one
* `clear` — Wipe the cache's data
* `clear-listens` — Clear all the listens from the database
* `init-database` — Initialise the database
* `load-dump` — Load a listen dump from the website
* `refresh-data` — 



## `alistral cache copy-to-debug`

Copy the release database to the debug one.

⚠️ This wipe the debug database.

⚠️ If there is migrations, do `cargo sqlx migrate run` next

**Usage:** `alistral cache copy-to-debug`



## `alistral cache clear`

Wipe the cache's data

This is useful if you need disk space, or need to manually rebuild in case of corruption

**Usage:** `alistral cache clear`



## `alistral cache clear-listens`

Clear all the listens from the database

**Usage:** `alistral cache clear-listens [USER]`

###### **Arguments:**

* `<USER>` — Only delete listens of user



## `alistral cache init-database`

Initialise the database

**Usage:** `alistral cache init-database [OPTIONS]`

###### **Options:**

* `--reset` — Wipe the database file beforehand



## `alistral cache load-dump`

Load a listen dump from the website

Allows to load an exported dump of you listens. This is often faster than using the app. This also prevent stumbling into LB-1584

You can get a listen dump [here](https://listenbrainz.org/settings/export/)

**Usage:** `alistral cache load-dump <PATH> [USERNAME]`

###### **Arguments:**

* `<PATH>` — Path to the dump file
* `<USERNAME>` — Name of the user to import those listens for



## `alistral cache refresh-data`

**Usage:** `alistral cache refresh-data [OPTIONS]`

###### **Options:**

* `-u`, `--username <USERNAME>` — Name of the user to refresh the data
* `-l`, `--limit <LIMIT>` — How many entities to refresh
* `-m`, `--max-ts <MAX_TS>` — Only refresh older than timestamp



## `alistral compatibility`

**Usage:** `alistral compatibility <USER_A> <USER_B>`

###### **Arguments:**

* `<USER_A>` — The name of the first user
* `<USER_B>` — The name of the second user



## `alistral config`

Commands to deal with the app's configuration

**Usage:** `alistral config <COMMAND>`

###### **Subcommands:**

* `blacklist-mapper-msid` — Prevent an MSID to appear in the mbid mapper
* `set-token` — Associate an user token to an username. This makes `--token` arguments optional, and prevent always having to insert it
* `timeout` — Prevent the recording to appear on radios for a while. If you're burn out of a track and need it gone, use this
* `listens` — Configuration targeting listen data
* `default-user` — Set the default username



## `alistral config blacklist-mapper-msid`

Prevent an MSID to appear in the mbid mapper

**Usage:** `alistral config blacklist-mapper-msid [OPTIONS] <MSID>`

###### **Arguments:**

* `<MSID>` — The msid to blacklist

###### **Options:**

* `--remove` — Remove it from the blacklist



## `alistral config set-token`

Associate an user token to an username. This makes `--token` arguments optional, and prevent always having to insert it

**Usage:** `alistral config set-token <USERNAME> <TOKEN>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to add the token
* `<TOKEN>` — User token



## `alistral config timeout`

Prevent the recording to appear on radios for a while. If you're burn out of a track and need it gone, use this

**Usage:** `alistral config timeout <RECORDING> <DURATION>`

###### **Arguments:**

* `<RECORDING>` — A string containing a MBID of a recording
* `<DURATION>` — A duration to timeout for



## `alistral config listens`

Configuration targeting listen data

**Usage:** `alistral config listens <COMMAND>`

###### **Subcommands:**

* `refresh-unmapped-listens` — Toggle / Set whether the unmapped listens should be automatically updated when fetching listens



## `alistral config listens refresh-unmapped-listens`

Toggle / Set whether the unmapped listens should be automatically updated when fetching listens

**Usage:** `alistral config listens refresh-unmapped-listens <STATE>`

###### **Arguments:**

* `<STATE>` — What do you want it set to?

  Possible values: `toggle`, `true`, `false`




## `alistral config default-user`

Set the default username

**Usage:** `alistral config default-user <USERNAME>`

###### **Arguments:**

* `<USERNAME>`



## `alistral daily`

Daily report

**Usage:** `alistral daily [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch stats listen from



## `alistral listens`

Commands to edit listens

**Usage:** `alistral listens <COMMAND>`

###### **Subcommands:**

* `remap-msid` — Changes all the listens of a recording into another. Useful if LB mapped to a recording you never listened
* `wrong-mapping` — 



## `alistral listens remap-msid`

Changes all the listens of a recording into another. Useful if LB mapped to a recording you never listened

**Usage:** `alistral listens remap-msid <ORIGINAL_ID> <NEW_ID> [USERNAME] [TOKEN]`

###### **Arguments:**

* `<ORIGINAL_ID>` — The MBID of the recording
* `<NEW_ID>` — The MBID of the recorind to replace it with
* `<USERNAME>` — Your username
* `<TOKEN>` — Your account token



## `alistral listens wrong-mapping`

**Usage:** `alistral listens wrong-mapping [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Your username



## `alistral lookup`

Get detailled information about an entity

**Usage:** `alistral lookup <ENTITY_TYPE> <ID> [USERNAME]`

###### **Arguments:**

* `<ENTITY_TYPE>` — The type of entity to look for

  Possible values: `recording`

* `<ID>` — The id of the entity (Accept URLs)
* `<USERNAME>` — Name of the user to look up stats from



## `alistral mapping`

Commands for interacting with listen mappings

**Usage:** `alistral mapping <COMMAND>`

###### **Subcommands:**

* `list-unmapped` — List all of your unlinked listens



## `alistral mapping list-unmapped`

List all of your unlinked listens

This command will list all your unmapped listens, grouped by similarity. It also gives a link to quickly look up the listen in listenbrainz, and go link it

```text

(1) Paul's Dream (Dune) - Caster -> <https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553>

(7) Raise Your Weapon - KLOUD -> <https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522>

Total: 8 unlinked recordings

```

> Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens > by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.

> This also means that the same recording can be shown twice in the list. > For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.

**Usage:** `alistral mapping list-unmapped [OPTIONS] [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch unlinked listen from

###### **Options:**

* `-s`, `--sort <SORT>` — Sort the listens by type

  Possible values:
  - `count`:
    The count of listens for this element. This is descending by default
  - `name`:
    The name of the associated element
  - `oldest`:
    The oldest element




## `alistral musicbrainz`

Commands for musicbrainz stuff

**Usage:** `alistral musicbrainz <COMMAND>`

###### **Subcommands:**

* `clippy` — Search for potential mistakes, missing data and style issues. This allows to quickly pin down errors that can be corrected



## `alistral musicbrainz clippy`

Search for potential mistakes, missing data and style issues. This allows to quickly pin down errors that can be corrected

⚠️ All tips are suggestions. Take them with a grain of salt. If you are unsure, it's preferable to skip.

**Usage:** `alistral musicbrainz clippy [OPTIONS] [START_MBID]`

###### **Arguments:**

* `<START_MBID>` — The MBID of a recording to start from

###### **Options:**

* `-n`, `--new-first` — Whether to check FILO (first in, last out) instead of FIFO (first in, first out)
* `-w`, `--whitelist <WHITELIST>` — List of lints that should only be checked (Note: Put this argument last or before another argument)
* `-b`, `--blacklist <BLACKLIST>` — List of lints that should not be checked (Note: Put this argument last or before another argument)



## `alistral radio`

Generate radio playlists for you

**Usage:** `alistral radio [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `circles` — Randomly adds recordings from artists you already listened to
* `underrated` — Generate a playlist containing your underrated listens
* `rate` — Generate playlists depending on the listen rate of recordings
* `overdue` — Generate playlists based on recording that the user should have listened to by now
* `shared` — Generate playlists based on the listened recordings of two users

###### **Options:**

* `--min-count <MIN_COUNT>` — The minimum count of tracks the radio should add to the playlist. (Default: 50, gets overidden by `--min-duration`)
* `--min-duration <MIN_DURATION>` — The minimum duration the playlist should last for. This accept natural language (Ex: "1 hour 36 mins")
* `--seed-listen-range <SEED_LISTEN_RANGE>` — For radios based on listens, what time range of listens to use as reference

  Possible values:
  - `last30-days`:
    Uses the last 30 days from now
  - `last90-days`:
    Uses the last 30 days from now
  - `last365-days`:
    Uses the last 365 days from now

* `--min-seed-listens <MIN_SEED_LISTENS>` — When used with `seed_listen_range`, how many listens should be given as a minimum, even if they are outside of the range (Default: 3)



## `alistral radio circles`

Randomly adds recordings from artists you already listened to

**Usage:** `alistral radio circles [OPTIONS] [USERNAME] [TOKEN]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from
* `<TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument

###### **Options:**

* `--unlistened` — Use this flag to only get unlistened recordings. This is great for exploration playlists



## `alistral radio underrated`

Generate a playlist containing your underrated listens

This radio will create a playlist containing all the tracks that you listen to, but seemingly no one else does.

> The mix is made by calculating a score for each listen. This score is composed of two values: > - The rank in the user's top 1000 recording of all time (First place get 100 points, second get 999.9, etc...) > - The percentage of the recording's listens being from the user (Made with this formula: (user listens / worldwide listens) *100)

**Usage:** `alistral radio underrated [OPTIONS] [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument



## `alistral radio rate`

Generate playlists depending on the listen rate of recordings

This algorythm bases itself on your listen rate of recording to get more forgotten tracks. It takes the recordings with the lowest listen rates, and put them into a playlist

**Usage:** `alistral radio rate [OPTIONS] [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `--min <MIN>` — Minimum listen count
* `-c`, `--cooldown <COOLDOWN>` — The amount of hours needed to wait after a recording have been given before it is re-suggested

  Default value: `0`



## `alistral radio overdue`

Generate playlists based on recording that the user should have listened to by now

Similar to listen rates, this algorithm calculate the average time between listens, and estimate when the next listen will happen. It then put together a playlist made out of recordings you should have listened by now.

**Usage:** `alistral radio overdue [OPTIONS] [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `--min <MIN>` — Minimum listen count
* `-c`, `--cooldown <COOLDOWN>` — The amount of hours needed to wait after a recording have been given before it is re-suggested

  Default value: `0`
* `-o`, `--overdue-factor` — Sort the recordings by the time overdue / the average time between listens

   Instead of sorting by date, the listens are sorted by how many estimated listens should have happened by now (Time elapsed since last listen / Average time per listens)

  Default value: `false`
* `-a`, `--at-listening-time` — Makes `overdue_factor` more accurate by calculating the score at the time the listen will be listened at instead of now.

   This may slowdown the playlist creation by a lot!

  Default value: `false`



## `alistral radio shared`

Generate playlists based on the listened recordings of two users

**Usage:** `alistral radio shared [OPTIONS] <USERNAME_A> <USERNAME_B>`

###### **Arguments:**

* `<USERNAME_A>`
* `<USERNAME_B>`

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `--min <MIN>` — Minimum listen count
* `-c`, `--cooldown <COOLDOWN>` — The amount of hours needed to wait after a recording have been given before it is re-suggested

  Default value: `0`



## `alistral stats`

Shows top statistics for a specific target

Target is the entity type to group the stats by. Currently, those entities stats are implemented:

- Recordings (`recording`)

- Artists (`artist`)

- Releases (`release`)

- Release Groups (`release_group`)

- Works (`work`)

**Usage:** `alistral stats [OPTIONS] <TARGET> [USERNAME]`

###### **Arguments:**

* `<TARGET>` — The type of entity to sort by

  Possible values: `recording`, `recording-playtime`, `artist`, `release`, `release-group`, `work`, `work-recursive`

* `<USERNAME>` — Name of the user to fetch stats listen from

###### **Options:**

* `-s`, `--sort <SORT>` — Sort by:

  Default value: `count`

  Possible values:
  - `count`:
    The count of listens for this element. This is descending by default
  - `name`:
    The name of the associated element
  - `oldest`:
    The oldest element




## `alistral unstable`

A CLI app containing a set of useful tools for Listenbrainz

**Usage:** `alistral unstable <COMMAND>`

###### **Subcommands:**

* `best-of-mc` — See what your favourite Monstercat releases of this year are, and have an easier time voting for this year's Best of 2024!



## `alistral unstable best-of-mc`

See what your favourite Monstercat releases of this year are, and have an easier time voting for this year's Best of 2024!

You can get a listen dump [here](https://listenbrainz.org/settings/export/)

**Usage:** `alistral unstable best-of-mc [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to look up stats from



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>