// (c) 2024, 384 inc
// MIT License

// NOTE: This is a straight port/translation of the reference Typescript implementation.
// Happy to receive PRs from people more familiar with Rust than I am ...

extern crate num_bigint;

use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;
use std::collections::HashMap;
use lazy_static::lazy_static;

// There are three common variations:
// const BASE62B64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
// const BASE62LEX: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
// const BASE62BASEN: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

// We use 'base64' ordering, also known as 'truncated base64'
const BASE62: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

// If you wish to be interoperable, you can append '62' to the end of the Uint8Array before
// encoding it, that will cause the string to end with 'M' (b64), 'C' (lex), or 'c' (baseN)

// Max chunk size, this is a hard design point.
const N: usize = 32;

lazy_static! {
    static ref M: HashMap<usize, usize> = {
        let mut m = HashMap::new();
        for x in 1..=N {
            let y = ((x as f64 * 8.0) / 62f64.log2()).ceil() as usize;
            m.insert(x, y);
        }
        // println!("m: {:?}", m);
        m
    };
    static ref INV_M: HashMap<usize, usize> = {
        let mut inv_m = HashMap::new();
        for (&k, &v) in M.iter() {
            inv_m.insert(v, k);
        }
        // println!("inv_m: {:?}", inv_m);
        inv_m
    };
    static ref MAX_CHUNK: usize = *M.get(&N).unwrap();
}

fn _array_buffer_to_base62(buffer: &[u8], c: usize) -> String {
    let mut result = String::new();
    let mut n = BigUint::from_bytes_be(buffer);
    let base = BigUint::from(62u32);
    let b62zero = BASE62.chars().next().unwrap();
    while &n > &BigUint::from(0u32) {
        let remainder = &n % &base;
        n /= &base;
        result.push(BASE62.chars().nth(remainder.to_usize().unwrap()).unwrap());
    }
    let padded_length = *M.get(&c).unwrap();
    result = result.chars().rev().collect();
    while result.len() < padded_length {
        result.insert(0, b62zero);
    }
    result
}

pub fn array_buffer_to_base62(buffer: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < buffer.len() {
        let c = if buffer.len() - i >= N {
            N
        } else {
            buffer.len() - i
        };
        result.push_str(&_array_buffer_to_base62(&buffer[i..i + c], c));
        i += c;
    }
    result
}


fn _base62_to_array_buffer(s: &str, t: usize) -> Result<Vec<u8>, String> {
    let mut n = BigUint::from(0u8);

    for c in s.chars() {
        let digit = BASE62
            .find(c)
            .ok_or_else(|| "Invalid Base62 string.".to_string())?;
        n = n * 62.to_biguint().unwrap() + digit.to_biguint().unwrap();
    }

    // Calculate the maximum value that can be represented with `t` bytes
    let max_value = (BigUint::from(2u32).pow((t * 8) as u32)) - BigUint::from(1u32);
    if n > max_value {
        return Err("base62ToArrayBuffer: Invalid Base62 string.".to_string());
    } 

    let mut buffer = vec![0u8; t];
    let bytes = n.to_bytes_be();

    for (i, &byte) in bytes.iter().rev().enumerate() {
        buffer[t - 1 - i] = byte;
    }

    Ok(buffer)
}

pub fn base62_to_array_buffer(s: &str) -> Result<Vec<u8>, String> {
    if !s.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("Invalid Base62 string.".to_string());
    }
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < s.len() {
        let c = std::cmp::min(s.len() - i, *MAX_CHUNK);
        let chunk_size = match INV_M.get(&c) {
            Some(&size) => size,
            // ToDo: with Typescript, these errors are caught elsewhere (?)
            None => return Err("Invalid Base62 string".to_string())
        };
        let new_buf = _base62_to_array_buffer(&s[i..i + c], chunk_size)?;
        result.extend_from_slice(&new_buf);
        i += c;
        j += new_buf.len();
    }
    result.truncate(j);
    Ok(result)
}