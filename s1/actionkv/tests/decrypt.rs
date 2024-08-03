#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]

use aes::cipher::{
    block_padding::{NoPadding, Pkcs7},
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use hex_literal::hex;
use std::str;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub fn decrypt(cipher: &[u8], iv: [u8; 16]) -> Vec<u8> {
    let KEY: String = String::from("9997cb6f76e08002"); // 模拟密钥，请勿在实际程序中使用
    let IV: String = String::from("pidms20180327!@#");
    let cipher_len = cipher.len();
    println!("length={:?}", cipher_len);
    let mut buf = [0u8; 64];
    buf[..cipher_len].copy_from_slice(cipher);
    // let  cipher2 = cipher.clone();

    // let _pt = Aes128CbcDec::new(KEY.as_bytes().into(), &iv.into())
    //     .decrypt_padded_b2b_mut::<Pkcs7>(cipher.into(), &mut buf);

    let desc = Aes128CbcDec::new(KEY.as_bytes().into(), &iv.into());
    let _pt = desc.decrypt_padded_b2b_mut::<Pkcs7>(cipher.into(), &mut buf);

    // println!("{:?}",_pt);
    match _pt {
        Ok(p) => p.to_vec(),
        Err(e) => {
            println!("error = {}", e);
            Vec::new()
        }
    }
}

pub fn encrypt_decrypt() {
    let KEY: String = String::from("9997cb6f76e08002"); // 模拟密钥，请勿在实际程序中使用
    let IV: String = String::from("pidms20180327!@#");
    println!(
        "key={:?},iv={:?}",
        KEY.as_bytes().len(),
        IV.as_bytes().len()
    );
    let mut key: [u8; 16] = [0; 16];
    let mut iv: [u8; 16] = [0; 16];

    key[0..16].copy_from_slice(KEY.as_bytes());
    iv[0..16].copy_from_slice(KEY.as_bytes());

    let plaintext = *b"hello world! this is my plaintext.";
    // let ciphertext = hex!(
    //     "c7fe247ef97b21f07cbdd26cb5d346bf"
    //     "d27867cb00d9486723e159978fb9a5f9"
    //     "14cfb228a710de4171e396e7b6cf859e"
    // );
    let ciphertext = hex!("54804CE44A8E430766D20A0D98AC494B186197FE728A2933F69F4775A1B63749");

    // encrypt/decrypt in-place
    // buffer must be big enough for padded plaintext
    let mut buf = [0u8; 96];
    let pt_len = plaintext.len();
    buf[..pt_len].copy_from_slice(&plaintext);
    let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    assert_eq!(ct, &ciphertext[..]);

    let pt = Aes128CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .unwrap();
    assert_eq!(pt, &plaintext);

    // encrypt/decrypt from buffer to buffer
    let mut buf = [0u8; 48];
    let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(&plaintext, &mut buf)
        .unwrap();
    assert_eq!(ct, &ciphertext[..]);

    let mut buf = [0u8; 48];
    let pt = Aes128CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&ct, &mut buf)
        .unwrap();
    assert_eq!(pt, &plaintext);
}

pub fn encrypt(plain: &[u8], iv: [u8; 16]) -> (Vec<u8>, [u8; 16]) {
    // 随机值
    // let iv = generate_iv();
    let KEY: String = String::from("9997cb6f76e08002");
    let mut buf = [0u8; 48];
    let pt_len = plain.len();
    buf[..pt_len].copy_from_slice(plain);
    let ct = Aes128CbcEnc::new(KEY.as_bytes().into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plain, &mut buf)
        .unwrap();

    (ct.to_vec(), iv)
}

pub fn decrypt2(cipher: &[u8], iv: [u8; 16]) -> Vec<u8> {
    let KEY: String = String::from("9997cb6f76e08002"); // 模拟密钥，请勿在实际程序中使用
    let IV: String = String::from("pidms20180327!@#");
    let cipher_len = cipher.len();
    println!("length={:?}", cipher_len);
    let mut buf = [0u8; 64];
    buf[..cipher_len].copy_from_slice(cipher);

    // let  cipher2 = cipher.clone();

    // let _pt = Aes128CbcDec::new(KEY.as_bytes().into(), &iv.into())
    //     .decrypt_padded_b2b_mut::<Pkcs7>(cipher.into(), &mut buf);

    let desc = Aes128CbcDec::new(KEY.as_bytes().into(), &iv.into());
    let _pt = desc.decrypt_padded_b2b_mut::<Pkcs7>(cipher, &mut buf);
    println!("here1");
    // println!("{:?}",_pt);
    match _pt {
        Ok(p) => p.to_vec(),
        Err(e) => {
            println!("error = {}", e);
            Vec::new()
        }
    }
}

