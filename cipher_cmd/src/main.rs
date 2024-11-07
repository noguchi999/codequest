use aes::Aes256;
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use sha2::{Sha256, Digest};

type Aes2Cbc = Cbc<Aes256, Pkcs7>;
const SALT: & str = "yH4dPkrXq5SkKrGmT8bYGZpXu5TxtdnM";

pub fn encrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let iv = gen_iv();
    let cipher = Aes2Cbc::new_from_slices(&key, &iv).unwrap();
    let result = cipher.encrypt_vec(data.as_bytes());
    let mut ivres: Vec<u8> = vec![];
    ivres.extend(iv);
    ivres.extend(result);
    base64::encode(ivres)
}

fn gen_iv() -> Vec<u8> {
    let mut res:Vec<u8> = vec![0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];
    getrandom::getrandom(&mut res).unwrap();
    res
}

fn get_key(password: &str) -> Vec<u8> {
    let pw:String = format!("{}::{}", password, SALT);
    let mut h = Sha256::new();
    h.update(pw.as_bytes());
    h.finalize().to_vec()
}

pub fn decrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let bytes = base64::decode(data).unwrap();
    let iv = &bytes[..16];
    let cipher = Aes2Cbc::new_from_slices(&key, iv).unwrap();
    let result = cipher.decrypt_vec(&bytes[16..]).unwrap();
    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod cipher_tests {
    use super::*;
    #[test]
}