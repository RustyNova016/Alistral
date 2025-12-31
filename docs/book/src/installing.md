# Installing

## Builds
The latest build can be found in the [release](https://github.com/RustyNova016/alistral/releases) tab

## Manual build

Manual builds require rust > 1.89.0 and openssl

Manually build with:

```shell
git clone https://github.com/RustyNova016/alistral.git
cd ./alistral
# Get the latest stable release (ignore this line for the beta builds)
git checkout $(git describe --tags $(git rev-list --tags --max-count=1))
export SQLX_OFFLINE=true
cargo build --features full --release
```

*Note: Prefer `--features full` over `--all-features` as it removes debug code*

## Packages:

Currently, no **official** package is available outside of github. However,
you can find community made packages on repology:

[![Packaging status](https://repology.org/badge/vertical-allrepos/alistral.svg)](https://repology.org/project/alistral/versions)
