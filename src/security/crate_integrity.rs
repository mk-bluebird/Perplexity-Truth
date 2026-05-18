//! Crate allowlist and checksum verification.
//!
//! CI will:
//! - Parse Cargo.lock to get name+version for each dependency.
//! - Look up expected checksums in config/crate-integrity.toml.
//! - Compute actual checksums of vendored crates or cached .crate files.
//! - Fail if any mismatch.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CrateEntry {
    name: String,
    version: String,
    checksum: String, // "SHA256:...."
}

#[derive(Debug, Deserialize)]
struct CrateIntegrityConfig {
    crate: Vec<CrateEntry>,
}

pub fn load_integrity_config(path: &str) -> anyhow::Result<HashMap<(String, String), String>> {
    let data = fs::read_to_string(path)?;
    let parsed: CrateIntegrityConfig = toml::from_str(&data)?;
    let mut map = HashMap::new();
    for c in parsed.crate {
        map.insert((c.name, c.version), c.checksum);
    }
    Ok(map)
}

fn sha256_hex_of_file(path: &Path) -> anyhow::Result<String> {
    use sha2::{Digest, Sha256};
    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(format!("SHA256:{:x}", result))
}

/// Verify a single crate tarball or vendored directory against the expected checksum.
pub fn verify_crate_checksum(
    name: &str,
    version: &str,
    expected: &str,
    tarball_path: &Path,
) -> anyhow::Result<bool> {
    let actual = sha256_hex_of_file(tarball_path)?;
    Ok(actual == expected)
}
