mod app;
mod cli;
mod config;
mod device;
mod keyboard;
mod model;
mod terminal;
mod ui;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("acer-predator-rgb: {error}");
        std::process::exit(1);
    }
}
