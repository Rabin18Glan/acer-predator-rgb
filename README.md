# acer-predator-rgb

Control your Acer Nitro or Predator keyboard lighting from the Linux terminal.
Choose colors for each zone, adjust brightness, and switch between lighting effects.
Changes apply immediately.

## Requirements

- x86_64 Linux (Intel or AMD).
- A compatible Acer four-zone RGB keyboard. Tested on Nitro AN515-45.
- The [predator_kbd_rgb driver](https://github.com/TomasUribe/predator-rgb), installed separately.
- A terminal at least 76 × 24 characters.

No Rust, Cargo, or Python is needed to run this app.

## Install

```bash
curl -fsSL https://github.com/Rabin18Glan/acer-predator-rgb/releases/latest/download/install.sh | bash
```

Installs to `~/.local/bin` without sudo. Run the same command to update.

## Use

```bash
acer-predator-rgb
```

If the command is not found, run `~/.local/bin/acer-predator-rgb` directly.

| Key | Action |
| --- | --- |
| `1–4` or `Tab` | Switch between Color, Effect, Speed, and Direction |
| `Shift+1–4` | Select a keyboard zone |
| `↑ / ↓` | Select a control or effect |
| `← / →` | Adjust a value |
| `Shift+← / →` | Adjust in larger steps |
| `h` | Enter a six-digit hex color |
| `s` / `l` | Save / load settings |
| `r` | Check driver status |
| `q` or `Ctrl+C` | Quit |

You can also click tabs, keys, colors, and sliders.

Target shows the selected zone in Static mode or the selected effect's name
in animation mode. Breathing, Shifting, and Zoom accept a custom color;
Wave and Neon generate their own colors.

Choose **Static** for individual zone colors. Select **All zones** in the Target
control to change the whole keyboard. Animated effects use a shared color or
generate their own colors.

If the app reports a missing driver, check that the required driver is installed
and `/dev/predator-kbd` exists.

## Uninstall

```bash
rm -- "$HOME/.local/bin/acer-predator-rgb"
```

Your keyboard driver and saved settings are kept.
