
use std::collections::HashMap;
use std::vec;

use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM for symmetric encryption
use aes_gcm::aead::{Aead, AeadMut,};
use ssss::gen_shares;
use x25519_dalek::{StaticSecret, PublicKey};
use rand::Rng;
use aes_gcm::KeyInit;

use crate::logic::encryption::SealingKey;


pub type EncryptedCard = Vec<u8>;

pub type EncryptedCards = Vec<EncryptedCard>;

pub struct PrivateEncryptedCard {
    pub card: EncryptedCard,
    pub nonce: [u8; 12],
    pub shares: Vec<String>,
}

pub type PrivateEncryptedCards = Vec<PrivateEncryptedCard>;

pub type SharedPrivateCardShares = HashMap<[u8; 12], Vec<u8>>;



pub fn to_encrypted_card(card: &PrivateEncryptedCard) -> EncryptedCard {
    card.card.clone()
}

pub fn to_own_shared(cards: &PrivateEncryptedCards) -> HashMap<[u8; 12], Vec<String>> {
    cards.iter().map(|p| (p.nonce.clone(), p.shares.clone())).collect()
}
    

// Encrypte all required card from the deck with a symmetric key and store it with the nonce and the shares
pub fn encrypt_cards_from_deck(startind_deck: &Vec<u32>, indexes: &Vec<u32>, num_shares: u8, threshold: u8) -> Result<PrivateEncryptedCards, ()> {
    let mut rng = rand::rngs::OsRng;
    let mut config = ssss::SsssConfig::default();

    config.set_num_shares(num_shares);
    config.set_threshold(threshold);

    let mut encrypted_cards = Vec::with_capacity(indexes.len());

    for i in indexes.iter() {
        let symmetric_key: [u8; 32] = rng.gen();
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&symmetric_key));
        let nonce_og = [rng.gen(); 12];
        let nonce = Nonce::from_slice(&nonce_og); 

        let ciphertext = cipher
            .encrypt(nonce, startind_deck.get(*i as usize).unwrap().to_le_bytes().as_slice())
            .expect("AES encryption failed");


        encrypted_cards.push(PrivateEncryptedCard {
            card: ciphertext,
            shares: gen_shares(&config, &symmetric_key).unwrap(),
            nonce: nonce_og,
        });
    }

    Ok(encrypted_cards)
}


// Encrypt all the shares for a player from the card we handle and remove them from our list
pub fn encrypt_shares_for_player(cards: &mut PrivateEncryptedCards, sealing_key: &SealingKey)  -> Result<SharedPrivateCardShares, ()> {
    let mut map = SharedPrivateCardShares::with_capacity(cards.len());
    for card in cards.iter_mut() {
        if card.shares.len() == 1 {
            eprintln!("only my share remaining");
            return Err(());
        }
        let next_share = card.shares.pop().unwrap();

        let ciphertext = sealing_key.encrypt(&card.nonce, &next_share.as_bytes().to_vec()).unwrap();

        map.insert(card.nonce.clone(), ciphertext);
    }


    return Ok(map);
}




/*
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
*/
