// mod args;
// use std::{fs::{self, File}, io::Write};

// use args::{CliAPP, UserSubCommand};
// use clap::Parser;
// mod encrypt;
// mod secret;
// use secret::Secret;
// use base64::*;
// use rand::*;
// use aes::Aes128;
// use block_modes::block_padding::Pkcs7;
// use block_modes::{BlockMode, Cbc};
// use hex as hex_;
// use hex_literal::hex;
// use std::str;


// fn main(){

// let key = hex!("000102030405060708090a0b0c0d0e0f");
//     println!("key : {:?}", key);
//     println!("key encoded : {:?}" , hex_::encode(key));
//     let iv = hex!("f0f1f2f3f4f5f6f7f8f9afbfcfdfeff");
//     println!("iv : {:?}", iv);
//     println!("iv encoded: {:?}" , hex_::encode(iv));
//     let plaintext = b"Hello world";
//     println!("plain text: {:?}", str::from_utf8(plaintext).unwrap());
    
//     let cipher = Aes128Cbc::new_from_slices(&key , &iv).unwrap();
//     let mut buffer = [0u8; 32];
//     let pos = plaintext.len();
//     buffer[..pos].copy_from_slice(plaintext);
//     let ciphertext = cipher.encrypt(&mut buffer , pos).unwrap();
//     println!("cipher text : {:?}" , ciphertext);

//     let mut buf = ciphertext.to_vec();
//     let cipher = Aes128Cbc::new_from_slices(&key , &iv).unwrap();
//     let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
//     println!(
//         "{}", str::from_utf8(decrypted_ciphertext).unwrap()
//     );
// }

mod args;
// use args::{UserSubCommand, CLIAPP};
use args::{CLIAPP,UserSubCommand};
use clap::Parser;
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
mod secret;
use secret::Secret;
mod encrypt;
use encrypt::*;
mod decrypt;
use decrypt::*;
pub type Aes128Cbc = Cbc<Aes128, Pkcs7>;
use hex::*;
use hex_literal::*;
use rand::*;
use std::*;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;
use base64::*;

fn main() {
    let args = CLIAPP::parse();
    match args.actions {
    UserSubCommand::Encrypt{ file_path , key } => {
    let key_file = fs::read(key).unwrap();
    let secret : Secret = serde_json::from_slice(&key_file).unwrap();
    let key = base64::decode(secret.keys()).unwrap();
    let iv = base64::decode(secret.iv()).unwrap();
    let result = encrypt_file(file_path.as_str() , &key , &iv);
    match result {
        Ok(()) => {
            println!("File Encrypted")
        },
        Err(err) => println!("Encryption failed : {}" , err),

    }
},
UserSubCommand::Decrypt { file_path, key } => {
    let file = fs::read_to_string(file_path).unwrap();
   println!("{}",file);
    let key_file = fs::read(key).unwrap();
    let secret : Secret = serde_json::from_slice(&key_file).unwrap();
    let key = base64::decode(secret.keys()).unwrap();
    let iv = base64::decode(secret.iv()).unwrap();
    let result = decrypt( &key , &iv,file.as_str() );
    match result {
        Ok(()) => {
            println!("File Decrypted")
        },
        Err(err) => println!("Decryption failed : {}" , err),

    }
        
},

UserSubCommand::Generate => {
    let secret = Secret::generate_secret();
        let secret_bytes = serde_json::to_vec(&secret).unwrap();
        let mut file = File::create("secret.txt").unwrap();
        file.write_all(&secret_bytes);
        println!("Secret Generated");
       }
    }
}

    // let plaintext = b"Hello world";
    // println!("plain text: {:?}", str::from_utf8(plaintext).unwrap());

//     let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
//     let mut buffer = [0u8; 32];
//     let pos = plaintext.len();
//     buffer[..pos].copy_from_slice(plaintext);
//     let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
//     println!("cipher text : {:?}", base64::encode(ciphertext));

//     let mut buf = ciphertext.to_vec();
//     let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
//     let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
//     println!("{}", str::from_utf8(decrypted_ciphertext).unwrap());
// }