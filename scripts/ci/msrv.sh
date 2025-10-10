#!/bin/bash
members=("musicbrainz_db_lite" "alistral_core" "interzic" "symphonize" "alistral_cli")

mv Cargo.lock ./Cargo.lock.bk
mv Cargo.toml ./Cargo.toml.bk

for member in "${members[@]}"; do
    cd "./${member}"

    echo ""
    echo "Verifying ${member}"
    echo ""

    cargo msrv verify --all-features
    if [ $? -ne 0 ] 
    then
        cargo msrv find --all-features
        exit 1
    fi
    cd ../
done

exit 0