SCRIPT_DIR=$(echo $(cd $(dirname $0) && pwd))

PROJECT_ROOT="${SCRIPT_DIR}/.."

if ! type cargo-docset > /dev/null 2>&1; then
  cargo install cargo-docset
fi

cd ${PROJECT_ROOT}

if [ -e "${PROJECT_ROOT}/Cargo.toml" ]; then
  docset_file_name=$(cat "${PROJECT_ROOT}/Cargo.toml" | grep -e '^name =' | awk '{print $3}' | tr -d '"' | tr -d '\n')
  cargo-docset docset --all-features

  OS_NAME=$(uname -s)
  if [ "${OS_NAME}" = "Darwin" ]; then
    open "${PROJECT_ROOT}/target/docset/${docset_file_name}.docset"
  fi
else
  echo "This directory does not contain a Cargo.toml file, so docset generation is not possible."
fi
