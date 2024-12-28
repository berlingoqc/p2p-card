pub mod private;

use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM for symmetric encryption
use aes_gcm::aead::{Aead, AeadMut,};
use x25519_dalek::{StaticSecret, PublicKey, SharedSecret};
use aes_gcm::KeyInit;




fn encrypt(nonce: &[u8; 12], plaintext: &[u8], key: &SharedSecret) -> Result<Vec<u8>, ()> {
    let key_bytes = key.as_bytes();
    encrypt_byte_key(nonce, plaintext, key_bytes)
}

pub fn encrypt_byte_key(nonce: &[u8; 12], plaintext: &[u8], key_bytes: &[u8; 32]) -> Result<Vec<u8>, ()> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let nonce = Nonce::from_slice(nonce); // Borrow the byte array

    let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();

    let mut output = nonce.to_vec();
    output.extend(ciphertext);
    
    Ok(output)
}



fn decrypt(ciphertext: &[u8], key: &SharedSecret) -> Result<Vec<u8>, ()> {
    let key_bytes = key.as_bytes();
    decrypt_with_key(ciphertext, Key::<Aes256Gcm>::from_slice(key_bytes))
}

pub fn decrypt_with_key(ciphertext: &[u8], key: &Key::<Aes256Gcm>) -> Result<Vec<u8>, ()> {

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

    pub fn encrypt(&self, nonce: &[u8; 12], data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        encrypt(nonce, data.as_slice(), &self.shared_secret)
    }

    pub fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, ()> {
        decrypt( &data, &self.shared_secret)
    }
}

#[cfg(test)]
mod tests {

    use rand::{rngs::OsRng, Rng};

    use crate::logic::{deck::encryption::{generate_random_nonce, get_encrypted_card_nonce}, players::{key_loader::get_key_loader, MyPlayerConfiguration}};

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


        let nonce: [u8; 12] = generate_random_nonce();
        let test_data = b"william quintal".to_vec();

        let encrypted_data = sealing_key_alice.encrypt(&nonce, &test_data).unwrap();

        let encrypted_data_nonce = get_encrypted_card_nonce(&encrypted_data).unwrap();


        assert_eq!(nonce, encrypted_data_nonce);

        let decrypted_data = sealing_key_john.decrypt(&encrypted_data).unwrap();

        assert_eq!(test_data, decrypted_data);
    }
}