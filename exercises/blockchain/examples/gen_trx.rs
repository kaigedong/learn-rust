use std::{env::current_dir, sync::Arc};

use blockchain::{Blockchain, SledDb, Transaction, UTXOSet};

fn main() {
    tracing_subscriber::fmt().init();

    let bobo_addr = "Bobo";
    let bob_addr = "Bob";

    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(SledDb::new(path));

    let mut bc = Blockchain::new(storage.clone(), bobo_addr);
    let utxos = UTXOSet::new(storage);

    let tx_1 = Transaction::new_utxo(bobo_addr, bob_addr, 4, &utxos);

    let txs = vec![tx_1];

    bc.mine_block(&txs);
    utxos.reindex(&bc).unwrap();

    bc.blocks_info();
}
