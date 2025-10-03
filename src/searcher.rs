use ed25519_dalek::{SigningKey, VerifyingKey};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;
use std::thread;
use tonlib_core::wallet::mnemonic::KeyPair;
use tonlib_core::wallet::ton_wallet::TonWallet;
use tonlib_core::wallet::wallet_version::WalletVersion;

use rand::{Rng, seq::SliceRandom};

use crate::export::{example_export, ton_v4r2_seed_from_private};




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

    // Настройки "массы" чанков (можешь подстроить под машину/нагрузку).
    // Оба значения обязаны помещаться в u32 и min <= max.
    let min_chunk: u32 = (2_000 / 10000).max(1000);
    let max_chunk: u32 = (end / 10000).max(10000).try_into().unwrap();
    // Гарантированно <= u32::MAX, проверено константами.

    // Готовим чанки заранее и перемешиваем
    let chunks = build_random_chunks(start, end, min_chunk, max_chunk);

    // Пул потоков
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("failed to build thread pool");

    let (tx, rx) = mpsc::channel::<(String, String)>();
    let found = Arc::new(AtomicBool::new(false));
    let checked = Arc::new(AtomicU64::new(0));
    let found_p = Arc::clone(&found);
    let checked_p = Arc::clone(&checked);
    let total = end - start;
    
    thread::spawn(move || {
        let bar = ProgressBar::new(total);
        bar.set_style(
    ProgressStyle::with_template(
        "[{bar:40.cyan/blue}] {percent}% | {human_pos}/{human_len} | {per_sec}/s | {elapsed_precise}"
    ).unwrap()
    .progress_chars("█░")
);

        let bar_ref = Arc::new(bar);
        let bar_upd = bar_ref.clone();
        loop {
            thread::sleep(Duration::from_secs(1));
            if found_p.load(Ordering::Relaxed) {
                break;
            }
            let c = checked_p.load(Ordering::Relaxed);
            bar_upd.set_position(c);
        }
    });

    // Параллельно обрабатываем чанки; внутри чанка — обычный for по локальному диапазону.
    pool.install(|| {
        chunks.par_iter().for_each(|&(cs, ce)| {
            // быстрый выход, если уже нашли
            if found.load(Ordering::Relaxed) {
                return;
            }

            // локальный последовательный перебор диапазона чанка
            let mut local_checked: u64 = 0;
            for n in cs..ce {
                if found.load(Ordering::Relaxed) {
                    break;
                }

                let key = private_key_from_u64(n);
                let addr = ton_v4r2_address_from_private(key);

                local_checked += 1;

                if addr.starts_with(pattern) {
                    if !found.swap(true, Ordering::SeqCst) {
                        let _ = tx.send((hex::encode(key), addr));
                      let wallet =  ton_v4r2_seed_from_private(key);
                      example_export(&wallet);

                    }
                    break;
                }
            }
            if local_checked > 0 {
                checked.fetch_add(local_checked, Ordering::Relaxed);
            }
        });
    });

    rx.try_recv().ok()
}

/// Строит вектор случайно-сегментированных чанков диапазона [start, end),
/// где длина каждого чанка в пределах [min_chunk, max_chunk] и <= u32::MAX.
/// Возвращает вектор кортежей (chunk_start, chunk_end), end не включительно.
fn build_random_chunks(start: u64, end: u64, min_chunk: u32, max_chunk: u32) -> Vec<(u64, u64)> {
    assert!(start < end, "start must be less than end");
    assert!(min_chunk > 0, "min_chunk must be > 0");
    assert!(min_chunk <= max_chunk, "min_chunk must be <= max_chunk");

    let mut chunks = Vec::new();
    let mut rng = rand::rng();

    let mut cur = start;
    let total = end - start;

    // жёстная «крышка» по u32
    let hard_cap = u32::MAX as u64;

    while cur < end {
        // случайная длина чанка в [min_chunk, max_chunk], но не больше остатка и HARD_CAP
        let want = rng.random_range(min_chunk..=max_chunk) as u64;
        let remain = end - cur;
        let len = want.min(remain).min(hard_cap);

        let next = cur + len;
        chunks.push((cur, next));
        cur = next;
    }

    // Перемешаем список (случайное распределение «масс» по потокам)
    chunks.shuffle(&mut rng);

    // На всякий — sanity check:
    debug_assert_eq!(
        chunks.iter().map(|(s, e)| e - s).sum::<u64>(),
        total,
        "chunk partition must cover the whole range"
    );

    chunks
}
