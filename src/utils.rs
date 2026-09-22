/* Eyal Kaghanovich
 * utilities module for cryptopals challenges
*/

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[macro_export]
macro_rules! string {
    ($val:expr) => {
        std::string::String::from($val)
    };
}

pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }

    let mut result = Vec::with_capacity(s.len() / 2);
    let chars: Vec<char> = s.chars().collect();

    for pair in chars.chunks(2) {
        let high = pair[0].to_digit(16)?;
        let low = pair[1].to_digit(16)?;
        result.push(((high << 4) | low) as u8);
    }

    Some(result)
}

// just printing, no need to take ownership, so we borrow immutably
pub fn print_hex(hex: &Vec<u8>) {
    for byte in hex {
        print!("{:02x}", byte);
    }
    println!();
}

pub fn base64_char(chunk: u8) -> char {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    ALPHABET[chunk as usize] as char
}

pub fn hex_to_str(buf: &Vec<u8>) -> String {
    String::from_utf8_lossy(buf).to_lowercase()
}

// lbl = line by line
pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = String>, io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    // lines() yields io::Result<String>; stop at the first read error
    Ok(reader.lines().map_while(Result::ok))
}

//challenge 1
pub fn print_hex_to_base64(bitstream: &Vec<u8>) {
    let mut buf: u32 = 0;
    let mut bits_in_buf: u32 = 0;

    for byte in bitstream {
        // push byte to buf
        buf = (buf << 8) | *byte as u32;
        bits_in_buf += 8;

        while bits_in_buf >= 6 {
            bits_in_buf -= 6;
            let chunk = (buf >> bits_in_buf) & 0b0011_1111;
            print!("{}", base64_char(chunk as u8));
        }
    }
    println!();
}
