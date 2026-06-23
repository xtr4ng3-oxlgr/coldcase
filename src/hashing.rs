use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn sha256_file(path: &Path, max_mb: u64) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let max_bytes = max_mb.saturating_mul(1024).saturating_mul(1024);
    let mut read_total: u64 = 0;
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
        read_total += n as u64;

        if read_total >= max_bytes {
            hasher.update(b"COLDCASE_PARTIAL_HASH_LIMIT_REACHED");
            break;
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn entropy_file(path: &Path, max_bytes: usize) -> f64 {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return 0.0,
    };

    let mut counts = [0u64; 256];
    let mut total = 0u64;
    let mut buffer = [0u8; 8192];

    loop {
        let n = match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        for b in &buffer[..n] {
            counts[*b as usize] += 1;
        }

        total += n as u64;
        if total as usize >= max_bytes {
            break;
        }
    }

    if total == 0 {
        return 0.0;
    }

    let total_f = total as f64;
    let mut entropy = 0.0;
    for c in counts {
        if c == 0 {
            continue;
        }
        let p = c as f64 / total_f;
        entropy -= p * p.log2();
    }

    entropy
}
