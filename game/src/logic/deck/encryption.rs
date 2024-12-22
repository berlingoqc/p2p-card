use rand_chacha::ChaCha8Rng;
use sha2::{Sha256, Digest};

use x25519_dalek::{PublicKey, StaticSecret};


use rand::{seq::SliceRandom, Rng, SeedableRng};

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

fn encrypt_card(card: &mut Vec<u8>, player: &mut MyPlayer) {
    for p in player.other_players.iter_mut() {
        p.sealing_key.seal(card);
    }
}


pub type EncryptedCard = Vec<u8>;



impl Deck {
    pub fn encrypt_deck(&self, player: &mut MyPlayer, accepted_order: Vec<String>) -> Deck {

        let mut cards = self.cards.clone();

        for card in cards.iter_mut() {
            encrypt_card(card, player);
        }

        Deck { cards: cards, encryption_order: accepted_order }
    }

    pub fn shuffle(&self, agreed_seed: u64) -> Deck {
        let mut rng = ChaCha8Rng::seed_from_u64(agreed_seed);

        let mut deck = Deck { encryption_order: self.encryption_order.clone(), cards: self.cards.clone()};

        deck.cards.shuffle(&mut rng);

        deck

    }
}