#!/bin/bash

# required environment variables
# - GITHUB_TOKEN: GitHub's access token
# - AWS_DEFAULT_REGION: AWS region
# - AWS_ACCESS_KEY_ID: AWS access key
# - AWS_SECRET_ACCESS_KEY: AWS secret key
# - DOCKER_BUILDKIT=1 : use buildkit
# - PROTOC_VERSION: protoc version. ref: https://github.com/protocolbuffers/protobuf/releases

set -e

CURRENT_DIR=$(echo $(cd $(dirname $0) && pwd))
PROJECT_ROOT="${CURRENT_DIR}/.."

REGISTRY_URL="ghcr.io/pyar6329"
RUST_VERSION=$(cat ${PROJECT_ROOT}/rust-toolchain | tr -d '\n')

# set git hash
tag_name="dev-$(git rev-parse HEAD)"
repo_name="$REGISTRY_URL/noticube"
image_name="$repo_name:$tag_name"

# create .git-credentials file by $GITHUB_TOKEN
echo "https://x-access-token:${GITHUB_TOKEN}@github.com" > $PROJECT_ROOT/git-credentials.txt

docker build \
  -t "$image_name" \
  -f $CURRENT_DIR/Dockerfile \
  --build-arg RUST_VERSION=${RUST_VERSION} \
  $PROJECT_ROOT

if [ -e $PROJECT_ROOT/git-credentials.txt ]; then
  rm -rf $PROJECT_ROOT/git-credentials.txt
fi

docker run --rm -it -d --name tmp_noticube -e "SLACK_BOT_TOKEN=foo" -e "SLACK_CHANNEL_ID=bar" ${image_name}

docker cp tmp_noticube:/usr/local/bin/noticube ${PROJECT_ROOT}/target/noticube
docker rm -f tmp_noticube

docker push $image_name
