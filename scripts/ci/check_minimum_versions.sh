mv Cargo.toml Cargo.toml.disabled
mv Cargo.lock Cargo.lock.disabled

printf "\n=== Checking package \`tuillez\` ===\n"
cd ./tuillez
cargo +nightly minimal-versions check --direct
cd ../

printf "\n=== Checking package \`musicbrainz_db_lite\` ===\n"
cd ./musicbrainz_db_lite
cargo +nightly minimal-versions check --direct
cd ../

printf "\n=== Checking package \`alistral_core\` ===\n"
cd ./alistral_core
cargo +nightly minimal-versions check --direct
cd ../

printf "\n=== Checking package \`interzic\` ===\n"
cd ./interzic
cargo +nightly minimal-versions check --direct
cd ../

printf "\n=== Checking package \`alistral_cli\` ===\n"
cd ./alistral_cli
cargo +nightly minimal-versions check --direct
cd ../