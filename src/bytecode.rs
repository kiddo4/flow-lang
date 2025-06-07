//! Bytecode Virtual Machine for FlowLang
//! 
//! This module implements a stack-based virtual machine that executes
//! FlowLang bytecode instructions.

use crate::ast::{Statement, Expression, BinaryOperator, UnaryOperator};
use crate::error::FlowError;
use crate::value::{Value, FlowArray, FlowObject};
use std::collections::HashMap;
use std::fmt;

// Bytecode instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack operations
    LoadConstant(usize),    // Load constant from constant pool
    LoadLocal(usize),       // Load local variable
    StoreLocal(usize),      // Store to local variable
    LoadGlobal(String),     // Load global variable
    StoreGlobal(String),    // Store to global variable
    
    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Comparison operations
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Unary operations
    Negate,
    Not,
    
    // Control flow
    Jump(usize),            // Unconditional jump
    JumpIfFalse(usize),     // Jump if top of stack is false
    JumpIfTrue(usize),      // Jump if top of stack is true
    
    // Function operations
    Call(usize),            // Call function with n arguments
    Return,                 // Return from function
    
    // Stack manipulation
    Pop,                    // Remove top of stack
    Duplicate,              // Duplicate top of stack
    
    // Array operations
    MakeArray(usize),       // Create array with n elements
    GetIndex,               // Get array element by index
    SetIndex,               // Set array element by index
    
    // Object operations
    MakeObject(usize),      // Create object with n key-value pairs
    GetProperty(String),    // Get object property
    SetProperty(String),    // Set object property
    
    // Print operation (for debugging)
    Print,
}





