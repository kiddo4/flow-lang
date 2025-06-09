//! BigInt implementation for FlowLang to handle large integers without overflow

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BigInt {
    digits: Vec<u32>,
    negative: bool,
}

impl BigInt {
    const BASE: u64 = 1_000_000_000; // 10^9 for easier decimal conversion
    
    pub fn new() -> Self {
        BigInt {
            digits: vec![0],
            negative: false,
        }
    }
    
    pub fn from_i64(mut value: i64) -> Self {
        if value == 0 {
            return BigInt::new();
        }
        
        let negative = value < 0;
        if negative {
            value = -value;
        }
        
        let mut digits = Vec::new();
        let mut remaining = value as u64;
        
        while remaining > 0 {
            digits.push((remaining % Self::BASE) as u32);
            remaining /= Self::BASE;
        }
        
        BigInt { digits, negative }
    }
    
    pub fn from_string(s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Err("Empty string".to_string());
        }
        
        let (s, negative) = if s.starts_with('-') {
            (&s[1..], true)
        } else {
            (s, false)
        };
        
        if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid number format".to_string());
        }
        
        let mut result = BigInt::new();
        
        for chunk in s.chars().rev().collect::<String>().as_bytes().chunks(9) {
            let chunk_str = std::str::from_utf8(chunk).unwrap().chars().rev().collect::<String>();
            let chunk_value = chunk_str.parse::<u32>().unwrap();
            result.digits.push(chunk_value);
        }
        
        result.negative = negative && !result.is_zero();
        result.normalize();
        
        Ok(result)
    }
    
    pub fn to_i64(&self) -> Option<i64> {
        if self.digits.len() > 2 {
            return None; // Too large for i64
        }
        
        let mut value = 0u64;
        for (i, &digit) in self.digits.iter().enumerate() {
            value += digit as u64 * Self::BASE.pow(i as u32);
        }
        
        if value > i64::MAX as u64 {
            return None;
        }
        
        let result = value as i64;
        Some(if self.negative { -result } else { result })
    }
    
    pub fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }
    
    fn normalize(&mut self) {
        while self.digits.len() > 1 && self.digits.last() == Some(&0) {
            self.digits.pop();
        }
        
        if self.is_zero() {
            self.negative = false;
        }
    }
    
    fn add_magnitude(&self, other: &BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut carry = 0u64;
        let max_len = self.digits.len().max(other.digits.len());
        
        for i in 0..max_len {
            let a = self.digits.get(i).copied().unwrap_or(0) as u64;
            let b = other.digits.get(i).copied().unwrap_or(0) as u64;
            let sum = a + b + carry;
            
            result.push((sum % Self::BASE) as u32);
            carry = sum / Self::BASE;
        }
        
        if carry > 0 {
            result.push(carry as u32);
        }
        
        let mut big_int = BigInt {
            digits: result,
            negative: false,
        };
        big_int.normalize();
        big_int
    }
    
    fn sub_magnitude(&self, other: &BigInt) -> BigInt {
        if self.cmp_magnitude(other) == Ordering::Less {
            let mut result = other.sub_magnitude(self);
            result.negative = !result.negative;
            return result;
        }
        
        let mut result = Vec::new();
        let mut borrow = 0i64;
        
        for i in 0..self.digits.len() {
            let a = self.digits[i] as i64;
            let b = other.digits.get(i).copied().unwrap_or(0) as i64;
            let diff = a - b - borrow;
            
            if diff < 0 {
                result.push((diff + Self::BASE as i64) as u32);
                borrow = 1;
            } else {
                result.push(diff as u32);
                borrow = 0;
            }
        }
        
        let mut big_int = BigInt {
            digits: result,
            negative: false,
        };
        big_int.normalize();
        big_int
    }
    
    fn cmp_magnitude(&self, other: &BigInt) -> Ordering {
        match self.digits.len().cmp(&other.digits.len()) {
            Ordering::Equal => {
                for i in (0..self.digits.len()).rev() {
                    match self.digits[i].cmp(&other.digits[i]) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}

impl Add for BigInt {
    type Output = BigInt;
    
    fn add(self, other: BigInt) -> BigInt {
        match (self.negative, other.negative) {
            (false, false) => self.add_magnitude(&other),
            (true, true) => {
                let mut result = self.add_magnitude(&other);
                result.negative = true;
                result
            }
            (false, true) => self.sub_magnitude(&other),
            (true, false) => other.sub_magnitude(&self),
        }
    }
}

impl Sub for BigInt {
    type Output = BigInt;
    
    fn sub(self, other: BigInt) -> BigInt {
        let mut other = other;
        other.negative = !other.negative;
        self + other
    }
}

impl Mul for BigInt {
    type Output = BigInt;
    
    fn mul(self, other: BigInt) -> BigInt {
        if self.is_zero() || other.is_zero() {
            return BigInt::new();
        }
        
        let mut result = vec![0u32; self.digits.len() + other.digits.len()];
        
        for i in 0..self.digits.len() {
            let mut carry = 0u64;
            for j in 0..other.digits.len() {
                let prod = self.digits[i] as u64 * other.digits[j] as u64 + result[i + j] as u64 + carry;
                result[i + j] = (prod % Self::BASE) as u32;
                carry = prod / Self::BASE;
            }
            if carry > 0 {
                result[i + other.digits.len()] += carry as u32;
            }
        }
        
        let mut big_int = BigInt {
            digits: result,
            negative: self.negative != other.negative,
        };
        big_int.normalize();
        big_int
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            write!(f, "-")?;
        }
        
        if let Some(&last) = self.digits.last() {
            write!(f, "{}", last)?;
            for &digit in self.digits.iter().rev().skip(1) {
                write!(f, "{:09}", digit)?;
            }
        }
        
        Ok(())
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => self.cmp_magnitude(other),
            (true, true) => other.cmp_magnitude(self),
        }
    }
}