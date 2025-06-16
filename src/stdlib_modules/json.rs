use crate::value::{Value, FlowObject, FlowArray};
use crate::error::FlowError;
use std::collections::HashMap;

// JSON parsing and stringification
pub fn json_parse(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime_error("json_parse expects 1 argument"));
    }
    
    match &args[0] {
        Value::String(json_str) => {
            match parse_json(json_str) {
                Ok(value) => Ok(value),
                Err(e) => Err(FlowError::runtime_error(&format!("JSON parse error: {}", e)))
            }
        }
        _ => Err(FlowError::runtime_error("json_parse expects string argument"))
    }
}

pub fn json_stringify(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime_error("json_stringify expects at least 1 argument"));
    }
    
    let pretty = if args.len() > 1 {
        match &args[1] {
            Value::Boolean(b) => *b,
            _ => false
        }
    } else {
        false
    };
    
    match stringify_json(&args[0], pretty, 0) {
        Ok(json_str) => Ok(Value::String(json_str)),
        Err(e) => Err(FlowError::runtime_error(&format!("JSON stringify error: {}", e)))
    }
}

// Simple JSON parser implementation
fn parse_json(input: &str) -> Result<Value, String> {
    let mut parser = JsonParser::new(input);
    parser.parse_value()
}

struct JsonParser {
    input: Vec<char>,
    pos: usize,
}

impl JsonParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }
    
    fn current_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }
    
    fn advance(&mut self) {
        self.pos += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        
        match self.current_char() {
            Some('"') => self.parse_string(),
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('t') | Some('f') => self.parse_boolean(),
            Some('n') => self.parse_null(),
            Some(ch) if ch.is_ascii_digit() || ch == '-' => self.parse_number(),
            Some(ch) => Err(format!("Unexpected character: {}", ch)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
    
    fn parse_string(&mut self) -> Result<Value, String> {
        self.advance(); // Skip opening quote
        let mut result = String::new();
        
        while let Some(ch) = self.current_char() {
            match ch {
                '"' => {
                    self.advance();
                    return Ok(Value::String(result));
                }
                '\\' => {
                    self.advance();
                    match self.current_char() {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('/') => result.push('/'),
                        Some('b') => result.push('\u{0008}'),
                        Some('f') => result.push('\u{000C}'),
                        Some('n') => result.push('\n'),
                        Some('r') => result.push('\r'),
                        Some('t') => result.push('\t'),
                        Some('u') => {
                            // Unicode escape sequence
                            self.advance();
                            let mut hex = String::new();
                            for _ in 0..4 {
                                match self.current_char() {
                                    Some(ch) if ch.is_ascii_hexdigit() => {
                                        hex.push(ch);
                                        self.advance();
                                    }
                                    _ => return Err("Invalid unicode escape".to_string()),
                                }
                            }
                            if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                if let Some(unicode_char) = char::from_u32(code) {
                                    result.push(unicode_char);
                                }
                            }
                            continue;
                        }
                        _ => return Err("Invalid escape sequence".to_string()),
                    }
                    self.advance();
                }
                ch => {
                    result.push(ch);
                    self.advance();
                }
            }
        }
        
        Err("Unterminated string".to_string())
    }
    
    fn parse_object(&mut self) -> Result<Value, String> {
        self.advance(); // Skip opening brace
        let mut object = HashMap::new();
        
        self.skip_whitespace();
        
        // Handle empty object
        if self.current_char() == Some('}') {
            self.advance();
            return Ok(Value::Object(FlowObject { properties: object }));
        }
        
        loop {
            self.skip_whitespace();
            
            // Parse key
            let key = match self.parse_string()? {
                Value::String(s) => s,
                _ => return Err("Object key must be string".to_string()),
            };
            
            self.skip_whitespace();
            
            // Expect colon
            if self.current_char() != Some(':') {
                return Err("Expected ':' after object key".to_string());
            }
            self.advance();
            
            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);
            
            self.skip_whitespace();
            
            match self.current_char() {
                Some(',') => {
                    self.advance();
                    continue;
                }
                Some('}') => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or '}' in object".to_string()),
            }
        }
        
        Ok(Value::Object(FlowObject { properties: object }))
    }
    
    fn parse_array(&mut self) -> Result<Value, String> {
        self.advance(); // Skip opening bracket
        let mut array = Vec::new();
        
        self.skip_whitespace();
        
        // Handle empty array
        if self.current_char() == Some(']') {
            self.advance();
            return Ok(Value::Array(FlowArray { elements: array }));
        }
        
        loop {
            let value = self.parse_value()?;
            array.push(value);
            
            self.skip_whitespace();
            
            match self.current_char() {
                Some(',') => {
                    self.advance();
                    continue;
                }
                Some(']') => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or ']' in array".to_string()),
            }
        }
        
        Ok(Value::Array(FlowArray { elements: array }))
    }
    
    fn parse_boolean(&mut self) -> Result<Value, String> {
        if self.input[self.pos..].starts_with(&['t', 'r', 'u', 'e']) {
            self.pos += 4;
            Ok(Value::Boolean(true))
        } else if self.input[self.pos..].starts_with(&['f', 'a', 'l', 's', 'e']) {
            self.pos += 5;
            Ok(Value::Boolean(false))
        } else {
            Err("Invalid boolean value".to_string())
        }
    }
    
    fn parse_null(&mut self) -> Result<Value, String> {
        if self.input[self.pos..].starts_with(&['n', 'u', 'l', 'l']) {
            self.pos += 4;
            Ok(Value::Null)
        } else {
            Err("Invalid null value".to_string())
        }
    }
    
    fn parse_number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        
        // Handle negative sign
        if self.current_char() == Some('-') {
            self.advance();
        }
        
        // Parse integer part
        if !self.current_char().map_or(false, |c| c.is_ascii_digit()) {
            return Err("Invalid number".to_string());
        }
        
        while self.current_char().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }
        
        // Check for decimal point
        let mut is_float = false;
        if self.current_char() == Some('.') {
            is_float = true;
            self.advance();
            
            if !self.current_char().map_or(false, |c| c.is_ascii_digit()) {
                return Err("Invalid number: expected digit after decimal point".to_string());
            }
            
            while self.current_char().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        
        // Check for exponent
        if self.current_char() == Some('e') || self.current_char() == Some('E') {
            is_float = true;
            self.advance();
            
            if self.current_char() == Some('+') || self.current_char() == Some('-') {
                self.advance();
            }
            
            if !self.current_char().map_or(false, |c| c.is_ascii_digit()) {
                return Err("Invalid number: expected digit in exponent".to_string());
            }
            
            while self.current_char().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        
        let number_str: String = self.input[start..self.pos].iter().collect();
        
        if is_float {
            match number_str.parse::<f64>() {
                Ok(f) => Ok(Value::Float(f)),
                Err(_) => Err("Invalid float number".to_string()),
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(i) => Ok(Value::Integer(i)),
                Err(_) => Err("Invalid integer number".to_string()),
            }
        }
    }
}

