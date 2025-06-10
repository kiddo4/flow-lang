//! Bytecode Virtual Machine for FlowLang
//! 
//! This module implements a stack-based virtual machine that executes
//! FlowLang bytecode instructions.

use crate::ast::{Statement, Expression, BinaryOperator, UnaryOperator, Literal, Program, Parameter};

use crate::error::{FlowError, Result};
use crate::value::{Value, FlowArray, FlowObject};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{Read, Write};
use std::time::Instant;

// Bytecode file format constants
const MAGIC_NUMBER: u32 = 0x464C4F57; // "FLOW"
const VERSION_MAJOR: u16 = 1;
const VERSION_MINOR: u16 = 0;

// Constant pool entry types
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantType {
    Integer = 0x01,
    Float = 0x02,
    String = 0x03,
    Boolean = 0x04,
    Null = 0x05,
    Function = 0x06,
}

// Bytecode file header
#[derive(Debug, Clone)]
pub struct BytecodeHeader {
    pub magic_number: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub constant_pool_size: u32,
    pub code_size: u32,
    pub entry_point: u32,
}

impl BytecodeHeader {
    pub fn new(constant_pool_size: u32, code_size: u32, entry_point: u32) -> Self {
        Self {
            magic_number: MAGIC_NUMBER,
            version_major: VERSION_MAJOR,
            version_minor: VERSION_MINOR,
            constant_pool_size,
            code_size,
            entry_point,
        }
    }
    
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.magic_number.to_le_bytes())?;
        writer.write_all(&self.version_major.to_le_bytes())?;
        writer.write_all(&self.version_minor.to_le_bytes())?;
        writer.write_all(&self.constant_pool_size.to_le_bytes())?;
        writer.write_all(&self.code_size.to_le_bytes())?;
        writer.write_all(&self.entry_point.to_le_bytes())?;
        Ok(())
    }
    
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 4];
        
        reader.read_exact(&mut buffer)?;
        let magic_number = u32::from_le_bytes(buffer);
        if magic_number != MAGIC_NUMBER {
            return Err(FlowError::runtime_error("Invalid bytecode file: wrong magic number"));
        }
        
        let mut buffer = [0u8; 2];
        reader.read_exact(&mut buffer)?;
        let version_major = u16::from_le_bytes(buffer);
        
        reader.read_exact(&mut buffer)?;
        let version_minor = u16::from_le_bytes(buffer);
        
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let constant_pool_size = u32::from_le_bytes(buffer);
        
        reader.read_exact(&mut buffer)?;
        let code_size = u32::from_le_bytes(buffer);
        
        reader.read_exact(&mut buffer)?;
        let entry_point = u32::from_le_bytes(buffer);
        
        Ok(Self {
            magic_number,
            version_major,
            version_minor,
            constant_pool_size,
            code_size,
            entry_point,
        })
    }
}



// Bytecode instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack operations
    LoadConstant(usize),    // Load constant from constant pool
    LoadLocal(usize),       // Load local variable
    StoreLocal(usize),      // Store to local variable
    LoadGlobal(String),     // Load global variable
    StoreGlobal(String),    // Store to global variable
    Duplicate,              // Duplicate top of stack
    Pop,                    // Remove top of stack
    Swap,                   // Swap top two values
    
    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,                 // Unary negation
    
    // Comparison operations
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Logical operations
    And,
    Or,
    Not,
    
    // Control flow
    Jump(usize),            // Unconditional jump
    JumpIfFalse(usize),     // Jump if top of stack is false
    JumpIfTrue(usize),      // Jump if top of stack is true
    Call(usize),            // Call function with argc arguments
    Return,                 // Return from function
    ReturnValue,            // Return with value from stack
    
    // Object operations
    NewArray(usize),        // Create array with size elements
    NewObject,              // Create empty object
    GetIndex,               // Pop index, array; push array[index]
    SetIndex,               // Pop value, index, array; array[index] = value
    GetProperty(String),    // Pop object; push object.name
    SetProperty(String),    // Pop value, object; object.name = value
    
    // Function operations
    NewFunction(usize),     // Create function from constant pool
    NewClosure(usize),      // Create closure capturing locals
    CallMethod(String),     // Call method on object
    
    // Built-in operations
    CallBuiltin(String),    // Call built-in function
    Print,                  // Print top of stack (for debugging)
    Halt,                   // Stop execution
}





