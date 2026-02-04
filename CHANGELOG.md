# Changelog

All notable changes to this project will be documented in this file.

## [0.6.6] - 2026-02-04

### 🚀 Features

- YIM tables: Listen duration up/down arrows should be swapped
- RUSTSEC-2026-0001: Potential Undefined Behaviors in `Arc<T>`/`Rc<T>` impls of `from_value` on OOM
- Added dev flake

### 🐛 Bug Fixes

- Zip with wrong minimum version

### ⚙️ Miscellaneous Tasks

- Fix building on windows
- Bump dependencies
- Bump deps
- Update sub_deps
- Bump deps
- Update sub deps

## [0.6.5] - 2026-01-05

### 🚀 Features

- Top table crash if there's less than 20 items to display

### 🐛 Bug Fixes

- Build release on features full

### ⚙️ Miscellaneous Tasks

- Release v0.6.5

## [0.6.4] - 2025-12-31

### 🚀 Features

- Show total minutes in yim
- Isolate youtube feature
- Add subsonic support
- Export radio to subsonic
- Playlist export to subsonic
- Add subsonic to interzic commands
- Added listen count column to yim
- Toggle listencount in tops
- Migrate other fields of row
- Fix installation instruction to add the `full` flag

### 🐛 Bug Fixes

- Listen progress bar
- Missing isrc lint not refreshing data
- Discovery Percent seems wrong again
- Missing feature gate
- Fix wording in new release report

### ⚙️ Miscellaneous Tasks

- Format
- Fix clippy lints
- Update listenbrainz_rs
- Release v0.6.4

## [0.6.3] - 2025-12-06

### 🚀 Features

- Add comparisons to tops
- Deleted entities crash fetching

### 🐛 Bug Fixes

- Division by zero in graph

### 💼 Other

- Discoveries having wrong stats

### ⚙️ Miscellaneous Tasks

- Bump hotpath
- Update lb_rs

## [0.6.2] - 2025-12-03

### 🐛 Bug Fixes

- Yim top

### ⚙️ Miscellaneous Tasks

- V0.6.2

## [0.6.1] - 2025-12-03

### 🚀 Features

- Lookup User: Add Duration Listened
- Today in history missing some recordings
- Add `first_release_date` or fetch or check_releases
- Convert release groups stats to new system
- Added releases to listen stats
- Add labels to user data
- Use dyn compatible format with async
- New top printer
- New releases of the year
- Top recordings
- Artist stats
- Added release ghroup stat compiling
- Discoveries page
- Month recap
- Month lookup
- Added previous year to monthly recap
- Label recap
- Added progress bar to YIM
- Add most listened hour

### 🐛 Bug Fixes

- Duplicate recording in recording iter
- Processed clippy lints not accepting urls
- No listen years

### 💼 Other

- Added channels-console
- Implemented new listen fetcher
- Rename function

### 🚜 Refactor

- Update mb_rs on beta
- Moved listen fetching to listenbrainz_rs
- Removed old lb client from mb rs

### ⚙️ Miscellaneous Tasks

- Cargo fmt
- Sqlx prepare
- Ci fixes
- Fix tests
- Remove hotpath features from clippy hack check
- Fix minimal version
- Update LB_RS
- Cleanup the code
- Release v0.6.1

## [0.6.0] - 2025-10-13

### 🚀 Features

- Add cargo-berger file
- Converted artist credit macro to trait
- User lookup
- Use client for holding strategies
- Convert user listen to client strat
- Migrate lookup recording command
- Added `ListenStatisticsData` struct
- Optimize artist fetching
- Run tests with nextest
- New lookup with time period
- Added date to daily
- Put stats under "top"

### 🐛 Bug Fixes

- Artists not set as complete
- Broken tests
- Prevent recalculating recording data in daily
- Backtraces
- Daily report

### 💼 Other

- Converted_recording
- Converted releases
- Moved Medias
- Converted DBClient to new database
- Migrate recording

### 🚜 Refactor

- Moved some cache comands
- Migrate cache command to tool folder
- Moved release substructs to their own folders
- Remove upsert macro
- Remove smart db pool
- Remove mb_db macros
- Remove RowID trait
- Moved database management to sequelles
- Removed macon from alistral_cli
- Removed macon from mb_db
- Moved bump commands
- Moved listens cli to tool folder
- Move interzic cli to tools
- Migrate user lookup to new components
- Remove conn from cli
- Move cleanup to client
- Move tuillez to its own crate

### ⚙️ Miscellaneous Tasks

- Sqlx prepare
- Regenerate help
- Sqlx prepare again
- Sqlx prepare
- Add sequelle ignore
- Update major deps
- Fix ci for caching
- Fix min_version ci
- Release v0.6.0

