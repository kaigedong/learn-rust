use crate::{
    utils::{hash_to_str, hash_to_u8, serialize},
    Block,
};
use anyhow::Result;
use bigint::U256;
use std::ops::Shl;

const MAX_NONCE: usize = usize::MAX;

pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    pub fn new(bits: usize) -> Self {
        // 这里用到了bitint库里的U256,先初始化为1
        let mut target = U256::from(1usize);
        // 将1向左移动256-bits位，bits是8的话，向左移动248位
        target = target.shl(256 - bits);

        Self { target }
    }

    pub fn run(&self, block: &mut Block) {
        let mut nonce = 0;

        //设置MAX_NONCE为usize::MAX,避免计算溢出
        while nonce < MAX_NONCE {
            if let Ok(pre_hash) = Self::prepare_data(block, nonce) {
                let mut hash_u: [u8; 32] = [0; 32];
                hash_to_u8(&pre_hash, &mut hash_u);
                let pre_hash_int = U256::from(hash_u);

                // 如果计算出的hash值小于target，则满足条件，跳出循环
                // 否则nonce加一，进入下次hash计算
                if pre_hash_int.lt(&(self.target)) {
                    block.set_hash(hash_to_str(&pre_hash));
                    break;
                } else {
                    nonce += 1;
                }
            }
        }
    }

    // 设置nonce值，对区块头进行序列化
    fn prepare_data(block: &mut Block, nonce: usize) -> Result<Vec<u8>> {
        block.set_nonce(nonce);
        Ok(serialize(&block.get_header())?)
    }
}
