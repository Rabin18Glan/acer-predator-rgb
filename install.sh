#!/usr/bin/env bash
# Install a prebuilt userspace client. Never modifies kernel drivers or services.
set -euo pipefail

fail() { printf 'ERROR: %s\n' "$*" >&2; exit 1; }
if [[ ${1:-} == --help ]]; then
    echo 'Install the latest Linux x86_64 binary without Rust or Cargo.'
    echo 'Options (environment): VERSION=v1.0.0 INSTALL_DIR=/absolute/bin/path'
    echo 'Default destination: ~/.local/bin/acer-predator-rgb'
    exit 0
fi
[[ $# == 0 ]] || fail 'Use --help for installer options.'
for command in curl tar sha256sum awk install mktemp uname; do
    command -v "$command" >/dev/null || fail "Required command missing: $command"
done
[[ $(uname -s) == Linux ]] || fail 'Only Linux is supported.'
case $(uname -m) in
    x86_64|amd64) target=x86_64-unknown-linux-musl ;;
    *) fail 'Prebuilt releases currently support x86_64 Linux only.' ;;
esac

version=${VERSION:-latest}
[[ $version == latest || $version =~ ^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] || fail 'VERSION must be latest or vMAJOR.MINOR.PATCH.'
if (( EUID == 0 )) && [[ -z ${INSTALL_DIR:-} ]]; then
    fail 'Run without sudo for a user install, or explicitly set INSTALL_DIR for a system install.'
fi
install_dir=${INSTALL_DIR:-"${HOME:?HOME is required}/.local/bin"}
[[ $install_dir == /* ]] || fail 'INSTALL_DIR must be an absolute path.'
repo=https://github.com/Rabin18Glan/acer-predator-rgb
if [[ $version == latest ]]; then
    base="$repo/releases/latest/download"
else
    base="$repo/releases/download/$version"
fi
archive="acer-predator-rgb-$target.tar.gz"
temp_dir=$(mktemp -d)
staged=''
cleanup() {
    rm -rf -- "$temp_dir"
    if [[ -n $staged ]]; then rm -f -- "$staged"; fi
}
trap cleanup EXIT
download() {
    curl --fail --silent --show-error --location --proto '=https' --tlsv1.2 \
        --retry 3 --connect-timeout 15 --max-time 180 "$1" --output "$2"
}

printf 'Downloading %s release for %s…\n' "$version" "$target"
download "$base/SHA256SUMS" "$temp_dir/SHA256SUMS" || fail "No release checksums available. See $repo/releases (the first release must be published first)."
download "$base/$archive" "$temp_dir/$archive" || fail 'Could not download release binary.'
awk -v name="$archive" '$2 == name { print }' "$temp_dir/SHA256SUMS" > "$temp_dir/verify.sha256"
[[ $(awk 'END { print NR }' "$temp_dir/verify.sha256") == 1 ]] || fail 'Release checksum is missing or ambiguous.'
(cd "$temp_dir" && sha256sum --check --status verify.sha256) || fail 'Checksum verification failed; nothing was installed.'
tar --extract --gzip --file "$temp_dir/$archive" --directory "$temp_dir" \
    --no-same-owner --no-same-permissions -- acer-predator-rgb
[[ -f $temp_dir/acer-predator-rgb && ! -L $temp_dir/acer-predator-rgb ]] || fail 'Release does not contain a regular binary.'
mkdir -p -- "$install_dir"
[[ ! -d $install_dir/acer-predator-rgb ]] || fail 'Installation destination is a directory.'
staged=$(mktemp "$install_dir/.acer-predator-rgb.XXXXXX")
install -m 0755 -- "$temp_dir/acer-predator-rgb" "$staged"
mv -fT -- "$staged" "$install_dir/acer-predator-rgb"
staged=''
printf 'Installed: %s/acer-predator-rgb\n' "$install_dir"
case :${PATH:-}: in
    *:"$install_dir":*) echo 'Run: acer-predator-rgb' ;;
    *) printf 'Add this directory to PATH: %s\nOr run: %s/acer-predator-rgb\n' "$install_dir" "$install_dir" ;;
esac
if [[ ! -c /dev/predator-kbd ]]; then
    echo 'Keyboard driver not detected at /dev/predator-kbd. The UI is installed, but lighting control needs the separately installed predator_kbd_rgb driver.'
fi
echo 'No Rust installation, kernel module changes, or startup services are required by this installer.'