## [0.5.14] - 2025-09-01

### 🚀 Features

- Added hackari  for faster builds
- Isolated Sambl command
- Additional samble providers
- Remove hakari due to CI breakage
- Add bandcamp to sambl

### 🐛 Bug Fixes

- Clippy lints
- Clippy fixes

### ⚙️ Miscellaneous Tasks

- Remove sequelles
- Bump version

## [0.5.13] - 2025-07-24

### 🚀 Features

- Added track chacking to label_as_artist
- Add tracks to dash_eti
- Label stats
- Split cli crate into features for faster dev compiling
- Create codeql.yml
- Add missing_sambl_release_lint
- Sort input clippy recording

### 🐛 Bug Fixes

- Indicatif yank
- Rename log path for Windows compatibility
- Check if the url is ended
- Remove apple from missing_artist_link as it's flaky
- Refresh releases for missing_recording_link

### ⚙️ Miscellaneous Tasks

- Clippy fixes
- Set exported modules as private to remove unused code
- Mb_rs update
- Remove useless tokio features
- Update mb_rs
- Remove wrong test

## [0.5.12] - 2025-06-23

### 🚀 Features

- Added processed entity count
- Smarter missing barcode
- Add ISRCs to DB
- Missing ISRC lint
- Add logging

### 🐛 Bug Fixes

- Use task to fetch data
- False positive on missing artist links
- Prevent trying to add playlist links to recordings
- Only retrieve active urls from the database
- Rename musicbrainz_rs_nova

### ⚙️ Miscellaneous Tasks

- Inspect missing links on harmony
- Update readme
- Update musicbrainz_rs
- Bump version

## [0.5.11] - 2025-06-13

### 🚀 Features

- Clippy seed from listens
- Missing_recording_link lint
- Missing_artist_link lint
- Label as artist lint

### ⚙️ Miscellaneous Tasks

- 1.87 clippy fixes
- Update zip due to yank
- Update dependancies
- Update dependancies

## [0.5.10] - 2025-05-12

### 🐛 Bug Fixes

- Crash when no env file is found

## [0.5.9] - 2025-05-09

### 🚀 Features

- Add FetchAndSave trait
- Get_or_fetch trait
- Fetch as complete
- Add entity genre relation
- Add entity crawler
- Redirection trait
- Add urls to database schema
- Url bindings
- Added relation fetching trait
- Add tests to `missing_recording_work`
- Added error for messyrecording upsert
- Add
- Added release harmony compatibility check
- Add sequelles crate
- Make recording compilation use the new traits
- Add ManyToManyJoin
- Impl many to many join to smaller joins

### 🐛 Bug Fixes

- Workplace change fixes
- Bump tuillez msrv due to deps

### 🚜 Refactor

- Moved MusicbrainzClippyCommand to tool file
- Move cli in its own folder
- Remove impl_redirections in favour of trait
- Make formater use task to get artist credits
- Rework clippy command
- Remove color eyre

### ⚙️ Miscellaneous Tasks

- Update tuillez's msrv
- Update mb_db's msrv
- Update core's msrv
- Update interzic's msrv
- Update symphonize's msrv
- Update cli's msrv
- Fix ci for new workplace
- Add sqlx prepared queries
- Ci fixes
- Update dependancies
- Add sequelles to ci
- Publish sequelles
- Bump alistral_cli

## [0.5.8] - 2025-04-22

### 🐛 Bug Fixes

- Broken doc build

### 📚 Documentation

- Document daily command

## [0.5.7] - 2025-04-22

### 🚀 Features

- Add tag stats

### 🚜 Refactor

- Speedup recording listen compiling
- Move Musicbrainz utils to the symphonize crate

### ⚙️ Miscellaneous Tasks

- Project cleanup

## [0.5.3] - 2025-03-10

### 🚀 Features

- Show error returned from fetching listens
- Add async_recursion to allow flexible trait bounds
- *(musicbrainz_db_lite)* [**breaking**] Made model formating into a trait
- *(core)* Add `from_listencollection`
- Use strategy struct for listen statistic compilation
- Add duration sort_by

### 🐛 Bug Fixes

- Commit listen transaction
- Lower listenbrainz rate limit
- Artist not fetcehd during prefetch

### 🚜 Refactor

- Change jiff for ligherweight remaintained humantime
- *(alistral_cli)* Apply new formatter
- Use strategy for recursive works
- Remove artist with listens
- Remove release with listens
- Remove work with listens
- Make stats use a generic writer

### ⚙️ Miscellaneous Tasks

- Update cargo
- Release v0.5.6

## [0.5.4] - 2025-03-13

### 🚀 Features

