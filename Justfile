login:
    cargo login

dry-run:
    cargo publish --dry-run

publish:
    cargo publish

cover:
    #!/bin/bash
    rm -rf ./coverage
    docker build . -t gdvariants-coverage
    docker cp $(docker create --rm gdvariants-coverage):/app/target/debug/coverage ./coverage

doc:
    cargo doc --package gdvariants --no-deps --open

check *FLAGS:
    cargo doc --quiet --package gdvariants --no-deps
    cargo run --quiet --manifest-path=tests/check/Cargo.toml --bin check {{FLAGS}}
