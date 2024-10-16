use aes::Aes256;
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use sha2::{Sha256, Digest};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;
const SALT: & str = "yH4dPkrXq5SkKrGmT8bYGZpXu5TxtdnM";

pub fn encrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let iv = get_iv();
}

fn get_key(password: &str) -> Vec<u8> {
}

fn gen_iv() -> Vec<u8> {
}
