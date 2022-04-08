login:
    cargo login

dry-run:
    cargo publish --dry-run

publish:
    cargo publish

doc:
    cargo doc --package gdvariants --no-deps --open

check *FLAGS:
    cargo run --manifest-path=tests/check/Cargo.toml --bin check {{FLAGS}}
