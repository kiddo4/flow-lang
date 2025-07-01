//! FlowLang Standard Library Modules
//! 
//! This module provides a comprehensive standard library for FlowLang,
//! organized into logical modules for different functionality areas.

pub mod io;
pub mod system;
pub mod net;
pub mod json;
pub mod crypto;

use crate::value::Value;
use crate::error::FlowError;
use std::collections::HashMap;

/// Standard library function type
pub type StdLibFunction = fn(Vec<Value>) -> Result<Value, FlowError>;

/// Registry for all standard library functions
pub struct StandardLibraryRegistry {
    functions: HashMap<String, StdLibFunction>,
}

impl StandardLibraryRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        
        registry.register_all_functions();
        registry
    }
    
    /// Register all standard library functions
    fn register_all_functions(&mut self) {
        // I/O functions
        self.register("read_file", io::read_file);
        self.register("write_file", io::write_file);
        self.register("append_file", io::append_file);
        self.register("read_lines", io::read_lines);
        self.register("copy_file", io::copy_file);
        self.register("file_exists", io::file_exists);
        self.register("is_directory", io::is_directory);
        self.register("is_file", io::is_file);
        self.register("create_dir", io::create_dir);
        self.register("remove_path", io::remove_path);
        self.register("list_dir", io::list_dir);
        self.register("file_size", io::file_size);
        
        // System functions
        self.register("get_env", system::get_env);
        self.register("set_env", system::set_env);
        self.register("remove_env", system::remove_env);
        self.register("get_all_env", system::get_all_env);
        self.register("execute_command", system::execute_command);
        self.register("get_current_dir", system::get_current_dir);
        self.register("change_dir", system::change_dir);
        self.register("get_args", system::get_args);
        self.register("exit_program", system::exit_program);
        
        // Networking functions
        self.register("http_get", net::http_get);
        self.register("http_post", net::http_post);
        self.register("http_put", net::http_put);
        self.register("http_delete", net::http_delete);
        self.register("url_encode", net::url_encode);
        self.register("url_decode", net::url_decode);
        
        // JSON functions
        self.register("json_parse", json::json_parse);
        self.register("json_stringify", json::json_stringify);
        
        // Cryptographic functions
        self.register("hash_string", crypto::hash_string);
        self.register("md5_hash", crypto::md5_hash);
        self.register("sha256_hash", crypto::sha256_hash);
        self.register("base64_encode", crypto::base64_encode);
        self.register("base64_decode", crypto::base64_decode);
        self.register("hex_encode", crypto::hex_encode);
        self.register("hex_decode", crypto::hex_decode);
        self.register("random_int", crypto::random_int);
        self.register("random_float", crypto::random_float);
        self.register("random_string", crypto::random_string);
        self.register("set_random_seed", crypto::set_random_seed);
        self.register("generate_uuid", crypto::generate_uuid);
    }
    
    /// Register a single function
    pub fn register(&mut self, name: &str, function: StdLibFunction) {
        self.functions.insert(name.to_string(), function);
    }
    
    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&StdLibFunction> {
        self.functions.get(name)
    }
    
    /// Get all function names
    pub fn get_function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
    
    /// Check if a function exists
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
    
    /// Call a function by name
    pub fn call_function(&self, name: &str, args: Vec<Value>) -> Result<Value, FlowError> {
        match self.functions.get(name) {
            Some(function) => function(args),
            None => Err(FlowError::runtime_error(&format!("Unknown function: {}", name)))
        }
    }
}

impl Default for StandardLibraryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a new standard library registry
pub fn create_stdlib() -> StandardLibraryRegistry {
    StandardLibraryRegistry::new()
}

/// Get the list of all available standard library functions
pub fn get_stdlib_functions() -> Vec<String> {
    let registry = StandardLibraryRegistry::new();
    registry.get_function_names()
}

/// Check if a function is part of the standard library
pub fn is_stdlib_function(name: &str) -> bool {
    let registry = StandardLibraryRegistry::new();
    registry.has_function(name)
}