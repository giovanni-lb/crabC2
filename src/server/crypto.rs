//use aes::cipher::{KeyInit, BlockCipher, BlockDecrypt, BlockEncrypt};
//use aes::Aes128;
//use block_modes::{block_padding::Pkcs7, Cbc};
//use std::error::Error;
//use hex_literal::hex;
//
//type Aes128Cbc = Cbc<Aes128, Pkcs7>;
//
//pub fn decrypt_data(encrypted_data: String) -> Result<String, Box<dyn Error>> {
//    let iv = hex!("00000000000000000000000000000000");
//    let tmp_key = hex::decode("SECRET_KEY")?; 
//
//    let cipher_key = GenericArray::from_slice(&tmp_key);
//    let iv = GenericArray::from_slice(&iv);
//    let cipher = Aes128Cbc::new(cipher_key, iv);
//
//	let encoded = hex::decode(encrypted_data)?;
//	let mut buf = encoded.to_vec();
//
//	let decrypted_ciphertext = cipher.decrypt(&mut buf)?;
//	let result = String::from_utf8(decrypted_ciphertext)?;
//
//	Ok(result)   
//}