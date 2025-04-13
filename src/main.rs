pub mod  block;
pub mod  blockchain;
use crate::blockchain::Blockchain;

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

