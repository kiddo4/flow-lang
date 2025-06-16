//! Standard I/O Module for FlowLang
//! 
//! Provides file operations, directory handling, and stream management.

use crate::error::{FlowError, Result};
use crate::value::{FlowArray, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader};
use std::path::Path;

/// Read entire file contents as string
pub fn read_file(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("read_file expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            match fs::read_to_string(path) {
                Ok(content) => Ok(Value::String(content)),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to read file: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("read_file requires a string argument".to_string())),
    }
}

/// Write string content to file
pub fn write_file(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("write_file requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => {
            match fs::write(path, content) {
                Ok(_) => Ok(Value::Null),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to write file: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("write_file requires two string arguments".to_string())),
    }
}

/// Append string content to file
pub fn append_file(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("append_file requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => {
            match OpenOptions::new().create(true).append(true).open(path) {
                Ok(mut file) => {
                    match file.write_all(content.as_bytes()) {
                        Ok(_) => Ok(Value::Null),
                        Err(e) => Err(FlowError::runtime_error(&format!("Failed to append to file: {}", e))),
                    }
                }
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to open file for append: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("append_file requires two string arguments".to_string())),
    }
}

/// Read file lines as array
pub fn read_lines(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("read_lines requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            match File::open(path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let mut array = FlowArray::new();
                    
                    for line in reader.lines() {
                        match line {
                            Ok(line_content) => array.push(Value::String(line_content)),
                            Err(e) => return Err(FlowError::runtime_error(&format!("Error reading line: {}", e))),
                        }
                    }
                    
                    Ok(Value::Array(array))
                }
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to open file: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("read_lines requires a string argument".to_string())),
    }
}

/// Check if file exists
pub fn file_exists(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("file_exists requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => Ok(Value::Boolean(Path::new(path).exists())),
        _ => Err(FlowError::type_error("file_exists requires a string argument".to_string())),
    }
}

/// Check if path is directory
pub fn is_directory(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("is_directory requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => Ok(Value::Boolean(Path::new(path).is_dir())),
        _ => Err(FlowError::type_error("is_directory requires a string argument".to_string())),
    }
}

/// Check if path is file
pub fn is_file(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("is_file requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => Ok(Value::Boolean(Path::new(path).is_file())),
        _ => Err(FlowError::type_error("is_file requires a string argument".to_string())),
    }
}

/// Create directory
pub fn create_dir(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("create_dir requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            match fs::create_dir_all(path) {
                Ok(_) => Ok(Value::Null),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to create directory: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("create_dir requires a string argument".to_string())),
    }
}

/// Remove file or directory
pub fn remove_path(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("remove_path requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            let path_obj = Path::new(path);
            if path_obj.is_dir() {
                match fs::remove_dir_all(path) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => Err(FlowError::runtime_error(&format!("Failed to remove directory: {}", e))),
                }
            } else {
                match fs::remove_file(path) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => Err(FlowError::runtime_error(&format!("Failed to remove file: {}", e))),
                }
            }
        }
        _ => Err(FlowError::type_error("remove_path requires a string argument".to_string())),
    }
}

/// List directory contents
pub fn list_dir(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("list_dir requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            match fs::read_dir(path) {
                Ok(entries) => {
                    let mut array = FlowArray::new();
                    
                    for entry in entries {
                        match entry {
                            Ok(entry) => {
                                if let Some(name) = entry.file_name().to_str() {
                                    array.push(Value::String(name.to_string()));
                                }
                            }
                            Err(e) => return Err(FlowError::runtime_error(&format!("Error reading directory entry: {}", e))),
                        }
                    }
                    
                    Ok(Value::Array(array))
                }
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to read directory: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("list_dir requires a string argument".to_string())),
    }
}

/// Copy file
pub fn copy_file(args: Vec<Value>) -> Result<Value> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("copy_file requires exactly two arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(src), Value::String(dst)) => {
            match fs::copy(src, dst) {
                Ok(_) => Ok(Value::Null),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to copy file: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("copy_file requires two string arguments".to_string())),
    }
}

/// Get file size
pub fn file_size(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("file_size requires exactly one argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            match fs::metadata(path) {
                Ok(metadata) => Ok(Value::Integer(metadata.len() as i64)),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to get file metadata: {}", e))),
            }
        }
        _ => Err(FlowError::type_error("file_size requires a string argument".to_string())),
    }
}