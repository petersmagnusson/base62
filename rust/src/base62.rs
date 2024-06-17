use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use lazy_static::lazy_static;

pub const BASE62_B64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
// pub const BASE62_LEX: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
// pub const BASE62_BASEN: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Using 'base64' ordering, also known as 'truncated base64'
pub const BASE62: &str = BASE62_B64;

const N: usize = 32;

lazy_static! {
    static ref M: Vec<usize> = {
        let mut m = vec![0; N + 1];
        for x in 1..=N {
            m[x] = ((x * 8) as f64 / 5.954).ceil() as usize;
        }
        m
    };
    static ref INV_M: Vec<usize> = {
        let mut inv = vec![0; *M.iter().max().unwrap() + 1];
        for (x, &y) in M.iter().enumerate().skip(1) {
            inv[y] = x;
        }
        inv
    };
}

pub fn array_buffer_to_base62(buffer: &[u8], base62: &str) -> String {
    let base62 = if base62.is_empty() { BASE62 } else { base62 };
    let mut result = String::new();
    let mut i = 0;

    while i < buffer.len() {
        let c = if buffer.len() - i >= N { N } else { buffer.len() - i };
        result += &_array_buffer_to_base62(&buffer[i..i + c], c, base62);
        i += c;
    }

    result
}

fn _array_buffer_to_base62(buffer: &[u8], c: usize, base62: &str) -> String {
    let mut n = BigUint::from(0u32);
    for &byte in buffer {
        n = (n << 8) | BigUint::from(byte);
    }

    let mut result = String::new();
    let base = BigUint::from(62u32);

    while &n > &BigUint::from(0u32) {
        let remainder = &n % &base;
        n /= &base;
        result.push(base62.chars().nth(remainder.to_usize().unwrap()).unwrap());
    }

    let padded_length = M[c];
    while result.len() < padded_length {
        result.insert(0, base62.chars().nth(0).unwrap());
    }

    result
}

pub fn base62_to_array_buffer(s: &str, base62: &str) -> Result<Vec<u8>, String> {
    let base62 = if base62.is_empty() { BASE62 } else { base62 };
    if !s.chars().all(|c| base62.contains(c)) {
        return Err("base62ToArrayBuffer: must be alphanumeric.".to_string());
    }

    let mut result = Vec::new();
    let mut i = 0;

    while i < s.len() {
        let c = std::cmp::min(s.len() - i, *M.last().unwrap());
        let new_buf = _base62_to_array_buffer(&s[i..i + c], INV_M[c], base62)?;
        result.extend(new_buf);
        i += c;
    }

    Ok(result)
}

fn _base62_to_array_buffer(s: &str, t: usize, base62: &str) -> Result<Vec<u8>, String> {
    let mut n = BigUint::from(0u32);
    for c in s.chars() {
        n = n * BigUint::from(62u32) + BigUint::from(base62.find(c).unwrap());
    }

    if n > (BigUint::from(1u32) << (t * 8)) - 1u32 {
        return Err("base62ToArrayBuffer: Invalid Base62 string.".to_string());
    }

    let mut buffer = vec![0u8; t];
    for i in (0..t).rev() {
        buffer[i] = (&n % BigUint::from(256u32)).to_u8().unwrap();
        n >>= 8;
    }

    Ok(buffer)
}
