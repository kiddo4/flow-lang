//! FlowLang Standard Library
//! 
//! This module provides built-in functions and utilities that are
//! available to all FlowLang programs.
//!
//! The standard library is organized into modules:
//! - Core functions (print, input, etc.)
//! - String manipulation
//! - Array operations
//! - Object operations
//! - Math functions
//! - I/O operations
//! - Time functions
//! - Type conversion
//! - Extended modules (io, system, net, json, crypto)

use crate::error::{FlowError, Result};
use crate::value::{FlowArray, FlowObject, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Write};

// Import the new modular standard library
use crate::stdlib_modules::StandardLibraryRegistry;

/// Standard library functions registry
pub struct StandardLibrary {
    pub functions: HashMap<String, fn(Vec<Value>) -> Result<Value>>,
    extended_registry: StandardLibraryRegistry,
}

impl StandardLibrary {
    pub fn new() -> Self {
        let mut stdlib = Self {
            functions: HashMap::new(),
            extended_registry: StandardLibraryRegistry::new(),
        };
        
        // Register all standard library functions
        stdlib.register_core_functions();
        stdlib.register_string_functions();
        stdlib.register_array_functions();
        stdlib.register_object_functions();
        stdlib.register_math_functions();
        stdlib.register_io_functions();
        stdlib.register_time_functions();
        stdlib.register_type_functions();
        
        stdlib
    }
    
    pub fn get_function(&self, name: &str) -> Option<&fn(Vec<Value>) -> Result<Value>> {
        self.functions.get(name)
    }
    
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name) || self.extended_registry.has_function(name)
    }
    
    pub fn list_functions(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }
    
    /// Call a function from either the legacy or extended registry
    pub fn call_function(&self, name: &str, args: &[Value]) -> Result<Value> {
        // First try legacy functions
        if let Some(func) = self.functions.get(name) {
            return func(args.to_vec());
        }
        
        // Then try extended registry with converted arguments
        if self.extended_registry.has_function(name) {
            let converted_args: Vec<crate::value::Value> = args.iter().cloned().collect();
            return self.extended_registry.call_function(name, converted_args)
                .map_err(|e| FlowError::runtime_error(&e.to_string()));
        }
        
        Err(FlowError::runtime_error(&format!("Unknown function: {}", name)))
    }
    
    /// Get all function names from both registries
    pub fn get_all_function_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.functions.keys().cloned().collect();
        names.extend(self.extended_registry.get_function_names());
        names.sort();
        names.dedup();
        names
    }
    
    fn register_core_functions(&mut self) {
        self.functions.insert("print".to_string(), stdlib_print);
        self.functions.insert("println".to_string(), stdlib_println);
        self.functions.insert("input".to_string(), stdlib_input);
        self.functions.insert("assert".to_string(), stdlib_assert);
        self.functions.insert("panic".to_string(), stdlib_panic);
    }
    
    fn register_string_functions(&mut self) {
        self.functions.insert("str_len".to_string(), stdlib_str_len);
        self.functions.insert("str_upper".to_string(), stdlib_str_upper);
        self.functions.insert("str_lower".to_string(), stdlib_str_lower);
        self.functions.insert("str_trim".to_string(), stdlib_str_trim);
        self.functions.insert("str_split".to_string(), stdlib_str_split);
        self.functions.insert("str_join".to_string(), stdlib_str_join);
        self.functions.insert("str_contains".to_string(), stdlib_str_contains);
        self.functions.insert("str_starts_with".to_string(), stdlib_str_starts_with);
        self.functions.insert("str_ends_with".to_string(), stdlib_str_ends_with);
        self.functions.insert("str_replace".to_string(), stdlib_str_replace);
        self.functions.insert("str_substring".to_string(), stdlib_str_substring);
    }
    
    fn register_array_functions(&mut self) {
        self.functions.insert("array_len".to_string(), stdlib_array_len);
        self.functions.insert("array_push".to_string(), stdlib_array_push);
        self.functions.insert("array_pop".to_string(), stdlib_array_pop);
        self.functions.insert("array_slice".to_string(), stdlib_array_slice);
        self.functions.insert("array_concat".to_string(), stdlib_array_concat);
        self.functions.insert("array_reverse".to_string(), stdlib_array_reverse);
        self.functions.insert("array_sort".to_string(), stdlib_array_sort);
        self.functions.insert("array_map".to_string(), stdlib_array_map);
        self.functions.insert("array_filter".to_string(), stdlib_array_filter);
        self.functions.insert("array_reduce".to_string(), stdlib_array_reduce);
        self.functions.insert("array_find".to_string(), stdlib_array_find);
        self.functions.insert("array_contains".to_string(), stdlib_array_contains);
    }
    
    fn register_object_functions(&mut self) {
        self.functions.insert("object_keys".to_string(), stdlib_object_keys);
        self.functions.insert("object_values".to_string(), stdlib_object_values);
        self.functions.insert("object_entries".to_string(), stdlib_object_entries);
        self.functions.insert("object_has_key".to_string(), stdlib_object_has_key);
        self.functions.insert("object_merge".to_string(), stdlib_object_merge);
    }
    
    fn register_math_functions(&mut self) {
        self.functions.insert("abs".to_string(), stdlib_abs);
        self.functions.insert("min".to_string(), stdlib_min);
        self.functions.insert("max".to_string(), stdlib_max);
        self.functions.insert("floor".to_string(), stdlib_floor);
        self.functions.insert("ceil".to_string(), stdlib_ceil);
        self.functions.insert("round".to_string(), stdlib_round);
        self.functions.insert("sqrt".to_string(), stdlib_sqrt);
        self.functions.insert("pow".to_string(), stdlib_pow);
        self.functions.insert("random".to_string(), stdlib_random);
    }
    
    fn register_io_functions(&mut self) {
        self.functions.insert("read_file".to_string(), stdlib_read_file);
        self.functions.insert("write_file".to_string(), stdlib_write_file);
        self.functions.insert("file_exists".to_string(), stdlib_file_exists);
    }
    
    fn register_time_functions(&mut self) {
        self.functions.insert("now".to_string(), stdlib_now);
        self.functions.insert("sleep".to_string(), stdlib_sleep);
    }
    
    fn register_type_functions(&mut self) {
        self.functions.insert("type_of".to_string(), stdlib_type_of);
        self.functions.insert("to_string".to_string(), stdlib_to_string);
        self.functions.insert("str".to_string(), stdlib_to_string); // Alias for to_string
        self.functions.insert("to_int".to_string(), stdlib_to_int);
        self.functions.insert("to_float".to_string(), stdlib_to_float);
        self.functions.insert("to_bool".to_string(), stdlib_to_bool);
    }
}

