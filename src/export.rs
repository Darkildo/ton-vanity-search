
use anyhow::{anyhow, Context, Result};
use bip39::{Language, Mnemonic};
use ed25519_dalek::{SigningKey, VerifyingKey};
use tonlib_core::wallet::mnemonic::KeyPair;
use tonlib_core::wallet::ton_wallet::TonWallet;
use tonlib_core::wallet::wallet_version::WalletVersion;

pub fn export_v4r2_to_mnemonic_24(wallet: &TonWallet) -> Result<String> {
    let sk = &wallet.key_pair.secret_key;
    if sk.len() != 32 {
        return Err(anyhow!("expected 32-byte ed25519 secret key, got {}", sk.len()));
    }
    let m = Mnemonic::from_entropy(sk)
        .context("failed to encode secret key into 24-word mnemonic")?;
    Ok(m.to_string())
}


pub fn example_export(wallet: &TonWallet) -> () {
    let phrase = export_v4r2_to_mnemonic_24(wallet).unwrap();
    println!("Your 24-word import phrase:\n{}", phrase);

}

pub fn ton_v4r2_seed_from_private(secret_key_32: [u8; 32]) -> TonWallet {
    let sk = SigningKey::from_bytes(&secret_key_32);
    let vk: VerifyingKey = sk.verifying_key();
    let public = vk.to_bytes();

    let kp = KeyPair {
        public_key: public.to_vec(),
        secret_key: secret_key_32.to_vec(),
    };

    let wallet = TonWallet::new(WalletVersion::V4R2, kp).unwrap();

    return wallet;
}