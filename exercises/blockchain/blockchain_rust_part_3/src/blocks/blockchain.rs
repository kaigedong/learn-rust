use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

use tracing::info;

use crate::{Block, SledDb, Storage};

pub const CURR_BITS: usize = 8;

// 默认使用sled数据库
pub struct Blockchain<T = SledDb> {
    storage: T,
    // 最后加入块的Hash值
    tip: Arc<RwLock<String>>,
    height: AtomicUsize,
}

impl<T: Storage> Blockchain<T> {
    pub fn new(storage: T) -> Self {
        // 如果数据库中有tip值，则加载到内存。
        // 否则创建一个创世块，并更新到数据库中。
        if let Ok(Some(tip)) = storage.get_tip() {
            let height = storage.get_height().unwrap();
            Self {
                storage,
                tip: Arc::new(RwLock::new(tip)),
                height: AtomicUsize::new(height.unwrap()),
            }
        } else {
            let genesis_block = Block::create_genesis_block(CURR_BITS);
            let hash = genesis_block.get_hash();
            storage.update_blocks(&hash, &genesis_block, 0_usize);

            Self {
                storage,
                tip: Arc::new(RwLock::new(hash)),
                height: AtomicUsize::new(0),
            }
        }
    }

    pub fn mine_block(&mut self, data: &str) {
        let block = Block::new(data, &self.tip.read().unwrap(), CURR_BITS);
        let hash = block.get_hash();
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage
            .update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));

        let mut tip = self.tip.write().unwrap();
        *tip = hash;
    }

    pub fn blocks_info(&self) {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            info!("{:#?}", block);
        }
    }
}
