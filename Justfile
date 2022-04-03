login:
    cargo login

dry-run:
    cargo publish --dry-run

publish:
    cargo publish

doc:
    cargo doc --package gdvariants --no-deps --open

check *FLAGS:
    #!/bin/bash
    (
        cd tests/doc_check;
        cargo run --bin check {{FLAGS}};
    )
