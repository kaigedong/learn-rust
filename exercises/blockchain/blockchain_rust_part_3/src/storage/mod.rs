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
    // 获取最后一个块的hash值
    fn get_tip(&self) -> Result<Option<String>, BlockchainError>;
    // 获取一个区块
    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError>;
    // 获取区块链的高度
    fn get_height(&self) -> Result<Option<usize>, BlockchainError>;
    // 以事务的方式更新区块链
    fn update_blocks(&self, key: &str, block: &Block, height: usize);
    // 获取区块的迭代器
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError>;
}

// 定义区块的迭代器
pub struct StorageIterator<T> {
    data: T,
}

impl<T> StorageIterator<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

// T泛型需要满足Iterator约束
// T的item类型需要满足能转换成Block
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
