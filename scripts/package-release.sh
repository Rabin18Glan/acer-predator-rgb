#!/usr/bin/env bash
set -euo pipefail
tag=${1:?Usage: bash scripts/package-release.sh vMAJOR.MINOR.PATCH}
[[ $tag =~ ^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] || { echo 'Invalid release tag' >&2; exit 1; }
version=$(cargo metadata --locked --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "acer-predator-rgb") | .version')
[[ $tag == "v$version" ]] || { echo 'Tag must match Cargo.toml version' >&2; exit 1; }
binary=target/x86_64-unknown-linux-musl/release/acer-predator-rgb
[[ -x $binary ]] || { echo "Build $binary first" >&2; exit 1; }
# A static ELF needs neither Rust nor the host distribution's libc loader.
program_headers=$(readelf -l "$binary")
if grep -q INTERP <<< "$program_headers"; then
    echo 'Refusing to package a dynamically linked executable' >&2
    exit 1
fi
mkdir -p dist
tar -czf dist/acer-predator-rgb-x86_64-unknown-linux-musl.tar.gz \
    -C target/x86_64-unknown-linux-musl/release acer-predator-rgb
install -m 0755 install.sh dist/install.sh
(cd dist && sha256sum acer-predator-rgb-x86_64-unknown-linux-musl.tar.gz install.sh > SHA256SUMS)
