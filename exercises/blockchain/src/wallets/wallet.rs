use crate::utils::{base58_encode, new_private_key, ripemd160_digest, sha256_digest};
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use serde::{Deserialize, Serialize};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN: usize = 4;

// 钱包包含公钥和私钥两个字段
#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    // 私钥
    pkcs8: Vec<u8>,
    // 公钥
    public_key: Vec<u8>,
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

impl Wallet {
    pub fn new() -> Self {
        let pkcs8 = new_private_key();
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();

        Self { pkcs8, public_key }
    }

    // 先使用SHA256对公钥进行一次哈希，对结果使用RIPEMD160进行二次哈希。
    // 给哈希值加上版本前缀，这里硬编码为 const VERSION: u8 = 0x00。
    // 对上一步生成的结果，使用 SHA256进行两次哈希。取结果的前四个字节作为校验和。
    // 将校验和附加到 version+PubKeyHash 的组合中。
    // 使用 Base58 对 version+PubKeyHash+checksum 组合进行编码。
    pub fn get_address(&self) -> String {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload = vec![VERSION];
        // payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        base58_encode(payload.as_slice())
    }

    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }
}

pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = sha256_digest(pub_key);
    // pub_key_ripemd160
    ripemd160_digest(&pub_key_sha256)
}

pub fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = sha256_digest(payload);
    let second_sha = sha256_digest(&first_sha);
    second_sha[0..ADDRESS_CHECKSUM_LEN].to_vec()
}
