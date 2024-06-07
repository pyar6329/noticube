ARG RUST_VERSION
# build image
FROM ghcr.io/dreamstock/rust-musl-builder/rust-musl-builder:${RUST_VERSION}-llvm-cov as build-base

ARG PROJECT_ROOT="/app"
ARG CARGO_CHEF_VERSION="0.1.50"

ENV LANG="C.UTF-8" \
    LC_ALL="C.UTF-8" \
    DEBIAN_FRONTEND="noninteractive" \
    CARGO="/opt/rust/cargo/bin/cargo" \
    CARGO_INCREMENTAL="0" \
    CARGO_NET_RETRY="10" \
    CARGO_HOME="/opt/rust/cargo" \
    RUSTFLAGS="-D warnings -C debuginfo=0" \
    RUST_BACKTRACE="1"

USER root

COPY rust-toolchain $PROJECT_ROOT/rust-toolchain
WORKDIR $PROJECT_ROOT

# install cargo-chef
RUN set -x && \
  curl -sL -o cargo-chef.tar.gz "https://github.com/LukeMathWalker/cargo-chef/releases/download/v${CARGO_CHEF_VERSION}/cargo-chef-x86_64-unknown-linux-musl.tar.gz" && \
  tar -zxvf cargo-chef.tar.gz && \
  mv cargo-chef /usr/local/bin/cargo-chef && \
  chmod +x /usr/local/bin/cargo-chef && \
  rm -rf cargo-chef.tar.gz

# create cargo-chef recipe
COPY Cargo.toml Cargo.lock $PROJECT_ROOT/
COPY src/lib.rs src/main.rs $PROJECT_ROOT/src/
RUN set -x && \
  cargo-chef chef prepare --recipe-path recipe.json

# library caching
RUN set -x && \
  rm -rf Cargo.toml Cargo.lock src && \
  cargo-chef chef cook --release --recipe-path recipe.json

COPY . $PROJECT_ROOT

RUN --mount=type=secret,id=git-credentials,target=/root/.git-credentials \
  set -x && \
  cargo build --locked --release --target x86_64-unknown-linux-musl && \
  mv $PROJECT_ROOT/target/x86_64-unknown-linux-musl/release/noticube /usr/local/bin/noticube && \
  chmod +x /usr/local/bin/noticube

# runtime image
FROM gcr.io/distroless/static-debian12:nonroot

COPY --from=build-base /usr/local/bin/noticube /usr/local/bin/noticube

CMD ["/usr/local/bin/noticube"]
