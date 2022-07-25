use crate::error::BlockchainError;
use anyhow::Result;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

// 使用serde和bincode库，实现区块头序列化成字节
pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: Serialize + ?Sized,
{
    Ok(bincode::serialize(data)?)
}

#[allow(dead_code)]
pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockchainError>
where
    T: Deserialize<'a> + ?Sized,
{
    Ok(bincode::deserialize(data)?)
}

// 将序列化的区块头，使用SHA256计算Hash
// 即对data求Hash
pub fn hash_to_str(data: &[u8]) -> String {
    // create a SHA3-256 object
    let mut hasher = Sha3::sha3_256();
    // write input message
    hasher.input(data);
    // read hash digest
    hasher.result_str()
}

// 将获取到的Hash放到out里
#[allow(dead_code)]
pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}
