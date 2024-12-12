pub mod encryption;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use solana_sdk::signer::{EncodableKeypair, Signer};

use super::{cards::{color::CardColor, color_card::ColorCard}, players::MyPlayer};



pub struct DeckCard {
    number: u16,
    color: u16
}

pub struct DeckCardConfiguration {

}

pub struct StartingDeck {
    cards: Vec<DeckCard>
}

pub struct Deck {
    cards: Vec<u32>,


    encryption_order: Vec<String>
}



impl DeckCard {

    fn from_u32(value: u32) -> Self {
        DeckCard { number: (value >> 16) as u16, color: (value & 0xFFFF) as u16 }
    }

    fn to_u32(&self) -> u32 {
        ((self.number as u32) << 16) | (self.color as u32)
    }

}

impl StartingDeck {

    fn create_default_deck() -> Self {
        StartingDeck { cards: vec![
            DeckCard { number: 1, color: CardColor::RED as u16},
            DeckCard { number: 2, color: CardColor::RED as u16},
            DeckCard { number: 3, color: CardColor::RED as u16},
            DeckCard { number: 4, color: CardColor::RED as u16},
            DeckCard { number: 5, color: CardColor::RED as u16},
            DeckCard { number: 6, color: CardColor::RED as u16},
            DeckCard { number: 7, color: CardColor::RED as u16},
            DeckCard { number: 8, color: CardColor::RED as u16},
            DeckCard { number: 9, color: CardColor::RED as u16},
        ] }
    }
}




impl Deck {
    fn builder(startin_deck: StartingDeck) -> Self {
        Deck { cards: startin_deck.cards.iter().map(|c| c.to_u32()).collect(), encryption_order: vec![] }
    }
}




#[cfg(test)]
mod tests {

    use crate::logic::players::MyPlayerConfiguration;

    use super::*;

    macro_rules! test_case {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/", $fname).to_string()
      )}

    #[test]
    fn load_alice() {
        let alice_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/alice.json"), name: "alice".into(), seed: 10 };
        let john_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/john.json"), name: "jogn".into(), seed: 20 };
        let bob_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/bob.json"), name: "bob".into(), seed: 30 };
        let tom_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/tom.json"), name: "tom".into(), seed: 40 };

        let mut alice = MyPlayer::load_locally(alice_config);
        let mut john = MyPlayer::load_locally(john_config);
        let mut bob = MyPlayer::load_locally(bob_config);
        let mut tom = MyPlayer::load_locally(tom_config);


        alice.add_other_player(john.name.clone(), john.keypair.pubkey().clone());
        alice.add_other_player(bob.name.clone(), bob.keypair.pubkey().clone());
        alice.add_other_player(tom.name.clone(), tom.keypair.pubkey().clone());

        john.add_other_player(alice.name.clone(), alice.keypair.pubkey().clone());
        john.add_other_player(bob.name.clone(), bob.keypair.pubkey().clone());
        john.add_other_player(tom.name.clone(), tom.keypair.pubkey().clone());

        bob.add_other_player(john.name.clone(), john.keypair.pubkey().clone());
        bob.add_other_player(alice.name.clone(), alice.keypair.pubkey().clone());
        bob.add_other_player(tom.name.clone(), tom.keypair.pubkey().clone());

        tom.add_other_player(alice.name.clone(), alice.keypair.pubkey().clone());
        tom.add_other_player(bob.name.clone(), bob.keypair.pubkey().clone());
        tom.add_other_player(john.name.clone(), john.keypair.pubkey().clone());


        let starting_deck = StartingDeck::create_default_deck();

        let deck = Deck::builder(starting_deck);

        deck.encrypted_deck(&alice, vec![]);


        assert_eq!(3, alice.other_players.len());


    }

}