pub fn vec_u32_to_u8(data: &[u32]) -> Vec<u8> {
    let capacity = data.len();
    let mut output = Vec::with_capacity(capacity);
    for value in data {
        let v: u8 = *value as u8;
        output.push(v);
    }
    output
}

pub fn splice<T>(v: &mut Vec<T>, start: usize, count: usize) -> Vec<T> {
    if start >= v.len() {
        return Vec::new(); // Handle out-of-bounds start index
    }

    let end = std::cmp::min(start + count, v.len()); // Clamp end index to vector length
    let removed = v.drain(start..end).collect();

    removed
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_to_u64(data: Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

use x25519_dalek::PublicKey;

pub fn get_pub_key_from_vec(data: Vec<u8>) -> Result<PublicKey, ()> {
    if data.len() != 32 {
        eprintln!("Invalid public key length");
        return Err(());
    }

    let key_bytes: [u8; 32] = data
        .try_into()
        .expect("Failed to convert Vec<u8> to [u8; 32]");

    Ok(PublicKey::from(key_bytes))
}