/// Bytecode chunk containing instructions and constants
#[derive(Debug, Clone)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>, // Line numbers for debugging
    pub header: Option<BytecodeHeader>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            header: None,
        }
    }
    
    pub fn write_instruction(&mut self, instruction: Instruction, line: usize) {
        self.instructions.push(instruction);
        self.lines.push(line);
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        // Check if constant already exists to avoid duplicates
        for (i, existing) in self.constants.iter().enumerate() {
            if self.values_equal(existing, &value) {
                return i;
            }
        }
        self.constants.push(value);
        self.constants.len() - 1
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => (x - y).abs() < f64::EPSILON,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
    
    pub fn finalize(&mut self) {
        let header = BytecodeHeader::new(
            self.constants.len() as u32,
            self.instructions.len() as u32,
            0, // Entry point at beginning
        );
        self.header = Some(header);
    }
    
    pub fn write_to_file<W: Write>(&self, writer: &mut W) -> Result<()> {
        if let Some(header) = &self.header {
            header.write_to(writer)?;
        } else {
            return Err(FlowError::runtime_error("Chunk not finalized"));
        }
        
        // Write constant pool
        for constant in &self.constants {
            self.write_constant(writer, constant)?;
        }
        
        // Write instructions
        for instruction in &self.instructions {
            self.write_instruction_to_file(writer, instruction)?;
        }
        
        Ok(())
    }
    
    fn write_constant<W: Write>(&self, writer: &mut W, value: &Value) -> Result<()> {
        match value {
            Value::Integer(i) => {
                writer.write_all(&[ConstantType::Integer as u8])?;
                writer.write_all(&i.to_le_bytes())?;
            }
            Value::Float(f) => {
                writer.write_all(&[ConstantType::Float as u8])?;
                writer.write_all(&f.to_le_bytes())?;
            }
            Value::String(s) => {
                writer.write_all(&[ConstantType::String as u8])?;
                let bytes = s.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(bytes)?;
            }
            Value::Boolean(b) => {
                writer.write_all(&[ConstantType::Boolean as u8])?;
                writer.write_all(&[if *b { 1 } else { 0 }])?;
            }
            Value::Null => {
                writer.write_all(&[ConstantType::Null as u8])?;
            }
            _ => {
                return Err(FlowError::runtime_error("Unsupported constant type for serialization"));
            }
        }
        Ok(())
    }
    
    fn write_instruction_to_file<W: Write>(&self, writer: &mut W, instruction: &Instruction) -> Result<()> {
        match instruction {
            Instruction::LoadConstant(index) => {
                writer.write_all(&[0x01])?; // Opcode
                writer.write_all(&(*index as u16).to_le_bytes())?;
            }
            Instruction::LoadLocal(index) => {
                writer.write_all(&[0x02])?;
                writer.write_all(&(*index as u16).to_le_bytes())?;
            }
            Instruction::StoreLocal(index) => {
                writer.write_all(&[0x03])?;
                writer.write_all(&(*index as u16).to_le_bytes())?;
            }
            Instruction::Add => writer.write_all(&[0x10])?,
            Instruction::Subtract => writer.write_all(&[0x11])?,
            Instruction::Multiply => writer.write_all(&[0x12])?,
            Instruction::Divide => writer.write_all(&[0x13])?,
            Instruction::Modulo => writer.write_all(&[0x14])?,
            Instruction::Negate => writer.write_all(&[0x15])?,
            Instruction::Equal => writer.write_all(&[0x20])?,
            Instruction::NotEqual => writer.write_all(&[0x21])?,
            Instruction::Less => writer.write_all(&[0x22])?,
            Instruction::LessEqual => writer.write_all(&[0x23])?,
            Instruction::Greater => writer.write_all(&[0x24])?,
            Instruction::GreaterEqual => writer.write_all(&[0x25])?,
            Instruction::And => writer.write_all(&[0x30])?,
            Instruction::Or => writer.write_all(&[0x31])?,
            Instruction::Not => writer.write_all(&[0x32])?,
            Instruction::Jump(offset) => {
                writer.write_all(&[0x40])?;
                writer.write_all(&(*offset as u16).to_le_bytes())?;
            }
            Instruction::JumpIfFalse(offset) => {
                writer.write_all(&[0x41])?;
                writer.write_all(&(*offset as u16).to_le_bytes())?;
            }
            Instruction::JumpIfTrue(offset) => {
                writer.write_all(&[0x42])?;
                writer.write_all(&(*offset as u16).to_le_bytes())?;
            }
            Instruction::Call(argc) => {
                writer.write_all(&[0x50])?;
                writer.write_all(&(*argc as u16).to_le_bytes())?;
            }
            Instruction::Return => writer.write_all(&[0x51])?,
            Instruction::ReturnValue => writer.write_all(&[0x52])?,
            Instruction::Pop => writer.write_all(&[0x60])?,
            Instruction::Duplicate => writer.write_all(&[0x61])?,
            Instruction::Swap => writer.write_all(&[0x62])?,
            Instruction::Print => writer.write_all(&[0x70])?,
            Instruction::Halt => writer.write_all(&[0xFF])?,
            _ => {
                return Err(FlowError::runtime_error("Unsupported instruction for serialization"));
            }
        }
        Ok(())
    }
}



/// Function reference for bytecode functions
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionRef {
    pub name: String,
    pub arity: usize,
    pub chunk_index: usize,
    pub locals_count: usize,
}

/// Closure reference with captured variables
#[derive(Debug, Clone, PartialEq)]
pub struct ClosureRef {
    pub function: FunctionRef,
    pub captured: Vec<Value>,
}

/// Call frame for function calls (matches specification)
#[derive(Debug, Clone)]
struct Frame {
    pub function: Option<FunctionRef>,
    pub locals: Vec<Value>,
    pub instruction_pointer: usize,
    pub stack_base: usize,
}

/// Garbage Collection Statistics
#[derive(Debug, Clone)]
pub struct GCStats {
    pub collections: usize,
    pub objects_collected: usize,
    pub bytes_collected: usize,
    pub last_collection_time: Option<Instant>,
}

