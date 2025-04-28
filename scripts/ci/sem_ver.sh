#!/bin/bash
#members=("tuillez" "musicbrainz_db_lite" "alistral_core" "interzic" "symphonize" "alistral_cli")
members=()

mv Cargo.lock ./Cargo.lock.bk
mv Cargo.toml ./Cargo.toml.bk

for member in "${members[@]}"; do
    cd "./${member}"

    echo ""
    echo "Verifying ${member}"
    echo ""

    cargo semver-checks
    if [ $? -ne 0 ] 
    then
        exit 1
    fi
    cd ../
done

exit 0