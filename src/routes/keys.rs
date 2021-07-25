use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32;
use bdk::bitcoin::Network;
use bdk::keys::bip39::{Language, Mnemonic, MnemonicType};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MasterKey {
    mnemonic: String,
    master_xprv: String,
    fingerprint: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct MasterKeyOpts {
    word_count: Option<u16>,
    password: Option<String>,
}

#[post("/generate", format = "json", data = "<opts>")]
pub fn key_generate(opts: Json<MasterKeyOpts>) -> Json<MasterKey> {
    let mnemonic_type = match opts.word_count {
        Some(12) => MnemonicType::Words12,
        _ => MnemonicType::Words24,
    };
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    let xprv = bip32::ExtendedPrivKey::new_master(Network::Bitcoin, mnemonic.entropy()).unwrap();
    let secp = Secp256k1::new();
    Json(MasterKey {
        mnemonic: mnemonic.phrase().to_string(),
        master_xprv: xprv.to_string(),
        fingerprint: xprv.fingerprint(&secp).to_string(),
    })
}
