# Listenbrainz CLI Tools

A collection of CLI based tools for Listenbrainz.

# Installing
## Builds
The latest build can be found in the [release](https://github.com/RustyNova016/listenbrainz-cli-tools/releases) tab

## Manual build
```shell
git clone https://github.com/RustyNova016/listenbrainz-cli-tools.git
cd ./listenbrainz-cli-tools
cargo build --release
```

# Tools
## Unmapped listens 
To search for your unmapped listens, use:
```shell
listenbrainz-cli-tools unmapped -u <username>
```

This will list all your unmapped listens, grouped by similarity. 
It also gives a link to quickly look up the listen in listenbrainz, and go link it

```
(1) Paul's Dream (Dune) - Caster
    -> https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553
(7) Raise Your Weapon - KLOUD
    -> https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522
Total: 8 unlinked recordings
```

> Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens 
> by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.
> 
> This also means that the same recording can be shown twice in the list. 
> For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.

## Interactive mass mapper

This tools allow for easy and faster mapping of recordings. It goes through each unmapped recordings, 
and give a few suggested recordings for the mapping. This is the exact same as mapping recording in the web UI.

```shell
listenbrainz-cli-tools mapping -u <username> -t <user token>
```

## Live statistics

While ListenBrainz have its own statistic page, it only refreshes daily, and is limited to only some entities.
With those commands, you'll be able to see your statistics in no time!

```shell
listenbrainz-cli-tools stats -u <username> -t <target>
```

Target is the entity type to group the stats by. Currently, those entities stats are implemented:

- Recordings (`recording`)
- Artists (`artist`)
- Releases (`release`)
- Release Groups (`release_group`)
- Works (`work`)

## Radio

### Artist Circles

This algorithm keep your playlist close to the artists you are listening to. The way it generate is as follow:

- Get a random listen
- Get its artist
- Add a random recording made by this artist

There is the option to only get unlistened recordings, making an alternative to ListenBrainz's own discovery playlists.

Usage:

```shell
listenbrainz-cli-tools radio circles -u <username> -t <token>
```

Only unlistened:

```shell
listenbrainz-cli-tools radio circles -u <username> -t <token> --unlistened
```

### Underrated tracks

This radio will create a playlist containing all the tracks that you listen to, but seemingly no one else does. 

Usage:
```shell
listenbrainz-cli-tools radio underrated -u <username> -t <token>
```

> The mix is made by calculating a score for each listen. This score is composed of two values:
>
> - The rank in the user's top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)
>
> - The percentage of the recording's listens being from the user (Made with this formula: (user listens / worldwide listens) *100)

### Listen rate

This algorythm bases itself on your listen rate of recording to get more forgotten tracks. It takes the recordings with the lowest listen rates, and put them into a playlist

Usage:
```shell
listenbrainz-cli-tools radio rate -u <username> -t <token> --min-rate=<Number of listens...> --min-per=<... Per range> --min=<Minimum listens (Default: 3)>
```

Exemple usage:
```shell
listenbrainz-cli-tools radio rate -u <username> -t <token> --min-rate=3 --min-per=year --min=10
```

### Overdue listens
Similar to listen rates, this algorithm calculate the average time between listens, and estimate when the next listen will happen. 
It thens put together a playlist made out of recordings you should have listened by now.

Usage:
```shell
listenbrainz-cli-tools radio rate -u <username> -t <token> --min=<Minimum listens (Default: 3)>
```

Another mode is the "Overdue factor". Instead of sorting by date, the listens are sorted by how many estimated listens should have happened by now (Time elapsed since last listen / Average time per listens)

Usage:
```shell
listenbrainz-cli-tools radio rate -u <username> -t <token> --min=<Minimum listens (Default: 3)> -o
```

# Other infos

This project is in beta. There's a lot of features I like to add, and need a lot of testing before 1.0. If you find a bug, or have a feature request, feel free to create a new issue.