- Put db_lite clients in arcs
- Turn EntityWithListensCollection into ListenCollection
- Use custom pool for DB connections
- Add DBLitePoolResult
- Change interzic's color to turquoize
- Separate Alistral client into AlistralCLIClient
- Upstream error
- Add reload command and tracing

### 🐛 Bug Fixes

- Cargo sqlx prepare
- Fix listen fetching timeouts
- Playlist user overwrite + 404
- Reverse mapping failing with multple mappings
- Reverse mapping user casing

### 💼 Other

- V0.5.3

### 🚜 Refactor

- Move db_ext to the actual crate
- Move entity prints to db_lite
- Move chrono_ext to tuillez
- Moved offline mode to client config
- Move listen fetching to core
- Speedup recording with listen compilation

### ⚙️ Miscellaneous Tasks

- Upgrade dependancies

## [0.5.2] - 2025-02-26

### 🚀 Features

- Add playlist converter
- Reload specific mbid

### 🐛 Bug Fixes

- Reload not overwritting the recordings

### 📚 Documentation

- Add full help to summary
- Add playlist docs

## [0.5.1] - 2025-02-25

### 🚀 Features

- Added Client
- Add interzic
- Add interzic to the client
- Add target to radios
- Set mb url in config
- Add initial tracing
- Moved tracing to tuillez
- Add verbosity flag
- Add common error ui
- Add get-mapping command
- Add reverse mapping search for interzic
- Add `listen submit` command
- Added debug print for youtube search
- Upgrade to 1.85

### 🐛 Bug Fixes

- Optional youtube client
- Mdbook location
- Book location again
- Rename mdbook.md to mdbook.yml
- Mdbook on wrong branch
- Wrong output path
- Wrong url for musicbrainz api
- Missing reverses
- Prevent refreshing listens of other users

### 🚜 Refactor

- Move logger
- Move progress_bars
- Remove moved modules
- Move EntityWithListens
- Move EntityWithListensCollection
- Move work with listens
- Move Recording with listens
- Move artists with listens
- Moved releases with listens
- Move release-group with listens
- Removed best of mc. Will be refactored for 2025
- Move messybrainz with listens
- Move work with recordings
- Move cleanup
- Remove once_cell
- Add static client where needed
- Removed global bar
- Removed pg
- Removed println_cli

### 📚 Documentation

- Created mdbook
- Basic docs
- Interzic setup
- Create read the docs page
- Move to GH pages
- Readme
- Contributing

### 🎨 Styling

- Reformat to 1.85 style

### ⚙️ Miscellaneous Tasks

- Change to run on any "develop" like branch
- Release v0.5.0
- Made github issues for todos
- Merged musicbrainz_db_lite into the repo

## [0.4.5] - 2025-01-14

### 💼 Other

- V0.4.5

## [0.4.3] - 2025-01-13

### 🐛 Bug Fixes

- Reenable export dump import

### 💼 Other

- V0.4.3
- V0.4.4

### 🚜 Refactor

- Create alistral_core subcrate
- Change lint to be workspace wide
- Move listencollection to core

### ⚙️ Miscellaneous Tasks

- Build on ubuntu 22.04

## [0.4.1] - 2024-12-12

### 🚀 Features

- Wrong Mapping finder
- Add lint filter
- *(lint)* Missing_remixer_rel
- Lint severity colors
- *(lint)* Missing_remix_rel
- *(lint)* Soundtrack_without_disambiguation
- Add listening time option on radio overdue
- *(lint)* Suspicious_remix
- *(data_storage)* Implement DataStorage for playlist count management
- Add fresh releases in daily
- Add no_cleanup option
- Shared radio
- Custome listenbrainz url
- Work recursive stats
- Add recording refetcher

### 🐛 Bug Fixes

- Remove unnecessary score calculation and logging in overdue factor sorter

### 🚜 Refactor

- Rename WorkWithListens to WorkWithRecordingListens

### ⚙️ Miscellaneous Tasks

- Release v0.4.1
- Remove actionrs

## [0.4.0] - 2024-12-11

### 🚀 Features

- *(lints)* Add improvisation hint

### 🐛 Bug Fixes

- Overdue line getting offset in lookup
- Remove database path print
- Rename recording time to playtime

## [0.3.3] - 2024-12-03

### 🚀 Features

- Added rank to lookup
- Add time listened stats
- Clippy + MissingWorkLint
- Add musicbrainz clippy command
- Add missing_release_barcode
- Display checked entities
- More display for main entities
- Change formatting to pretty prints

### 🐛 Bug Fixes

- Missing recording prefetching in stats
- Migration

### 🚜 Refactor

