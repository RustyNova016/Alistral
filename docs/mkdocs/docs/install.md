# Install

Most platforms have builds on the [release](https://github.com/RustyNova016/alistral/releases) page. Choose the correct executable for your platform and download it

## Nix 

An official package is available in Nixpkgs (May be slow to update as I often forget and the update PR bot is slow)

https://search.nixos.org/packages?channel=unstable&query=alistral#show=alistral

The repository's flake is discouraged due to issues with unpublished crates hashes


## Build it yourself

You can install rust on your machine by using the simple [rustup](https://rustup.rs/) utility

Alistral target the latest version, so if you suddenly cannot compile it, you can run `rustup update` to update your rust installation

To compile alistral, you can download the source using:

```bash
git clone https://github.com/RustyNova016/alistral.git
cd ./alistral

# Get the latest stable release (ignore this line for the beta builds)
git checkout $(git describe --tags $(git rev-list --tags --max-count=1))
```

And to build it you can run:

```bash
export SQLX_OFFLINE=true
cargo build --features full --release
```
*Note: Prefer `--features full` over `--all-features` as it removes debug code*

The compiled binary should be at `./.target/release/alistral`

## Unofficial builds:

You can find community made packages on repology. However always prefer addressing bugs on the packager's bug tracker instead, as it may not be a bug that I caused:

[![Packaging status](https://repology.org/badge/vertical-allrepos/alistral.svg)](https://repology.org/project/alistral/versions)
