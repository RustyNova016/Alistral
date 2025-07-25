#!/bin/bash
members=("tuillez" "musicbrainz_db_lite" "alistral_core" "interzic" "symphonize" "alistral_cli")

mv Cargo.lock ./Cargo.lock.bk
mv Cargo.toml ./Cargo.toml.bk

for member in "${members[@]}"; do
    cd "./${member}"

    echo ""
    echo "Preparing ${member}"
    echo ""

    cargo sqlx prepare
    if [ $? -ne 0 ] 
    then
        exit 1
    fi

    cargo clean
    rm Cargo.lock

    cd "../"
done

mv Cargo.lock.bk ./Cargo.lock
mv Cargo.toml.bk ./Cargo.toml

exit 0