// Core functions
fn stdlib_print(args: Vec<Value>) -> Result<Value> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    io::stdout().flush().map_err(|e| FlowError::runtime_error(&format!("IO error: {}", e)))?;
    Ok(Value::Null)
}

fn stdlib_println(args: Vec<Value>) -> Result<Value> {
    stdlib_print(args)?;
    println!();
    Ok(Value::Null)
}

fn stdlib_input(args: Vec<Value>) -> Result<Value> {
    if !args.is_empty() {
        print!("{}", args[0]);
        io::stdout().flush().map_err(|e| FlowError::runtime_error(&format!("IO error: {}", e)))?;
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| FlowError::runtime_error(&format!("IO error: {}", e)))?;
    
    // Remove trailing newline
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    
    Ok(Value::String(input))
}

fn stdlib_assert(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(FlowError::runtime_error("assert requires at least one argument"));
    }
    
    let condition = &args[0];
    if !condition.is_truthy() {
        let message = if args.len() > 1 {
            format!("{}", args[1])
        } else {
            "Assertion failed".to_string()
        };
        return Err(FlowError::runtime_error(&message));
    }
    
    Ok(Value::Null)
}

fn stdlib_panic(args: Vec<Value>) -> Result<Value> {
    let message = if args.is_empty() {
        "panic called".to_string()
    } else {
        format!("{}", args[0])
    };
    Err(FlowError::runtime_error(&message))
}

// String functions
fn stdlib_str_len(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("str_len requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::Integer(s.len() as i64)),
        _ => Err(FlowError::type_error("str_len requires a string argument".to_string())),
    }
}

fn stdlib_str_upper(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("str_upper requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(FlowError::type_error("str_upper requires a string argument".to_string())),
    }
}

fn stdlib_str_lower(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("str_lower requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(FlowError::type_error("str_lower requires a string argument".to_string())),
    }
}

fn stdlib_str_trim(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("str_trim requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(FlowError::type_error("str_trim requires a string argument".to_string())),
    }
}

