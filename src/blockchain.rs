pub mod Blockcahin {
    

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
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

}