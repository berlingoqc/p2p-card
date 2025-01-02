use super::deck::Deck;
use super::players::OtherPlayer;

use protocol::generated::card::GameConfiguration;

#[cfg(test)]
mod tests {
    use std::default;

    use protocol::generated::card::{AgreementDefinition, DeckEncyption, PlayerEncryption, PlayingArea, PlayingField, Vec3};

    use crate::logic::{deck::{encryption::get_encrypted_card_nonce}, encryption::private::{PrivatePlayerGameState, ShareRequest}, players::{MyPlayer, MyPlayerConfiguration}};

    use super::*;

    macro_rules! test_case {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/", $fname).to_string()
      )}

    #[test]
    fn load_alice() {
        let alice_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/alice.json"), position: [0.0; 3], profile_public_key: None, name: "alice".into() };
        let john_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/john.json"), position: [0.0; 3], profile_public_key: None, name: "john".into() };

        let alice = MyPlayer::load(alice_config);
        let john = MyPlayer::load(john_config);

        let alice_indexes: Vec<u32> = vec![0, 1, 2, 3, 4];
        let john_indexes: Vec<u32> = vec![5, 6, 7, 8, 9 ];

        let game_configuration = GameConfiguration {
            game_name: Some("4color".into()),
            game_type_name: Some("regular".into()),
            max_number_players: Some(4),
            min_number_players: Some(2),
            starting_deck: Deck::create_default_deck().cards,
            agreement_definition: Some(AgreementDefinition{ threashold: 2 }),
            deck_encryption: Some(DeckEncyption{
                resuffling_seed: Some(2151314124),
                reconstruction_order: vec![
                    alice.hash, john.hash,
                ],
                player_encryption: vec![
                    PlayerEncryption { player_name: alice.hash, indexes: alice_indexes },
                    PlayerEncryption { player_name: john.hash, indexes: john_indexes }
                ]
            }),
            playing_field: None,
            players: vec![
                alice.to_protoc_player(),
                john.to_protoc_player(),
            ],
            ..Default::default()
        };

        let alice_other_player = alice.to_other_player();
        let john_other_player = john.to_other_player();
        
        let mut alice_game_state = PrivatePlayerGameState::new(&alice, &vec![john_other_player]);
        let mut john_game_state = PrivatePlayerGameState::new(&john, &vec![alice_other_player]);

        let starting_deck = game_configuration.starting_deck.clone();

        let alice_starting_data = alice_game_state.generate_starting_data(
            &starting_deck, 
            &alice_indexes,
            game_configuration.agreement_definition.unwrap().threashold as u8,
        ).unwrap();

        let john_starting_data = john_game_state.generate_starting_data(
            &starting_deck, 
            &john_indexes,
            game_configuration.agreement_definition.unwrap().threashold as u8,
        ).unwrap();


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