/// Virtual Machine state
pub struct VirtualMachine {
    pub stack: Vec<Value>,
    pub call_stack: Vec<Frame>,
    pub chunks: Vec<Chunk>,
    pub current_chunk: usize,
    pub instruction_pointer: usize,
    pub globals: std::collections::HashMap<String, Value>,
    pub builtins: HashMap<String, fn(&mut VirtualMachine, &[Value]) -> Result<Value>>,
    pub gc_stats: GCStats,
    pub gc_threshold: usize,
    pub allocated_objects: usize,
    pub locals: Vec<Value>,
    pub constant_pool: Vec<Value>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut vm = Self {
            stack: Vec::new(),
            call_stack: Vec::new(),
            chunks: Vec::new(),
            current_chunk: 0,
            instruction_pointer: 0,
            globals: HashMap::new(),
            builtins: HashMap::new(),
            gc_stats: GCStats {
                collections: 0,
                objects_collected: 0,
                bytes_collected: 0,
                last_collection_time: None,
            },
            gc_threshold: 1000, // Trigger GC after 1000 allocations
            allocated_objects: 0,
            locals: Vec::new(),
            constant_pool: Vec::new(),
        };
        vm.add_builtins();
        vm
    }
    
    pub fn load_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
        if self.chunks.len() == 1 {
            self.current_chunk = 0;
            self.instruction_pointer = 0;
        }
    }
    
    /// Get current chunk
    fn current_chunk(&self) -> &Chunk {
        &self.chunks[self.current_chunk]
    }
    
    /// Get current chunk mutably
    fn current_chunk_mut(&mut self) -> &mut Chunk {
         &mut self.chunks[self.current_chunk]
     }
     
     /// Add built-in functions to the VM
     fn add_builtins(&mut self) {
         self.builtins.insert("show".to_string(), Self::builtin_show);
         self.builtins.insert("print".to_string(), Self::builtin_print);
         self.builtins.insert("len".to_string(), Self::builtin_len);
         self.builtins.insert("type".to_string(), Self::builtin_type);
         self.builtins.insert("str".to_string(), Self::builtin_str);
         self.builtins.insert("int".to_string(), Self::builtin_int);
         self.builtins.insert("float".to_string(), Self::builtin_float);
     }
     
     /// Trigger garbage collection if threshold is reached
     fn maybe_gc(&mut self) {
         if self.allocated_objects >= self.gc_threshold {
             self.collect_garbage();
         }
     }
     
     /// Mark and sweep garbage collection
     fn collect_garbage(&mut self) {
         let start_time = Instant::now();
         let mut marked = HashSet::new();
         
         // Mark phase: traverse from roots
         self.mark_roots(&mut marked);
         
         // Sweep phase: collect unmarked objects
         let collected = self.sweep_unmarked(&marked);
         
         // Update statistics
         self.gc_stats.collections += 1;
         self.gc_stats.objects_collected += collected;
         self.gc_stats.last_collection_time = Some(start_time);
         self.allocated_objects = self.allocated_objects.saturating_sub(collected);
         
         // Adjust threshold based on collection efficiency
         if collected < self.gc_threshold / 4 {
             self.gc_threshold = (self.gc_threshold * 3) / 2; // Increase threshold
         }
     }
     
     /// Mark all reachable objects from GC roots
     fn mark_roots(&self, marked: &mut HashSet<*const Value>) {
         // Mark stack values
         for value in &self.stack {
             self.mark_value(value, marked);
         }
         
         // Mark global variables
         for value in self.globals.values() {
             self.mark_value(value, marked);
         }
         
         // Mark local variables in call frames
         for frame in &self.call_stack {
             for local in &frame.locals {
                 self.mark_value(local, marked);
             }
         }
         
         // Mark constants in chunks
         for chunk in &self.chunks {
             for constant in &chunk.constants {
                 self.mark_value(constant, marked);
             }
         }
     }
     
     /// Mark a value and its references
     fn mark_value(&self, value: &Value, marked: &mut HashSet<*const Value>) {
         let ptr = value as *const Value;
         if marked.contains(&ptr) {
             return;
         }
         marked.insert(ptr);
         
         match value {
             Value::Array(array) => {
                let elements = &array.elements;
                 for element in elements {
                     self.mark_value(element, marked);
                 }
             }
             Value::Object(object) => {
                let properties = &object.properties;
                 for prop_value in properties.values() {
                     self.mark_value(prop_value, marked);
                 }
             }
             _ => {} // Other types don't contain references
         }
     }
     
     /// Sweep unmarked objects (simplified - in a real implementation,
     /// this would work with a proper heap allocator)
     fn sweep_unmarked(&mut self, _marked: &HashSet<*const Value>) -> usize {
         // In a real implementation, this would traverse the heap
         // and deallocate unmarked objects. For now, we'll just
         // return a placeholder count.
         0
     }
    
    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.instruction_pointer >= self.current_chunk().instructions.len() {
                break;
            }
            
            let instruction = self.current_chunk().instructions[self.instruction_pointer].clone();
            self.instruction_pointer += 1;
            
            match instruction {
                Instruction::LoadConstant(index) => {
                    if index >= self.current_chunk().constants.len() {
                        return Err(FlowError::runtime_error("Invalid constant index"));
                    }
                    let value = self.current_chunk().constants[index].clone();
                    self.stack.push(value);
                }
                
                Instruction::LoadLocal(index) => {
                    if let Some(frame) = self.call_stack.last() {
                        if let Some(local) = frame.locals.get(index) {
                            self.stack.push(local.clone());
                        } else {
                            return Err(FlowError::runtime_error("Invalid local index"));
                        }
                    } else {
                        return Err(FlowError::runtime_error("No active frame for local access"));
                    }
                }
                
                Instruction::StoreLocal(index) => {
                    let value = self.stack.pop().ok_or_else(|| {
                        FlowError::runtime_error("Stack underflow")
                    })?;
                    
                    if let Some(frame) = self.call_stack.last_mut() {
                        if index < frame.locals.len() {
                            frame.locals[index] = value;
                        } else {
                            return Err(FlowError::runtime_error("Invalid local index"));
                        }
                    } else {
                        return Err(FlowError::runtime_error("No active frame for local storage"));
                    }
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
                
                Instruction::Add => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.add_values(&a, &b)?;
                    self.stack.push(result);
                }
                Instruction::Subtract => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.subtract_values(&a, &b)?;
                    self.stack.push(result);
                }
                Instruction::Multiply => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.multiply_values(&a, &b)?;
                    self.stack.push(result);
                }
                Instruction::Divide => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.divide_values(&a, &b)?;
                    self.stack.push(result);
                }
                Instruction::Modulo => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.modulo_values(&a, &b)?;
                    self.stack.push(result);
                }
                
                Instruction::Equal => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = Value::Boolean(self.values_equal(&a, &b));
                    self.stack.push(result);
                }
                Instruction::NotEqual => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = Value::Boolean(!self.values_equal(&a, &b));
                    self.stack.push(result);
                }
                Instruction::Greater => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.compare_values(&a, &b, |ord| ord.is_gt())?;
                    self.stack.push(result);
                }
                Instruction::GreaterEqual => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.compare_values(&a, &b, |ord| ord.is_ge())?;
                    self.stack.push(result);
                }
                Instruction::Less => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.compare_values(&a, &b, |ord| ord.is_lt())?;
                    self.stack.push(result);
                }
                Instruction::LessEqual => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = self.compare_values(&a, &b, |ord| ord.is_le())?;
                    self.stack.push(result);
                }
                
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
                
                Instruction::CallBuiltin(name) => {
                    if let Some(builtin) = self.builtins.get(&name).cloned() {
                        // Get arguments from stack
                        let arg_count = if let Some(Value::Integer(count)) = self.stack.pop() {
                            count as usize
                        } else {
                            return Err(FlowError::runtime_error("Expected argument count for builtin call"));
                        };
                        
                        if self.stack.len() < arg_count {
                            return Err(FlowError::runtime_error("Not enough arguments for builtin call"));
                        }
                        
                        let mut args = Vec::new();
                        for _ in 0..arg_count {
                            args.push(self.stack.pop().unwrap());
                        }
                        args.reverse(); // Arguments were pushed in reverse order
                        
                        let result = builtin(self, &args)?;
                        self.stack.push(result);
                    } else {
                        return Err(FlowError::runtime_error(format!("Unknown builtin function: {}", name)));
                    }
                }
                
                Instruction::NewArray(size) => {
                    if self.stack.len() < size {
                        return Err(FlowError::runtime_error("Not enough elements for array creation"));
                    }
                    
                    let mut elements = Vec::new();
                    for _ in 0..size {
                        elements.push(self.stack.pop().unwrap());
                    }
                    elements.reverse();
                    
                    self.stack.push(Value::Array(FlowArray { elements }));
                }
                
                Instruction::NewObject => {
                    let properties = std::collections::HashMap::new();
                    self.stack.push(Value::Object(FlowObject { properties }));
                }
                
                Instruction::GetIndex => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow for index access"));
                    }
                    
                    let index = self.stack.pop().unwrap();
                    let array = self.stack.pop().unwrap();
                    
                    match (&array, &index) {
                        (Value::Array(array_ref), Value::Integer(i)) => {
                            let elements = &array_ref.elements;
                            if *i >= 0 && (*i as usize) < elements.len() {
                                self.stack.push(elements[*i as usize].clone());
                            } else {
                                return Err(FlowError::runtime_error("Array index out of bounds"));
                            }
                        }
                        _ => return Err(FlowError::runtime_error("Invalid index operation")),
                    }
                }
                
                Instruction::SetIndex => {
                    if self.stack.len() < 3 {
                        return Err(FlowError::runtime_error("Stack underflow for index assignment"));
                    }
                    
                    let value = self.stack.pop().unwrap();
                    let index = self.stack.pop().unwrap();
                    let mut array = self.stack.pop().unwrap();
                    
                    match (&mut array, &index) {
                        (Value::Array(array_ref), Value::Integer(i)) => {
                            let elements = &mut array_ref.elements;
                            if *i >= 0 && (*i as usize) < elements.len() {
                                elements[*i as usize] = value;
                                self.stack.push(array);
                            } else {
                                return Err(FlowError::runtime_error("Array index out of bounds"));
                            }
                        }
                        _ => return Err(FlowError::runtime_error("Invalid index assignment")),
                    }
                }
                
                Instruction::GetProperty(property) => {
                    if let Some(object) = self.stack.pop() {
                        match object {
                            Value::Object(object) => {
                let properties = &object.properties;
                                if let Some(value) = properties.get(&property) {
                                    self.stack.push(value.clone());
                                } else {
                                    self.stack.push(Value::Null);
                                }
                            }
                            _ => return Err(FlowError::runtime_error("Property access on non-object")),
                        }
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                Instruction::SetProperty(property) => {
                    if self.stack.len() < 2 {
                        return Err(FlowError::runtime_error("Stack underflow for property assignment"));
                    }
                    
                    let value = self.stack.pop().unwrap();
                    let mut object = self.stack.pop().unwrap();
                    
                    match &mut object {
                        Value::Object(object_ref) => {
                            object_ref.properties.insert(property, value);
                            self.stack.push(object);
                        }
                        _ => return Err(FlowError::runtime_error("Property assignment on non-object")),
                    }
                }
                
                Instruction::NewFunction(_index) => {
                    // For now, just push a placeholder function value
                    self.stack.push(Value::Function {
                        name: "anonymous".to_string(),
                        parameters: Vec::new(),
                        body: Vec::new(),
                    });
                }
                
                Instruction::NewClosure(_index) => {
                    // For now, just push a placeholder function value
                    self.stack.push(Value::Function {
                        name: "closure".to_string(),
                        parameters: Vec::new(),
                        body: Vec::new(),
                    });
                }
                
                Instruction::Call(argc) => {
                    // Pop function from stack
                    let function = self.stack.pop().ok_or_else(|| {
                        FlowError::runtime_error("Stack underflow: no function to call")
                    })?;
                    
                    // Pop arguments from stack (in reverse order)
                    let mut args = Vec::new();
                    for _ in 0..argc {
                        args.push(self.stack.pop().ok_or_else(|| {
                            FlowError::runtime_error("Stack underflow: not enough arguments")
                        })?);
                    }
                    args.reverse(); // Arguments were pushed in reverse order
                    
                    match function {
                        Value::Function { name: _, parameters, body } => {
                            // Create new call frame
                            let mut locals = vec![Value::Null; parameters.len()];
                            
                            // Bind arguments to parameters
                            for (i, arg) in args.into_iter().enumerate() {
                                if i < locals.len() {
                                    locals[i] = arg;
                                }
                            }
                            
                            let frame = Frame {
                                function: None,
                                locals,
                                instruction_pointer: self.instruction_pointer,
                                stack_base: self.stack.len(),
                            };
                            
                            self.call_stack.push(frame);
                            
                            // For now, we'll execute the function body using the interpreter
                            // This is a simplified implementation
                            let mut interpreter = crate::interpreter::Interpreter::new();
                            
                            // Set up local variables in interpreter
                            for (i, param) in parameters.iter().enumerate() {
                                if i < self.call_stack.last().unwrap().locals.len() {
                                    interpreter.set_variable(param.name.clone(), self.call_stack.last().unwrap().locals[i].clone());
                                }
                            }
                            
                            // Execute function body
                            let mut result = Value::Null;
                            for stmt in &body {
                                match stmt {
                                    crate::ast::Statement::Return(Some(expr)) => {
                                        result = interpreter.evaluate_expression(expr)?;
                                        break;
                                    }
                                    _ => {
                                        interpreter.execute_statement(stmt)?;
                                    }
                                }
                            }
                            
                            // Pop call frame
                            self.call_stack.pop();
                            
                            // Push result onto stack
                            self.stack.push(result);
                        }
                        _ => {
                            return Err(FlowError::runtime_error("Cannot call non-function value"));
                        }
                    }
                }
                
                Instruction::CallMethod(_method) => {
                    // For now, just return an error as method calls are not fully implemented
                    return Err(FlowError::runtime_error("Method calls not yet implemented"));
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
    
    /// Check if a value is truthy
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Float(f) => *f != 0.0,
            Value::Integer(i) => *i != 0,
            Value::BigInteger(bi) => !bi.is_zero(),
            Value::String(s) => !s.is_empty(),
            Value::Array(array) => !array.elements.is_empty(),
            Value::Object(object) => !object.properties.is_empty(),
            _ => true,
        }
    }
    
    fn add_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                match x.checked_add(*y) {
                    Some(result) => Ok(Value::Integer(result)),
                    None => {
                        // Overflow, promote to BigInt
                        let big_a = crate::bigint::BigInt::from_i64(*x);
                        let big_b = crate::bigint::BigInt::from_i64(*y);
                        Ok(Value::BigInteger(big_a + big_b))
                    }
                }
            }
            (Value::BigInteger(x), Value::BigInteger(y)) => {
                Ok(Value::BigInteger(x.clone() + y.clone()))
            }
            (Value::Integer(x), Value::BigInteger(y)) => {
                let big_x = crate::bigint::BigInt::from_i64(*x);
                Ok(Value::BigInteger(big_x + y.clone()))
            }
            (Value::BigInteger(x), Value::Integer(y)) => {
                let big_y = crate::bigint::BigInt::from_i64(*y);
                Ok(Value::BigInteger(x.clone() + big_y))
            }
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 + y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x + *y as f64)),
            (Value::BigInteger(x), Value::Float(y)) => {
                if let Some(int_val) = x.to_i64() {
                    Ok(Value::Float(int_val as f64 + y))
                } else {
                    Err(FlowError::runtime_error("BigInteger too large for float conversion"))
                }
            }
            (Value::Float(x), Value::BigInteger(y)) => {
                if let Some(int_val) = y.to_i64() {
                    Ok(Value::Float(x + int_val as f64))
                } else {
                    Err(FlowError::runtime_error("BigInteger too large for float conversion"))
                }
            }
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
            (Value::BigInteger(x), Value::BigInteger(y)) => {
                Ok(Value::BigInteger(x.clone() - y.clone()))
            }
            (Value::Integer(x), Value::BigInteger(y)) => {
                let big_x = crate::bigint::BigInt::from_i64(*x);
                Ok(Value::BigInteger(big_x - y.clone()))
            }
            (Value::BigInteger(x), Value::Integer(y)) => {
                let big_y = crate::bigint::BigInt::from_i64(*y);
                Ok(Value::BigInteger(x.clone() - big_y))
            }
            _ => Err(FlowError::type_error(format!(
                "Cannot subtract {} and {}",
                a.type_name(),
                b.type_name()
            ))),
        }
    }
    
    fn multiply_values(&self, a: &Value, b: &Value) -> Result<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                match x.checked_mul(*y) {
                    Some(result) => Ok(Value::Integer(result)),
                    None => {
                        // Overflow, promote to BigInt
                        let big_a = crate::bigint::BigInt::from_i64(*x);
                        let big_b = crate::bigint::BigInt::from_i64(*y);
                        Ok(Value::BigInteger(big_a * big_b))
                    }
                }
            }
            (Value::BigInteger(x), Value::BigInteger(y)) => {
                Ok(Value::BigInteger(x.clone() * y.clone()))
            }
            (Value::Integer(x), Value::BigInteger(y)) => {
                let big_x = crate::bigint::BigInt::from_i64(*x);
                Ok(Value::BigInteger(big_x * y.clone()))
            }
            (Value::BigInteger(x), Value::Integer(y)) => {
                let big_y = crate::bigint::BigInt::from_i64(*y);
                Ok(Value::BigInteger(x.clone() * big_y))
            }
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
            (Value::Integer(x), Value::Float(y)) => Ok(Value::Float(*x as f64 * y)),
            (Value::Float(x), Value::Integer(y)) => Ok(Value::Float(x * *y as f64)),
            (Value::BigInteger(x), Value::Float(y)) => {
                if let Some(int_val) = x.to_i64() {
                    Ok(Value::Float(int_val as f64 * y))
                } else {
                    Err(FlowError::runtime_error("BigInteger too large for float conversion"))
                }
            }
            (Value::Float(x), Value::BigInteger(y)) => {
                if let Some(int_val) = y.to_i64() {
                    Ok(Value::Float(x * int_val as f64))
                } else {
                    Err(FlowError::runtime_error("BigInteger too large for float conversion"))
                }
            }
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

