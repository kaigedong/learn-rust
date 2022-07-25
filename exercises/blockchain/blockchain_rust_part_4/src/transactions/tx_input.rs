use serde::{Deserialize, Serialize};

// 交易输入
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Txinput {
    // 前一笔交易的ID
    txid: String,
    // 前一笔交易输出的序号
    vout: usize,
    // 交易的发起方，下一部分实现钱包功能时，替换为交易发起方的公钥
    from_addr: String,
}

impl Txinput {
    pub fn new(txid: String, vout: usize, from_addr: &str) -> Self {
        Self {
            txid,
            vout,
            from_addr: from_addr.into(),
        }
    }

    pub fn can_unlock_output(&self, address: &str) -> bool {
        self.from_addr.eq(address)
    }

    pub fn get_txid(&self) -> String {
        self.txid.clone()
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }
}
