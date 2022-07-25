use crate::{error::BlockchainError, Block};

mod sleddb;
pub use sleddb::SledDb;

// 数据库的结构
// Key              Value
// tip_hash 	    区块链中最后加入块的hash值
// height           区块链中的高度
// blocks:{hash}    区块的hash值，blocks为前缀。
pub const TIP_KEY: &str = "tip_hash";
pub const HEIGHT: &str = "height";
pub const TABLE_OF_BLOCK: &str = "blocks";

// 考虑到存储的可扩展性，将来使用其他KV数据库，如：RocksDB。
// 我们定义了storage trait。
pub trait Storage: Send + Sync + 'static {
    fn get_tip(&self) -> Result<Option<String>, BlockchainError>;
    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError>;
    fn get_height(&self) -> Result<Option<usize>, BlockchainError>;
    fn update_blocks(&self, key: &str, block: &Block, height: usize);
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError>;
}

pub struct StorageIterator<T> {
    data: T,
}

impl<T> StorageIterator<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIterator<T>
where
    T: Iterator,
    T::Item: Into<Block>,
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}
