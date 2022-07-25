use crate::{error::BlockchainError, Blockchain, Storage};
use std::{collections::HashMap, sync::Arc};

pub struct UTXOSet<T> {
    storage: Arc<T>,
}

// UTXO 集，也就是未花费交易输出的集合，用它来计算余额和验证新的交易，存储在数据库中。
impl<T: Storage> UTXOSet<T> {
    pub fn new(storage: Arc<T>) -> Self {
        Self { storage }
    }
    // 当产生新的区块时，重建UTXO集合索引
    pub fn reindex(&self, bc: &Blockchain) -> Result<(), BlockchainError> {
        self.storage.clear_utxo_set();
        let map = bc.find_utxo();
        for (txid, outs) in map {
            self.storage.write_utxo(&txid, outs)?;
        }
        Ok(())
    }
    // 找到交易发起方可花费的交易输出
    pub fn find_spendable_outputs(
        &self,
        from_addr: &str,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<usize>>) {
        let mut unspent_outputs = HashMap::new();
        let mut accumulated = 0;
        let utxo_set = self.storage.get_utxo_set();

        for (txid, outs) in utxo_set.iter() {
            for (idx, out) in outs.iter().enumerate() {
                if out.is_locked(from_addr) && accumulated < amount {
                    accumulated += out.get_value();
                    unspent_outputs
                        .entry(txid.to_string())
                        .and_modify(|v: &mut Vec<usize>| v.push(idx))
                        .or_insert_with(|| vec![idx]);
                }
            }
        }

        (accumulated, unspent_outputs)
    }
}
