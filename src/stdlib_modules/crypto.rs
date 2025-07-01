use crate::value::Value;
use crate::error::FlowError;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Hashing functions
pub fn hash_string(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("hash_string expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let mut hasher = DefaultHasher::new();
            s.hash(&mut hasher);
            let hash = hasher.finish();
            Ok(Value::Integer(hash as i64))
        }
        _ => Err(FlowError::runtime_error("hash_string expects string argument"))
    }
}

pub fn md5_hash(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("md5_hash expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            // Simple MD5-like hash implementation (not cryptographically secure)
            let hash = simple_md5(s.as_bytes());
            Ok(Value::String(hash))
        }
        _ => Err(FlowError::runtime_error("md5_hash expects string argument"))
    }
}

pub fn sha256_hash(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("sha256_hash expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            // Simple SHA256-like hash implementation (not cryptographically secure)
            let hash = simple_sha256(s.as_bytes());
            Ok(Value::String(hash))
        }
        _ => Err(FlowError::runtime_error("sha256_hash expects string argument"))
    }
}

// Base64 encoding/decoding
pub fn base64_encode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("base64_encode expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let encoded = base64_encode_string(s.as_bytes());
            Ok(Value::String(encoded))
        }
        _ => Err(FlowError::runtime_error("base64_encode expects string argument"))
    }
}

pub fn base64_decode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("base64_decode expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            match base64_decode_string(s) {
                Ok(decoded) => {
                    match String::from_utf8(decoded) {
                        Ok(string) => Ok(Value::String(string)),
                        Err(_) => Err(FlowError::runtime_error("Base64 decoded data is not valid UTF-8"))
                    }
                }
                Err(e) => Err(FlowError::runtime_error(&format!("Base64 decode error: {}", e)))
            }
        }
        _ => Err(FlowError::runtime_error("base64_decode expects string argument"))
    }
}

// Hex encoding/decoding
pub fn hex_encode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("hex_encode expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let hex = s.as_bytes().iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            Ok(Value::String(hex))
        }
        _ => Err(FlowError::runtime_error("hex_encode expects string argument"))
    }
}

pub fn hex_decode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("hex_decode expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            if s.len() % 2 != 0 {
                return Err(FlowError::runtime_error("Hex string must have even length"));
            }
            
            let mut bytes = Vec::new();
            for chunk in s.chars().collect::<Vec<_>>().chunks(2) {
                let hex_str: String = chunk.iter().collect();
                match u8::from_str_radix(&hex_str, 16) {
                    Ok(byte) => bytes.push(byte),
                    Err(_) => return Err(FlowError::runtime_error("Invalid hex character"))
                }
            }
            
            match String::from_utf8(bytes) {
                Ok(string) => Ok(Value::String(string)),
                Err(_) => Err(FlowError::runtime_error("Hex decoded data is not valid UTF-8"))
            }
        }
        _ => Err(FlowError::runtime_error("hex_decode expects string argument"))
    }
}

// Random number generation
pub fn random_int(args: Vec<Value>) -> Result<Value, FlowError> {
    let (min, max) = if args.is_empty() {
        (0, 100)
    } else if args.len() == 1 {
        match &args[0] {
            Value::Integer(max_val) => (0, *max_val),
            _ => return Err(FlowError::runtime_error("random_int expects integer argument"))
        }
    } else if args.len() == 2 {
        match (&args[0], &args[1]) {
            (Value::Integer(min_val), Value::Integer(max_val)) => (*min_val, *max_val),
            _ => return Err(FlowError::runtime_error("random_int expects integer arguments"))
        }
    } else {
        return Err(FlowError::runtime_error("random_int expects 0, 1, or 2 arguments"));
    };
    
    if min >= max {
        return Err(FlowError::runtime_error("random_int: min must be less than max"));
    }
    
    // Simple pseudo-random number generator
    let random_val = simple_random() % (max - min) + min;
    Ok(Value::Integer(random_val))
}

pub fn random_float(_args: Vec<Value>) -> Result<Value, FlowError> {
    // Generate random float between 0.0 and 1.0
    let random_val = (simple_random() % 1000000) as f64 / 1000000.0;
    Ok(Value::Float(random_val))
}

pub fn random_string(args: Vec<Value>) -> Result<Value, FlowError> {
    let length = if args.is_empty() {
        10
    } else {
        match &args[0] {
            Value::Integer(len) => *len as usize,
            _ => return Err(FlowError::runtime_error("random_string expects integer length"))
        }
    };
    
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut result = String::new();
    
    for _ in 0..length {
        let idx = (simple_random() % CHARS.len() as i64) as usize;
        result.push(CHARS[idx] as char);
    }
    
    Ok(Value::String(result))
}

// Helper functions for cryptographic operations
// Note: These are simplified implementations for demonstration purposes
// In production, use proper cryptographic libraries

fn simple_md5(data: &[u8]) -> String {
    // This is a very simplified hash function, not actual MD5
    let mut hash = 0u64;
    for &byte in data {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    format!("{:016x}", hash)
}

fn simple_sha256(data: &[u8]) -> String {
    // This is a very simplified hash function, not actual SHA256
    let mut hash = 0x6a09e667f3bcc908u64;
    for &byte in data {
        hash = hash.wrapping_mul(0x100000001b3).wrapping_add(byte as u64);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
    }
    format!("{:016x}", hash)
}

fn base64_encode_string(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        
        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(CHARS[((b >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARS[(b & 63) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

fn base64_decode_string(data: &str) -> Result<Vec<u8>, String> {
    let data = data.trim_end_matches('=');
    let mut result = Vec::new();
    
    for chunk in data.chars().collect::<Vec<_>>().chunks(4) {
        let mut values = [0u8; 4];
        
        for (i, &ch) in chunk.iter().enumerate() {
            values[i] = match ch {
                'A'..='Z' => (ch as u8) - b'A',
                'a'..='z' => (ch as u8) - b'a' + 26,
                '0'..='9' => (ch as u8) - b'0' + 52,
                '+' => 62,
                '/' => 63,
                _ => return Err("Invalid base64 character".to_string()),
            };
        }
        
        let b = ((values[0] as u32) << 18) | ((values[1] as u32) << 12) | ((values[2] as u32) << 6) | (values[3] as u32);
        
        result.push((b >> 16) as u8);
        if chunk.len() > 2 {
            result.push((b >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(b as u8);
        }
    }
    
    Ok(result)
}

// Simple pseudo-random number generator
static mut RANDOM_SEED: u64 = 1;

fn simple_random() -> i64 {
    unsafe {
        RANDOM_SEED = RANDOM_SEED.wrapping_mul(1103515245).wrapping_add(12345);
        (RANDOM_SEED >> 16) as i64
    }
}

// Initialize random seed
pub fn set_random_seed(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("set_random_seed expects 1 argument"));
    }
    
    match &args[0] {
        Value::Integer(seed) => {
            unsafe {
                RANDOM_SEED = *seed as u64;
            }
            Ok(Value::Null)
        }
        _ => Err(FlowError::runtime_error("set_random_seed expects integer argument"))
    }
}

// UUID generation
pub fn generate_uuid(_args: Vec<Value>) -> Result<Value, FlowError> {
    // Generate a simple UUID v4-like string
    let uuid = format!(
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        simple_random() as u32,
        (simple_random() as u16) & 0xffff,
        (simple_random() as u16) & 0x0fff,
        ((simple_random() as u16) & 0x3fff) | 0x8000,
        (simple_random() as u64) & 0xffffffffffff
    );
    Ok(Value::String(uuid))
}