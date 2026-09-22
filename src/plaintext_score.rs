/* Eyal Kaghanovich
 * plaintext scoring functions
*/

const ENGLISH_FREQS: [f64; 26] = [
    8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966, 0.153, 0.772, 4.025, 2.406,
    6.749, 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.974, 0.074,
];

const SPACE_FREQ: f64 = 15.0;

const PENALTY: f64 = 10.0;

const RARE_BYTES: [u8; 30] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, /* skip 0x09 tab, 0x0A newline — these can legitimately appear */
    0x0B, 0x0C, /* skip 0x0D carriage return */
    0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
    0x1E, 0x1F, 0x7F, // DEL
];

fn is_rare(c: u8) -> bool {
    for byte in RARE_BYTES {
        if c == byte {
            return true;
        }
    }
    false
}

pub fn score_buffer(buf: &Vec<u8>) -> f64 {
    let mut total: f64 = 0.00;

    for byte in buf {
        if byte.is_ascii_alphabetic() {
            total += ENGLISH_FREQS[(byte.to_ascii_lowercase() - b'a') as usize];
        } else if *byte == b' ' {
            total += SPACE_FREQ;
        } else if is_rare(*byte) {
            total -= PENALTY;
        }
    }
    total
}
