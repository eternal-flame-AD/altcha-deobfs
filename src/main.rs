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
    let nonce_num = nonce
        .iter()
        .rev()
        .fold(0u128, |acc, &b| acc * 256 + u128::from(b));
    eprintln!("Nonce: {:?} (number: {})", nonce, nonce_num);
    let mut cipher = gcm_nonceless::instantiate_keystream(cipher, &y0);
    cipher.apply_keystream(&mut data_decoded);
    std::io::stdout().write_all(&data_decoded).unwrap();
    if is_demo_data {
        assert_eq!(data_decoded, b"mailto:security@altcha.org");
    }
}
