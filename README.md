# Acer RGB Studio

A Rust / ratatui client for the **working TomasUribe/predator-rgb driver**.
Uses `/dev/predator-kbd`; does not load, unload, patch, or install kernel modules.
Does not use facer devices, raw EC writes, or direct ACPI calls.

```sh
cargo run
```

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
cargo run -- --status   # read only
cargo run -- --dry-run  # inspect commands without changing lighting
cargo run -- --apply    # explicitly apply saved/imported settings
```

Run as your normal user. The installed driver normally grants access to the device.
Missing devices, permissions, and driver rejection appear in the UI status panel.
Hardware control requires the separately installed `predator_kbd_rgb` driver;
this repository is only a userspace client.

## Verification

```sh
cargo test
cargo clippy --all-targets -- -D warnings
```

Protocol reference: https://github.com/TomasUribe/predator-rgb
# acer-predator-rgb
