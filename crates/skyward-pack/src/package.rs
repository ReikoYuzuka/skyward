//! The stock build's full update package: the address of the maker's own
//! copy on its server, or a copy of the package kept on this machine.

use anyhow::{Context, Result};
use std::path::Path;
use unpayload::Payload;

/// Opens the stock build's full update package.
///
/// A package named by an address is read only where the parts wanted lie in
/// it, so a build several gigabytes long is not downloaded whole. A package
/// kept here is read from the disk: the OTA zip itself, or the `payload.bin`
/// taken out of one. Both say the same of the build, and the recovery taken
/// out of either is checked against the SHA-256 the configuration names.
pub fn open(package: &str) -> Result<Payload> {
    if package.contains("://") {
        Payload::open_url(package).with_context(|| format!("open {package}"))
    } else {
        Payload::open(Path::new(package)).with_context(|| format!("open {package}"))
    }
}
