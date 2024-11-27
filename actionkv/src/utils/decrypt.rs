use aes::cipher::{
    block_padding::{NoPadding, Pkcs7},
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
// use hex_literal::hex;
use hex;
use std::str;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// cipher:密文
/// iv: AES-128-CBC
/// key:
pub fn decrypt(key: &str, iv: &str, cipher: &str) -> Option<String> {
    /*
        &str -> String--| String::from(s) or s.to_string() or s.to_owned()
        &str -> &[u8]---| s.as_bytes()
        &str -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
        String -> &str----| &s if possible* else s.as_str()
        String -> &[u8]---| s.as_bytes()
        String -> Vec<u8>-| s.into_bytes()
        &[u8] -> &str----| std::str::from_utf8(s).unwrap()
        &[u8] -> String--| String::from_utf8(s).unwrap()
        &[u8] -> Vec<u8>-| s.to_vec()
        Vec<u8> -> &str----| std::str::from_utf8(&s).unwrap()
        Vec<u8> -> String--| String::from_utf8(s).unwrap()
        Vec<u8> -> &[u8]---| &s if possible* else s.as_slice()
    */
    let cipher = hex::decode(cipher);
    if cipher.is_ok() {
        let crypted_bytes = cipher.unwrap();
        let cipher_len = crypted_bytes.len();
        let mut buf = [0u8; 48];
        buf[..cipher_len].copy_from_slice(&crypted_bytes.as_slice());

        let mut iv_bytes: [u8; 16] = [0u8; 16];
        iv_bytes[0..iv.as_bytes().len()].copy_from_slice(&iv.as_bytes());

        let desc = Aes128CbcDec::new(key.as_bytes().into(), &iv_bytes.into());
        let _pt = desc.decrypt_padded_b2b_mut::<Pkcs7>(crypted_bytes.as_slice().into(), &mut buf);

        match _pt {
            Ok(p) => Some(String::from_utf8(p.to_vec()).unwrap()),
            Err(e) => {
                println!("error = {}", e);
                None
            }
        }
    } else {
        None
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_decrypt_password() {
        let KEY = "9997cb6f76e08002"; // 模拟密钥，请勿在实际程序中使用
        let IV = "pidms20180327!@#";
        let encrypted = "54804CE44A8E430766D20A0D98AC494B186197FE728A2933F69F4775A1B63749";
        let password = decrypt(KEY, IV, encrypted);
        match password {
            None => println!("password is None"),
            Some(p) => println!("p = {:?}", p),
        }
    }
}
