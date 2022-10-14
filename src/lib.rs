#![no_std]

use heapless::Vec;

const ENCODE_MAPPING: [u8; 26] = [
    0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, // J
    0, 0, 0b010, // M
    0, 0, 0b001, // P
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[inline]
fn encode_one(c: char) -> u64 {
    ENCODE_MAPPING[c as usize - 0x41] as u64
}
pub fn encode(seq: &[char]) -> u64 {
    let mut out = 0u64;
    let n = seq.len();
    for i in 0..n {
        let c = seq[i];
        let m = encode_one(c);
        let off = 3 * i;
        out |= m << off;
    }
    out
}

const DECODE_MAPPING: [char; 3] = [' ', 'P', 'M'];

#[inline]
fn decode_one(x: u64) -> char {
    DECODE_MAPPING[x as usize]
}
pub fn decode(v: u64) -> Vec<char, 21> {
    if v == 0 {
        return Vec::new();
    }
    let msb = 63 - v.leading_zeros() as usize;
    let n = msb / 3 + 1;
    let mut out = Vec::new();
    unsafe { out.set_len(n) };
    for i in 0..n {
        let shift = 3 * i;
        let mask = 0b111 << shift;
        let x = (v & mask) >> shift;
        let c = decode_one(x);
        out[i] = c;
    }
    out
}
