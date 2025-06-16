use crate::value::{Value, FlowObject};
use crate::error::FlowError;
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

// HTTP client functionality
pub fn http_get(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime_error("http_get expects at least 1 argument (URL)"));
    }
    
    let url = match &args[0] {
        Value::String(u) => u,
        _ => return Err(FlowError::runtime_error("http_get expects string URL"))
    };
    
    // Parse headers if provided
    let headers = if args.len() > 1 {
        match &args[1] {
            Value::Object(h) => Some(&h.properties),
            Value::Null => None,
            _ => return Err(FlowError::runtime_error("http_get headers must be an object"))
        }
    } else {
        None
    };
    
    // Simple HTTP GET implementation using std library
    // Note: In a real implementation, you'd want to use a proper HTTP client like reqwest
    match make_http_request("GET", url, headers, None) {
        Ok(response) => Ok(response),
        Err(e) => Err(FlowError::runtime_error(&format!("HTTP GET failed: {}", e)))
    }
}

pub fn http_post(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 {
        return Err(FlowError::runtime_error("http_post expects at least 2 arguments (URL, body)"));
    }
    
    let url = match &args[0] {
        Value::String(u) => u,
        _ => return Err(FlowError::runtime_error("http_post expects string URL"))
    };
    
    let body = match &args[1] {
        Value::String(b) => Some(b.as_str()),
        Value::Null => None,
        _ => return Err(FlowError::runtime_error("http_post body must be a string"))
    };
    
    // Parse headers if provided
    let headers = if args.len() > 2 {
        match &args[2] {
            Value::Object(h) => Some(&h.properties),
            Value::Null => None,
            _ => return Err(FlowError::runtime_error("http_post headers must be an object"))
        }
    } else {
        None
    };
    
    match make_http_request("POST", url, headers, body) {
        Ok(response) => Ok(response),
        Err(e) => Err(FlowError::runtime_error(&format!("HTTP POST failed: {}", e)))
    }
}

pub fn http_put(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 {
        return Err(FlowError::runtime_error("http_put expects at least 2 arguments (URL, body)"));
    }
    
    let url = match &args[0] {
        Value::String(u) => u,
        _ => return Err(FlowError::runtime_error("http_put expects string URL"))
    };
    
    let body = match &args[1] {
        Value::String(b) => Some(b.as_str()),
        Value::Null => None,
        _ => return Err(FlowError::runtime_error("http_put body must be a string"))
    };
    
    let headers = if args.len() > 2 {
        match &args[2] {
            Value::Object(h) => Some(&h.properties),
            Value::Null => None,
            _ => return Err(FlowError::runtime_error("http_put headers must be an object"))
        }
    } else {
        None
    };
    
    match make_http_request("PUT", url, headers, body) {
        Ok(response) => Ok(response),
        Err(e) => Err(FlowError::runtime_error(&format!("HTTP PUT failed: {}", e)))
    }
}

pub fn http_delete(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime_error("http_delete expects at least 1 argument (URL)"));
    }
    
    let url = match &args[0] {
        Value::String(u) => u,
        _ => return Err(FlowError::runtime_error("http_delete expects string URL"))
    };
    
    let headers = if args.len() > 1 {
        match &args[1] {
            Value::Object(h) => Some(&h.properties),
            Value::Null => None,
            _ => return Err(FlowError::runtime_error("http_delete headers must be an object"))
        }
    } else {
        None
    };
    
    match make_http_request("DELETE", url, headers, None) {
        Ok(response) => Ok(response),
        Err(e) => Err(FlowError::runtime_error(&format!("HTTP DELETE failed: {}", e)))
    }
}

// Helper function for making HTTP requests
// Note: This is a simplified implementation. In production, use a proper HTTP client
fn make_http_request(
    method: &str,
    url: &str,
    headers: Option<&HashMap<String, Value>>,
    body: Option<&str>
) -> Result<Value, String> {
    // This is a placeholder implementation
    // In a real implementation, you would use a proper HTTP client library
    
    let mut response = HashMap::new();
    response.insert("status".to_string(), Value::Integer(200));
    response.insert("body".to_string(), Value::String(format!("Mock response for {} {}", method, url)));
    response.insert("headers".to_string(), Value::Object(FlowObject { properties: HashMap::new() }));
    
    // Add request info for debugging
    let mut request_info = HashMap::new();
    request_info.insert("method".to_string(), Value::String(method.to_string()));
    request_info.insert("url".to_string(), Value::String(url.to_string()));
    
    if let Some(h) = headers {
        request_info.insert("headers".to_string(), Value::Object(FlowObject { properties: h.clone() }));
    }
    
    if let Some(b) = body {
        request_info.insert("body".to_string(), Value::String(b.to_string()));
    }
    
    response.insert("request".to_string(), Value::Object(FlowObject { properties: request_info }));

    Ok(Value::Object(FlowObject { properties: response }))
}

// URL utilities
pub fn url_encode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("url_encode expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let encoded = urlencoding::encode(s);
            Ok(Value::String(encoded.to_string()))
        }
        _ => Err(FlowError::runtime_error("url_encode expects string argument"))
    }
}

pub fn url_decode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("url_decode expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            match urlencoding::decode(s) {
                Ok(decoded) => Ok(Value::String(decoded.to_string())),
                Err(e) => Err(FlowError::runtime_error(&format!("URL decode failed: {}", e)))
            }
        }
        _ => Err(FlowError::runtime_error("url_decode expects string argument"))
    }
}

// Simple URL encoding implementation (fallback if urlencoding crate not available)
mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut result = String::new();
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
        result
    }
    
    pub fn decode(input: &str) -> Result<String, String> {
        let mut result = Vec::new();
        let mut chars = input.chars();
        
        while let Some(ch) = chars.next() {
            match ch {
                '%' => {
                    let hex1 = chars.next().ok_or("Invalid URL encoding")?;
                    let hex2 = chars.next().ok_or("Invalid URL encoding")?;
                    let hex_str = format!("{}{}", hex1, hex2);
                    let byte = u8::from_str_radix(&hex_str, 16)
                        .map_err(|_| "Invalid hex in URL encoding")?;
                    result.push(byte);
                }
                '+' => result.push(b' '),
                _ => result.push(ch as u8),
            }
        }
        
        String::from_utf8(result).map_err(|_| "Invalid UTF-8 in decoded URL".to_string())
    }
}