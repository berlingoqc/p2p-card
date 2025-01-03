use ed25519_dalek::{SecretKey as EdSecretKey, Signature};
use rand::rngs::OsRng;
use serde_json::Value;
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};
fn derive_x25519_from_ed25519(ed_secret: &EdSecretKey) -> StaticSecret {
    let mut x_secret_bytes = [0u8; 32];
    x_secret_bytes.copy_from_slice(ed_secret.as_bytes());
    StaticSecret::from(x_secret_bytes)
}

pub trait KeyLoader {
    fn load_key_pair(&self) -> Result<(XPublicKey, StaticSecret), ()>;
}

pub struct SolanaLocalFileKeyLoader {
    path: String,
}

impl SolanaLocalFileKeyLoader {
    pub fn create(path: String) -> Self {
        Self { path: path }
    }
}

pub struct RandomKeyLoader {}

impl KeyLoader for RandomKeyLoader {
    fn load_key_pair(&self) -> Result<(XPublicKey, StaticSecret), ()> {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = XPublicKey::from(&secret);

        Ok((public, secret))
    }
}

// TODO if plateform pas web
use std::{fs::File, io::BufReader};

use super::MyPlayerConfiguration;

impl KeyLoader for SolanaLocalFileKeyLoader {
    fn load_key_pair(&self) -> Result<(XPublicKey, StaticSecret), ()> {
        let file = File::open(self.path.as_str()).unwrap();
        let reader = BufReader::new(file);

        let json: Value = serde_json::from_reader(reader).unwrap();
        let keypair_array = json.as_array().unwrap();

        if keypair_array.len() != 64 {
            return Err(());
        }

        let secret_key_bytes: Vec<u8> = keypair_array
            .iter()
            .map(|v| v.as_u64().unwrap() as u8)
            .collect();

        let secret_key = EdSecretKey::from_bytes(&secret_key_bytes[..32]).unwrap();
        let secret_key = derive_x25519_from_ed25519(&secret_key);
        Ok(((&secret_key).into(), secret_key))
    }
}

pub fn get_key_loader(player_config: &MyPlayerConfiguration) -> Result<Box<dyn KeyLoader>, ()> {
    if (player_config.wallet_path.is_empty()) {
        Ok(Box::new(RandomKeyLoader {}))
    } else {
        Ok(Box::new(SolanaLocalFileKeyLoader::create(
            player_config.wallet_path.clone(),
        )))
    }
}
