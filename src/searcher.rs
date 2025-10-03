use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};
use crc32fast;
use hex;

pub fn generate_address(private_key: &[u8; 32]) -> String {
    let signing_key = SigningKey::from_bytes(private_key);
    let public_key = signing_key.verifying_key().to_bytes();
    let hash = Sha256::new().chain_update(public_key).finalize();
    let workchain: i32 = 0;
    let mut data = Vec::new();
    data.extend_from_slice(&workchain.to_be_bytes());
    data.extend_from_slice(&hash);
    let crc = crc32fast::hash(&data);
    data.extend_from_slice(&crc.to_be_bytes());
    general_purpose::URL_SAFE_NO_PAD.encode(&data)
}

fn private_key_from_u64(n: u64) -> [u8; 32] {
    let mut key = [0u8; 32];
    key[24..32].copy_from_slice(&n.to_be_bytes());
    key
}

pub fn search_vanity(pattern: &str, start: u64, end: u64, threads: usize) {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();

    let found = Arc::new(AtomicBool::new(false));

    (start..end).into_par_iter().for_each(|n| {
        if found.load(Ordering::Relaxed) {
            return;
        }
        let key = private_key_from_u64(n);
        let addr = generate_address(&key);
        if addr.starts_with(pattern) {
            if !found.swap(true, Ordering::Relaxed) {
                println!("Found: {} {}", hex::encode(key), addr);
                std::process::exit(0);
            }
        }
    });
}
