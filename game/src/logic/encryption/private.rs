use std::collections::HashMap;

use bevy::prelude::Res;

use crate::logic::{deck::encryption::{encrypt_cards_from_deck, encrypt_shares_for_player, to_encrypted_card, to_own_shared, EncryptedCards, PrivateEncryptedCards, SharedPrivateCardShares}, players::{MyPlayer, OtherPlayer}};

use super::SealingKey;

pub struct PrivatePlayerGameState {
    // My parts for each of the card , the id is the noune
    pub parts: HashMap<[u8; 12], Vec<String>>,
    pub sealing_keys: HashMap<String, SealingKey>,
}

impl PrivatePlayerGameState {
    pub fn new(my_player: &MyPlayer, other_players: &Vec<OtherPlayer>) -> Self {

        let mut sealing_keys = HashMap::new();
        for player in other_players.iter() {
            sealing_keys.insert(player.name.clone(), SealingKey::create(&my_player.private, player.pub_key.clone()));
        }

        Self {
            parts: HashMap::new(),
            sealing_keys
        }
    }


    pub fn generate_starting_data(&mut self, starting_deck: &Vec<u32>, indexes: &Vec<u32>, threshold: u8) -> Result<(
        HashMap<String, SharedPrivateCardShares>,
        EncryptedCards,
    ), ()> {
        let mut private_encrypted_cards = encrypt_cards_from_deck(starting_deck, indexes, (self.sealing_keys.len() + 1) as u8, threshold)?;

        let mut players_shares: HashMap<String, SharedPrivateCardShares> = HashMap::with_capacity(self.sealing_keys.len());

        for player_name in self.sealing_keys.keys() {
            let sealing_key = self.sealing_keys.get(player_name).unwrap();
            players_shares.insert(player_name.clone(), encrypt_shares_for_player(&mut private_encrypted_cards, sealing_key)?);
        }

        self.parts = to_own_shared(&private_encrypted_cards);

        Ok((
            players_shares,
            private_encrypted_cards.iter().map(to_encrypted_card).collect(),
        ))

    }

    pub fn add_other_player_starting_data(&mut self, other_player: &String, shared: &SharedPrivateCardShares) -> Result<(), ()> {

        for (nonce, cipher) in shared.iter() {

            let sealing_key = self.sealing_keys.get(other_player).unwrap();


            let my_share = String::from_utf8(sealing_key.decrypt(cipher)?).unwrap();


            self.parts.insert(nonce.clone(), vec![my_share]);

        }


        Ok(())
    }



    // Basically the process is that,
    // Each card is encrypted with a random symmetric key and a random nonce
    // The symmetric key is encrypted with SSS to create a split secret shared amongst other player

    // So basically each player are assign a random set of card from the deck to encrypt and shuffle

}
