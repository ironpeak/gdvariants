FROM rust

ENV RUSTFLAGS="-Cinstrument-coverage"
ENV LLVM_PROFILE_FILE="gdvariants-%p-%m.profraw"

WORKDIR /app

RUN apt update && \
    apt install -y --no-install-recommends clang && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install grcov
RUN rustup component add llvm-tools-preview

COPY Cargo.toml Cargo.toml
RUN mkdir src && \
    touch src/lib.rs && \
    cargo build && \
    rm -rf src
COPY src src

RUN cargo build --verbose
RUN cargo test --verbose --tests -v

RUN grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
