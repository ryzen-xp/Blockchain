use sha2::{Sha256, Digest}; // SHA-256 hashing
use chrono::Utc; // Timestamp
use serde::{Serialize, Deserialize}; // JSON serialization

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {  
    index: u64,
    timestamp: i64,
    previous_hash: String,
    hash: String,
    data: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            hash: String::new(),
            data,
            nonce: 0,
        };
        block.mine_block(5); // Adjust difficulty here
        block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

#[derive(Debug)]
struct Blockchain {  
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    println!("Mining block 1...");
    blockchain.add_block("Alice pays Bob 10 BTC".to_string());

    println!("Mining block 2...");
    blockchain.add_block("Bob pays Charlie 5 BTC".to_string());

    for block in &blockchain.chain {
        println!("{:?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}
