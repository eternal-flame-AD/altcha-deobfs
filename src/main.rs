#![allow(deprecated)]
use std::io::Write;

use base64::prelude::*;
use clap::Parser;
use gcm_nonceless::ctr::cipher::{KeyInit, StreamCipher};
use sha2::Digest;

type C = gcm_nonceless::aes::Aes256;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "")]
    key: String,

    #[arg(short, long, default_value = "")]
    data: String,
}

fn main() {
    let args = Args::parse();
    let key = sha2::Sha256::digest(args.key.as_bytes());
    let cipher = C::new(&key);

    // https://altcha.org/contact/#reporting-security-issues
    let mut data = args.data;

    let mut is_demo_data = false;
    if data.is_empty() {
        data = String::from("HD9IT+QrWtjss/0IgpfKbifkkNsSTFxS6PLv0vTjYfcIrcP1l+TfDpNZ");
        is_demo_data = true;
    }

    let mut data_decoded = BASE64_STANDARD.decode(data).unwrap();
    let y0 = gcm_nonceless::recover_counter(&cipher, &data_decoded, None, &[]);
    data_decoded.truncate(data_decoded.len() - 16);
    let nonce = gcm_nonceless::extract_nonce::<C>(&y0).unwrap();
    let lsb = u64::from_le_bytes(nonce[..8].try_into().unwrap());
    let msb = u32::from_le_bytes(nonce[8..].try_into().unwrap());
    let nonce_num = (u128::from(msb) << 64) | u128::from(lsb);
    let mut nonce_hex = [0; 24];
    for (i, b) in nonce.iter().enumerate() {
        let low = b & 0x0f;
        let high = (b & 0xf0) >> 4;
        nonce_hex[i * 2] = if high < 10 {
            b'0' + high as u8
        } else {
            b'a' + high as u8 - 10
        };
        nonce_hex[i * 2 + 1] = if low < 10 {
            b'0' + low as u8
        } else {
            b'a' + low as u8 - 10
        };
    }
    eprintln!(
        "Nonce: {} (number: {})",
        std::str::from_utf8(&nonce_hex).unwrap(),
        nonce_num
    );
    let mut cipher = gcm_nonceless::instantiate_keystream(cipher, &y0);
    cipher.apply_keystream(&mut data_decoded);
    std::io::stdout().write_all(&data_decoded).unwrap();
    if is_demo_data {
        assert_eq!(data_decoded, b"mailto:security@altcha.org");
    }
}
