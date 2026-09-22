/* Eyal Kaghanovich
 * my solutions to the cryptopals crypto challenges
*/

use std::result;

use crate::utils::{hex_to_bytes, hex_to_str, print_hex, print_hex_to_base64};
use crate::weak_xor::{
    decrypt_single_byte_xor, detect_single_byte_xor, repeating_key_xor, xor_buffers_fixed,
};

mod plaintext_score;
mod utils;
mod weak_xor;
mod break_repeating_key_xor;

fn main() {
    println!("Hello, cryptopals!");

    print!("challenge 1, hex to base64: ");
    print_hex_to_base64(
        &hex_to_bytes(
            &String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
        ).unwrap()
    );

    print!("challenge 2, fixed xor: ");
    print_hex(
        &xor_buffers_fixed(
            hex_to_bytes(&String::from("1c0111001f010100061a024b53535009181c")).unwrap(),
            hex_to_bytes(&String::from("686974207468652062756c6c277320657965")).unwrap(),
        )
        .unwrap(),
    );

    print!("challenge 3, single byte xor cipher: ");
    let result = hex_to_str(&decrypt_single_byte_xor(
        &hex_to_bytes(&String::from(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        ))
        .unwrap(),
    ));
    println!("{}", result);

    print!("challenge 4, detect single byte xor: ");
    let result = hex_to_str(&detect_single_byte_xor("challenge_files/chall4.txt").unwrap());
    print!("{}", result);

    print!("challenge 5, repeating key xor: ");
    let pt: &[u8] = b"Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";
    let k = b"ICE";
    let result = &repeating_key_xor(pt, k).unwrap();
    print_hex(result);
}
