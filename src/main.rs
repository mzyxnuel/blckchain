use blckchain::{Blockchain, Transaction};

fn main() {
    let mut blockchain = Blockchain::new();

    println!("Creating blockchain\n");

    blockchain.create_transaction(Transaction::new(
        "User1".to_string(),
        "User2".to_string(),
        50.0,
    ));

    blockchain.create_transaction(Transaction::new(
        "User2".to_string(),
        "User3".to_string(),
        25.0,
    ));

    println!("Starting mining block 1");
    blockchain.mine_pending_transactions("Miner".to_string());

    blockchain.create_transaction(Transaction::new(
        "User3".to_string(),
        "User1".to_string(),
        10.0,
    ));

    println!("Starting mining block 2");
    blockchain.mine_pending_transactions("Miner".to_string());

    println!("Blockchain state:");
    println!("{}", blockchain);

    println!("\nBalances:");
    println!("User1: {} coins", blockchain.get_balance("User1"));
    println!("User2: {} coins", blockchain.get_balance("User2"));
    println!("User3: {} coins", blockchain.get_balance("User3"));
    println!("Miner: {} coins", blockchain.get_balance("Miner"));

    println!("Blockchain valid: {}", blockchain.is_chain_valid());
}