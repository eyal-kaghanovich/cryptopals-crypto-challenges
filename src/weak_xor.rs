/* Eyal Kaghanovich
 * cryptopals, weak xor challenges
*/

use std::{path::Path, vec};

use crate::{
    plaintext_score::score_buffer,
    utils::{hex_to_bytes, read_lines},
};

pub fn xor_buffers_fixed(buf: Vec<u8>, key: Vec<u8>) -> Option<Vec<u8>> {
    if buf.len() != key.len() {
        return None;
    }

    Some(
        buf.iter()
            .zip(key.iter())
            .map(|(&byte, &key_byte)| byte ^ key_byte)
            .collect(),
    )
}

// to print: str_to_hex
pub fn decrypt_single_byte_xor(ciphertext_buf: &Vec<u8>) -> Vec<u8> {
    let mut highest_score: f64 = 0.00;
    let mut best_key: u8 = 0;

    for byte in 0u8..=u8::MAX {
        let buf: Vec<u8> = ciphertext_buf.iter().map(|&b| b ^ byte).collect();

        let score = score_buffer(&buf);

        if score > highest_score {
            highest_score = score;
            best_key = byte;
        }
    }
    ciphertext_buf.iter().map(|&byte| byte ^ best_key).collect()
}

pub fn detect_single_byte_xor<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    let mut highest_score: f64 = 0.00;
    let mut best_buf: Vec<u8> = vec![];

    for line in read_lines(path).ok()? {
        let buf = hex_to_bytes(&line)?;

        for key in 0u8..=u8::MAX {
            let buf: Vec<u8> = buf.iter().map(|&byte| byte ^ key).collect();

            let score = score_buffer(&buf);
            if score > highest_score {
                highest_score = score;
                best_buf = buf;
            }
        }
    }
    Some(best_buf)
}

pub fn repeating_key_xor(plaintext_buf: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    if plaintext_buf.len() == 0 || key.len() == 0 {
        return None;
    }

    let mut ciphertext_buf: Vec<u8> = Vec::new();
    // iterate over the buffer with the greater len()

    for (pt, k) in plaintext_buf.iter().zip(key.iter().cycle()) {
        ciphertext_buf.push(*pt ^ *k);
    }

    Some(ciphertext_buf)
}
