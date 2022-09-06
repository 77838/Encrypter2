use std::fmt::Error;
use std::fs;
use super::*;

pub fn decrypt(key:&[u8],iv:&[u8],encrypted_file: &str) -> Result<(),Error>{
    let mut ciphertext = base64::decode(encrypted_file).unwrap();
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt(&mut ciphertext).unwrap();
    println!("{}", str::from_utf8(decrypted_ciphertext).unwrap());
    Ok(())
}
// let mut block = GenericArray::from([42u8; 16]);

// fn decrypt_file(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
//     let mut decrypt = crypto::aes::ecb_decryptor(
//         KeySize128,
//         key,
//         PkcsPadding,
//     );
//     let mut read_buffer = RefReadBuffer::new(text);
//     let mut result = vec![0; text.len()];
//     let mut write_buffer = RefWriteBuffer::new(&mut result);
//     let fina=decrypt.decrypt_file(&mut read_buffer, &mut write_buffer, true);
//     match fina{
//         Ok(fa)=>info!("succ"),
//         Err(e)=>info!("{:?}",e)
//     }
//     Ok(result)
// }


