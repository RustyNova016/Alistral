cargo run --all-features -- docs config-schema > ./docs/json_schemas/config.schema.json   
echo "$(jq '. += { "$id": "path"}' ./docs/json_schemas/config.schema.json)" > ./docs/json_schemas/config.schema.json   
jsonschema2md -d ./docs/json_schemas/ -o ./docs/mkdocs/docs/schemas/

cargo run --all-features -- --markdown-help > ./docs/mkdocs/docs/CommandLineHelp.md 

zensical build --clean -f ./docs/mkdocs/mkdocs.yml