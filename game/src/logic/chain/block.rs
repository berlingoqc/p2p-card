use protocol::generated::chain::Block;
use sha2::{Sha256, Digest};
use prost::Message;

use base64::Engine;

pub fn calculate_hash_block(block: &Block) -> Result<String, ()> {

    let block_payload = block.encode_to_vec();

    let mut hasher = Sha256::new();
    hasher.update(block_payload);

    let hash = hasher.finalize().to_vec();

    let data: String = base64::engine::general_purpose::STANDARD.encode(&hash);

    Ok(data)
}