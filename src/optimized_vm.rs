//! Optimized Virtual Machine with performance improvements
//! 
//! This module implements performance optimizations including:
//! - Inline caching for function calls
//! - Optimized instruction dispatch
//! - Stack frame optimization
//! - Constant folding

use crate::bytecode::{Instruction, VirtualMachine};
use crate::value::Value;
use crate::error::{FlowError, Result};
// Removed unused HashMap import
use std::time::Instant;

/// Optimized instruction variants for better performance
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizedInstruction {
    // Fast arithmetic operations for common cases
    AddIntInt,
    AddIntFloat,
    AddFloatFloat,
    MulIntInt,
    MulIntFloat,
    MulFloatFloat,
    
    // Optimized constant loading
    LoadConstInt(i64),
    LoadConstFloat(f64),
    LoadConstString(String),
    LoadConstBool(bool),
    
    // Fast variable access
    LoadLocalFast(usize),
    StoreLocalFast(usize),
    
    // Optimized function calls
    CallBuiltinFast(BuiltinFunction),
    CallFunctionCached(usize, usize), // address, cache_slot
    
    // Control flow optimizations
    JumpIfTrueFast(usize),
    JumpIfFalseFast(usize),
    JumpFast(usize),
    
    // Fallback to original instruction
    Original(Instruction),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFunction {
    Show,
    Length,
    Type,
    ToString,
}

/// Call site cache for function calls
#[derive(Debug, Clone)]
struct CallSiteCache {
    target_address: Option<usize>,
    hit_count: u32,
    miss_count: u32,
}

impl CallSiteCache {
    fn new() -> Self {
        Self {
            target_address: None,
            hit_count: 0,
            miss_count: 0,
        }
    }
    
    fn hit(&mut self, address: usize) -> bool {
        if self.target_address == Some(address) {
            self.hit_count += 1;
            true
        } else {
            self.miss_count += 1;
            self.target_address = Some(address);
            false
        }
    }
    
    fn hit_rate(&self) -> f64 {
        if self.hit_count + self.miss_count == 0 {
            0.0
        } else {
            self.hit_count as f64 / (self.hit_count + self.miss_count) as f64
        }
    }
}

/// Optimized Virtual Machine with performance enhancements
pub struct OptimizedVM {
    base_vm: VirtualMachine,
    optimized_instructions: Vec<OptimizedInstruction>,
    call_site_caches: Vec<CallSiteCache>,
    execution_stats: ExecutionStats,
    optimization_enabled: bool,
}

#[derive(Debug, Clone, Default)]
struct ExecutionStats {
    instructions_executed: u64,
    function_calls: u64,
    cache_hits: u64,
    cache_misses: u64,
    optimization_time: std::time::Duration,
}

impl OptimizedVM {
    pub fn new() -> Self {
        Self {
            base_vm: VirtualMachine::new(),
            optimized_instructions: Vec::new(),
            call_site_caches: Vec::new(),
            execution_stats: ExecutionStats::default(),
            optimization_enabled: true,
        }
    }
    
    pub fn with_base_vm(vm: VirtualMachine) -> Self {
        Self {
            base_vm: vm,
            optimized_instructions: Vec::new(),
            call_site_caches: Vec::new(),
            execution_stats: ExecutionStats::default(),
            optimization_enabled: true,
        }
    }
    
    /// Optimize bytecode instructions for better performance
    pub fn optimize_instructions(&mut self, instructions: &[Instruction]) -> Result<()> {
        let start_time = Instant::now();
        
        self.optimized_instructions.clear();
        self.call_site_caches.clear();
        
        for instruction in instructions {
            let optimized = self.optimize_single_instruction(instruction);
            self.optimized_instructions.push(optimized);
        }
        
        // Perform peephole optimizations
        self.peephole_optimize();
        
        self.execution_stats.optimization_time += start_time.elapsed();
        Ok(())
    }
    
    fn optimize_single_instruction(&mut self, instruction: &Instruction) -> OptimizedInstruction {
        match instruction {
            Instruction::LoadConstant(index) => {
                // Try to inline common constants
                if let Some(constant) = self.base_vm.get_constant(*index) {
                    match constant {
                        Value::Integer(i) => OptimizedInstruction::LoadConstInt(*i),
                        Value::Float(f) => OptimizedInstruction::LoadConstFloat(*f),
                        Value::String(s) => OptimizedInstruction::LoadConstString(s.clone()),
                        Value::Boolean(b) => OptimizedInstruction::LoadConstBool(*b),
                        _ => OptimizedInstruction::Original(instruction.clone()),
                    }
                } else {
                    OptimizedInstruction::Original(instruction.clone())
                }
            }
            
            Instruction::LoadLocal(index) => {
                OptimizedInstruction::LoadLocalFast(*index)
            }
            
            Instruction::StoreLocal(index) => {
                OptimizedInstruction::StoreLocalFast(*index)
            }
            
            Instruction::Add => {
                // Will be optimized during peephole pass
                OptimizedInstruction::Original(instruction.clone())
            }
            
            Instruction::Multiply => {
                // Will be optimized during peephole pass
                OptimizedInstruction::Original(instruction.clone())
            }
            
            Instruction::Call(_arity) => {
                // Create cache slot for this call site
                let cache_slot = self.call_site_caches.len();
                self.call_site_caches.push(CallSiteCache::new());
                OptimizedInstruction::CallFunctionCached(0, cache_slot) // address will be resolved at runtime
            }
            
            Instruction::JumpIfTrue(target) => {
                OptimizedInstruction::JumpIfTrueFast(*target)
            }
            
            Instruction::JumpIfFalse(target) => {
                OptimizedInstruction::JumpIfFalseFast(*target)
            }
            
            Instruction::Jump(target) => {
                OptimizedInstruction::JumpFast(*target)
            }
            
            _ => OptimizedInstruction::Original(instruction.clone()),
        }
    }
    
    /// Perform peephole optimizations on the instruction sequence
    fn peephole_optimize(&mut self) {
        // Simplified peephole optimization to avoid borrowing issues
        // TODO: Implement more sophisticated optimizations later
        let mut new_instructions = Vec::new();
        let mut i = 0;
        
        while i < self.optimized_instructions.len() {
            if i + 2 < self.optimized_instructions.len() {
                // Check for optimization patterns
                match (
                    &self.optimized_instructions[i],
                    &self.optimized_instructions[i + 1],
                    &self.optimized_instructions[i + 2]
                ) {
                    (
                        OptimizedInstruction::LoadConstInt(a),
                        OptimizedInstruction::LoadConstInt(b),
                        OptimizedInstruction::Original(Instruction::Add)
                    ) => {
                        new_instructions.push(OptimizedInstruction::LoadConstInt(*a));
                        new_instructions.push(OptimizedInstruction::LoadConstInt(*b));
                        new_instructions.push(OptimizedInstruction::AddIntInt);
                        i += 3;
                        continue;
                    }
                    _ => {
                        new_instructions.push(self.optimized_instructions[i].clone());
                        i += 1;
                    }
                }
            } else {
                new_instructions.push(self.optimized_instructions[i].clone());
                i += 1;
            }
        }
        
        self.optimized_instructions = new_instructions;
    }
    
    /// Execute optimized bytecode
    pub fn execute_optimized(&mut self) -> Result<()> {
        let mut pc = 0;
        
        while pc < self.optimized_instructions.len() {
            self.execution_stats.instructions_executed += 1;
            
            match &self.optimized_instructions[pc].clone() {
                OptimizedInstruction::LoadConstInt(value) => {
                    self.base_vm.push_stack(Value::Integer(*value));
                    pc += 1;
                }
                
                OptimizedInstruction::LoadConstFloat(value) => {
                    self.base_vm.push_stack(Value::Float(*value));
                    pc += 1;
                }
                
                OptimizedInstruction::LoadConstString(value) => {
                    self.base_vm.push_stack(Value::String(value.clone()));
                    pc += 1;
                }
                
                OptimizedInstruction::LoadConstBool(value) => {
                    self.base_vm.push_stack(Value::Boolean(*value));
                    pc += 1;
                }
                
                OptimizedInstruction::AddIntInt => {
                    if let (Some(Value::Integer(b)), Some(Value::Integer(a))) = 
                        (self.base_vm.pop_stack(), self.base_vm.pop_stack()) {
                        match a.checked_add(b) {
                            Some(result) => self.base_vm.push_stack(Value::Integer(result)),
                            None => {
                                let big_a = crate::bigint::BigInt::from_i64(a);
                                let big_b = crate::bigint::BigInt::from_i64(b);
                                self.base_vm.push_stack(Value::BigInteger(big_a + big_b));
                            }
                        }
                    } else {
                        return Err(FlowError::runtime_error("Type mismatch in optimized add"));
                    }
                    pc += 1;
                }
                
                OptimizedInstruction::AddIntFloat => {
                    if let (Some(Value::Float(b)), Some(Value::Integer(a))) = 
                        (self.base_vm.pop_stack(), self.base_vm.pop_stack()) {
                        self.base_vm.push_stack(Value::Float(a as f64 + b));
                    } else {
                        return Err(FlowError::runtime_error("Type mismatch in optimized add"));
                    }
                    pc += 1;
                }
                
                OptimizedInstruction::AddFloatFloat => {
                    if let (Some(Value::Float(b)), Some(Value::Float(a))) = 
                        (self.base_vm.pop_stack(), self.base_vm.pop_stack()) {
                        self.base_vm.push_stack(Value::Float(a + b));
                    } else {
                        return Err(FlowError::runtime_error("Type mismatch in optimized add"));
                    }
                    pc += 1;
                }
                
                OptimizedInstruction::LoadLocalFast(index) => {
                    if let Some(value) = self.base_vm.get_local(*index) {
                        self.base_vm.push_stack(value.clone());
                    } else {
                        return Err(FlowError::runtime_error("Local variable not found"));
                    }
                    pc += 1;
                }
                
                OptimizedInstruction::StoreLocalFast(index) => {
                    if let Some(value) = self.base_vm.pop_stack() {
                        self.base_vm.set_local(*index, value)?;
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                    pc += 1;
                }
                
                OptimizedInstruction::JumpIfTrueFast(target) => {
                    if let Some(value) = self.base_vm.pop_stack() {
                        if self.base_vm.is_truthy(&value) {
                            pc = *target;
                        } else {
                            pc += 1;
                        }
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                OptimizedInstruction::JumpIfFalseFast(target) => {
                    if let Some(value) = self.base_vm.pop_stack() {
                        if !self.base_vm.is_truthy(&value) {
                            pc = *target;
                        } else {
                            pc += 1;
                        }
                    } else {
                        return Err(FlowError::runtime_error("Stack underflow"));
                    }
                }
                
                OptimizedInstruction::JumpFast(target) => {
                    pc = *target;
                }
                
                OptimizedInstruction::CallBuiltinFast(builtin) => {
                    self.execute_builtin_fast(builtin)?;
                    pc += 1;
                }
                
                OptimizedInstruction::Original(instruction) => {
                    // Fall back to original VM execution
                    self.base_vm.execute_single_instruction(instruction)?;
                    pc += 1;
                }
                
                _ => {
                    return Err(FlowError::runtime_error("Unimplemented optimized instruction"));
                }
            }
        }
        
        Ok(())
    }
    
    fn execute_builtin_fast(&mut self, builtin: &BuiltinFunction) -> Result<()> {
        match builtin {
            BuiltinFunction::Show => {
                if let Some(value) = self.base_vm.pop_stack() {
                    println!("{}", value);
                    self.base_vm.push_stack(Value::Null);
                } else {
                    return Err(FlowError::runtime_error("Stack underflow in show"));
                }
            }
            
            BuiltinFunction::Length => {
                if let Some(value) = self.base_vm.pop_stack() {
                    let length = match &value {
                        Value::String(s) => s.len() as i64,
                        Value::Array(arr) => arr.len() as i64,
                        _ => return Err(FlowError::type_error("Length not supported for this type")),
                    };
                    self.base_vm.push_stack(Value::Integer(length));
                } else {
                    return Err(FlowError::runtime_error("Stack underflow in length"));
                }
            }
            
            BuiltinFunction::Type => {
                if let Some(value) = self.base_vm.pop_stack() {
                    let type_name = value.type_name();
                    self.base_vm.push_stack(Value::String(type_name.to_string()));
                } else {
                    return Err(FlowError::runtime_error("Stack underflow in type"));
                }
            }
            
            BuiltinFunction::ToString => {
                if let Some(value) = self.base_vm.pop_stack() {
                    self.base_vm.push_stack(Value::String(value.to_string()));
                } else {
                    return Err(FlowError::runtime_error("Stack underflow in toString"));
                }
            }
        }
        Ok(())
    }
    
    pub fn get_execution_stats(&self) -> &ExecutionStats {
        &self.execution_stats
    }
    
    pub fn print_performance_report(&self) {
        println!("=== FlowLang VM Performance Report ===");
        println!("Instructions executed: {}", self.execution_stats.instructions_executed);
        println!("Function calls: {}", self.execution_stats.function_calls);
        println!("Cache hits: {}", self.execution_stats.cache_hits);
        println!("Cache misses: {}", self.execution_stats.cache_misses);
        
        if self.execution_stats.cache_hits + self.execution_stats.cache_misses > 0 {
            let hit_rate = self.execution_stats.cache_hits as f64 / 
                (self.execution_stats.cache_hits + self.execution_stats.cache_misses) as f64 * 100.0;
            println!("Cache hit rate: {:.2}%", hit_rate);
        }
        
        println!("Optimization time: {:?}", self.execution_stats.optimization_time);
        println!("======================================");
    }
}

// Extension trait to add methods to the base VM
trait VMExtensions {
    fn push_stack(&mut self, value: Value);
    fn pop_stack(&mut self) -> Option<Value>;
    fn get_local(&self, index: usize) -> Option<&Value>;
    fn set_local(&mut self, index: usize, value: Value) -> Result<()>;
    fn get_constant(&self, index: usize) -> Option<&Value>;
    fn is_truthy(&self, value: &Value) -> bool;
    fn execute_single_instruction(&mut self, instruction: &Instruction) -> Result<()>;
}

impl VMExtensions for VirtualMachine {
    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }
    
    fn pop_stack(&mut self) -> Option<Value> {
        self.stack.pop()
    }
    
    fn get_local(&self, index: usize) -> Option<&Value> {
        self.locals.get(index)
    }
    
    fn set_local(&mut self, index: usize, value: Value) -> Result<()> {
        if index < self.locals.len() {
            self.locals[index] = value;
            Ok(())
        } else {
            Err(FlowError::runtime_error("Local variable index out of bounds"))
        }
    }
    
    fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constant_pool.get(index)
    }
    
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
    
    fn execute_single_instruction(&mut self, _instruction: &Instruction) -> Result<()> {
        // This would need to be implemented by exposing the instruction execution logic
        // from the base VM. For now, we'll return an error.
        Err(FlowError::runtime_error("Single instruction execution not implemented"))
    }
}