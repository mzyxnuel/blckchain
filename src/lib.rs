use std::fmt;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64) -> Self {
        Transaction {
            from,
            to,
            amount,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now(),
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        
        block.mine_block(4);
        block
    }

    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp.timestamp(),
            serde_json::to_string(&self.transactions).unwrap(),
            self.previous_hash,
            self.nonce
        );
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        loop {
            let hash = self.calculate_hash();
            if hash.starts_with(&target) {
                self.hash = hash;
                break;
            }
            self.nonce += 1;
        }
        
        println!("Block mined: {}", self.hash);
    }

    pub fn genesis() -> Self {
        Block {
            index: 0,
            timestamp: Utc::now(),
            transactions: vec![],
            previous_hash: "0".to_string(),
            nonce: 0,
            hash: "0".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()],
            difficulty: 4,
            pending_transactions: vec![],
            mining_reward: 100.0,
        }
    }

    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        let reward_transaction = Transaction::new(
            "System".to_string(),
            mining_reward_address,
            self.mining_reward,
        );
        self.pending_transactions.push(reward_transaction);

        let block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            self.get_latest_block().hash.clone(),
        );

        self.chain.push(block);
        self.pending_transactions.clear();
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from == address {
                    balance -= transaction.amount;
                }
                if transaction.to == address {
                    balance += transaction.amount;
                }
            }
        }

        balance
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, block) in self.chain.iter().enumerate() {
            writeln!(f, "=== BLOCK {} ===", i)?;
            writeln!(f, "Hash: {}", block.hash)?;
            writeln!(f, "Previous Hash: {}", block.previous_hash)?;
            writeln!(f, "Timestamp: {}", block.timestamp)?;
            writeln!(f, "Nonce: {}", block.nonce)?;
            writeln!(f, "Transactions: {}", block.transactions.len())?;
            for tx in &block.transactions {
                writeln!(f, "  {} -> {}: {} coins", tx.from, tx.to, tx.amount)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}