fn stdlib_str_split(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("str_split requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(delimiter)) => {
            let parts: Vec<Value> = s.split(delimiter)
                .map(|part| Value::String(part.to_string()))
                .collect();
            
            let mut array = FlowArray::new();
            for part in parts {
                array.push(part);
            }
            
            Ok(Value::Array(array))
        }
        _ => Err(FlowError::type_error("str_split requires two string arguments".to_string())),
    }
}

fn stdlib_str_join(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("str_join requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::String(separator)) => {
            let strings: Result<Vec<String>> = arr.elements.iter()
                .map(|elem| match elem {
                    Value::String(s) => Ok(s.clone()),
                    other => Ok(format!("{}", other)),
                })
                .collect();
            
            match strings {
                Ok(strs) => Ok(Value::String(strs.join(separator))),
                Err(e) => Err(e),
            }
        }
        _ => Err(FlowError::type_error("str_join requires an array and a string".to_string())),
    }
}

fn stdlib_str_contains(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("str_contains requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(needle)) => {
            Ok(Value::Boolean(s.contains(needle)))
        }
        _ => Err(FlowError::type_error("str_contains requires two string arguments".to_string())),
    }
}

fn stdlib_str_starts_with(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("str_starts_with requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(prefix)) => {
            Ok(Value::Boolean(s.starts_with(prefix)))
        }
        _ => Err(FlowError::type_error("str_starts_with requires two string arguments".to_string())),
    }
}

fn stdlib_str_ends_with(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("str_ends_with requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(suffix)) => {
            Ok(Value::Boolean(s.ends_with(suffix)))
        }
        _ => Err(FlowError::type_error("str_ends_with requires two string arguments".to_string())),
    }
}

fn stdlib_str_replace(args: Vec<Value>) -> Result<Value> {
    if args.len() != 3 {
        return Err(FlowError::runtime_error("str_replace requires exactly three arguments"));
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::String(from), Value::String(to)) => {
            Ok(Value::String(s.replace(from, to)))
        }
        _ => Err(FlowError::type_error("str_replace requires three string arguments".to_string())),
    }
}

fn stdlib_str_substring(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(FlowError::runtime_error("str_substring requires 2 or 3 arguments"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let start = match &args[1] {
                Value::Integer(i) => *i as usize,
                _ => return Err(FlowError::type_error("str_substring start index must be an integer".to_string())),
            };
            
            let end = if args.len() == 3 {
                match &args[2] {
                    Value::Integer(i) => *i as usize,
                    _ => return Err(FlowError::type_error("str_substring end index must be an integer".to_string())),
                }
            } else {
                s.len()
            };
            
            if start > s.len() || end > s.len() || start > end {
                return Err(FlowError::runtime_error("str_substring: invalid indices"));
            }
            
            Ok(Value::String(s[start..end].to_string()))
        }
        _ => Err(FlowError::type_error("str_substring requires a string as first argument".to_string())),
    }
}

// Array functions
fn stdlib_array_len(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("array_len requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => Ok(Value::Integer(arr.len() as i64)),
        _ => Err(FlowError::type_error("array_len requires an array argument".to_string())),
    }
}

fn stdlib_array_push(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("array_push requires exactly two arguments"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Ok(Value::Array(new_arr))
        }
        _ => Err(FlowError::type_error("array_push requires an array as first argument".to_string())),
    }
}

fn stdlib_array_pop(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("array_pop requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            match new_arr.pop() {
                Some(value) => Ok(value),
                None => Ok(Value::Null),
            }
        }
        _ => Err(FlowError::type_error("array_pop requires an array argument".to_string())),
    }
}