- Migrate bumps
- Move connection to top level
- Migrate config
- Migrate cache clear
- Remove MBID from used code
- Underrated radio
- Migrate timeouts
- Migrate playlist stub
- Move interactive_mapper to dev branch
- Removed unused errors
- Remove musicbrainz_rs_nova
- Changed deque order

### 📚 Documentation

- Add CONTRIBUTING.mb

### 🧪 Testing

- Created test fixtures

### ⚙️ Miscellaneous Tasks

- Remove useless dependancies
- Release v0.4.0

## [0.3.2] - 2024-11-27

### 🚀 Features

- Copy release db to debug
- Seed time range for radio
- Add listen msid remapper
- Daily report

### 🐛 Bug Fixes

- Refresh recording on lookup
- Rust 1.83 fixes
- Bump score in lookup
- Comment out tests
- Remove broken badges

### 🚜 Refactor

- Move lookup report to recording_with_listens

### ⚙️ Miscellaneous Tasks

- Remove old migrations
- Macos Build
- Update manual release build
- Bump version
- Update CMD help
- Release v0.3.3

## [0.3.1] - 2024-11-24

### 🚀 Features

- Init database

### 🐛 Bug Fixes

- Remove dotenvy
- Disable load dump

### ⚙️ Miscellaneous Tasks

- Bump version

## [0.3.0] - 2024-11-22

### 🚀 Features

- Convert overdue radio to sqlite
- Convert recording lookup
- Recursive group by
- ReleaseWithListens
- Convert recording statistics
- Convert release statistics
- Use modular radio generation
- Added cleanup for old values
- Work stats
- Release group stats
- Migrate artist stats
- BestOf2024

### 🐛 Bug Fixes

- Clippy fixes
- Clippy Fixes
- Debug assertion only lint
- Create database if not exist
- Release CI not being in OFFLINE mode
- Not updated

### 💼 Other

- Fix release CI

### 🚜 Refactor

- Remove Welds
- Set some deprecations
- Migrated compatibility
- Migrate listen rate radio
- Listen seeder giving RecordingWithListensCollection
- Change collector to only need recordings
- Migrate radio circles
- Radio cleanup
- Migrate unmapped listen list
- Cleanup
- Migrate listen dump import
- Remove old listen importer
- Changed project name

### 🎨 Styling

- Cargo fmt
- Cargo fmt

### 🧪 Testing

- Remove broken tests
- Remove test as way too slow to run

### ⚙️ Miscellaneous Tasks

- Deactivate underrated radio for now
- Update depandencies
- Fix SQLX offline
- Bump version

## [0.2.2] - 2024-10-10

### 🚀 Features

- Shell completion generation
- Use separate folder for debug builds
- Add default username for CLI
- Add bump command
- Add bumps to overdue radio
- Bumps for listen rate radio

### 🐛 Bug Fixes

- 1.80 clippy warnings
- Migrate derive_more to 1.0
- Ignore deleted mbids
- 1.81.0 clippy update
- Remove target prefix on stats command
- Clippy fixes
- MBID parsing
- Clippy fixes
- Properly calculate average time between listens
- 1.80 clippy warnings

### 🧪 Testing

- Added clippy lints

### ⚙️ Miscellaneous Tasks

- Add .env to gitingore
- Release

## [0.2.1] - 2024-07-08

### 🚀 Features

- Compatibility checker
- Colored help
- Added auto unmapped refresh in configuration

### 🐛 Bug Fixes

- Readme cli docs links

### 💼 Other

- Run release workflow on release publishing
- Added release_manual.yml

### ⚙️ Miscellaneous Tasks

- Remove useless deps and update
- Release
- Update readme

## [0.2.0] - 2024-07-03

### 🚀 Features

- Read MBIDs from strings
- Added config file
- Token Saving
- Import listens from dump
- Radio generation configuration
- Recording timeout
- Specialized mbid for primary alias checking
- Associated recording ID to its listens
- Added mapper blacklist
- Added `cache clear` command
- Statistics lookup for recording
- Added markdown document for CLI

### 🐛 Bug Fixes

- Removed old stats structs
- PR Fixes
- Update CI on develop
- Use fully remove for index deletion
- Downgrade triomphe to 0.1.12
- Revert cargo update

### 🚜 Refactor

- Removed Useless modules
- Specfying returned string mapping
- Push cleanup after job
- More work on typestate MBIDs

### ⚙️ Miscellaneous Tasks

- Job dependancies
- Caching fix
- Msrv check
- Remove lazy_static
- Update musicbrainz_rs
- Release

## [0.1.1] - 2024-05-31

### 🐛 Bug Fixes

- Debug assertion using the wrong name

### 🚜 Refactor

- Cleanup Recording Entity
- Cleanup Artist Entity

### ⚙️ Miscellaneous Tasks

- Release
- Release

