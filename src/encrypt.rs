use std::io::Read;

use super::*;
pub fn encrypt_file (
    file_path: &str,
    key: &[u8],
    iv: &[u8],
)-> Result<(), anyhow::Error> {
  
    let  file_contents = fs::read(file_path).unwrap();
   
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
   let pos = file_contents.len().clone();
   let mut buffer = vec![0u8;pos+pos];
   buffer[..pos].copy_from_slice(&file_contents);

    let ciphertext = cipher.encrypt( &mut buffer, pos).unwrap();
    fs::write("enc.txt", base64::encode(ciphertext)).unwrap();
    Ok(())
}

