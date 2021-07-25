use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32;
use bdk::bitcoin::Network;
use bdk::keys::bip39::{Language, Mnemonic, MnemonicType};

use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Key {
    mnemonic: String,
    xprv: String,
    fingerprint: String,
}

#[get("/generate")]
pub fn key_generate() -> Json<Key> {
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
    let xprv = bip32::ExtendedPrivKey::new_master(Network::Bitcoin, mnemonic.entropy()).unwrap();
    let secp = Secp256k1::new();
    Json(Key {
        mnemonic: mnemonic.phrase().to_string(),
        xprv: xprv.to_string(),
        fingerprint: xprv.fingerprint(&secp).to_string(),
    })
}
