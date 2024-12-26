use std::collections::HashMap;


use super::encryption::SealingKey;
use super::players::MyPlayer;
use super::{deck::StartingDeck, players::OtherPlayer};

pub struct PreGameConfiguration {
    pub max_number_players: u32,
    pub min_number_players: u32,

    pub game_name: String,
    pub game_type_name: String,

    pub starting_deck: StartingDeck,

}

pub struct GameConfiguration {
    pub player_order: Vec<String>,
    pub threashold: u32,
}

pub struct PrivatePlayerGameState {
    // My parts for each of the card , the id is the noune
    pub parts: HashMap<u32, Vec<u8>>,
    pub sealing_keys: HashMap<String, SealingKey>,

}


pub struct AgreedSharedGameState {
    pub pregame_configuration: PreGameConfiguration,
    pub game_configuration: GameConfiguration,
    pub players: Vec<OtherPlayer>,
}

pub struct SharedGameState {
    pub turn: String,
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

    fn encrypt_share_for_player(&self, card_nonce: u32, other_player: &OtherPlayer) -> Vec<u8> {
        let sealing_key = self.sealing_keys.get(&other_player.name).unwrap();

        sealing_key.encrypt(card_nonce, self.parts.get(&card_nonce).unwrap()).unwrap()

    }

    fn decrypt_share_from_player(&mut self, payload: Vec<u8>, other_player: &OtherPlayer) {
        let sealing_key = self.sealing_keys.get(&other_player.name).unwrap();

        //sealing_key.decrypt(card_nonce, self.parts.get(&card_nonce).unwrap()).unwrap() 
    }

    fn decrypt_symmetric_key(&mut self) -> Result<(), ()> {

        Ok(())
    }


}




#[cfg(test)]
mod tests {
    use crate::logic::players::{MyPlayer, MyPlayerConfiguration};

    use super::*;

    macro_rules! test_case {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/", $fname).to_string()
      )}

    #[test]
    fn load_alice() {
        let alice_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/alice.json"), name: "alice".into() };
        let john_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/john.json"), name: "john".into() };

        let alice = MyPlayer::load(alice_config);
        let john = MyPlayer::load(john_config);

        let pre_game_configuraton = PreGameConfiguration {
            game_name: "4color".into(),
            game_type_name: "regular".into(),
            max_number_players: 4,
            min_number_players: 2,
            starting_deck: StartingDeck::create_default_deck(),
        };

        let game_configuration = GameConfiguration {
            player_order: vec![alice.name.clone(), john.name.clone()],
            threashold: 4,
        };

        let alice_other_player = alice.to_other_player();
        let john_other_player = john.to_other_player();



        let agreed_shared_game_state = AgreedSharedGameState {
            game_configuration: game_configuration,
            pregame_configuration: pre_game_configuraton,
            players: vec![alice.to_other_player(), john.to_other_player()],
        };


        let shared_game_state = SharedGameState {
            turn: alice.name.clone(),
        };

        /*
        let mut alice_game_state = PrivatePlayerGameState::new(agreed_shared_game_state.game_configuration.threashold, &alice, &agreed_shared_game_state.players);
        let mut john_game_state = PrivatePlayerGameState::new(agreed_shared_game_state.game_configuration.threashold, &john, &agreed_shared_game_state.players);

        let encrypted_john_from_alice = alice_game_state.encrypt_share_for_player(&john_other_player);
        let encrypted_alice_from_john = john_game_state.encrypt_share_for_player(&alice_other_player);

        alice_game_state.store_share_from_player(encrypted_alice_from_john, &john_other_player);
        john_game_state.store_share_from_player(encrypted_john_from_alice, &alice_other_player);


        //assert_eq!(alice_game_state.shares.get("john").unwrap().their_share, john_game_state.shares.get("alice").unwrap().share);


        alice_game_state.calculate_final_share().unwrap();
        john_game_state.calculate_final_share().unwrap();
        */

    }

}