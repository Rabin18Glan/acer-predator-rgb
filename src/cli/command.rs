use crate::{config, device};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Command {
    Interactive,
    Help,
    Status,
    DryRun,
    Apply,
}

impl Command {
    pub(super) fn parse(args: &[String]) -> Result<Self, String> {
        match args {
            [] => Ok(Self::Interactive),
            [arg] => match arg.as_str() {
                "--help" | "-h" => Ok(Self::Help),
                "--status" => Ok(Self::Status),
                "--dry-run" => Ok(Self::DryRun),
                "--apply" => Ok(Self::Apply),
                _ => Err("Unknown arguments. Use --help.".into()),
            },
            _ => Err("Unknown arguments. Use --help.".into()),
        }
    }

    pub(super) fn execute(self) -> Result<(), String> {
        match self {
            Self::Interactive => crate::terminal::run(),
            Self::Help => {
                println!(
                    "Acer RGB Studio\n\nRun without arguments for the TUI.\n--status   Read driver status (no lighting changes)\n--dry-run  Show commands for saved/imported settings\n--apply    Apply saved/imported settings explicitly\n\nUses the installed predator_kbd_rgb driver at /dev/predator-kbd.\nNo modules, services, or firmware are installed or changed automatically."
                );
                Ok(())
            }
            Self::Status => {
                println!(
                    "{}",
                    device::status().map_err(|e| e.to_string())?.describe()
                );
                Ok(())
            }
            Self::DryRun => {
                let (settings, source) = config::load()?;
                println!("{source}");
                for command in settings.commands()? {
                    println!("{command}");
                }
                Ok(())
            }
            Self::Apply => {
                let (settings, source) = config::load()?;
                println!("{source}");
                println!(
                    "{}",
                    device::Device
                        .apply(&settings)
                        .map_err(|e| e.to_string())?
                        .describe()
                );
                Ok(())
            }
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/cli.rs"]
mod tests;
