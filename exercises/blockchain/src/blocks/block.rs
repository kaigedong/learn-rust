use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::pow::ProofOfWork;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct BlockHeader {
    timestamp: i64,
    // 前一个区块的Hash值
    prev_hash: String,
    // 计算难度，也就是区块hash值的前多少位是0
    bits: usize,
    // 随机数，用于计算工作量证明
    // 记录满足bits难度，重复计算的次数
    nonce: usize,
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: usize) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            prev_hash: prev_hash.into(),
            bits,
            nonce: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct Block {
    // 区块头
    header: BlockHeader,
    // 区块存储的数据，后面在实现交易功能的时候，这个字段会修改为交易集合
    data: String,
    // 块的Hash值；下一个区块头存放这个hash
    // 对区块头进行Hash就行了，区块头包含了区块的所有信息，后面会把交易hash值也加入到区块头
    hash: String,
}

impl Block {
    // 新建区块及实现创世块
    pub fn new(data: &str, prev_hash: &str, bits: usize) -> Self {
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            data: data.into(),
            hash: String::new(),
        };
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn create_genesis_block(bits: usize) -> Self {
        Self::new("创世区块", "", bits)
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }

    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
}
