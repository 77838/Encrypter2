use serde_derive::{Serialize, Deserialize};
use super::*;
#[derive(Serialize, Deserialize,Debug)]
pub struct Secret{
    key: String,
    iv: String,
}

impl Secret {
    pub fn generate_secret() -> Secret{

        let mut key =  [0u8;16];
        thread_rng().try_fill(&mut key[..]).expect("failed to generate key");
        // let mut nonce = [0u8;24];
        // thread_rng().try_fill(&mut nonce[..]).expect("failed to generate Nonce");
        let mut iv = [0u8; 16];
        thread_rng().try_fill(&mut iv[..]).expect("failed to generate iv");

        Secret { key: base64::encode(key) , iv : base64::encode(iv)}
    }

    pub fn keys(&self) -> &String{
        &self.key
    }
    pub fn iv(&self) -> &String{
        &self.iv
    }
}
