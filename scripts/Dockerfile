ARG RUST_VERSION
# docker build用のOS
FROM ghcr.io/dreamstock/rust-musl-builder/rust-musl-builder:${RUST_VERSION}-llvm-cov as build-base

ARG PROJECT_ROOT="/app"
ARG GRPC_HEALTH_PROBE_VERSION="0.4.14"
ARG CARGO_CHEF_VERSION="0.1.50"
ARG PROTOC_VERSION

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

RUN set -x && \
  git config --global --add credential.helper store && \
  git config --global --add url."https://github.com/".insteadOf "ssh://git@github.com/" && \
  git config --global --add url."https://github.com/".insteadOf "git@github.com:"

RUN set -x && \
  apt-get update && \
  apt-get install -y unzip

RUN set -x && \
  curl -sL -o protoc.zip "https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip" && \
  unzip protoc.zip && \
  chmod +x bin/protoc && \
  sudo mv bin/protoc /usr/local/bin/protoc && \
  rm -rf bin include protoc.zip readme.txt

# cargo-chefのインストール
RUN set -x && \
  curl -sL -o cargo-chef.tar.gz "https://github.com/LukeMathWalker/cargo-chef/releases/download/v${CARGO_CHEF_VERSION}/cargo-chef-x86_64-unknown-linux-musl.tar.gz" && \
  tar -zxvf cargo-chef.tar.gz && \
  mv cargo-chef /usr/local/bin/cargo-chef && \
  chmod +x /usr/local/bin/cargo-chef && \
  rm -rf cargo-chef.tar.gz

# Cargo.tomlからcargo-chef用のファイルを作成する
COPY Cargo.toml Cargo.lock $PROJECT_ROOT/
COPY src/lib.rs src/main.rs $PROJECT_ROOT/src/
RUN set -x && \
  cargo-chef chef prepare --recipe-path recipe.json

# ライブラリをインストールしてキャッシュする
RUN --mount=type=secret,id=git-credentials,target=/root/.git-credentials \
  set -x && \
  rm -rf Cargo.toml Cargo.lock src && \
  cargo-chef chef cook --release --recipe-path recipe.json

COPY . $PROJECT_ROOT

RUN --mount=type=secret,id=git-credentials,target=/root/.git-credentials \
  set -x && \
  cargo build --locked --release --target x86_64-unknown-linux-musl && \
  mv $PROJECT_ROOT/target/x86_64-unknown-linux-musl/release/noticube /usr/local/bin/noticube && \
  chmod +x /usr/local/bin/noticube

RUN set -x && \
  curl -sL -o /usr/local/bin/grpc_health_probe "https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/v${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64" && \
  chmod +x /usr/local/bin/grpc_health_probe

# 実行環境の作成
# FROM alpine:3.19
FROM gcr.io/distroless/static-debian12:nonroot

# ARG USER_ID=1000
# ARG USER_NAME=dao
# ARG HOME_DIR="/home/noticube"
# ARG PROJECT_ROOT="/app"

COPY --from=build-base /usr/local/bin/noticube /usr/local/bin/noticube
COPY --from=build-base /usr/local/bin/grpc_health_probe /usr/local/bin/grpc_health_probe

# RUN set -x && \
#   apk add --no-cache bash && \
#   addgroup -g ${USER_ID} -S ${USER_NAME} && \
#   adduser -u ${USER_ID} -S -D -G ${USER_NAME} -H -h ${HOME_DIR} -s /bin/bash ${USER_NAME} && \
#   mkdir -p ${PROJECT_ROOT} && \
#   chown -R ${USER_NAME}:${USER_NAME} ${PROJECT_ROOT}

# WORKDIR $PROJECT_ROOT
# USER $USER_NAME
CMD ["/usr/local/bin/noticube"]