/// Bytecode compiler that converts AST to bytecode
#[derive(Debug)]
pub struct BytecodeCompiler {
    chunks: Vec<Chunk>,
    current_chunk: usize,
    locals: Vec<String>,
    scope_depth: usize,
}

impl BytecodeCompiler {
    /// Create a new bytecode compiler
    pub fn new() -> Self {
        let mut compiler = Self {
            chunks: Vec::new(),
            current_chunk: 0,
            locals: Vec::new(),
            scope_depth: 0,
        };
        compiler.chunks.push(Chunk::new());
        compiler
    }
    
    /// Compile a program to bytecode
    pub fn compile_program(&mut self, program: &Program) -> Result<Vec<Chunk>> {
        for statement in &program.statements {
            self.compile_statement(statement)?;
        }
        
        // Add halt instruction at the end
        self.emit_instruction(Instruction::Halt);
        
        Ok(self.chunks.clone())
    }
    
    /// Compile a statement
    fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::VariableDeclaration { name, value } => {
                self.compile_expression(value)?;
                
                if self.scope_depth == 0 {
                    // Global variable
                    self.emit_instruction(Instruction::StoreGlobal(name.clone()));
                } else {
                    // Local variable
                    self.locals.push(name.clone());
                    let index = self.locals.len() - 1;
                    self.emit_instruction(Instruction::StoreLocal(index));
                }
            }
            
