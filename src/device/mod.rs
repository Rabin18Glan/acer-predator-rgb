//! Access to the existing predator_kbd_rgb userspace interface.
mod client;
mod node;
mod status;

use std::io;
use thiserror::Error;

pub(crate) use client::Device;
pub(crate) use status::{Status, read as status};

const DEVICE: &str = "/dev/predator-kbd";

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Invalid(String),
    #[error("{0}: {1}")]
    Io(&'static str, io::Error),
}
