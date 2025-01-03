pub mod logic;
pub mod random;
pub mod utils;

/*

use bev::MyPlayerResource;
use bevy::{prelude::*, time::common_conditions::on_timer, utils::Duration};
use logic::players::MyPlayer;

fn main() {

    let user_config = arg_parser::load_my_player_config().unwrap();
    let my_player = MyPlayer::load(user_config);

    let my_player_resource = MyPlayerResource {
        player: my_player,
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, network::start_socket)
        .add_systems(Update, network::receive_messages)
        .add_systems(
            Update,
            network::send_message.run_if(on_timer(Duration::from_secs(5))),
        )
        .insert_resource(my_player_resource)
        .run();
}
*/
/*
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM for symmetric encryption
use aes_gcm::aead::{Aead, AeadMut,};
use rand::seq::SliceRandom;
use ssss::{gen_shares, unlock, SsssConfig};
use x25519_dalek::{StaticSecret, PublicKey};
use rand::Rng;
use aes_gcm::KeyInit;



#[derive(Clone)]
struct Card {
    value: u32,
}

fn main() {

    let mut rng = rand::rngs::OsRng;

    let mut deck: Vec<Vec<u8>> = vec![b"cart -1".to_vec(), b"cart 0".to_vec(), b"cart 1".to_vec()];

    let encrypted_deck: Vec<(Vec<u8>, [u8; 32])> = deck.iter().map(|card| {
        let symmetric_key: [u8; 32] = rng.gen();
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&symmetric_key));
        let nonce = [0; 12];
        let nonce = Nonce::from_slice(&nonce); // Use random nonce

        (cipher
                .encrypt(nonce, card.as_slice())
                .expect("AES encryption failed"), symmetric_key)
    }).collect();

    let mut config = SsssConfig::default();
    config.set_num_shares(3);
    config.set_threshold(3);


    let mut shares: Vec<Vec<String>> = encrypted_deck.iter().map(|item| {
        let shares = gen_shares(&config, &item.1).unwrap();

        shares
    }).collect();


    for (i, (encrypted_card, _)) in encrypted_deck.iter().enumerate() {

        let mut shares = shares.get_mut(i).unwrap();
        shares.shuffle(&mut rng);
        //shares.remove(1);

        let symmetric_key = unlock(shares).unwrap();

        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&symmetric_key));
        let nonce = [0; 12];
        let nonce = Nonce::from_slice(&nonce); // Use random nonce

        let card = cipher.decrypt(&nonce, encrypted_card.as_slice()).unwrap();
        println!("Card {}", String::from_utf8(card).unwrap());

    }


}

*/