            Statement::FunctionDeclaration { name, parameters, body } => {
                // Create function reference
                let func_ref = FunctionRef {
                    name: name.clone(),
                    arity: parameters.len(),
                    chunk_index: self.chunks.len(),
                    locals_count: parameters.len(),
                };
                
                // Create new chunk for function body
                let old_chunk = self.current_chunk;
                self.chunks.push(Chunk::new());
                self.current_chunk = self.chunks.len() - 1;
                
                // Enter function scope
                self.scope_depth += 1;
                let old_locals = self.locals.clone();
                self.locals.clear();
                
                // Add parameters as locals
                for param in parameters {
                    self.locals.push(param.name.clone());
                }
                
                // Compile function body
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // Add return instruction if not present
                self.emit_instruction(Instruction::Return);
                
                // Restore scope
                self.scope_depth -= 1;
                self.locals = old_locals;
                self.current_chunk = old_chunk;
                
                // Emit function creation instruction
                self.emit_instruction(Instruction::NewFunction(func_ref.chunk_index));
                self.emit_instruction(Instruction::StoreGlobal(name.clone()));
            }
            
            Statement::If { condition, then_branch, else_branch } => {
                // Compile condition
                self.compile_expression(condition)?;
                
                // Jump if false
                let else_jump = self.emit_jump(Instruction::JumpIfFalse(0));
                
                // Compile then branch
                for stmt in then_branch {
                    self.compile_statement(stmt)?;
                }
                
                if let Some(else_stmts) = else_branch {
                    // Jump over else branch
                    let end_jump = self.emit_jump(Instruction::Jump(0));
                    
                    // Patch else jump
                    self.patch_jump(else_jump)?;
                    
                    // Compile else branch
                    for stmt in else_stmts {
                        self.compile_statement(stmt)?;
                    }
                    
                    // Patch end jump
                    self.patch_jump(end_jump)?;
                } else {
                    // Patch else jump
                    self.patch_jump(else_jump)?;
                }
            }
            
