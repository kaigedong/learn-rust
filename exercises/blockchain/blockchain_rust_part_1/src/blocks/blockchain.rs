use crate::Block;
use tracing::info;

// 现在将区块存储到内存中，后面会存储到KV数据库
pub struct Blockchain {
    // 区块集合
    blocks: Vec<Block>,
    // 区块的高度
    height: usize,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::create_genesis_block()],
            height: 0,
        }
    }

    // 挖矿就是将区块加入到链上
    pub fn mine_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let block = Block::new(data, prev_block.get_hash().as_str());
        self.blocks.push(block);
        self.height += 1;
    }

    // 将当前所有的区块打印出来
    pub fn blocks_info(&self) {
        for block in self.blocks.iter() {
            info!("{:#?}", block);
        }
    }
}
