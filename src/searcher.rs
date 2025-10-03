use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};
use hex;
use std::sync::mpsc;

fn crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

pub fn generate_address(private_key: &[u8; 32]) -> String {
    let signing_key = SigningKey::from_bytes(private_key);
    let public_key = signing_key.verifying_key().to_bytes();
    let hash = Sha256::new().chain_update(public_key).finalize();
    let workchain: i8 = 0;
    let mut data = Vec::new();
    data.push(workchain as u8);
    data.extend_from_slice(&hash);
    let crc = crc16_ccitt(&data);
    data.extend_from_slice(&crc.to_be_bytes());
    data.push(0x11);
    general_purpose::URL_SAFE_NO_PAD.encode(&data)
}

fn private_key_from_u64(n: u64) -> [u8; 32] {
    let mut key = [0u8; 32];
    key[24..32].copy_from_slice(&n.to_be_bytes());
    key
}

pub fn search_vanity(pattern: &str, start: u64, end: u64, threads: usize) -> Option<(String, String)> {
    if start >= end {
        panic!("start must be less than end");
    }
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let (tx, rx) = mpsc::channel();
    let found = Arc::new(AtomicBool::new(false));

    pool.install(|| {
        (start..end).into_par_iter().for_each(|n| {
            if found.load(Ordering::SeqCst) {
                return;
            }
            let key = private_key_from_u64(n);
            let addr = generate_address(&key);
            if addr.starts_with(pattern) {
                if !found.swap(true, Ordering::SeqCst) {
                    let _ = tx.send((hex::encode(key), addr));
                }
            }
        });
    });
    rx.try_recv().ok()
}
