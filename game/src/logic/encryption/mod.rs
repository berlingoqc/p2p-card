

use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM for symmetric encryption
use aes_gcm::aead::{Aead, AeadMut,};
use x25519_dalek::{StaticSecret, PublicKey, SharedSecret};
use aes_gcm::KeyInit;

fn encrypt(nonce: u32, plaintext: &[u8], key: &SharedSecret) -> Result<Vec<u8>, ()> {
    let key_bytes = key.as_bytes();
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let counter_bytes = nonce.to_le_bytes(); // [42, 0, 0, 0]

    // Step 2: Pad to 12 bytes (Nonce size for AES-GCM)
    let mut nonce_bytes = [0u8; 12]; // Initialize a 12-byte array
    nonce_bytes[..4].copy_from_slice(&counter_bytes); // Copy the u32 bytes into the first 4 bytes

    // Step 3: Create a Nonce
    let nonce = Nonce::from_slice(&nonce_bytes); // Borrow the byte array

    let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();

    let mut output = nonce.to_vec();
    output.extend(ciphertext);
    
    Ok(output)

}

fn decrypt(ciphertext: &[u8], key: &SharedSecret) -> Result<Vec<u8>, ()> {
    let key_bytes = key.as_bytes();
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);

    let cipher = Aes256Gcm::new(&key);

   let nonce = Nonce::from_slice(&ciphertext[..12]);

   Ok(
    cipher.decrypt(nonce, &ciphertext[12..]).unwrap()
   )
 
}


pub struct SealingKey {
    shared_secret: SharedSecret,
}


impl SealingKey {

    pub fn create(sender_private: &StaticSecret, other: PublicKey) -> SealingKey {
        let shared_secret = sender_private.diffie_hellman(&other);

        SealingKey {
            shared_secret: shared_secret,
        }
    }

    pub fn encrypt(&self, nonce: u32, data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        encrypt(nonce, data.as_slice(), &self.shared_secret)
    }

    pub fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        decrypt( &data, &self.shared_secret)
    }
}

#[cfg(test)]
mod tests {

    use rand::rngs::OsRng;

    use crate::logic::players::{key_loader::get_key_loader, MyPlayerConfiguration};

    use super::*;

        macro_rules! test_case {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/", $fname).to_string()
      )}


    #[test]
    fn load_alice() {

        let alice_secret = StaticSecret::random_from_rng(OsRng);
        let alice_public = PublicKey::from(&alice_secret);

        let (john_public, john_secret) = get_key_loader(&MyPlayerConfiguration{
            name: "".into(),
            wallet_path: test_case!("/wallets/john.json"),
        }).unwrap().load_key_pair().unwrap();

 
        let sealing_key_alice = SealingKey::create(&alice_secret, john_public.clone());
        let sealing_key_john = SealingKey::create(&john_secret, alice_public.clone());


        let test_data = b"william quintal".to_vec();

        let encrypted_data = sealing_key_alice.encrypt(14, &test_data).unwrap();

        let decrypted_data = sealing_key_john.decrypt(&encrypted_data).unwrap();

        assert_eq!(test_data, decrypted_data);
    }
}