use blockchain_rust_part_1::Blockchain;

fn main() {
    tracing_subscriber::fmt().init();

    let mut bc = Blockchain::new();

    // 取出将前一个区块的Hash，构建一个新的区块，并放到上面bc里
    bc.mine_block("Justin -> Bob 2 btc");
    bc.mine_block("Justin -> Bruce 2 btc");

    // 打印所有区块的信息
    bc.blocks_info();
}
