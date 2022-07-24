use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{hash_to_str, serialize},
    Transaction,
};

use super::pow::ProofOfWork;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct BlockHeader {
    timestamp: i64,
    // 前一个区块的Hash值
    prev_hash: String,
    txs_hash: String,
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
            txs_hash: String::new(),
            bits,
            nonce: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Block {
    // 区块头
    header: BlockHeader,
    // 区块存储的数据，后面在实现交易功能的时候，这个字段会修改为交易集合
    // data: String,
    tranxs: Vec<Transaction>,
    // 块的Hash值；下一个区块头存放这个hash
    // 对区块头进行Hash就行了，区块头包含了区块的所有信息，后面会把交易hash值也加入到区块头
    hash: String,
}

impl Block {
    // 新建区块及实现创世块
    pub fn new(txs: &[Transaction], prev_hash: &str, bits: usize) -> Self {
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            tranxs: txs.to_vec(),
            hash: String::new(),
        };
        block.set_txs_hash(txs);

        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn create_genesis_block(bits: usize, genesis_addr: &str) -> Self {
        let coinbase = Transaction::new_coinbase(genesis_addr);
        Self::new(&[coinbase], "", bits)
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

    fn set_txs_hash(&mut self, txs: &[Transaction]) {
        if let Ok(txs_ser) = serialize(txs) {
            self.header.txs_hash = hash_to_str(&txs_ser);
        }
    }

    pub fn get_tranxs(&self) -> Vec<Transaction> {
        self.tranxs.clone()
    }
}
