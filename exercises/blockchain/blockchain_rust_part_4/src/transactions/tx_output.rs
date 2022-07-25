use serde::{Deserialize, Serialize};

// 交易输出
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Txoutput {
    // 交易值
    value: i32,
    // 交易接收方，下一部分实现钱包功能时，替换为交易接收方的公钥hash。
    to_addr: String,
}

impl Txoutput {
    pub fn new(value: i32, to_addr: &str) -> Self {
        Self {
            value,
            to_addr: to_addr.into(),
        }
    }

    pub fn is_locked(&self, address: &str) -> bool {
        self.to_addr.eq(address)
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}
