
use ring::aead::Aad;
use ring::error::Unspecified;

use ring::aead::NonceSequence;
use ring::aead::NONCE_LEN;
use ring::aead::Nonce;
use x25519_dalek::PublicKey;
use x25519_dalek::StaticSecret;

use ring::aead::AES_256_GCM;
use ring::aead::UnboundKey;
use ring::aead::BoundKey;


pub struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    // called once for each seal operation
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        //println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

        self.0 += 1; // advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}



pub struct SealingKey {
    sealing_key: ring::aead::SealingKey<CounterNonceSequence>
}


impl SealingKey {

    pub fn create(sender_private: &StaticSecret, other: PublicKey) -> SealingKey {
        let shared_secret = sender_private.diffie_hellman(&other);

        let shared_secret_bytes = shared_secret.to_bytes();
        let key = UnboundKey::new(&AES_256_GCM, &shared_secret_bytes).unwrap();
        let nonce_sequence = CounterNonceSequence(1);
        let sealing_key = ring::aead::SealingKey::new(key, nonce_sequence);

        SealingKey {
            sealing_key
        }
    }



    pub fn seal(&mut self, in_out: &mut Vec<u8>) -> () {
        self.sealing_key.seal_in_place_append_tag(Aad::empty(), in_out).unwrap();
    }
}
