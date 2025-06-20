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

// Helper function for making HTTP requests using std library
fn make_http_request(
    method: &str,
    url: &str,
    headers: Option<&HashMap<String, Value>>,
    body: Option<&str>
) -> Result<Value, String> {
    use std::io::{Write, BufRead, BufReader, Read, BufRead as _};
    use std::net::{TcpStream, ToSocketAddrs};
    
    // Parse URL
    let url_parts = parse_url(url)?;
    let host = &url_parts.host;
    let port = url_parts.port;
    let path = &url_parts.path;
    let is_https = url_parts.scheme == "https";
    
    // For HTTPS, we'll need to use a different approach
    // For now, let's implement HTTP only to avoid TLS dependencies
    if is_https {
        return Err("HTTPS not supported in this simple implementation. Use HTTP URLs for testing.".to_string());
    }
    
    // Connect to server
    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(&addr)
        .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;
    
    // Build HTTP request
    let mut request = format!("{} {} HTTP/1.1\r\n", method.to_uppercase(), path);
    request.push_str(&format!("Host: {}\r\n", host));
    request.push_str("Connection: close\r\n");
    request.push_str("User-Agent: FlowLang/1.0\r\n");
    
    // Add custom headers
    if let Some(h) = headers {
        for (key, value) in h {
            if let Value::String(val_str) = value {
                request.push_str(&format!("{}: {}\r\n", key, val_str));
            }
        }
    }
    
    // Add body if present
    if let Some(body_content) = body {
        request.push_str(&format!("Content-Length: {}\r\n", body_content.len()));
        request.push_str("\r\n");
        request.push_str(body_content);
    } else {
        request.push_str("\r\n");
    }
    
    // Send request
    stream.write_all(request.as_bytes())
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Read response
    let mut reader = BufReader::new(stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line)
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    // Parse status line
    let status_code = parse_status_line(&response_line)?;
    
    // Read headers
    let mut response_headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)
            .map_err(|e| format!("Failed to read headers: {}", e))?;
        
        if line.trim().is_empty() {
            break; // End of headers
        }
        
        if let Some((key, value)) = parse_header_line(&line) {
            response_headers.insert(key, Value::String(value));
        }
    }
    
    // Read body (handle chunked encoding if present)
    let mut response_body = String::new();
    
    // Check if response uses chunked encoding
    let is_chunked = response_headers.get("transfer-encoding")
        .map(|v| {
            if let Value::String(s) = v {
                s.to_lowercase().contains("chunked")
            } else {
                false
            }
        })
        .unwrap_or(false);
    
    if is_chunked {
        // Handle chunked encoding
        loop {
            let mut chunk_size_line = String::new();
            reader.read_line(&mut chunk_size_line)
                .map_err(|e| format!("Failed to read chunk size: {}", e))?;
            
            let chunk_size_str = chunk_size_line.trim();
            if chunk_size_str.is_empty() {
                continue;
            }
            
            // Parse chunk size (hexadecimal)
            let chunk_size = match usize::from_str_radix(chunk_size_str, 16) {
                Ok(size) => size,
                Err(_) => {
                    // If we can't parse as hex, might be end of chunks
                    if chunk_size_str == "0" {
                        break;
                    }
                    return Err(format!("Invalid chunk size: {}", chunk_size_str));
                }
            };
            
            if chunk_size == 0 {
                break; // End of chunks
            }
            
            // Read chunk data
            let mut chunk_data = vec![0u8; chunk_size];
            reader.read_exact(&mut chunk_data)
                .map_err(|e| format!("Failed to read chunk data: {}", e))?;
            
            // Convert to string and append
            let chunk_str = String::from_utf8_lossy(&chunk_data);
            response_body.push_str(&chunk_str);
            
            // Read trailing CRLF after chunk
            let mut trailing = String::new();
            reader.read_line(&mut trailing)
                .map_err(|e| format!("Failed to read chunk trailing: {}", e))?;
        }
        
        // Read any trailing headers (usually empty)
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)
                .map_err(|e| format!("Failed to read trailing headers: {}", e))?;
            
            if line.trim().is_empty() {
                break;
            }
        }
    } else {
        // Regular body reading
        reader.read_to_string(&mut response_body)
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    }
    
    // Build response object
    let mut response_map = HashMap::new();
    response_map.insert("status".to_string(), Value::Integer(status_code));
    response_map.insert("body".to_string(), Value::String(response_body));
    response_map.insert("headers".to_string(), Value::Object(FlowObject { properties: response_headers }));
    
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
    
    response_map.insert("request".to_string(), Value::Object(FlowObject { properties: request_info }));

    Ok(Value::Object(FlowObject { properties: response_map }))
}

// Helper struct for URL parsing
struct UrlParts {
    scheme: String,
    host: String,
    port: u16,
    path: String,
}

// Simple URL parser
fn parse_url(url: &str) -> Result<UrlParts, String> {
    let url = url.trim();
    
    // Parse scheme
    let (scheme, rest) = if url.starts_with("http://") {
        ("http".to_string(), &url[7..])
    } else if url.starts_with("https://") {
        ("https".to_string(), &url[8..])
    } else {
        return Err("URL must start with http:// or https://".to_string());
    };
    
    // Find path separator
    let (host_port, path) = if let Some(slash_pos) = rest.find('/') {
        (&rest[..slash_pos], &rest[slash_pos..])
    } else {
        (rest, "/")
    };
    
    // Parse host and port
    let (host, port) = if let Some(colon_pos) = host_port.find(':') {
        let host = host_port[..colon_pos].to_string();
        let port_str = &host_port[colon_pos + 1..];
        let port = port_str.parse::<u16>()
            .map_err(|_| format!("Invalid port number: {}", port_str))?;
        (host, port)
    } else {
        let default_port = if scheme == "https" { 443 } else { 80 };
        (host_port.to_string(), default_port)
    };
    
    Ok(UrlParts {
        scheme,
        host,
        port,
        path: path.to_string(),
    })
}

// Parse HTTP status line
fn parse_status_line(line: &str) -> Result<i64, String> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Invalid status line".to_string());
    }
    
    parts[1].parse::<i64>()
        .map_err(|_| format!("Invalid status code: {}", parts[1]))
}

// Parse HTTP header line
fn parse_header_line(line: &str) -> Option<(String, String)> {
    if let Some(colon_pos) = line.find(':') {
        let key = line[..colon_pos].trim().to_lowercase();
        let value = line[colon_pos + 1..].trim().to_string();
        Some((key, value))
    } else {
        None
    }
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