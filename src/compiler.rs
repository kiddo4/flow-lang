//! Compiler for FlowLang
//! 
//! This module compiles FlowLang AST into bytecode instructions
//! for execution by the virtual machine.

use crate::ast::*;
use crate::bytecode::{Chunk, Instruction};
use crate::value::Value;
use crate::error::{FlowError, Result};
use crate::value::{FlowArray, FlowObject};
use std::collections::HashMap;

/// Compiler state for generating bytecode
pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
    function_type: FunctionType,
    loop_starts: Vec<usize>,
    loop_exits: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
    is_captured: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum FunctionType {
    Script,
    Function,
    Lambda,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
            function_type: FunctionType::Script,
            loop_starts: Vec::new(),
            loop_exits: Vec::new(),
        }
    }
    
    pub fn compile(&mut self, statements: &[Statement]) -> Result<Chunk> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        
        // Add halt instruction at the end
        self.emit_instruction(Instruction::Halt, 0);
        
        Ok(self.chunk.clone())
    }
    
    fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                self.emit_instruction(Instruction::Pop, 0);
            }
            
            Statement::VariableDeclaration { name, value } => {
                self.compile_expression(value)?;
                
                if self.scope_depth == 0 {
                    self.emit_instruction(Instruction::StoreGlobal(name.clone()), 0);
                } else {
                    self.add_local(name.clone());
                }
            }
            
            Statement::FunctionDeclaration { name, parameters, body } => {
                self.compile_function(name, parameters, body)?;
            }
            
            Statement::If { condition, then_branch, else_branch } => {
                self.compile_expression(condition)?;
                
                let else_jump = self.emit_jump(Instruction::JumpIfFalse(0));
                self.emit_instruction(Instruction::Pop, 0);
                
                for stmt in then_branch {
                    self.compile_statement(stmt)?;
                }
                
                if let Some(else_stmts) = else_branch {
                    let end_jump = self.emit_jump(Instruction::Jump(0));
                    self.patch_jump(else_jump);
                    self.emit_instruction(Instruction::Pop, 0);
                    
                    for stmt in else_stmts {
                        self.compile_statement(stmt)?;
                    }
                    self.patch_jump(end_jump);
                } else {
                    self.patch_jump(else_jump);
                    self.emit_instruction(Instruction::Pop, 0);
                }
            }
            
            Statement::While { condition, body } => {
                let loop_start = self.chunk.instructions.len();
                self.loop_starts.push(loop_start);
                self.loop_exits.push(Vec::new());
                
                self.compile_expression(condition)?;
                let exit_jump = self.emit_jump(Instruction::JumpIfFalse(0));
                self.emit_instruction(Instruction::Pop, 0);
                
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                self.emit_loop(loop_start);
                
                self.patch_jump(exit_jump);
                self.emit_instruction(Instruction::Pop, 0);
                
                // Patch break statements
                if let Some(exits) = self.loop_exits.pop() {
                    for exit in exits {
                        self.patch_jump(exit);
                    }
                }
                self.loop_starts.pop();
            }
            
            Statement::For { variable, start, end, body } => {
                // Compile start expression
                self.compile_expression(start)?;
                // Compile end expression
                self.compile_expression(end)?;
                // TODO: Implement proper for loop with range
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                self.emit_instruction(Instruction::Pop, 0);
                self.emit_instruction(Instruction::Pop, 0);
            }
            
            Statement::Return(expr) => {
                if let Some(value) = expr {
                    self.compile_expression(value)?;
                } else {
                    self.emit_constant(Value::Null, 0);
                }
                self.emit_instruction(Instruction::Return, 0);
            }
            
            Statement::Show(expr) => {
                self.compile_expression(expr)?;
                self.emit_instruction(Instruction::Print, 0);
            }
            
            Statement::Export { .. } => {
                // TODO: Implement module system
                return Err(FlowError::compilation_error("Export statements not yet implemented"));
            }
            
            Statement::Import { .. } => {
                // TODO: Implement module system
                return Err(FlowError::compilation_error("Import statements not yet implemented"));
            }
            
            Statement::TryCatch { .. } => {
                // TODO: Implement error handling
                return Err(FlowError::compilation_error("Try-catch statements not yet implemented"));
            }
        }
        
        Ok(())
    }
    
    fn compile_expression(&mut self, expression: &Expression) -> Result<()> {
        match expression {
            Expression::Literal(literal) => {
                let value = match literal {
                    Literal::String(s) => Value::String(s.clone()),
                    Literal::Integer(i) => Value::Integer(*i),
                    Literal::Float(f) => Value::Float(*f),
                    Literal::Boolean(b) => Value::Boolean(*b),
                    Literal::Null => Value::Null,
                    Literal::Array(_) => {
                        // TODO: Implement array literal compilation
                        return Err(FlowError::compilation_error("Array literals not yet implemented"));
                    },
                    Literal::Object(_) => {
                        // TODO: Implement object literal compilation
                        return Err(FlowError::compilation_error("Object literals not yet implemented"));
                    },
                };
                self.emit_constant(value, 0);
            }
            
            Expression::Identifier(name) => {
                if let Some(local_index) = self.resolve_local(name) {
                    self.emit_instruction(Instruction::LoadLocal(local_index), 0);
                } else {
                    self.emit_instruction(Instruction::LoadGlobal(name.clone()), 0);
                }
            }
            
            Expression::Binary { left, operator, right } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                
                match operator {
                    BinaryOperator::Add => self.emit_instruction(Instruction::Add, 0),
                    BinaryOperator::Subtract => self.emit_instruction(Instruction::Subtract, 0),
                    BinaryOperator::Multiply => self.emit_instruction(Instruction::Multiply, 0),
                    BinaryOperator::Divide => self.emit_instruction(Instruction::Divide, 0),
                    BinaryOperator::Modulo => self.emit_instruction(Instruction::Modulo, 0),
                    BinaryOperator::Equal => self.emit_instruction(Instruction::Equal, 0),
                    BinaryOperator::NotEqual => self.emit_instruction(Instruction::NotEqual, 0),
                    BinaryOperator::Greater => self.emit_instruction(Instruction::Greater, 0),
                    BinaryOperator::GreaterEqual => self.emit_instruction(Instruction::GreaterEqual, 0),
                    BinaryOperator::Less => self.emit_instruction(Instruction::Less, 0),
                    BinaryOperator::LessEqual => self.emit_instruction(Instruction::LessEqual, 0),
                    BinaryOperator::And => {
                        // Short-circuit evaluation
                        return Err(FlowError::compilation_error("Logical AND not yet implemented in bytecode"));
                    }
                    BinaryOperator::Or => {
                        // Short-circuit evaluation
                        return Err(FlowError::compilation_error("Logical OR not yet implemented in bytecode"));
                    }
                }
            }
            
            Expression::Unary { operator, operand } => {
                self.compile_expression(operand)?;
                
                match operator {
                    UnaryOperator::Minus => self.emit_instruction(Instruction::Negate, 0),
                    UnaryOperator::Not => self.emit_instruction(Instruction::Not, 0),
                }
            }
            
            Expression::FunctionCall { name, arguments } => {
                // Load function by name
                self.emit_instruction(Instruction::LoadGlobal(name.clone()), 0);
                
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                
                self.emit_instruction(Instruction::Call(arguments.len()), 0);
            }
            
            Expression::MethodCall { object, method, arguments } => {
                self.compile_expression(object)?;
                
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                
                self.emit_instruction(Instruction::CallMethod(method.clone()), 0);
            }
            
            Expression::Array { elements } => {
                for element in elements {
                    self.compile_expression(element)?;
                }
                self.emit_instruction(Instruction::NewArray(elements.len()), 0);
            }
            
            Expression::Object { properties } => {
                for (key, value) in properties {
                    self.emit_constant(Value::String(key.clone()), 0);
                    self.compile_expression(value)?;
                }
                self.emit_instruction(Instruction::NewObject, 0);
            }
            
            Expression::Index { object, index } => {
                self.compile_expression(object)?;
                self.compile_expression(index)?;
                self.emit_instruction(Instruction::GetIndex, 0);
            }
            
            Expression::PropertyAccess { object, property } => {
                self.compile_expression(object)?;
                self.emit_constant(Value::String(property.clone()), 0);
                self.emit_instruction(Instruction::GetProperty(property.clone()), 0);
            }
            
            Expression::Lambda { parameters, body } => {
                // TODO: Implement lambda compilation
                return Err(FlowError::compilation_error("Lambda expressions not yet implemented in bytecode"));
            }
        }
        
        Ok(())
    }
    
    fn compile_function(&mut self, name: &str, parameters: &[Parameter], body: &[Statement]) -> Result<()> {
        let mut function_compiler = Compiler::new();
        function_compiler.function_type = FunctionType::Function;
        
        // Add parameters as locals
        for param in parameters {
            function_compiler.add_local(param.name.clone());
        }
        
        // Compile function body
        for statement in body {
            function_compiler.compile_statement(statement)?;
        }
        
        // Implicit return null if no explicit return
        function_compiler.emit_constant(Value::Null, 0);
        function_compiler.emit_instruction(Instruction::Return, 0);
        
        let function_chunk = function_compiler.chunk;
        
        // Store function in constants and create function value
        let function_address = self.chunk.constants.len();
        let function_value = Value::BytecodeFunction {
            address: function_address,
            arity: parameters.len(),
            locals_count: function_compiler.locals.len(),
        };
        
        // For now, store the chunk as a constant (in a real implementation,
        // we'd have a separate function table)
        self.chunk.constants.push(function_value.clone());
        
        // Define the function globally
        self.emit_constant(function_value, 0);
        self.emit_instruction(Instruction::StoreGlobal(name.to_string()), 0);
        
        Ok(())
    }
    
    fn emit_instruction(&mut self, instruction: Instruction, line: usize) {
        self.chunk.write_instruction(instruction, line);
    }
    
    fn emit_constant(&mut self, value: Value, line: usize) {
        let constant_index = self.make_constant(value);
        self.emit_instruction(Instruction::LoadConstant(constant_index), line);
    }
    
    fn make_constant(&mut self, value: Value) -> usize {
        self.chunk.add_constant(value)
    }
    
    fn emit_jump(&mut self, instruction: Instruction) -> usize {
        self.emit_instruction(instruction, 0);
        self.chunk.instructions.len() - 1
    }
    
    fn patch_jump(&mut self, offset: usize) {
        let jump_target = self.chunk.instructions.len();
        
        match &mut self.chunk.instructions[offset] {
            Instruction::Jump(ref mut target) => *target = jump_target,
            Instruction::JumpIfFalse(ref mut target) => *target = jump_target,
            Instruction::JumpIfTrue(ref mut target) => *target = jump_target,
            _ => panic!("Invalid jump instruction to patch"),
        }
    }
    
    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_instruction(Instruction::Jump(loop_start), 0);
    }
    
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    
    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        
        // Remove locals from this scope
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            self.locals.pop();
            self.emit_instruction(Instruction::Pop, 0);
        }
    }
    
    fn add_local(&mut self, name: String) {
        let local = Local {
            name,
            depth: self.scope_depth,
            is_captured: false,
        };
        self.locals.push(local);
    }
    
    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i);
            }
        }
        None
    }
}

/// Compile a list of statements into bytecode
pub fn compile_program(statements: &[Statement]) -> Result<Chunk> {
    let mut compiler = Compiler::new();
    compiler.compile(statements)
}