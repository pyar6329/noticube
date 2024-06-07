#!/bin/bash

set -e

CURRENT_DIR=$(echo $(cd $(dirname $0) && pwd))

PROJECT_ROOT="${CURRENT_DIR}/.."

cd "${PROJECT_ROOT}"

# files are removed directly if target is symlink
if [ -L "${PROJECT_ROOT}/target" ]; then
  rm -rf ${PROJECT_ROOT}/target/.* ${PROJECT_ROOT}/target/*
else
  cargo clean
fi