fn stdlib_array_slice(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 || args.len() > 3 {
        return Err(FlowError::runtime_error("array_slice requires 2 or 3 arguments"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let start = match &args[1] {
                Value::Integer(i) => *i as usize,
                _ => return Err(FlowError::type_error("array_slice start index must be an integer".to_string())),
            };
            
            let end = if args.len() == 3 {
                match &args[2] {
                    Value::Integer(i) => *i as usize,
                    _ => return Err(FlowError::type_error("array_slice end index must be an integer".to_string())),
                }
            } else {
                arr.len()
            };
            
            match arr.slice(start, end) {
                Ok(sliced) => Ok(Value::Array(sliced)),
                Err(_) => Err(FlowError::runtime_error("array_slice: invalid indices")),
            }
        }
        _ => Err(FlowError::type_error("array_slice requires an array as first argument".to_string())),
    }
}

// Placeholder implementations for remaining functions
fn stdlib_array_concat(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_concat not yet implemented"))
}

fn stdlib_array_reverse(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_reverse not yet implemented"))
}

fn stdlib_array_sort(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_sort not yet implemented"))
}

fn stdlib_array_map(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_map not yet implemented"))
}

fn stdlib_array_filter(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_filter not yet implemented"))
}

fn stdlib_array_reduce(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_reduce not yet implemented"))
}

fn stdlib_array_find(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_find not yet implemented"))
}

fn stdlib_array_contains(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("array_contains not yet implemented"))
}

// Object functions
fn stdlib_object_keys(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("object_keys requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Object(obj) => {
            let mut array = FlowArray::new();
            for key in obj.properties.keys() {
                array.push(Value::String(key.clone()));
            }
            Ok(Value::Array(array))
        }
        _ => Err(FlowError::type_error("object_keys requires an object argument".to_string())),
    }
}

fn stdlib_object_values(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("object_values requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Object(obj) => {
            let mut array = FlowArray::new();
            for value in obj.properties.values() {
                array.push(value.clone());
            }
            Ok(Value::Array(array))
        }
        _ => Err(FlowError::type_error("object_values requires an object argument".to_string())),
    }
}

fn stdlib_object_entries(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("object_entries not yet implemented"))
}

fn stdlib_object_has_key(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("object_has_key not yet implemented"))
}

fn stdlib_object_merge(_args: Vec<Value>) -> Result<Value> {
    Err(FlowError::runtime_error("object_merge not yet implemented"))
}

// Math functions
fn stdlib_abs(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("abs requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(i) => Ok(Value::Integer(i.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        _ => Err(FlowError::type_error("abs requires a numeric argument".to_string())),
    }
}

fn stdlib_min(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(FlowError::runtime_error("min requires at least one argument"));
    }
    
    let mut min_val = &args[0];
    for arg in &args[1..] {
        match (min_val, arg) {
            (Value::Integer(a), Value::Integer(b)) => {
                if b < a {
                    min_val = arg;
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if b < a {
                    min_val = arg;
                }
            }
            (Value::Integer(a), Value::Float(b)) => {
                if b < &(*a as f64) {
                    min_val = arg;
                }
            }
            (Value::Float(a), Value::Integer(b)) => {
                if (*b as f64) < *a {
                    min_val = arg;
                }
            }
            _ => return Err(FlowError::type_error("min requires numeric arguments".to_string())),
        }
    }
    
    Ok(min_val.clone())
}

fn stdlib_max(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(FlowError::runtime_error("max requires at least one argument"));
    }
    
    let mut max_val = &args[0];
    for arg in &args[1..] {
        match (max_val, arg) {
            (Value::Integer(a), Value::Integer(b)) => {
                if b > a {
                    max_val = arg;
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if b > a {
                    max_val = arg;
                }
            }
            (Value::Integer(a), Value::Float(b)) => {
                if b > &(*a as f64) {
                    max_val = arg;
                }
            }
            (Value::Float(a), Value::Integer(b)) => {
                if (*b as f64) > *a {
                    max_val = arg;
                }
            }
            _ => return Err(FlowError::type_error("max requires numeric arguments".to_string())),
        }
    }
    
    Ok(max_val.clone())
}

fn stdlib_floor(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("floor requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(i) => Ok(Value::Integer(*i)),
        Value::Float(f) => Ok(Value::Integer(f.floor() as i64)),
        _ => Err(FlowError::type_error("floor requires a numeric argument".to_string())),
    }
}

fn stdlib_ceil(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("ceil requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(i) => Ok(Value::Integer(*i)),
        Value::Float(f) => Ok(Value::Integer(f.ceil() as i64)),
        _ => Err(FlowError::type_error("ceil requires a numeric argument".to_string())),
    }
}

fn stdlib_round(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("round requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(i) => Ok(Value::Integer(*i)),
        Value::Float(f) => Ok(Value::Integer(f.round() as i64)),
        _ => Err(FlowError::type_error("round requires a numeric argument".to_string())),
    }
}

