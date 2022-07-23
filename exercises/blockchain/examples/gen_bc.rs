use blockchain::{Blockchain, SledDb};
use std::env::current_dir;

fn main() {
    tracing_subscriber::fmt().init();

    let path = current_dir().unwrap().join("data");
    let mut bc = Blockchain::new(SledDb::new(path));

    bc.mine_block("Justin -> Bob 2 btc");
    bc.mine_block("Justin -> Bruce 2 btc");

    bc.blocks_info();
}
