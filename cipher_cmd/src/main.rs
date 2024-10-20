use aes::Aes256;
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use sha2::{Sha256, Digest};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;
const SALT: & str = "yH4dPkrXq5SkKrGmT8bYGZpXu5TxtdnM";

pub fn encrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let iv = get_iv();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let result = cipher.encrypt_vec(data.as_bytes());
    let mut ivres: Vec<u8> = vec![];
}

fn get_key(password: &str) -> Vec<u8> {
}

fn gen_iv() -> Vec<u8> {
}
