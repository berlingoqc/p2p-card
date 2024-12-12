use ring::error::Unspecified;
use sha2::{Sha256, Digest};
use solana_sdk::{pubkey::Pubkey, signer::keypair::Keypair};

use solana_sdk::signature::{Signer};
use x25519_dalek::{PublicKey, StaticSecret};
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use ring::aead::Algorithm;
use ring::aead::AES_128_GCM;
use ring::aead::AES_256_GCM;
use ring::aead::CHACHA20_POLY1305;
use ring::aead::UnboundKey;
use ring::aead::BoundKey;
use ring::aead::SealingKey;
use ring::aead::OpeningKey;
use ring::aead::Aad;
use ring::aead::Tag;
use ring::aead::NonceSequence;
use ring::aead::NONCE_LEN;
use ring::aead::Nonce;
use rand::rngs::OsRng;

use rand::Rng;

use crate::logic::players::MyPlayer;

use super::Deck;

fn generate_random_nonce() -> u64 {
    rand::thread_rng().gen()
}


fn generate_commitment(card: u32, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", card, nonce));
    format!("{:x}", hasher.finalize())
}

struct CounterNonceSequence(u32);

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

fn encrypt_card(card: u32, player: &MyPlayer) -> Vec<u8> {

    let sender_private = StaticSecret::from(player.keypair.secret().to_bytes());


    let mut in_out = b"heelo world".to_vec();

    for p in player.other_players.iter() {
        let recipient_x25519_pubkey = PublicKey::from(p.pubkey.to_bytes());

        let shared_secret = sender_private.diffie_hellman(&recipient_x25519_pubkey);

        let shared_secret_bytes = shared_secret.to_bytes();
        let key = UnboundKey::new(&AES_256_GCM, &shared_secret_bytes).unwrap();
        let nonce_sequence = CounterNonceSequence(1);
        let mut sealing_key = SealingKey::new(key, nonce_sequence);

        sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out).unwrap();

        println!("data {:?}", in_out);
    }



    vec![]
}

impl Deck {
    pub fn encrypted_deck(&self, player: &MyPlayer, accepted_order: Vec<String>) -> Deck {

        let mut order = self.encryption_order.clone();
        //order.push(player.keypair.pubkey().to_string());

        let mut cards = self.cards.clone();

        for card in self.cards.iter() {
            encrypt_card(*card, &player);

        }

        Deck { cards: cards, encryption_order: order }
    }
}