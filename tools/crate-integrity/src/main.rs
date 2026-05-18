use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Serialize)]
struct CrateEntry {
    name: String,
    version: String,
    checksum: String, // "SHA256:..."
}

#[derive(Debug, Serialize)]
struct IntegrityToml {
    crate_: Vec<CrateEntry>,
}

fn sha256_hex_of_file(path: &Path) -> Result<String> {
    let data = fs::read(path).with_context(|| format!("reading {:?}", path))?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(format!("SHA256:{:x}", result))
}

/// Detect crate name and version from a vendored path:
/// e.g., vendor/serde-1.0.202/ -> ("serde", "1.0.202")
fn detect_name_version(dir: &Path) -> Option<(String, String)> {
    let stem = dir.file_name()?.to_string_lossy();
    // Expect pattern name-version
    if let Some(idx) = stem.rfind('-') {
        let (name, ver) = stem.split_at(idx);
        let version = &ver[1..]; // skip '-'
        Some((name.to_string(), version.to_string()))
    } else {
        None
    }
}

fn find_vendored_crate_dirs(vendor_root: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(entries) = fs::read_dir(vendor_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }
    }
    dirs
}

fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS crate_integrity (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            checksum TEXT NOT NULL,
            path TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            UNIQUE(name, version)
        );
        "#,
    )?;
    Ok(())
}

fn upsert_entry(conn: &Connection, entry: &CrateEntry, path: &Path) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO crate_integrity (name, version, checksum, path)
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(name, version) DO UPDATE SET
            checksum = excluded.checksum,
            path = excluded.path,
            created_at = datetime('now');
        "#,
        params![entry.name, entry.version, entry.checksum, path.to_string_lossy()],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let vendor_root = PathBuf::from("vendor");
    let config_path = PathBuf::from("config/crate-integrity.toml");
    let db_path = PathBuf::from("config/crate-integrity.db");

    fs::create_dir_all("config").context("creating config directory")?;

    // Open SQLite DB
    let conn = Connection::open(&db_path).context("opening crate-integrity.db")?;
    init_db(&conn)?;

    // Walk vendor/ looking for crate directories
    let crate_dirs = find_vendored_crate_dirs(&vendor_root);

    let mut entries = Vec::new();

    for dir in crate_dirs {
        if let Some((name, version)) = detect_name_version(&dir) {
            // Hash all files in this crate dir deterministically.
            let mut hasher = Sha256::new();

            for entry in WalkDir::new(&dir).into_iter().flatten() {
                let p = entry.path();
                if p.is_file() {
                    let data = fs::read(p).with_context(|| format!("reading {:?}", p))?;
                    hasher.update(&data);
                }
            }

            let digest = hasher.finalize();
            let checksum = format!("SHA256:{:x}", digest);

            let entry = CrateEntry {
                name,
                version,
                checksum,
            };

            upsert_entry(&conn, &entry, &dir)?;
            entries.push(entry);
        }
    }

    // Render TOML
    #[derive(Serialize)]
    struct Wrapper {
        #[serde(rename = "crate")]
        crate_: Vec<CrateEntry>,
    }

    let wrapper = Wrapper { crate_: entries };
    let toml_str =
        toml::to_string_pretty(&wrapper).context("serializing crate-integrity.toml")?;
    fs::write(&config_path, toml_str).context("writing crate-integrity.toml")?;

    println!("Updated config/crate-integrity.toml and config/crate-integrity.db");
    Ok(())
}
