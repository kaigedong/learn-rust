use crate::{
    utils::{hash_to_str, serialize},
    Storage, Txinput, Txoutput, UTXOSet,
};
use serde::{Deserialize, Serialize};

const SUBSIDY: i32 = 10;

// 交易
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    // 交易hash值
    id: String,
    // 交易输入集合
    vin: Vec<Txinput>,
    // 交易输出集合
    vout: Vec<Txoutput>,
}

impl Transaction {
    // 挖矿奖励，没有交易输入
    pub fn new_coinbase(to: &str) -> Self {
        let txin = Txinput::default();
        let txout = Txoutput::new(SUBSIDY, to);

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![txin],
            vout: vec![txout],
        };
        tx.set_hash();

        tx
    }
    // 产生一笔交易
    pub fn new_utxo<T: Storage>(from: &str, to: &str, amount: i32, utxo_set: &UTXOSet<T>) -> Self {
        // 从UTXO集合中取出未花费的交易输出
        let (accumulated, valid_outputs) = utxo_set.find_spendable_outputs(from, amount);
        if accumulated < amount {
            panic!("Error not enough funds");
        }

        // 构建交易输入
        let mut inputs = vec![];
        for (txid, outputs) in valid_outputs {
            for idx in outputs {
                let input = Txinput::new(txid.clone(), idx, from);
                inputs.push(input);
            }
        }

        // 构建交易输出
        let mut outputs = vec![Txoutput::new(amount, to)];
        if accumulated > amount {
            // 找零
            outputs.push(Txoutput::new(accumulated - amount, from));
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: inputs,
            vout: outputs,
        };
        tx.set_hash();

        tx
    }

    fn set_hash(&mut self) {
        if let Ok(tx_ser) = serialize(self) {
            self.id = hash_to_str(&tx_ser)
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_vout(&self) -> &[Txoutput] {
        self.vout.as_slice()
    }

    pub fn get_vin(&self) -> &[Txinput] {
        self.vin.as_slice()
    }
}
