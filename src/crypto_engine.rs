use cast5::Cast5;
use cipher::{KeyInit, generic_array::GenericArray, BlockEncrypt};

pub fn use_cast5(key: &[u8]) -> Result<(), String> {
    let mut key_buf = [0u8; 16];
    let take = std::cmp::min(key.len(), 16);
    key_buf[..take].copy_from_slice(&key[..take]);

    let mut block = GenericArray::clone_from_slice(&key_buf);
    //SINK
    let _cipher = Cast5::new(GenericArray::from_slice(b"16byteslongkey!!")).encrypt_block(&mut block);
    Ok(())
}