// JSON stringifier
fn stringify_json(value: &Value, pretty: bool, indent: usize) -> Result<String, String> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Boolean(b) => Ok(b.to_string()),
        Value::Integer(i) => Ok(i.to_string()),
        Value::Float(f) => Ok(f.to_string()),
        Value::String(s) => Ok(format!("\"{}\"", escape_json_string(s))),
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok("[]".to_string());
            }
            
            let mut result = String::from("[");
            
            for (i, item) in arr.elements.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                
                if pretty {
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent + 1));
                }
                
                result.push_str(&stringify_json(item, pretty, indent + 1)?);
            }
            
            if pretty {
                result.push('\n');
                result.push_str(&"  ".repeat(indent));
            }
            
            result.push(']');
            Ok(result)
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                return Ok("{}".to_string());
            }
            
            let mut result = String::from("{");
            
            for (i, (key, value)) in obj.properties.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                
                if pretty {
                    result.push('\n');
                    result.push_str(&"  ".repeat(indent + 1));
                }
                
                result.push_str(&format!("\"{}\"", escape_json_string(key)));
                result.push(':');
                
                if pretty {
                    result.push(' ');
                }
                
                result.push_str(&stringify_json(value, pretty, indent + 1)?);
            }
            
            if pretty {
                result.push('\n');
                result.push_str(&"  ".repeat(indent));
            }
            
            result.push('}');
            Ok(result)
        }
        _ => Err("Cannot stringify this value type to JSON".to_string()),
    }
}

fn escape_json_string(s: &str) -> String {
    let mut result = String::new();
    
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\u{0008}' => result.push_str("\\b"),
            '\u{000C}' => result.push_str("\\f"),
            ch if ch.is_control() => {
                result.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => result.push(ch),
        }
    }
    
    result
}