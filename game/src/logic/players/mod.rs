pub mod key_loader;

use key_loader::get_key_loader;
use x25519_dalek::{PublicKey, StaticSecret};




pub struct MyPlayerConfiguration {
    pub profile_public_key: Option<[u8; 32]>,
    pub wallet_path: String,
    pub name: String,
}


pub struct OtherPlayer {
    pub name: String,

    // Profile public_key
    pub profile_public_key: PublicKey,

    // Use for encryption
    pub pub_key: PublicKey,
}

pub struct MyPlayer {
    pub name: String,

    // Profile public key
    pub profile_public_key: PublicKey,

    // Use for encryption
    pub pub_key: PublicKey,
    pub private: StaticSecret,
}


impl Default for MyPlayerConfiguration {
    fn default() -> Self {
        MyPlayerConfiguration { wallet_path: String::new(), name: String::new(), profile_public_key: None }
    }
}


impl MyPlayer {

    pub fn load(config: MyPlayerConfiguration) -> Self {

        let (pub_key, secret) = get_key_loader(&config).unwrap().load_key_pair().unwrap();

        let profile_public_key = config.profile_public_key.map(|d| PublicKey::from(d)).or_else(|| Some(pub_key.clone())).unwrap();

        MyPlayer {
            private:secret,
            pub_key: pub_key,
            profile_public_key: profile_public_key,
            name: config.name,
        }
    }

    pub fn to_other_player(&self) -> OtherPlayer {
        OtherPlayer {
            name: self.name.clone(),
            profile_public_key: self.profile_public_key.clone(),
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
        let alice_config = MyPlayerConfiguration { wallet_path: test_case!("/wallets/alice.json"), profile_public_key: None, name: "alice".into() };

        let _ = MyPlayer::load(alice_config);
    }

}