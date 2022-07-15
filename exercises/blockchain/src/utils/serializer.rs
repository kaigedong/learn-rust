use anyhow::Result;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

use crate::error::BlockchainError;

// 使用serde和bincode库，把区块头序列化成字节
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

// 将序列化成字节的区块头，计算hash值
pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}

#[allow(dead_code)]
pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}