            Statement::While { condition, body } => {
                let loop_start = self.current_chunk().instructions.len();
                
                // Compile condition
                self.compile_expression(condition)?;
                
                // Jump if false
                let exit_jump = self.emit_jump(Instruction::JumpIfFalse(0));
                
                // Compile body
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // Jump back to condition
                let current_pos = self.current_chunk().instructions.len();
                let offset = if loop_start <= current_pos {
                    current_pos - loop_start + 1
                } else {
                    loop_start - current_pos - 1
                };
                self.emit_instruction(Instruction::Jump(offset));
                
                // Patch exit jump
                self.patch_jump(exit_jump)?;
            }
            
            Statement::For { variable, start, end, body } => {
                // Initialize loop variable
                self.compile_expression(start)?;
                self.locals.push(variable.clone());
                let var_index = self.locals.len() - 1;
                self.emit_instruction(Instruction::StoreLocal(var_index));
                
                let loop_start = self.current_chunk().instructions.len();
                
                // Check condition (variable < end)
                self.emit_instruction(Instruction::LoadLocal(var_index));
                self.compile_expression(end)?;
                self.emit_instruction(Instruction::Less);
                
                let exit_jump = self.emit_jump(Instruction::JumpIfFalse(0));
                
                // Compile body
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // Increment variable
                self.emit_instruction(Instruction::LoadLocal(var_index));
                self.emit_constant(Value::Integer(1));
                self.emit_instruction(Instruction::Add);
                self.emit_instruction(Instruction::StoreLocal(var_index));
                
                // Jump back to condition
                let current_pos = self.current_chunk().instructions.len();
                let offset = if loop_start <= current_pos {
                    current_pos - loop_start + 1
                } else {
                    loop_start - current_pos - 1
                };
                self.emit_instruction(Instruction::Jump(offset));
                
                // Patch exit jump
                self.patch_jump(exit_jump)?;
                
                // Remove loop variable
                self.locals.pop();
            }
            
