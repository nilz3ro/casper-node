#!/usr/bin/env bash

set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." >/dev/null 2>&1 && pwd)"
NODE_BUILD_TARGET="$ROOT_DIR/target/release/casper-node"
UPGRADE_DIR="$ROOT_DIR/target/upgrade_build"
BIN_DIR="$UPGRADE_DIR/bin"
CONFIG_DIR="$UPGRADE_DIR/config"
NODE_BUILD_DIR="$ROOT_DIR/node"
GENESIS_FILES_DIR="$ROOT_DIR/resources/production"

check_python_has_toml() {
    set +e
    python3 -c "import toml" 2>/dev/null
    if [[ $? -ne 0 ]]; then
        printf "Ensure you have 'toml' installed for Python3\n"
        printf "e.g. run\n"
        printf "    pip3 install toml --user\n\n"
        exit 3
    fi
    set -e
}

protocol_version() {
    PROTOCOL_VERSION=$(cat "$GENESIS_FILES_DIR/chainspec.toml" | python3 -c "import sys, toml; print(toml.load(sys.stdin)['protocol']['version'].replace('.','_'))")
    printf "Protocol version: %s\n" $PROTOCOL_VERSION
}

build_casper_node() {
    echo "Building casper-node"
    cd "$NODE_BUILD_DIR"
    cargo build --release
}

make_bin_readme() {
    mkdir -p "$BIN_DIR"
    echo "Generating bin README.md"
    local readme="$BIN_DIR/README.md"
    {
      echo "Build for Ubuntu 18.04."
      echo ""
      echo "To run on other platforms, build from https://github.com/CasperLabs/casper-node"
      echo " cd node"
      echo " cargo build --release"
      echo ""
      echo "git commit hash: $(git rev-parse HEAD)"
    } > "$readme"
}

package_bin_tar_gz() {
    printf "Packaging ${PROTOCOL_VERSION}_bin.tar.gz"
    mkdir -p "$BIN_DIR"
    cp "$NODE_BUILD_TARGET" "$BIN_DIR"
    # To get no path in tar, need to cd in.
    cd "$BIN_DIR"
    tar -czvf "../${PROTOCOL_VERSION}_bin.tar.gz" .
    cd ..
    rm -rf "$BIN_DIR"
}

package_config_tar_gz() {
    printf "Packaging ${PROTOCOL_VERSION}_config.tar.gz"
    mkdir -p "$CONFIG_DIR"
    cp "$GENESIS_FILES_DIR/chainspec.toml" "$CONFIG_DIR"
    cp "$GENESIS_FILES_DIR/config-example.toml" "$CONFIG_DIR"
    cp "$GENESIS_FILES_DIR/accounts.csv" "$CONFIG_DIR"
    # To get no path in tar, need to cd in.
    cd "$CONFIG_DIR"
    tar -czvf "../${PROTOCOL_VERSION}_config.tar.gz" .
    cd ..
    rm -rf "$CONFIG_DIR"
}

check_python_has_toml
protocol_version

build_casper_node
make_bin_readme
package_bin_tar_gz
package_config_tar_gz


