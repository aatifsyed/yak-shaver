FROM rust:1.70-bullseye
# using rust:1.70-bookworm breaks cargo-udeps:
# error while loading shared libraries: libssl.so.1.1: cannot open shared object file: No such file or directory

RUN curl --silent --show-error --location \
    https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    | tar --extract --verbose --ungzip --directory=/usr/local/bin

RUN cargo binstall --no-confirm \
    cargo-nextest \
    cargo-spellcheck \
    cargo-udeps \
    cargo-zigbuild \
    taplo-cli

RUN rustup target add \
    aarch64-apple-darwin \
    aarch64-unknown-linux-gnu \
    x86_64-apple-darwin \
    x86_64-unknown-linux-gnu

# Needed for cargo-udeps
RUN rustup toolchain add nightly
