//! Command-line entry point.
mod command;

use command::Command;

pub(crate) fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    Command::parse(&args)?.execute()
}
