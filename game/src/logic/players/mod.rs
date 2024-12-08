use std::{fs::File, io::{BufReader, Read}};

use solana_sdk::{pubkey::Pubkey, signer::keypair::Keypair};

use rand::prelude::*;
use x25519_dalek::PublicKey;


pub struct MyPlayerConfiguration {
    pub wallet_path: String,
    pub seed: u64,
    pub name: String,
}

pub struct OtherPlayer {
    pub name: String,
    pub pubkey: Pubkey,
}

pub struct MyPlayer {
    pub keypair: Keypair, 
    pub seed: u64,
    pub name: String,


    pub other_players: Vec<OtherPlayer>
}


impl Default for MyPlayerConfiguration {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        MyPlayerConfiguration { wallet_path: String::new(), seed: rng.gen(), name: String::new() }
    }
}


impl MyPlayer {

    pub fn load_locally(config: MyPlayerConfiguration) -> Self {


        let file = File::open(config.wallet_path.as_str()).unwrap();
        let reader = BufReader::new(file);


        let keypair_vec: Vec<u8> = serde_json::from_reader(reader).unwrap();
        let keypair = Keypair::from_bytes(&keypair_vec).unwrap();

        MyPlayer {
            keypair: keypair,
            seed: config.seed,
            name: config.name,
            other_players: vec![]
        }
    }


    pub fn add_other_player(&mut self, name: String, pubkey: Pubkey) {
        self.other_players.push(OtherPlayer { name, pubkey })
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! test_case {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/", $fname).to_string()
      )}

    #[test]
    fn load_alice() {
        let alice_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/alice.json"), name: "alice".into(), seed: 10 };

        let alice = MyPlayer::load_locally(alice_config);

        assert_eq!(10, alice.seed);
    }

}