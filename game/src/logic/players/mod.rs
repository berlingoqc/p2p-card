pub mod key_loader;

use key_loader::get_key_loader;
use x25519_dalek::{PublicKey, StaticSecret};




pub struct MyPlayerConfiguration {
    pub wallet_path: String,
    pub name: String,
}


pub struct OtherPlayer {
    pub name: String,
    pub pub_key: PublicKey,
}

pub struct MyPlayer {
    pub name: String,

    pub pub_key: PublicKey,
    pub private: StaticSecret,
}


impl Default for MyPlayerConfiguration {
    fn default() -> Self {
        MyPlayerConfiguration { wallet_path: String::new(), name: String::new() }
    }
}


impl MyPlayer {

    pub fn load(config: MyPlayerConfiguration) -> Self {

        let (pub_key, secret) = get_key_loader(&config).unwrap().load_key_pair().unwrap();

        MyPlayer {
            private:secret,
            pub_key: pub_key,
            name: config.name,
        }
    }

    pub fn to_other_player(&self) -> OtherPlayer {
        OtherPlayer {
            name: self.name.clone(),
            pub_key: self.pub_key.clone(),
        }
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
        let alice_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/alice.json"), name: "alice".into() };

        let _ = MyPlayer::load(alice_config);
    }

}