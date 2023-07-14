# This dockerfile is currently single-arch: x86_64
# See:
# - installation of cargo-binstall
# - installation of cargo udeps
# - installation of zig

# We want quick installs using cargo binstall, but that regime breaks in the following cases...

# using rust:1.70-bookworm breaks cargo-udeps:
# error while loading shared libraries: libssl.so.1.1: cannot open shared object file: No such file or directory

# using rust:1.70-bullseye breaks cargo-spellcheck:
# /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by /usr/local/cargo/bin/cargo-spellcheck)

# take the later glibc version in bookworm, and install the musl build of cargo-udeps

FROM rust:1.70-bookworm


RUN curl --silent --show-error --location \
    https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    | tar --extract --verbose --ungzip --directory=/usr/local/bin

RUN cargo binstall --no-confirm \
    cargo-nextest \
    cargo-spellcheck \
    cargo-zigbuild \
    taplo-cli

RUN curl --silent --show-error --location \
    https://github.com/est31/cargo-udeps/releases/download/v0.1.40/cargo-udeps-v0.1.40-x86_64-unknown-linux-musl.tar.gz \
    | tar --extract --verbose --ungzip --strip-components=2 --directory=/usr/local/bin \
    # only extract this file. Note the leading '.' counts as a component, hence 2 components
    ./cargo-udeps-v0.1.40-x86_64-unknown-linux-musl/cargo-udeps

RUN : \
    && cargo nextest --version \
    && cargo spellcheck --version \
    && cargo udeps --version \
    # always one, isn't there...
    && cargo zigbuild -h \
    && taplo --version


RUN rustup target add \
    aarch64-apple-darwin \
    aarch64-unknown-linux-gnu \
    x86_64-apple-darwin \
    x86_64-unknown-linux-gnu

# Needed for cargo-udeps
RUN rustup toolchain add nightly

RUN rustup component add clippy

ENV PATH=$PATH:/usr/local/zig
RUN mkdir /usr/local/zig && \
    curl --silent --show-error --location \
    https://ziglang.org/download/0.10.1/zig-linux-x86_64-0.10.1.tar.xz \
    | tar --extract --verbose --xz --strip-components=1 --directory=/usr/local/zig \
    && zig version
