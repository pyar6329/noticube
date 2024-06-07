#!/bin/bash

set -e

CURRENT_DIR=$(echo $(cd $(dirname $0) && pwd))
PROJECT_ROOT="${CURRENT_DIR}/.."

REGISTRY_URL="ghcr.io/pyar6329"
RUST_VERSION=$(cat ${PROJECT_ROOT}/rust-toolchain | tr -d '\n')

# set git hash
tag_name="dev-$(git rev-parse HEAD)"
repo_name="$REGISTRY_URL/noticube"
image_name="$repo_name:$tag_name"

docker build \
  -t "$image_name" \
  -f $CURRENT_DIR/Dockerfile \
  --build-arg RUST_VERSION=${RUST_VERSION} \
  $PROJECT_ROOT

docker run --rm -it -d --name tmp_noticube -e "SLACK_BOT_TOKEN=foo" -e "SLACK_CHANNEL_ID=bar" ${image_name}

docker cp tmp_noticube:/usr/local/bin/noticube ${PROJECT_ROOT}/target/noticube
docker rm -f tmp_noticube

docker push $image_name
