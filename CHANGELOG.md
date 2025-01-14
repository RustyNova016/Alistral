# Changelog

All notable changes to this project will be documented in this file.

No changelog for now as git cliff is broken :(

## [0.4.2] - 2024-12-20

### 🚀 Features

- Wrong Mapping finder
- Add lint filter
- *(lint)* Missing_remixer_rel
- Lint severity colors
- *(lint)* Missing_remix_rel
- *(lint)* Soundtrack_without_disambiguation
- Add listening time option on radio overdue
- *(lint)* Suspicious_remix

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

### CI

- Fix release CI

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

### ⚙️ Miscellaneous Tasks

- Remove useless deps and update
- Release
- Update readme

### CI

- Run release workflow on release publishing
- Added release_manual.yml

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

<!-- generated by git-cliff -->
