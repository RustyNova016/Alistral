mv Cargo.toml Cargo.toml.disabled
mv Cargo.lock Cargo.lock.disabled

printf "\n=== Checking package \`tuillez\` ===\n"
cd ./tuillez
cargo hack --rust-version minimal-versions check --direct
rm -r target
rm Cargo.lock
cd ../

printf "\n=== Checking package \`musicbrainz_db_lite\` ===\n"
cd ./musicbrainz_db_lite
cargo hack --rust-version minimal-versions check --direct
rm -r target
rm Cargo.lock
cd ../

printf "\n=== Checking package \`alistral_core\` ===\n"
cd ./alistral_core
cargo hack --rust-version minimal-versions check --direct
rm -r target
rm Cargo.lock
cd ../

printf "\n=== Checking package \`interzic\` ===\n"
cd ./interzic
cargo hack --rust-version minimal-versions check --direct
rm -r target
rm Cargo.lock
cd ../

printf "\n=== Checking package \`alistral_cli\` ===\n"
cd ./alistral_cli
cargo hack --rust-version minimal-versions check --direct
rm -r target
rm Cargo.lock
cd ../

mv Cargo.toml.disabled Cargo.toml
mv Cargo.lock.disabled Cargo.lock