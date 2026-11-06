use super::{DEVICE, Error};
use std::{fs, os::unix::fs::FileTypeExt};
pub(super) fn check() -> Result<(), Error> {
    let metadata = fs::symlink_metadata(DEVICE).map_err(|e| {
        Error::Io(
            "Cannot access /dev/predator-kbd (requires the working predator_kbd_rgb driver)",
            e,
        )
    })?;
    if !metadata.file_type().is_char_device() {
        return Err(Error::Invalid(
            "/dev/predator-kbd is not a character device".into(),
        ));
    }
    let taint = fs::read_to_string("/proc/sys/kernel/tainted")
        .map_err(|e| Error::Io("Cannot inspect kernel state", e))?;
    let taint: u64 = taint
        .trim()
        .parse()
        .map_err(|_| Error::Invalid("Invalid kernel taint value".into()))?;
    if taint & 128 != 0 {
        return Err(Error::Invalid(
            "Kernel recorded an Oops; restart before accessing keyboard devices".into(),
        ));
    }
    Ok(())
}