            Statement::Show(expr) => {
                self.compile_expression(expr)?;
                self.emit_instruction(Instruction::Print);
            }
            
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.compile_expression(expr)?;
                    self.emit_instruction(Instruction::ReturnValue);
                } else {
                    self.emit_instruction(Instruction::Return);
                }
            }
            
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                self.emit_instruction(Instruction::Pop);
            }
            
            Statement::Import(_) => {
                // Import handling would be done at a higher level
            }
            
            Statement::Export(_) => {
                // Export handling would be done at a higher level
            }
            
            Statement::TryCatch { try_block, catch_variable: _, catch_block: _ } => {
                // Basic try-catch compilation (simplified)
                for stmt in try_block {
                    self.compile_statement(stmt)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Compile an expression
    fn compile_expression(&mut self, expression: &Expression) -> Result<()> {
        match expression {
            Expression::Literal(literal) => {
                let value = self.literal_to_value(literal)?;
                self.emit_constant(value);
            }
            
            Expression::Identifier(name) => {
                // Check if it's a local variable
                if let Some(index) = self.locals.iter().position(|local| local == name) {
                    self.emit_instruction(Instruction::LoadLocal(index));
                } else {
                    // Global variable
                    self.emit_instruction(Instruction::LoadGlobal(name.clone()));
                }
            }
            
            Expression::Binary { left, operator, right } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                
                let instruction = match operator {
                    BinaryOperator::Add => Instruction::Add,
                    BinaryOperator::Subtract => Instruction::Subtract,
                    BinaryOperator::Multiply => Instruction::Multiply,
                    BinaryOperator::Divide => Instruction::Divide,
                    BinaryOperator::Modulo => Instruction::Modulo,
                    BinaryOperator::Equal => Instruction::Equal,
                    BinaryOperator::NotEqual => Instruction::NotEqual,
                    BinaryOperator::Greater => Instruction::Greater,
                    BinaryOperator::GreaterEqual => Instruction::GreaterEqual,
                    BinaryOperator::Less => Instruction::Less,
                    BinaryOperator::LessEqual => Instruction::LessEqual,
                    BinaryOperator::And => Instruction::And,
                    BinaryOperator::Or => Instruction::Or,
                };
                
                self.emit_instruction(instruction);
            }
            
            Expression::Unary { operator, operand } => {
                self.compile_expression(operand)?;
                
                let instruction = match operator {
                    UnaryOperator::Minus => Instruction::Negate,
                    UnaryOperator::Not => Instruction::Not,
                };
                
                self.emit_instruction(instruction);
            }
            
            Expression::FunctionCall { name, arguments } => {
                // Compile arguments
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                
                // Load function
                self.emit_instruction(Instruction::LoadGlobal(name.clone()));
                
                // Call function
                self.emit_instruction(Instruction::Call(arguments.len()));
            }
            
            Expression::MethodCall { object, method, arguments } => {
                // Compile object
                self.compile_expression(object)?;
                
                // Compile arguments
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                
                // Call method
                self.emit_instruction(Instruction::CallMethod(method.clone()));
            }
            
            Expression::Array { elements } => {
                // Compile elements
                for element in elements {
                    self.compile_expression(element)?;
                }
                
                // Create array
                self.emit_instruction(Instruction::NewArray(elements.len()));
            }
            
            Expression::Object { properties } => {
                // Compile properties
                for (key, value) in properties {
                    self.emit_constant(Value::String(key.clone()));
                    self.compile_expression(value)?;
                }
                
                // Create object
                self.emit_instruction(Instruction::NewObject);
            }
            
            Expression::Index { object, index } => {
                self.compile_expression(object)?;
                self.compile_expression(index)?;
                self.emit_instruction(Instruction::GetIndex);
            }
            
            Expression::PropertyAccess { object, property } => {
                self.compile_expression(object)?;
                self.emit_instruction(Instruction::GetProperty(property.clone()));
            }
            
            Expression::Lambda { parameters, body } => {
                // Create closure reference
                let closure_ref = ClosureRef {
                    function: FunctionRef {
                        name: "<lambda>".to_string(),
                        arity: parameters.len(),
                        chunk_index: self.chunks.len(),
                        locals_count: parameters.len(),
                    },
                    captured: Vec::new(), // Would need to analyze captured variables
                };
                
                // Create new chunk for lambda body
                let old_chunk = self.current_chunk;
                self.chunks.push(Chunk::new());
                self.current_chunk = self.chunks.len() - 1;
                
                // Enter lambda scope
                self.scope_depth += 1;
                let old_locals = self.locals.clone();
                self.locals.clear();
                
                // Add parameters as locals
                for param in parameters {
                    self.locals.push(param.name.clone());
                }
                
                // Compile lambda body
                self.compile_expression(body)?;
                self.emit_instruction(Instruction::ReturnValue);
                
                // Restore scope
                self.scope_depth -= 1;
                self.locals = old_locals;
                self.current_chunk = old_chunk;
                
                // Emit closure creation instruction
                self.emit_instruction(Instruction::NewClosure(closure_ref.function.chunk_index));
            }
        }
        
        Ok(())
    }
    
    /// Convert a literal to a value
    fn literal_to_value(&self, literal: &Literal) -> Result<Value> {
        let value = match literal {
            Literal::String(s) => Value::String(s.clone()),
            Literal::Integer(i) => Value::Integer(*i),
            Literal::BigInteger(bi) => Value::BigInteger(bi.clone()),
            Literal::Float(f) => Value::Float(*f),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Null => Value::Null,
            Literal::Array(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.literal_to_value(element)?);
                }
                Value::Array(FlowArray { elements: values })
            }
            Literal::Object(properties) => {
                let mut props = std::collections::HashMap::new();
                for (key, value) in properties {
                    props.insert(key.clone(), self.literal_to_value(value)?);
                }
                Value::Object(FlowObject { properties: props })
            }
        };
        Ok(value)
    }
    
    /// Emit a constant
    fn emit_constant(&mut self, value: Value) {
        let index = self.current_chunk_mut().add_constant(value);
        self.emit_instruction(Instruction::LoadConstant(index));
    }
    
    /// Emit an instruction
    fn emit_instruction(&mut self, instruction: Instruction) {
        self.current_chunk_mut().instructions.push(instruction);
    }
    
    /// Emit a jump instruction and return its index for patching
    fn emit_jump(&mut self, instruction: Instruction) -> usize {
        self.emit_instruction(instruction);
        self.current_chunk().instructions.len() - 1
    }
    
    /// Patch a jump instruction with the correct offset
    fn patch_jump(&mut self, jump_index: usize) -> Result<()> {
        let offset = self.current_chunk().instructions.len() - jump_index - 1;
        
        match &mut self.current_chunk_mut().instructions[jump_index] {
            Instruction::Jump(ref mut jump_offset) => *jump_offset = offset,
            Instruction::JumpIfFalse(ref mut jump_offset) => *jump_offset = offset,
            Instruction::JumpIfTrue(ref mut jump_offset) => *jump_offset = offset,
            _ => return Err(FlowError::runtime_error("Invalid jump instruction to patch")),
        }
        
        Ok(())
    }
    
    /// Get current chunk
    fn current_chunk(&self) -> &Chunk {
        &self.chunks[self.current_chunk]
    }
    
    /// Get current chunk mutably
    fn current_chunk_mut(&mut self) -> &mut Chunk {
         &mut self.chunks[self.current_chunk]
     }
}

