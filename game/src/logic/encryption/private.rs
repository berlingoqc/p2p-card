use std::collections::HashMap;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM for symmetric encryption
use aes_gcm::aead::{Aead, AeadMut,};
use rand::seq::SliceRandom;
use ssss::{gen_shares, unlock, SsssConfig};
use x25519_dalek::{StaticSecret, PublicKey};
use rand::Rng;
use aes_gcm::KeyInit;

use crate::logic::{deck::encryption::{encrypt_cards_from_deck, encrypt_shares_for_player, get_encrypted_card_nonce, to_encrypted_card, to_own_shared, EncryptedCard, EncryptedCards, SharedPrivateCardShares}, players::{MyPlayer, OtherPlayer}};

use super::{decrypt_with_key, SealingKey};

pub struct PrivatePlayerGameState {
    // My parts for each of the card , the id is the noune
    pub parts: HashMap<[u8; 12], Vec<String>>,
    pub sealing_keys: HashMap<u64, SealingKey>,
}

pub struct ShareRequest {
    pub player: u64,
    pub cards_nonce: Vec<[u8; 12]>,
}

pub struct ShareResponse {
    pub cards_nonce: HashMap<[u8; 12], Vec<u8>>
}

impl PrivatePlayerGameState {

    // Create the private game state , creating encryption keys for each other player
    pub fn new(my_player: &MyPlayer, other_players: &Vec<OtherPlayer>) -> Self {

        let mut sealing_keys = HashMap::new();
        for player in other_players.iter() {
            sealing_keys.insert(player.hash, SealingKey::create(&my_player.private, player.pub_key.clone()));
        }

        Self {
            parts: HashMap::new(),
            sealing_keys
        }
    }


    // When game is starting this will generate the part of the deck from this player
    pub fn generate_starting_data(&mut self, starting_deck: &Vec<Vec<u8>>, indexes: &Vec<u32>, threshold: u8) -> Result<(
        HashMap<u64, SharedPrivateCardShares>,
        EncryptedCards,
    ), ()> {
        let mut private_encrypted_cards = encrypt_cards_from_deck(starting_deck, indexes, (self.sealing_keys.len() + 1) as u8, threshold)?;

        let mut players_shares: HashMap<u64, SharedPrivateCardShares> = HashMap::with_capacity(self.sealing_keys.len());

        for player_id in self.sealing_keys.keys() {
            let sealing_key = self.sealing_keys.get(player_id).unwrap();
            players_shares.insert(*player_id, encrypt_shares_for_player(&mut private_encrypted_cards, sealing_key)?);
        }

        self.parts = to_own_shared(&private_encrypted_cards);

        Ok((
            players_shares,
            private_encrypted_cards.iter().map(to_encrypted_card).collect(),
        ))

    }

    // Add the data receive by the other player , your share for the card the generated
    pub fn add_other_player_starting_data(&mut self, other_player: u64, shared: &SharedPrivateCardShares) -> Result<(), ()> {

        for (nonce, cipher) in shared.iter() {

            let sealing_key = self.sealing_keys.get(&other_player).unwrap();


            let my_share = String::from_utf8(sealing_key.decrypt(cipher)?).unwrap();


            self.parts.insert(nonce.clone(), vec![my_share]);

        }

        Ok(())
    }


    pub fn give_other_player_your_shares(&mut self, share_request: &ShareRequest) -> Result<ShareResponse, ()> {
        let sealing_key = self.sealing_keys.get(&share_request.player).unwrap();

        let items = share_request.cards_nonce.iter()
            .map(|nonce| {
                let share = self.parts.get(nonce).unwrap().get(0).unwrap();

                (
                    nonce.clone(),
                    sealing_key.encrypt(nonce, &share.as_bytes().to_vec()).unwrap()
                )
            }).collect();

        Ok(ShareResponse { cards_nonce: items })
    }

    pub fn add_other_player_shares(&mut self, other_player: u64, share_response: &ShareResponse) -> Result<(), ()> {
        let sealing_key = self.sealing_keys.get(&other_player).unwrap();

        for (nonce, encrypted_share) in share_response.cards_nonce.iter() {
            let parts = self.parts.get_mut(nonce).unwrap();
            let share = sealing_key.decrypt(encrypted_share).unwrap();
            let share = String::from_utf8(share).unwrap();
            parts.push(share);
        }

        Ok(())
    }
    
    pub fn read_encrypted_card(&self, card: &EncryptedCard) -> Result<Vec<u8>, ()> {
        let nonce = get_encrypted_card_nonce(card).unwrap();

        let shares = self.parts.get(&nonce).unwrap();

        let symmetric_key = unlock(shares).unwrap();

        let card = decrypt_with_key(&card, &Key::<Aes256Gcm>::from_slice(&symmetric_key)).unwrap();

        Ok(card)
    }
}
