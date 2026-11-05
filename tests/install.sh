#!/usr/bin/env bash
# No network, root access, kernel modules, or real user install directories.
set -euo pipefail
binary=$(realpath "${1:?Pass a built binary to test installer}")
project_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
test_dir=$(mktemp -d)
trap 'rm -rf -- "$test_dir"' EXIT
mkdir -p "$test_dir/server" "$test_dir/mock"
install -m 0755 "$project_dir/tests/fixtures/curl" "$test_dir/mock/curl"
install -m 0755 "$project_dir/tests/fixtures/uname" "$test_dir/mock/uname"
cp "$binary" "$test_dir/server/acer-predator-rgb"
archive=acer-predator-rgb-x86_64-unknown-linux-musl.tar.gz
tar -czf "$test_dir/server/$archive" -C "$test_dir/server" acer-predator-rgb
(cd "$test_dir/server" && sha256sum "$archive" > SHA256SUMS)
export TEST_RELEASE_DIR="$test_dir/server" TEST_URL_LOG="$test_dir/urls"
export INSTALL_DIR="$test_dir/install with spaces" PATH="$test_dir/mock:/usr/bin:/bin"

expect_failure() {
    if "$@" > "$test_dir/error.log" 2>&1; then
        echo "Expected failure: $*" >&2
        exit 1
    fi
}

# Works from a pipe and with Rust/Cargo absent from the test PATH.
bash "$project_dir/install.sh" < /dev/null
cmp "$binary" "$INSTALL_DIR/acer-predator-rgb"
[[ -x $INSTALL_DIR/acer-predator-rgb ]]
"$INSTALL_DIR/acer-predator-rgb" --help > "$test_dir/help"
grep -q 'Acer RGB Studio' "$test_dir/help"
grep -q '/releases/latest/download/' "$TEST_URL_LOG"

# Version pinning and streaming installation need no source checkout.
VERSION=v1.0.0 bash < "$project_dir/install.sh"
grep -q '/releases/download/v1.0.0/' "$TEST_URL_LOG"
cmp "$binary" "$INSTALL_DIR/acer-predator-rgb"

expect_failure env VERSION=invalid bash "$project_dir/install.sh"
expect_failure env TEST_ARCH=aarch64 bash "$project_dir/install.sh"
expect_failure env TEST_OS=Darwin bash "$project_dir/install.sh"
expect_failure env TEST_DOWNLOAD_FAIL=1 bash "$project_dir/install.sh"
cmp "$binary" "$INSTALL_DIR/acer-predator-rgb"

# A bad checksum cannot replace an existing installation.
printf 'corrupt\n' >> "$test_dir/server/$archive"
expect_failure bash "$project_dir/install.sh"
grep -q 'Checksum verification failed' "$test_dir/error.log"
cmp "$binary" "$INSTALL_DIR/acer-predator-rgb"

# Missing checksum records fail closed as well.
printf '' > "$test_dir/server/SHA256SUMS"
expect_failure bash "$project_dir/install.sh"
grep -q 'checksum is missing' "$test_dir/error.log"
cmp "$binary" "$INSTALL_DIR/acer-predator-rgb"
echo 'Installer tests passed (offline; no Rust installation or driver operations).'
