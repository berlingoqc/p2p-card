use std::collections::HashMap;

use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM for symmetric encryption
use rand::distributions::Alphanumeric;
use rand::Rng;
use ssss::gen_shares;

use crate::logic::encryption::{encrypt_byte_key, SealingKey};

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
    cards
        .iter()
        .map(|p| (p.nonce.clone(), p.shares.clone()))
        .collect()
}

pub fn get_encrypted_card_nonce(ec: &EncryptedCard) -> Result<[u8; 12], ()> {
    let mut result = [0; 12];
    result.copy_from_slice(&ec[..12]);
    Ok(result)
}

pub fn generate_random_nonce() -> [u8; 12] {
    let mut rng = rand::thread_rng();
    let mut result = [0u8; 12];

    for i in 0..12 {
        result[i] = rng.sample(Alphanumeric).into();
    }

    result
}

// Encrypte all required card from the deck with a symmetric key and store it with the nonce and the shares
pub fn encrypt_cards_from_deck(
    startind_deck: &Vec<Vec<u8>>,
    indexes: &Vec<u32>,
    num_shares: u8,
    threshold: u8,
) -> Result<PrivateEncryptedCards, ()> {
    let mut rng = rand::rngs::OsRng;
    let mut config = ssss::SsssConfig::default();

    config.set_num_shares(num_shares);
    config.set_threshold(threshold);

    let mut encrypted_cards = Vec::with_capacity(indexes.len());

    for i in indexes.iter() {
        let symmetric_key: [u8; 32] = rng.gen();
        let nonce_og = generate_random_nonce();

        let ciphertext = encrypt_byte_key(
            &nonce_og,
            startind_deck.get(*i as usize).unwrap().as_slice(),
            &symmetric_key,
        )
        .unwrap();

        encrypted_cards.push(PrivateEncryptedCard {
            card: ciphertext,
            shares: gen_shares(&config, &symmetric_key).unwrap(),
            nonce: nonce_og,
        });
    }

    Ok(encrypted_cards)
}

// Encrypt all the shares for a player from the card we handle and remove them from our list
pub fn encrypt_shares_for_player(
    cards: &mut PrivateEncryptedCards,
    sealing_key: &SealingKey,
) -> Result<SharedPrivateCardShares, ()> {
    let mut map = SharedPrivateCardShares::with_capacity(cards.len());
    for card in cards.iter_mut() {
        if card.shares.len() == 1 {
            eprintln!("only my share remaining");
            return Err(());
        }
        let next_share = card.shares.pop().unwrap();

        let ciphertext = sealing_key
            .encrypt(&card.nonce, &next_share.as_bytes().to_vec())
            .unwrap();

        map.insert(card.nonce.clone(), ciphertext);
    }

    return Ok(map);
}
