use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::utils::{hash_to_str, serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    // 时间戳
    timestamp: i64,
    // 前一个区块的Hashi值
    prev_hash: String,
    // 随机数，用于计算工作量证明
    nonce: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    // 区块头
    header: BlockHeader,
    // 区块中村塾的数据，后面在实现交易功能时，这个字段会修改为交易集合
    data: String,
    // 块的Hash值，我们对区块头进行hash就行了，因为区块头包含了区块的所有信息，
    // 后面会把交易hash也加入到区块头中
    hash: String,
}

impl Block {
    pub fn new(data: &str, prev_hash: &str) -> Self {
        let mut block = Block {
            header: BlockHeader {
                timestamp: Utc::now().timestamp(),
                prev_hash: prev_hash.into(),
                nonce: 0,
            },
            data: data.into(),
            hash: String::new(),
        };
        // 对块头求Hash
        block.set_hash();

        block
    }

    // 创世区块中的prev_hash没有值
    pub fn create_genesis_block() -> Self {
        Self::new("创世区块", "")
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    fn set_hash(&mut self) {
        if let Ok(serialized) = serialize(&self.header) {
            self.hash = hash_to_str(&serialized)
        }
    }
}
