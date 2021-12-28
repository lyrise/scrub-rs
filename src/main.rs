use std::path::Path;
use std::{env, fs, io};

use anyhow::Result;
use sha2::{Digest, Sha256};
use tracing;
use tracing::info;
use tracing_subscriber;

#[async_std::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let current_dir = env::current_dir()?;

    scrub(&current_dir)?;

    Ok(())
}

fn scrub(path: &Path) -> Result<()> {
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Ok(metadata) = fs::metadata(&path) {
                if metadata.is_file() {
                    compute_hash(&path)?;
                } else if metadata.is_dir() {
                    scrub(&path)?;
                }
            }
        }
    }
    Ok(())
}

fn compute_hash(path: &Path) -> Result<()> {
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(path)?;

    let bytes_written = io::copy(&mut file, &mut hasher)?;
    let hash_bytes = hasher.finalize();

    info!(
        "{} {} {}",
        hex::encode(hash_bytes),
        bytes_written,
        path.to_str().unwrap()
    );

    Ok(())
}
