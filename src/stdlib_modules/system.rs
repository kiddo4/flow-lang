use crate::value::{Value, FlowObject, FlowArray};
use crate::error::FlowError;
use std::env;
use std::process::Command;
use std::collections::HashMap;

// Environment variable operations
pub fn get_env(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("get_env expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(key) => {
            match env::var(key) {
                Ok(value) => Ok(Value::String(value)),
                Err(_) => Ok(Value::Null)
            }
        }
        _ => Err(FlowError::runtime_error("get_env expects string argument"))
    }
}

pub fn set_env(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime_error("set_env expects 2 arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(key), Value::String(value)) => {
            env::set_var(key, value);
            Ok(Value::Null)
        }
        _ => Err(FlowError::runtime_error("set_env expects string arguments"))
    }
}

pub fn remove_env(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("remove_env expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(key) => {
            env::remove_var(key);
            Ok(Value::Null)
        }
        _ => Err(FlowError::runtime_error("remove_env expects string argument"))
    }
}

pub fn get_all_env(_args: Vec<Value>) -> Result<Value, FlowError> {
    let mut env_map = HashMap::new();
    
    for (key, value) in env::vars() {
        env_map.insert(key, Value::String(value));
    }
    
    Ok(Value::Object(FlowObject { properties: env_map }))
}

// Process operations
pub fn execute_command(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime_error("execute_command expects at least 1 argument"));
    }
    
    let command = match &args[0] {
        Value::String(cmd) => cmd,
        _ => return Err(FlowError::runtime_error("execute_command expects string command"))
    };
    
    let mut cmd = Command::new(command);
    
    // Add arguments if provided
    for arg in &args[1..] {
        match arg {
            Value::String(s) => { cmd.arg(s); }
            _ => return Err(FlowError::runtime_error("execute_command arguments must be strings"))
        }
    }
    
    match cmd.output() {
        Ok(output) => {
            let mut result = HashMap::new();
            result.insert("stdout".to_string(), Value::String(String::from_utf8_lossy(&output.stdout).to_string()));
            result.insert("stderr".to_string(), Value::String(String::from_utf8_lossy(&output.stderr).to_string()));
            result.insert("exit_code".to_string(), Value::Integer(output.status.code().unwrap_or(-1) as i64));
            result.insert("success".to_string(), Value::Boolean(output.status.success()));
            Ok(Value::Object(FlowObject { properties: result }))
        }
        Err(e) => Err(FlowError::runtime_error(&format!("Failed to execute command: {}", e)))
    }
}

pub fn get_current_dir(_args: Vec<Value>) -> Result<Value, FlowError> {
    match env::current_dir() {
        Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
        Err(e) => Err(FlowError::runtime_error(&format!("Failed to get current directory: {}", e)))
    }
}

pub fn change_dir(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("change_dir expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(path) => {
            match env::set_current_dir(path) {
                Ok(_) => Ok(Value::Null),
                Err(e) => Err(FlowError::runtime_error(&format!("Failed to change directory: {}", e)))
            }
        }
        _ => Err(FlowError::runtime_error("change_dir expects string argument"))
    }
}

pub fn get_args(_args: Vec<Value>) -> Result<Value, FlowError> {
    let args: Vec<Value> = env::args().map(|arg| Value::String(arg)).collect();
    Ok(Value::Array(FlowArray { elements: args }))
}

pub fn exit_program(args: Vec<Value>) -> Result<Value, FlowError> {
    let exit_code = if args.is_empty() {
        0
    } else {
        match &args[0] {
            Value::Integer(code) => *code as i32,
            _ => return Err(FlowError::runtime_error("exit expects integer argument"))
        }
    };
    
    std::process::exit(exit_code);
}