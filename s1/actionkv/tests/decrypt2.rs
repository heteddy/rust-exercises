// #![cfg_attr(
//     debug_assertions,
//     allow(
//         unused,
//         dead_code,
//         unused_imports,
//         unused_variables,
//         unused_assignments,
//         non_snake_case
//     )
// )]
// use aes::Aes256;
// use block_modes::block_padding::Pkcs7;
// use block_modes::{BlockMode, Cbc};

// type AesCbc = Cbc<Aes256, Pkcs7>;

// const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
// // fn gen_ascii_chars(size: usize) -> String {
// //     let mut rng = &mut rand::thread_rng();
// //     String::from_utf8(
// //         BASE_STR
// //             .as_bytes()
// //             .choose_multiple(&mut rng, size)
// //             .cloned()
// //             .collect(),
// //     )
// //     .unwrap()
// // }

// // fn encrypt(key: &str, data: &str) -> String {
// //     let iv_str = gen_ascii_chars(16);
// //     let iv = iv_str.as_bytes();
// //     let cipher = AesCbc::new_from_slices(key.as_bytes(), iv).unwrap();
// //     let ciphertext = cipher.encrypt_vec(data.as_bytes());
// //     let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
// //     buffer.write_bytes(&ciphertext);
// //     base64::encode(buffer.to_bytes())
// // }

// fn decrypt(key: &str, data: &str, iv: &[u8]) -> String {
//     let bytes = base64::decode(data).unwrap();
//     // let cipher = AesCbc::new_from_slices(key.as_bytes(), &bytes[0..16]).unwrap();
//     let cipher = AesCbc::new_from_slices(key.as_bytes(), iv).unwrap();
//     // String::from_utf8(cipher.decrypt_vec(&bytes[16..]).unwrap()).unwrap()
//     String::from_utf8(cipher.decrypt_vec(&bytes[16..]).unwrap()).unwrap()
// }

// mod tests {
//     use super::*;
//     #[test]
//     fn test_decrypt_cyberark1() {
//         let KEY: String = String::from("9997cb6f76e08002"); // 模拟密钥，请勿在实际程序中使用
//         let IV: String = String::from("pidms20180327!@#");

//         let src = "._ir2.Ujz2kA_nAP_3-^z7.^";

//         let iv_bytes = IV.as_bytes();

//         // unsafe {
//         //     println!("dest={:?},{:?}", std::str::from_utf8_unchecked(&dest), iv);
//         // }
//         let mut c =
//             String::from("54804CE44A8E430766D20A0D98AC494B186197FE728A2933F69F4775A1B63749");

//         println!("{:?}", iv_bytes);

//         let v = decrypt(&KEY, &c, iv_bytes);
//         // unsafe {
//         //     let s = str::from_utf8_unchecked(&v);
//         //     //._ir2.Ujz2kA_nAP_3-^z7.^
//         //     //._ir2.Ujz2kA_nAP_3-^z7.^
//         //     println!("decrypted={:?}", s);
//         //     println!("decrypted={:?}", "._ir2.Ujz2kA_nAP_3-^z7.^");
//         // }
//         println!("decrypted={:?}", v);
//         println!("decrypted={:?}", "._ir2.Ujz2kA_nAP_3-^z7.^");
//     }
// }
