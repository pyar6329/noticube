#!/bin/bash

set -e

if ! type swaks > /dev/null 2>&1; then
  echo "swaks is not installed yet. Installing swaks..."
  if type brew > /dev/null 2>&1; then
    brew install swaks
  elif type pacman > /dev/null 2>&1; then
    sudo pacman -S swaks
  elif type apt-get > /dev/null 2>&1; then
    sudo apt-get install -y swaks
  else
    echo "This OS is not supported. Please install swaks manually."
    exit 1
  fi
fi

CONTENT=$(cat << EOF
hello world!!!!
あいうえお
漢字
カタカナ
😁
EOF
)

swaks \
  --server "localhost" \
  --port "${NOTICUBE_IP:-2525}" \
  --from "sender@example.com" \
  --to "receiver@example.com" \
  --body "${CONTENT}"
