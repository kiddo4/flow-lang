//! Shared value types for the Flow language

use std::collections::HashMap;
use std::fmt;
use crate::ast::{Statement, Expression, Parameter};
use crate::bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    BigInteger(BigInt),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(FlowArray),
    Object(FlowObject),
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Lambda {
        parameters: Vec<Parameter>,
        body: Box<Expression>,
        closure: Environment,
    },
    BytecodeFunction {
        address: usize,
        arity: usize,
        locals_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowArray {
    pub elements: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowObject {
    pub properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl FlowArray {
    pub fn new() -> Self {
        FlowArray {
            elements: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        FlowArray {
            elements: Vec::with_capacity(capacity),
        }
    }

    pub fn from_values(values: Vec<Value>) -> Self {
        FlowArray { elements: values }
    }

    pub fn push(&mut self, value: Value) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.elements.pop()
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    pub fn set(&mut self, index: usize, value: Value) -> Result<(), String> {
        if index < self.elements.len() {
            self.elements[index] = value;
            Ok(())
        } else {
            Err(format!("Index {} out of bounds for array of length {}", index, self.elements.len()))
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn contains(&self, value: &Value) -> bool {
        self.elements.contains(value)
    }

    pub fn index_of(&self, value: &Value) -> Option<usize> {
        self.elements.iter().position(|x| x == value)
    }

    pub fn reverse(&mut self) {
        self.elements.reverse();
    }

    pub fn sort(&mut self) {
        self.elements.sort_by(|a, b| {
            match (a, b) {
                (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
                (Value::Float(a), Value::Float(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
                (Value::String(a), Value::String(b)) => a.cmp(b),
                (Value::Boolean(a), Value::Boolean(b)) => a.cmp(b),
                _ => std::cmp::Ordering::Equal,
            }
        });
    }

    pub fn slice(&self, start: usize, end: usize) -> Result<FlowArray, String> {
        if start > end {
            return Err("Start index cannot be greater than end index".to_string());
        }
        if start > self.elements.len() {
            return Err("Start index out of bounds".to_string());
        }
        let end = end.min(self.elements.len());
        Ok(FlowArray {
            elements: self.elements[start..end].to_vec(),
        })
    }
}

impl FlowObject {
    pub fn new() -> Self {
        FlowObject {
            properties: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        FlowObject {
            properties: HashMap::with_capacity(capacity),
        }
    }

    pub fn from_map(map: HashMap<String, Value>) -> Self {
        FlowObject { properties: map }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.properties.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.properties.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }

    pub fn values(&self) -> Vec<&Value> {
        self.properties.values().collect()
    }

    pub fn len(&self) -> usize {
        self.properties.len()
    }

    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    pub fn clear(&mut self) {
        self.properties.clear();
    }

    pub fn merge(&mut self, other: &FlowObject) {
        for (key, value) in &other.properties {
            self.properties.insert(key.clone(), value.clone());
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn define_function(&mut self, name: String, value: Value) {
        self.functions.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value)
        } else if let Some(parent) = &self.parent {
            parent.get_variable(name)
        } else {
            None
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.functions.get(name) {
            Some(value)
        } else if let Some(parent) = &self.parent {
            parent.get_function(name)
        } else {
            None
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.set_variable(name, value)
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }

    pub fn push_scope(&mut self) {
        let new_env = Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        };
        let old_env = std::mem::replace(self, new_env);
        self.parent = Some(Box::new(old_env));
    }

    pub fn pop_scope(&mut self) {
        if let Some(parent) = self.parent.take() {
            *self = *parent;
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::BigInteger(_) => "biginteger",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function { .. } => "function",
            Value::Lambda { .. } => "lambda",
            Value::BytecodeFunction { .. } => "bytecode_function",
        }
    }
    
    pub fn to_bigint(&self) -> Option<BigInt> {
        match self {
            Value::Integer(i) => Some(BigInt::from_i64(*i)),
            Value::BigInteger(bi) => Some(bi.clone()),
            _ => None,
        }
    }
    
    pub fn promote_to_bigint(self) -> Value {
        match self {
            Value::Integer(i) => Value::BigInteger(BigInt::from_i64(i)),
            other => other,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::BigInteger(bi) => write!(f, "{}", bi),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, elem) in arr.elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")?;
        Ok(())
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in &obj.properties {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                    first = false;
                }
                write!(f, "}}");
                Ok(())
            }
            Value::Function { name, .. } => write!(f, "<function {}>", name),
            Value::Lambda { .. } => write!(f, "<lambda>"),
            Value::BytecodeFunction { .. } => write!(f, "<bytecode function>"),
        }
    }


}

impl fmt::Display for FlowArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, elem) in self.elements.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl fmt::Display for FlowObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for (key, value) in &self.properties {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "\"{}\": {}", key, value)?;
            first = false;
        }
        write!(f, "}}");
        Ok(())
    }
}