fn stdlib_sqrt(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("sqrt requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(i) => {
            if *i < 0 {
                Err(FlowError::runtime_error("sqrt of negative number"))
            } else {
                Ok(Value::Float((*i as f64).sqrt()))
            }
        }
        Value::Float(f) => {
            if *f < 0.0 {
                Err(FlowError::runtime_error("sqrt of negative number"))
            } else {
                Ok(Value::Float(f.sqrt()))
            }
        }
        _ => Err(FlowError::type_error("sqrt requires a numeric argument".to_string())),
    }
}

fn stdlib_pow(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("pow requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(base), Value::Integer(exp)) => {
            if *exp < 0 {
                Ok(Value::Float((*base as f64).powf(*exp as f64)))
            } else {
                Ok(Value::Integer(base.pow(*exp as u32)))
            }
        }
        (Value::Float(base), Value::Float(exp)) => {
            Ok(Value::Float(base.powf(*exp)))
        }
        (Value::Integer(base), Value::Float(exp)) => {
            Ok(Value::Float((*base as f64).powf(*exp)))
        }
        (Value::Float(base), Value::Integer(exp)) => {
            Ok(Value::Float(base.powf(*exp as f64)))
        }
        _ => Err(FlowError::type_error("pow requires numeric arguments".to_string())),
    }
}

fn stdlib_random(_args: Vec<Value>) -> Result<Value> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Simple pseudo-random number generator
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    
    let hash = hasher.finish();
    let random_float = (hash as f64) / (u64::MAX as f64);
    
    Ok(Value::Float(random_float))
}

// IO functions
fn stdlib_read_file(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("read_file requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            match std::fs::read_to_string(path) {
                Ok(content) => Ok(Value::String(content)),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to read file: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("read_file requires a string argument".to_string())),
    }
}

fn stdlib_write_file(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("write_file requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => {
            match std::fs::write(path, content) {
                Ok(_) => Ok(Value::Null),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to write file: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("write_file requires two string arguments".to_string())),
    }
}

fn stdlib_file_exists(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("file_exists requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            Ok(Value::Boolean(std::path::Path::new(path).exists()))
        }
        _ => Err(FlowError::type_error("file_exists requires a string argument".to_string())),
    }
}

// Time functions
fn stdlib_now(_args: Vec<Value>) -> Result<Value> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    Ok(Value::Integer(timestamp as i64))
}

fn stdlib_sleep(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("sleep requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(ms) => {
            std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
            Ok(Value::Null)
        }
        Value::Float(ms) => {
            std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
            Ok(Value::Null)
        }
        _ => Err(FlowError::type_error("sleep requires a numeric argument (milliseconds)".to_string())),
    }
}

// Type functions
fn stdlib_type_of(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("type_of requires exactly one argument"));
    }
    
    let type_name = args[0].type_name();
    Ok(Value::String(type_name.to_string()))
}

fn stdlib_to_string(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("to_string requires exactly one argument"));
    }
    
    Ok(Value::String(format!("{}", args[0])))
}

fn stdlib_to_int(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("to_int requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(i) => Ok(Value::Integer(*i)),
        Value::Float(f) => Ok(Value::Integer(*f as i64)),
        Value::String(s) => {
            match s.parse::<i64>() {
                Ok(i) => Ok(Value::Integer(i)),
                Err(_) => Err(FlowError::runtime_error(&format!("Cannot convert '{}' to integer", s))),
            }
        }
        Value::Boolean(b) => Ok(Value::Integer(if *b { 1 } else { 0 })),
        _ => Err(FlowError::type_error("Cannot convert to integer".to_string())),
    }
}

fn stdlib_to_float(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("to_float requires exactly one argument"));
    }
    
    match &args[0] {
        Value::Integer(i) => Ok(Value::Float(*i as f64)),
        Value::Float(f) => Ok(Value::Float(*f)),
        Value::String(s) => {
            match s.parse::<f64>() {
                Ok(f) => Ok(Value::Float(f)),
                Err(_) => Err(FlowError::runtime_error(&format!("Cannot convert '{}' to float", s))),
            }
        }
        _ => Err(FlowError::type_error("Cannot convert to float".to_string())),
    }
}

fn stdlib_to_bool(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("to_bool requires exactly one argument"));
    }
    
    Ok(Value::Boolean(args[0].is_truthy()))
}