# acer-predator-rgb

A Rust terminal app for Acer Nitro and Predator keyboard lighting on Linux.
Prebuilt releases run **without Rust, Cargo, or Python installed**.

## Install

For **x86_64 Linux** (Intel/AMD), install the latest release with:

```bash
curl -fsSL https://github.com/Rabin18Glan/acer-predator-rgb/releases/latest/download/install.sh | bash
```

Then launch:

```bash
acer-predator-rgb
```

The installer downloads a portable, statically linked binary, verifies its
SHA-256 checksum, and installs it in `~/.local/bin`. No compiler or sudo is needed.
It requires Bash, curl, tar, and standard Linux coreutils (including sha256sum).
If the command is not found, add the directory to your shell's startup file:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Or launch directly with `~/.local/bin/acer-predator-rgb`.

**Driver requirement:** lighting control still needs the separately installed
[`predator_kbd_rgb` driver](https://github.com/TomasUribe/predator-rgb), exposing
`/dev/predator-kbd`. Rust and Python are not needed to run our app; the driver
itself may require DKMS, kernel headers, and C build tools to install. This
installer does not install, unload, or change kernel drivers or startup services.
Your Nitro AN515-45 has been tested; other models require driver compatibility.

The download command becomes available **after the first GitHub Release is
published**. Re-run it to update. For a specific release or custom directory:

```bash
curl -fsSL https://github.com/Rabin18Glan/acer-predator-rgb/releases/download/v1.0.0/install.sh | VERSION=v1.0.0 bash
# From a local checkout:
INSTALL_DIR="$HOME/bin" bash install.sh
```

To inspect the installer first, download `install.sh` from
[Releases](https://github.com/Rabin18Glan/acer-predator-rgb/releases), read it, and
run `bash install.sh`. Checksums detect download corruption, not publisher identity.

To uninstall just this app:

```bash
rm -- "$HOME/.local/bin/acer-predator-rgb"
```

This leaves the working keyboard driver and saved colors intact.

## Controls

A Rust / ratatui client for the **working TomasUribe/predator-rgb driver**.
Uses `/dev/predator-kbd`; does not load, unload, patch, or install kernel modules.
Does not use facer devices, raw EC writes, or direct ACPI calls.

Four tabs: **1 Color selection**, **2 Speed**, **3 Direction**, **4 Effect**.
Use Tab / Shift+Tab or 1–4 to switch, or click a tab. ↑↓ selects a color field
or an effect; ←→ adjusts values (Shift for larger steps). Changes apply live
after a 40 ms debounce; no Enter is needed. Navigation alone sends no commands.

The keyboard preview shows labeled keys, modifiers, arrows and a numpad with
per-zone colors and simulated animation. It is an illustration, not LED readback.
Click a key or zone label to target one static zone; the Target field cycles
back to All zones. **Shift+1 / Shift+2 / Shift+3 / Shift+4** selects a zone
and opens Color selection; plain 1–4 still switches tabs. Standard terminals
may send these as `!`, `@`, `#`, `$`, which are also supported. Selecting a zone
alone does not send lighting commands or change the current effect.
Click palette colors and sliders, or use R/G/B/P/W/O/C for
presets (uppercase R is red; lowercase r checks driver status). h accepts six
hex digits and applies automatically when complete. Esc cancels hex input.
Speed/direction explain when static mode makes them inapplicable.

q / Ctrl+C exits; r refreshes driver status. Starting does not write to hardware.
Use a terminal at least 76 × 24 cells. Mouse controls require terminal mouse support.

Static, breathing, neon, wave, shifting and zoom use the working Python client's
command order. Static sends mode first, then each of four zones separately.
Firmware status is checked after every command, stopping on a reported error.
Changes can be partial if an intermediate command fails. Avoid operating other
RGB apps at the same time: the driver's status is global and commands from two
clients can interleave. Status 0 is the driver's interpretation of firmware
acceptance; it does not physically verify the LEDs.

## Settings

s saves to `$XDG_CONFIG_HOME/acer-predator-rgb/settings.json` (normally
`~/.config/acer-predator-rgb/settings.json`); l loads and applies changed settings live.
If no Rust settings exist, the app reads the Python client's
`predator-rgb/settings.json` from the same configuration root. It does not modify
Python settings, autostart, or services. No login-time apply is configured.
The existing driver's resume behavior and any Python login restore remain active.

```sh
acer-predator-rgb --status   # read only
acer-predator-rgb --dry-run  # inspect commands without changing lighting
acer-predator-rgb --apply    # explicitly apply saved/imported settings
```

Run as your normal user. The installed driver normally grants access to the device.
Missing devices, permissions, and driver rejection appear in the UI status panel.
Hardware control requires the separately installed `predator_kbd_rgb` driver;
this repository is only a userspace client.

## Build from source (developers only)

Only developers need Rust. From this Rust repository (not the Python repo):

```bash
git clone https://github.com/Rabin18Glan/acer-predator-rgb.git
cd acer-predator-rgb
cargo run --locked
```

Verification:

```sh
cargo fmt --all --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
bash tests/install.sh target/debug/acer-predator-rgb
```

## Publish a release

GitHub Actions runs CI on pull requests and pushes to `main` or `master`.
Pushing a version tag such as **v1.0.0** builds and publishes the Linux binary,
`install.sh`, and `SHA256SUMS` as GitHub Release assets. The tag must match the
package version in both `Cargo.toml` and `Cargo.lock` (prepared as 1.0.0).

Commit and push these files first, then:

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

Follow the **Release** workflow in the GitHub Actions tab. Tests, formatting,
Clippy, static-link validation, and offline installer tests must pass before
publication. It uses the built-in `GITHUB_TOKEN`; no personal token is needed.
Publishing requires Actions to be allowed to write repository contents.
No tag or release is created merely by running the local installer.

For the next release, update the Cargo package version, run `cargo check` to
update the lockfile, commit both files, and push the matching new tag.

Protocol reference: https://github.com/TomasUribe/predator-rgb
