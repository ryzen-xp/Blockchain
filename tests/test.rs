#[cfg(test)]
mod test {

    use sha2::digest::block_buffer::Block;

    use super::*;
    use Blockchain::block::Block;

    #[test]
    pub fn test_new_block() {
        let index = 12_u64;
        let pre_hash = String::from("test");
        let  data = String::from("value");

        let block = Block::
        assert!(true, "you right");
    }
}
