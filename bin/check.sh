#!/bin/bash

set -uo pipefail

check() {
    cargo run --quiet --manifest-path=tests/check/Cargo.toml --bin check $@
}

echo "Building docs"
cargo doc --quiet --package gdvariants --no-deps

mkdir -p ./tmp
check list-sources | while read struct ; do
    echo "Comparing ${struct}"
    check implements ${struct}
done
