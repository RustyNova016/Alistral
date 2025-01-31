# Contributing

# How to initialize the project

You first need to set your environment file. In the terminal:

```shell
export SQLX_OFFLINE=true
cargo run cache init-database 
```

This will set the app in "offline" mode, AKA without a database.
The second line will let Alistral initialize the database.

You can then create a new `.env` file in the root of the project:

```dotenv
DATABASE_URL=sqlite:<Full path to your debug file>
```

On linux, it should be at `~/.cache/alistral/debug/debug_db.db`.

Close the previous terminal, reopen it, and it will compile against the debug database

## Why is `develop` the default branch?

This project uses [gitflow](https://www.gitkraken.com/learn/git/git-flow) as a project management, but github makes it difficult to work with. 
To force github to use develop as the main changes branch, `develop` as been set as default. The `master` branch still exist, and contain "clean" release code, which is able to be built from source without unreleased and potentially broken code

# Pull requests
### Which branch should I PR onto?

This project uses [gitflow](https://www.gitkraken.com/learn/git/git-flow) as a project management. In short, PR onto `develop`.

### What commit message should I use?

Anything you want, as long as it is descriptive. But consider using [Conventional Commits](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13) if you want to
be mentioned in the changelogs.

## Deal with CI Fails

### Clippy

All pull requests must pass without any clippy warnings or errors before merging. If you are unsure how to fix an issue, please tell it in the PR message so we can help

### FMT

All code must be passed through `cargo fmt` before the PR. It is generally better to merge the formatting commit with the code, but reformating commits are allowed (Use `style: cargo fmt` for the commit message if needed)

### MSRV

All code must pass the MSRV set. If the MSRV needs a bump, feel free to do it.

Hint:
- Check your new MSRV with [cargo-msrv](https://github.com/foresterre/cargo-msrv): `cargo msrv find`