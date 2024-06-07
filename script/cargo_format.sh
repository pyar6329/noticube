#!/bin/bash

set -e

# このdocker_push.sh scriptがあるdirectoryのpath
CURRENT_DIR=$(echo $(cd $(dirname $0) && pwd))

# repositoryのroot
PROJECT_ROOT="${CURRENT_DIR}/.."

# repositoryのrootに移動
cd $PROJECT_ROOT

# clippyが入っていない場合は入れる
if ! $(rustup component list | grep "clippy" | grep "installed" > /dev/null); then
  rustup component add clippy
fi

# rustfmtが入っていない場合は入れる
if ! $(rustup component list | grep "rustfmt" | grep "installed" > /dev/null); then
  rustup component add rustfmt
fi

echo "run: cargo clippy --no-deps --fix --allow-dirty"

# clippyで上書き保存する (このリポジトリの対象のファイルをGit Commitしないとエラーになる)
cargo clippy --no-deps --fix --allow-dirty

echo "run: cargo fmt"

# formatする
cargo fmt