mod tests {
    // use rdkafka::message::ToBytes;

    // use rdkafka::message::ToBytes;

    use core::str;

    use super::*;

    #[test]
    fn test_en_de() {
        let KEY: String = String::from("9997cb6f76e08002"); // 模拟密钥，请勿在实际程序中使用
        let IV: String = String::from("pidms20180327!@#");
        println!(
            "key={:?},iv={:?}",
            KEY.as_bytes().len(),
            IV.as_bytes().len()
        );
        let mut key: [u8; 16] = [0; 16];
        let mut iv: [u8; 16] = [0; 16];

        key[0..16].copy_from_slice(KEY.as_bytes());
        iv[0..16].copy_from_slice(KEY.as_bytes());

        let plaintext = *b"hello world! this is my plaintext.";
        // let ciphertext = hex!(
        //     "c7fe247ef97b21f07cbdd26cb5d346bf"
        //     "d27867cb00d9486723e159978fb9a5f9"
        //     "14cfb228a710de4171e396e7b6cf859e"
        // );

        // encrypt/decrypt in-place
        // buffer must be big enough for padded plaintext
        // let mut buf = [0u8; 48];
        // let pt_len = plaintext.len();
        // buf[..pt_len].copy_from_slice(&plaintext);
        // let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        //     .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        //     .unwrap();
        // // assert_eq!(ct, &ciphertext[..]);
        // println!("ct={:?}",ct);

        // encrypt/decrypt from buffer to buffer
        // let mut buf = [0u8; 48];
        // let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        //     .encrypt_padded_b2b_mut::<Pkcs7>(&plaintext, &mut buf)
        //     .unwrap();
        // assert_eq!(ct, &ciphertext[..]);
        let mut c =
            String::from("54804CE44A8E430766D20A0D98AC494B186197FE728A2933F69F4775A1B63749");
        let mut buf = [0u8; 64];
        let c_b = c.as_bytes();

        let pt = Aes128CbcDec::new(&key.into(), &iv.into())
            .decrypt_padded_b2b_mut::<Pkcs7>(c_b, &mut buf);
        println!("{:?}", pt)
    }

    #[test]
    fn test_decrypt2() {
        // let ciphertext = "tCmGLnG7+tKGSFBhtkn+sg==";
        // let ciphertext = base64::decode(&ciphertext).unwrap();
    }
    #[test]
    fn test_decrypt_cyberark_0() {
        let KEY: String = String::from("9997cb6f76e08002"); // 模拟密钥，请勿在实际程序中使用
        let IV: String = String::from("pidms20180327!@#");

        let src = "._ir2.Ujz2kA_nAP_3-^z7.^";

        let iv_bytes = IV.as_bytes().try_into().unwrap();

        // let (dest, iv) = encrypt(src.as_bytes(), iv_bytes);
        // unsafe {
        //     println!("dest={:?},{:?}", std::str::from_utf8_unchecked(&dest), iv);
        // }
        // let mut c =
        //     String::from("54804CE44A8E430766D20A0D98AC494B186197FE728A2933F69F4775A1B63749");
        let mut c = hex!("54804CE44A8E430766D20A0D98AC494B186197FE728A2933F69F4775A1B63749");
        println!("iv bytes = {:?}", iv_bytes);

        let v = decrypt(&mut c, iv_bytes);
        println!("output length= {:?}", v.len());
        println!("actually decrypted = {:?}", str::from_utf8(&v).unwrap());
        let should = "._ir2.Ujz2kA_nAP_3-^z7.^";
        println!("should decrypted   = {:?}", "._ir2.Ujz2kA_nAP_3-^z7.^");
        println!("decrypted bytes should ={:?}", should.as_bytes());
    }   
}
