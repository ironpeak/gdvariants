#!/bin/bash

set -euo pipefail

check() {
    cargo run --quiet --manifest-path=tests/check/Cargo.toml --bin check $@
}

echo "Building docs"
cargo doc --quiet --package gdvariants --no-deps

mkdir -p ./tmp
check list-sources | while read struct ; do
    echo "Comparing ${struct}"
    check get-api ${struct} "local" | jq --sort-keys . > ./tmp/local.json
    check get-api ${struct} "std" | jq --sort-keys . > ./tmp/std.json

    jq .name ./tmp/local.json
    jq .name ./tmp/std.json
    exit 0
done
