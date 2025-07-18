use sha2::{Digest, Sha256};
use std::fs;

pub fn compute_checksum(path: &Path) -> Result<String, Box<dyn Error>> {
    let data = fs::read(path)?;
    let hash = Sha256::digest(&data);

    Ok(format!("{:x}", hash))
}
