use super::deck::Deck;
use super::players::OtherPlayer;

pub struct PreGameConfiguration {
    pub max_number_players: u8,
    pub min_number_players: u8,

    pub game_name: String,
    pub game_type_name: String,

    pub starting_deck: Deck,

}

pub struct GameConfiguration {
    pub player_order: Vec<String>,
    pub threashold: u8,
}



pub struct AgreedSharedGameState {
    pub pregame_configuration: PreGameConfiguration,
    pub game_configuration: GameConfiguration,
    pub players: Vec<OtherPlayer>,
}

pub struct SharedGameState {
    pub turn: String,
    pub deck: Deck,
}




#[cfg(test)]
mod tests {
    use crate::logic::{deck::{encryption::get_encrypted_card_nonce}, encryption::private::{PrivatePlayerGameState, ShareRequest}, players::{MyPlayer, MyPlayerConfiguration}};

    use super::*;

    macro_rules! test_case {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/", $fname).to_string()
      )}

    #[test]
    fn load_alice() {
        let alice_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/alice.json"), profile_public_key: None, name: "alice".into() };
        let john_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/john.json"), profile_public_key: None, name: "john".into() };

        let alice = MyPlayer::load(alice_config);
        let john = MyPlayer::load(john_config);

        let pre_game_configuraton = PreGameConfiguration {
            game_name: "4color".into(),
            game_type_name: "regular".into(),
            max_number_players: 4,
            min_number_players: 2,
            starting_deck: Deck::create_default_deck(),
        };

        let game_configuration = GameConfiguration {
            player_order: vec![alice.name.clone(), john.name.clone()],
            threashold: 2,
        };

        let alice_other_player = alice.to_other_player();
        let john_other_player = john.to_other_player();



        let agreed_shared_game_state = AgreedSharedGameState {
            game_configuration: game_configuration,
            pregame_configuration: pre_game_configuraton,
            players: vec![alice.to_other_player(), john.to_other_player()],
        };


        let mut shared_game_state = SharedGameState {
            turn: alice.name.clone(),
            deck: Deck::default()
        };

        let mut alice_game_state = PrivatePlayerGameState::new(&alice, &vec![john_other_player]);
        let mut john_game_state = PrivatePlayerGameState::new(&john, &vec![alice_other_player]);

        let alice_indexes: Vec<u32> = vec![0, 1, 2, 3, 4];
        let john_indexes: Vec<u32> = vec![5, 6, 7, 8, 9 ];

        let starting_deck = agreed_shared_game_state.pregame_configuration.starting_deck.cards.clone();

        let alice_starting_data = alice_game_state.generate_starting_data(
            &starting_deck, 
            &alice_indexes,
            agreed_shared_game_state.game_configuration.threashold,
        ).unwrap();

        let john_starting_data = john_game_state.generate_starting_data(
            &starting_deck, 
            &john_indexes,
            agreed_shared_game_state.game_configuration.threashold,
        ).unwrap();

        shared_game_state.deck.add_encrypted_card_from_player(&mut alice_starting_data.1.clone());
        shared_game_state.deck.add_encrypted_card_from_player(&mut john_starting_data.1.clone());


        alice_game_state.add_other_player_starting_data(&john.name, john_starting_data.0.get(&alice.name).unwrap()).unwrap();
        john_game_state.add_other_player_starting_data(&alice.name, alice_starting_data.0.get(&john.name).unwrap()).unwrap();


        assert_eq!(agreed_shared_game_state.pregame_configuration.starting_deck.cards.len(), shared_game_state.deck.cards.len());

        assert_eq!(agreed_shared_game_state.pregame_configuration.starting_deck.cards.len(), alice_game_state.parts.len());
        assert_eq!(agreed_shared_game_state.pregame_configuration.starting_deck.cards.len(), john_game_state.parts.len());


        // Each player does this but only the one picking will keep them and be able to decypt them
        let encrypted_draw_cards = shared_game_state.deck.draw_cards(1).unwrap();

        // Alice is the one drawing so she ask for decryption
        let share_request = ShareRequest {
            player: alice.name.clone(),
            cards_nonce: encrypted_draw_cards.iter().map(|c| get_encrypted_card_nonce(c).unwrap()).collect(),
        };

        let john_share_response_for_alice = john_game_state.give_other_player_your_shares(&share_request).unwrap();

        alice_game_state.add_other_player_shares(&john.name, &john_share_response_for_alice).unwrap();


        for encrypted_draw_card in encrypted_draw_cards.iter() {

            let draw_card = alice_game_state.read_encrypted_card(encrypted_draw_card).unwrap();

            println!("draw card: {:?}", draw_card);
        }

    }

}