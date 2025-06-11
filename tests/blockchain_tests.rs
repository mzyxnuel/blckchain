use blckchain::{Blockchain, Transaction};

#[test]
fn test_blockchain_creation() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.chain.len(), 1);
    assert!(blockchain.is_chain_valid());
}

#[test]
fn test_mining_and_balance() {
    let mut blockchain = Blockchain::new();
    
    blockchain.create_transaction(Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        50.0,
    ));
    
    blockchain.mine_pending_transactions("Miner".to_string());
    
    assert_eq!(blockchain.get_balance("Bob"), 50.0);
    assert_eq!(blockchain.get_balance("Miner"), 100.0);
}

#[test]
fn test_multiple_transactions() {
    let mut blockchain = Blockchain::new();
    
    blockchain.create_transaction(Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        100.0,
    ));
    
    blockchain.create_transaction(Transaction::new(
        "Bob".to_string(),
        "Charlie".to_string(),
        50.0,
    ));
    
    blockchain.mine_pending_transactions("Miner".to_string());
    
    assert_eq!(blockchain.get_balance("Alice"), -100.0);
    assert_eq!(blockchain.get_balance("Bob"), 50.0);
    assert_eq!(blockchain.get_balance("Charlie"), 50.0);
    assert_eq!(blockchain.get_balance("Miner"), 100.0);
}

#[test]
fn test_chain_validation() {
    let mut blockchain = Blockchain::new();
    
    blockchain.create_transaction(Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        25.0,
    ));
    
    blockchain.mine_pending_transactions("Miner".to_string());
    
    assert!(blockchain.is_chain_valid());
    
    blockchain.chain[1].transactions[0].amount = 1000.0;
    assert!(!blockchain.is_chain_valid());
}

#[test]
fn test_genesis_block() {
    let blockchain = Blockchain::new();
    let genesis = &blockchain.chain[0];
    
    assert_eq!(genesis.index, 0);
    assert_eq!(genesis.previous_hash, "0");
    assert_eq!(genesis.transactions.len(), 0);
}