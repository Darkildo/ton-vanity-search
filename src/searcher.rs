use ed25519_dalek::{SigningKey, VerifyingKey};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use tonlib_core::wallet::mnemonic::KeyPair;
use tonlib_core::wallet::ton_wallet::TonWallet;
use tonlib_core::wallet::wallet_version::WalletVersion;

fn private_key_from_u64(n: u64) -> [u8; 32] {
    let mut key = [0u8; 32];
    key[24..32].copy_from_slice(&n.to_be_bytes());
    key
}

pub fn ton_v4r2_address_from_private(secret_key_32: [u8; 32]) -> String {
    // Получаем публичный из приватного (ed25519).
    let sk = SigningKey::from_bytes(&secret_key_32);
    let vk: VerifyingKey = sk.verifying_key();
    let public = vk.to_bytes();

    let kp = KeyPair {
        public_key: public.to_vec(),
        secret_key: secret_key_32.to_vec(),
    };

    let wallet = TonWallet::new(WalletVersion::V4R2, kp).unwrap();

    wallet.address.to_string()
}

pub fn search_vanity(
    pattern: &str,
    start: u64,
    end: u64,
    threads: usize,
) -> Option<(String, String)> {
    assert!(start < end, "start must be less than end");
    assert!(!pattern.is_empty(), "pattern must not be empty");

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("failed to build thread pool");

    let (tx, rx) = mpsc::channel::<(String, String)>();
    let found = Arc::new(AtomicBool::new(false));
    let counter = Arc::new(AtomicU64::new(0));

    // Таймер для вывода прогресса раз в минуту
    let counter_clone = Arc::clone(&counter);
    let found_clone = Arc::clone(&found);
    let start_time = Instant::now();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
            if found_clone.load(Ordering::SeqCst) {
                break;
            }
            let checked = counter_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f64().max(1.0);
            let speed = checked as f64 / elapsed;
            println!(
                "[progress] Checked: {checked} | Elapsed: {:.1} min | Speed: {:.0}/s",
                elapsed / 60.0,
                speed
            );
        }
    });

    pool.install(|| {
        (start..end).into_par_iter().for_each(|n| {
            if found.load(Ordering::SeqCst) {
                return;
            }

            let addr = ton_v4r2_address_from_private(private_key_from_u64(n));

            counter.fetch_add(1, Ordering::Relaxed);

            if addr.starts_with(pattern) {
                if !found.swap(true, Ordering::SeqCst) {
                    let _ = tx.send((hex::encode(private_key_from_u64(n)), addr));
                }
            }
        });
    });

    rx.try_recv().ok()
}