impl VirtualMachine {
    // Built-in function implementations
    fn builtin_show(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(FlowError::runtime_error("show() takes exactly 1 argument"));
        }
        println!("{}", args[0]);
        Ok(Value::Null)
    }

    fn builtin_print(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(FlowError::runtime_error("print() takes exactly 1 argument"));
        }
        print!("{}", args[0]);
        Ok(Value::Null)
    }

    fn builtin_len(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(FlowError::runtime_error("len() takes exactly 1 argument"));
        }
        match &args[0] {
            Value::String(s) => Ok(Value::Integer(s.len() as i64)),
            Value::Array(array) => Ok(Value::Integer(array.elements.len() as i64)),
            Value::Object(object) => Ok(Value::Integer(object.properties.len() as i64)),
            _ => Err(FlowError::runtime_error("len() can only be called on strings, arrays, or objects")),
        }
    }

    fn builtin_type(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(FlowError::runtime_error("type() takes exactly 1 argument"));
        }
        let type_name = match &args[0] {
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
            Value::BytecodeFunction { .. } => "function",
        };
        Ok(Value::String(type_name.to_string()))
    }

    fn builtin_str(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(FlowError::runtime_error("str() takes exactly 1 argument"));
        }
        Ok(Value::String(args[0].to_string()))
    }

    fn builtin_int(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(FlowError::runtime_error("int() takes exactly 1 argument"));
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
            _ => Err(FlowError::runtime_error("Cannot convert value to integer")),
        }
    }

    fn builtin_float(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value> {
        if args.len() != 1 {
            return Err(FlowError::runtime_error("float() takes exactly 1 argument"));
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
            Value::Boolean(b) => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
            _ => Err(FlowError::runtime_error("Cannot convert value to float")),
        }
    }
}