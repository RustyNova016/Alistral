cd ./musicbrainz_db_lite
cargo sqlx prepare
cd ../

cd ./alistral_core
cargo sqlx prepare
cd ../

cd ./interzic
cargo sqlx prepare
cd ../

cd ./alistral_cli
cargo sqlx prepare
cd ../