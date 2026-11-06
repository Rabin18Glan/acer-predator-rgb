# Source guide

`src/main.rs` only declares modules and reports startup errors.

| Module | Responsibility |
| --- | --- |
| `cli/mod.rs` | Read arguments and invoke the selected command |
| `cli/command.rs` | Define, parse, and execute the command enum |
| `terminal.rs` | Own terminal setup, event loop, and cleanup through `Drop` |
| `app/mod.rs` | Define application state and expose application types |
| `app/state.rs` | Load/save settings and refresh connection state |
| `app/editing.rs` | Change lighting settings and handle typed actions |
| `app/input.rs` | Handle keys and hex input |
| `app/shortcuts.rs` | Decode terminal representations of zone shortcuts |
| `app/live.rs` | Schedule and apply debounced hardware requests |
| `ui/mod.rs` | Declare UI parts and export `AppUi` |
| `ui/app_ui.rs` | Assemble the complete screen using component render methods |
| `ui/layout.rs` | Calculate named screen regions |
| `ui/header.rs`, `ui/tabs.rs` | Render the header and tab navigation |
| `ui/color.rs`, `ui/palette.rs` | Render color sliders and palette swatches |
| `ui/speed.rs`, `ui/direction.rs`, `ui/effect.rs` | Render each remaining tab separately |
| `ui/status.rs`, `ui/footer.rs`, `ui/resize_notice.rs` | Render status, shortcuts, and the resize prompt |
| `ui/preview.rs`, `ui/theme.rs` | Draw the keyboard and share visual styles |
| `ui/hit.rs` | Convert click coordinates into application actions |
| `model/` | Define colors/effects/settings and encode driver commands |
| `device/` | Validate the device, read status, and send commands |
| `keyboard/` | Describe key geometry and simulate preview colors |
| `config.rs` | Read/write configuration files |

The `mod.rs` files declare submodules and export the small set of types used
elsewhere. Implementation details stay in their owning module.

## State and events

`App` owns lighting and navigation state. Its methods implement edits,
persistence, and live updates. `AppUi::draw` receives `&App`, so rendering cannot
change settings or schedule device writes. `AppUi` owns terminal hit regions;
clicks return an `Action` that `App::handle_action` interprets.

The terminal loop passes keys to `App::handle_key`, calls `App::tick`, and asks
`AppUi` to draw. `TerminalSession` restores the terminal and disables mouse
capture when it is dropped, including on errors.

Device writes go through `Device::apply`. Request ordering is defined in
`Settings::commands`; single-write handling and firmware checks live in
`device/client.rs`. Geometry and animation code never access hardware.

Unit tests remain in `tests/unit/`, included by their owning source modules.
Run `cargo test --locked` and `cargo clippy --locked --all-targets -- -D warnings`.