/// Bytecode chunk containing instructions and constants
#[derive(Debug, Clone)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>, // Line numbers for debugging
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }
    
    pub fn write_instruction(&mut self, instruction: Instruction, line: usize) {
        self.instructions.push(instruction);
        self.lines.push(line);
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

/// Call frame for function calls
#[derive(Debug, Clone)]
struct CallFrame {
    function_address: usize,
    instruction_pointer: usize,
    stack_offset: usize,
    locals: Vec<Value>,
}

/// Virtual Machine state
pub struct VirtualMachine {
    chunk: Chunk,
    instruction_pointer: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    call_frames: Vec<CallFrame>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            instruction_pointer: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
            call_frames: Vec::new(),
        }
    }
    
    pub fn load_chunk(&mut self, chunk: Chunk) {
        self.chunk = chunk;
        self.instruction_pointer = 0;
    }
    
    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.instruction_pointer >= self.chunk.instructions.len() {
                break;
            }
            
            let instruction = self.chunk.instructions[self.instruction_pointer].clone();
            self.instruction_pointer += 1;
            
            match instruction {
                Instruction::LoadConst(index) => {
                    if index >= self.chunk.constants.len() {
                        return Err(FlowError::runtime_error("Invalid constant index"));
                    }
                    let value = self.chunk.constants[index].clone();
                    self.stack.push(value);
                }
                
                Instruction::LoadGlobal(name) => {
                    if let Some(value) = self.globals.get(&name) {
                        self.stack.push(value.clone());
                    } else {
                        return Err(FlowError::undefined_variable(name.clone()));
                    }
                }
                
                Instruction::StoreGlobal(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.globals.insert(name, value);
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                Instruction::Add => self.binary_op(|a, b| self.add_values(a, b))?,
                Instruction::Subtract => self.binary_op(|a, b| self.subtract_values(a, b))?,
                Instruction::Multiply => self.binary_op(|a, b| self.multiply_values(a, b))?,
                Instruction::Divide => self.binary_op(|a, b| self.divide_values(a, b))?,
                Instruction::Modulo => self.binary_op(|a, b| self.modulo_values(a, b))?,
                
                Instruction::Equal => self.binary_op(|a, b| Ok(Value::Boolean(self.values_equal(a, b))))?,
                Instruction::NotEqual => self.binary_op(|a, b| Ok(Value::Boolean(!self.values_equal(a, b))))?,
                Instruction::Greater => self.binary_op(|a, b| self.compare_values(a, b, |ord| ord.is_gt()))?,
                Instruction::GreaterEqual => self.binary_op(|a, b| self.compare_values(a, b, |ord| ord.is_ge()))?,
                Instruction::Less => self.binary_op(|a, b| self.compare_values(a, b, |ord| ord.is_lt()))?,
                Instruction::LessEqual => self.binary_op(|a, b| self.compare_values(a, b, |ord| ord.is_le()))?,
                
                Instruction::Not => {
                    if let Some(value) = self.stack.pop() {
                        self.stack.push(Value::Boolean(!value.is_truthy()));
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                Instruction::Negate => {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            Value::Integer(i) => self.stack.push(Value::Integer(-i)),
                            Value::Float(f) => self.stack.push(Value::Float(-f)),
                            _ => return Err(FlowError::type_error(format!("Cannot negate {}", value.type_name()))),
                        }
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                Instruction::Jump(address) => {
                    self.instruction_pointer = address;
                }
                
                Instruction::JumpIfFalse(address) => {
                    if let Some(value) = self.stack.pop() {
                        if !value.is_truthy() {
                            self.instruction_pointer = address;
                        }
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                Instruction::Print => {
                    if let Some(value) = self.stack.pop() {
                        println!("{}", value);
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                Instruction::Pop => {
                    if self.stack.pop().is_none() {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                Instruction::Halt => break,
                
                _ => {
                    return Err(FlowError::runtime_error(format!(
                        "Unimplemented instruction: {:?}",
                        instruction
                    )));
                }
            }
        }
        
        Ok(())
    }
    
    fn binary_op<F>(&mut self, op: F) -> Result<()>
    where
        F: FnOnce(&Value, &Value) -> Result<Value>,
    {
        if self.stack.len() < 2 {
            return Err(FlowError::runtime_error("Stack underflow"));
        }
        
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = op(&a, &b)?;
        self.stack.push(result);
        Ok(())
    }
    
    fn add_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x + y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 + y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x + *y as f64)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{}{}", x, y))),
            _ => Err(FlowError::type_error(format!(
                "Cannot add {} and {}",
                a.type_name(),
                b.type_name()
            ))),
        }
    }
    
    fn subtract_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x - y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x - y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 - y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x - *y as f64)),
            _ => Err(FlowError::type_error(format!(
                "Cannot subtract {} and {}",
                a.type_name(),
                b.type_name()
            ))),
        }
    }
    
    fn multiply_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x * y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 * y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x * *y as f64)),
            _ => Err(FlowError::type_error(format!(
                "Cannot multiply {} and {}",
                a.type_name(),
                b.type_name()
            ))),
        }
    }
    
    fn divide_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                if *y == 0 {
                    Err(FlowError::runtime_error("Division by zero"))
                } else {
                    Ok(Value::Float(*x as f64 / *y as f64))
                }
            }
            (Value::Float(x), Value::Float(y)) => {
                if *y == 0.0 {
                    Err(FlowError::runtime_error("Division by zero"))
                } else {
                    Ok(Value::Float(x / y))
                }
            }
            (Value::Integer(x), Value::Float(y)) => {
                if *y == 0.0 {
                    Err(FlowError::runtime_error("Division by zero"))
                } else {
                    Ok(Value::Float(*x as f64 / y))
                }
            }
            (Value::Float(x), Value::Integer(y)) => {
                if *y == 0 {
                    Err(FlowError::runtime_error("Division by zero"))
                } else {
                    Ok(Value::Float(x / *y as f64))
                }
            }
            _ => Err(FlowError::type_error(format!(
                "Cannot divide {} and {}",
                a.type_name(),
                b.type_name()
            ))),
        }
    }
    
    fn modulo_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                if *y == 0 {
                    Err(FlowError::runtime_error("Modulo by zero"))
                } else {
                    Ok(Value::Integer(x % y))
                }
            }
            _ => Err(FlowError::type_error(format!(
                "Cannot modulo {} and {}",
                a.type_name(),
                b.type_name()
            ))),
        }
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => (x - y).abs() < f64::EPSILON,
            (Value::Integer(x), Value::Float(y)) => (*x as f64 - y).abs() < f64::EPSILON,
            (Value::Float(x), Value::Integer(y)) => (x - *y as f64).abs() < f64::EPSILON,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
    
    fn compare_values<F>(&self, a: &Value, b: &Value, op: F) -> Result<Value>
    where
        F: FnOnce(std::cmp::Ordering) -> bool,
    {
        let ordering = match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
            (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
            (Value::Integer(x), Value::Float(y)) => (*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
            (Value::Float(x), Value::Integer(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal),
            (Value::String(x), Value::String(y)) => x.cmp(y),
            _ => {
                return Err(FlowError::type_error(format!(
                    "Cannot compare {} and {}",
                    a.type_name(),
                    b.type_name()
                )));
            }
        };
        
        Ok(Value::Boolean(op(ordering)))
    }
}