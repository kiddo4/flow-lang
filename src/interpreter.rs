use crate::ast::*;
use crate::error::{FlowError, Result};
use crate::value::{Value, FlowArray, FlowObject, Environment};
use crate::stdlib::StandardLibrary;
use std::collections::HashMap;

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Integer(i) => *i != 0,
            Value::BigInteger(bi) => !bi.is_zero(),
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
            Value::Function { .. } => true,
            Value::Lambda { .. } => true,
            Value::BytecodeFunction { .. } => true,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Integer(i) => i.to_string(),
            Value::BigInteger(bi) => bi.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(arr) => format!("{}", arr),
            Value::Object(obj) => format!("{}", obj),
            Value::Null => "null".to_string(),
            Value::Function { name, .. } => format!("<function {}>", name),
            Value::Lambda { .. } => "<lambda>".to_string(),
            Value::BytecodeFunction { .. } => "<bytecode function>".to_string(),
        }
    }
    
    // type_name method moved to value.rs to avoid duplication
}



pub struct Interpreter {
    environment: Environment,
    return_value: Option<Value>,
    stdlib: StandardLibrary,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            environment: Environment::new(),
            return_value: None,
            stdlib: StandardLibrary::new(),
        };
        
        // Add built-in functions
        interpreter.add_builtins();
        interpreter
    }
    
    fn add_builtins(&mut self) {
        // Standard library functions are now handled directly in call_function
        // No need to pre-register them in the environment
    }
    
    fn handle_import(&mut self, module_path: &str, imports: &ImportType) -> Result<()> {
        match module_path {
            "std" => self.handle_std_import(imports),
            "io" => self.handle_io_import(imports),
            "sys" => self.handle_sys_import(imports),
            "json" => self.handle_json_import(imports),
            "crypto" => self.handle_crypto_import(imports),
            "random" => self.handle_random_import(imports),
            "url" => self.handle_url_import(imports),
            "dir" => self.handle_dir_import(imports),
            _ => {
                return Err(FlowError::runtime_error(format!("Unknown module: {}", module_path)));
            }
        }
    }
    
    fn handle_std_import(&mut self, imports: &ImportType) -> Result<()> {
        match imports {
            ImportType::All => {
                // Create a module object with all std functions
                let mut module_obj = FlowObject::new();
                for func_name in self.get_std_function_names() {
                    let func = Value::Function {
                        name: func_name.clone(),
                        parameters: vec![],
                        body: vec![],
                    };
                    module_obj.set(func_name, func);
                }
                self.environment.define_variable("std".to_string(), Value::Object(module_obj));
                Ok(())
            }
            ImportType::Specific(functions) => {
                for func_name in functions {
                    if self.stdlib.has_function(func_name) {
                        // Create a wrapper function in the environment
                        let func = Value::Function {
                            name: func_name.clone(),
                            parameters: vec![], // Will be handled dynamically
                            body: vec![], // Stdlib functions don't have AST bodies
                        };
                        self.environment.define_variable(func_name.clone(), func);
                    } else {
                        return Err(FlowError::runtime_error(format!(
                            "Function '{}' not found in std module", 
                            func_name
                        )));
                    }
                }
                Ok(())
            }
            ImportType::Aliased(_, alias) => {
                // Create a module object with all std functions
                let mut module_obj = FlowObject::new();
                for func_name in self.get_std_function_names() {
                    let func = Value::Function {
                        name: func_name.clone(),
                        parameters: vec![],
                        body: vec![],
                    };
                    module_obj.set(func_name, func);
                }
                self.environment.define_variable(alias.clone(), Value::Object(module_obj));
                Ok(())
            }
            ImportType::SpecificAliased(func_aliases) => {
                for (func_name, alias) in func_aliases {
                    if self.stdlib.has_function(func_name) {
                        let func = Value::Function {
                            name: func_name.clone(),
                            parameters: vec![],
                            body: vec![],
                        };
                        self.environment.define_variable(alias.clone(), func);
                    } else {
                        return Err(FlowError::runtime_error(format!(
                            "Function '{}' not found in std module", 
                            func_name
                        )));
                    }
                }
                Ok(())
            }
        }
    }
    
    fn handle_io_import(&mut self, imports: &ImportType) -> Result<()> {
        let io_functions = vec!["write_file", "read_file", "append_file", "file_exists", "delete_file"];
        self.handle_module_import("io", &io_functions, imports)
    }
    
    fn handle_sys_import(&mut self, imports: &ImportType) -> Result<()> {
        let sys_functions = vec!["get_env", "set_env", "get_cwd", "execute_command"];
        self.handle_module_import("sys", &sys_functions, imports)
    }
    
    fn handle_json_import(&mut self, imports: &ImportType) -> Result<()> {
        let json_functions = vec!["json_stringify", "json_parse"];
        self.handle_module_import("json", &json_functions, imports)
    }
    
    fn handle_crypto_import(&mut self, imports: &ImportType) -> Result<()> {
        let crypto_functions = vec!["hash_sha256", "hash_md5", "generate_uuid"];
        self.handle_module_import("crypto", &crypto_functions, imports)
    }
    
    fn handle_random_import(&mut self, imports: &ImportType) -> Result<()> {
        let random_functions = vec!["random_int", "random_float", "random_choice"];
        self.handle_module_import("random", &random_functions, imports)
    }
    
    fn handle_url_import(&mut self, imports: &ImportType) -> Result<()> {
        let url_functions = vec!["url_encode", "url_decode", "parse_url"];
        self.handle_module_import("url", &url_functions, imports)
    }
    
    fn handle_dir_import(&mut self, imports: &ImportType) -> Result<()> {
        let dir_functions = vec!["list_directory", "create_directory", "remove_directory"];
        self.handle_module_import("dir", &dir_functions, imports)
    }
    
    fn handle_module_import(
        &mut self, 
        module_name: &str, 
        available_functions: &[&str], 
        imports: &ImportType
    ) -> Result<()> {
        match imports {
            ImportType::All => {
                for func_name in available_functions {
                    if self.stdlib.has_function(func_name) {
                        let func = Value::Function {
                            name: func_name.to_string(),
                            parameters: vec![],
                            body: vec![],
                        };
                        self.environment.define_variable(func_name.to_string(), func);
                    }
                }
                Ok(())
            }
            ImportType::Specific(functions) => {
                for func_name in functions {
                    if available_functions.contains(&func_name.as_str()) {
                        if self.stdlib.has_function(func_name) {
                            let func = Value::Function {
                                name: func_name.clone(),
                                parameters: vec![],
                                body: vec![],
                            };
                            self.environment.define_variable(func_name.clone(), func);
                        }
                    } else {
                        return Err(FlowError::runtime_error(format!(
                            "Function '{}' not found in {} module", 
                            func_name, 
                            module_name
                        )));
                    }
                }
                Ok(())
            }
            ImportType::Aliased(_, alias) => {
                let mut module_obj = FlowObject::new();
                for func_name in available_functions {
                    if self.stdlib.has_function(func_name) {
                        let func = Value::Function {
                            name: func_name.to_string(),
                            parameters: vec![],
                            body: vec![],
                        };
                        module_obj.set(func_name.to_string(), func);
                    }
                }
                self.environment.define_variable(alias.clone(), Value::Object(module_obj));
                Ok(())
            }
            ImportType::SpecificAliased(func_aliases) => {
                for (func_name, alias) in func_aliases {
                    if available_functions.contains(&func_name.as_str()) {
                        if self.stdlib.has_function(func_name) {
                            let func = Value::Function {
                                name: func_name.clone(),
                                parameters: vec![],
                                body: vec![],
                            };
                            self.environment.define_variable(alias.clone(), func);
                        }
                    } else {
                        return Err(FlowError::runtime_error(format!(
                            "Function '{}' not found in {} module", 
                            func_name, 
                            module_name
                        )));
                    }
                }
                Ok(())
            }
        }
    }
    
    fn get_std_function_names(&self) -> Vec<String> {
        // Return all available standard library function names
        vec![
            "write_file".to_string(), "read_file".to_string(), "append_file".to_string(),
            "file_exists".to_string(), "delete_file".to_string(),
            "get_env".to_string(), "set_env".to_string(), "get_cwd".to_string(), "execute_command".to_string(),
            "json_stringify".to_string(), "json_parse".to_string(),
            "hash_sha256".to_string(), "hash_md5".to_string(), "generate_uuid".to_string(),
            "random_int".to_string(), "random_float".to_string(), "random_choice".to_string(),
            "url_encode".to_string(), "url_decode".to_string(), "parse_url".to_string(),
            "list_directory".to_string(), "create_directory".to_string(), "remove_directory".to_string(),
        ]
    }
    
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.environment.define_variable(name, value);
    }
    
    pub fn get_environment_mut(&mut self) -> &mut Environment {
        &mut self.environment
    }
    
    pub fn execute(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            match self.execute_statement(statement) {
                Ok(_) => {},
                Err(FlowError::Return { value }) => {
                    return Ok(());
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    
    pub fn execute_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::VariableDeclaration { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.environment.define_variable(name.clone(), val);
                Ok(())
            }
            
            Statement::FunctionDeclaration { name, parameters, body } => {
                let func = Value::Function {
                    name: name.clone(),
                    parameters: parameters.clone(),
                    body: body.clone(),
                };
                self.environment.define_function(name.clone(), func);
                Ok(())
            }
            
            Statement::Export(statement) => {
                // For now, just execute the statement
                // In a full implementation, this would register the export
                self.execute_statement(statement)
            }
            
            Statement::TryCatch { try_block, catch_variable, catch_block } => {
                // Execute try block
                for stmt in try_block {
                    match self.execute_statement(stmt) {
                        Ok(_) => continue,
                        Err(error) => {
                            // Catch the error and execute catch block
                            self.environment.define_variable(
                                catch_variable.clone(),
                                Value::String(error.to_string())
                            );
                            
                            for catch_stmt in catch_block {
                                self.execute_statement(catch_stmt)?;
                            }
                            
                            return Ok(());
                        }
                    }
                }
                Ok(())
            }
            
            Statement::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition)?;
                
                if condition_value.is_truthy() {
                    for stmt in then_branch {
                        self.execute_statement(stmt)?;
                        if self.return_value.is_some() {
                            break;
                        }
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.execute_statement(stmt)?;
                        if self.return_value.is_some() {
                            break;
                        }
                    }
                }
                Ok(())
            }
            
            Statement::While { condition, body } => {
                while self.evaluate_expression(condition)?.is_truthy() {
                    for stmt in body {
                        self.execute_statement(stmt)?;
                        if self.return_value.is_some() {
                            return Ok(());
                        }
                    }
                }
                Ok(())
            }
            
            Statement::For { variable, start, end, body } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;
                
                let (start_int, end_int) = match (start_val, end_val) {
                    (Value::Integer(s), Value::Integer(e)) => (s, e),
                    _ => return Err(FlowError::type_error("For loop bounds must be integers")),
                };
                
                for i in start_int..=end_int {
                    self.environment.define_variable(variable.clone(), Value::Integer(i));
                    
                    for stmt in body {
                        self.execute_statement(stmt)?;
                        if self.return_value.is_some() {
                            return Ok(());
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Show(expression) => {
                let value = self.evaluate_expression(expression)?;
                println!("{}", value);
                Ok(())
            }
            
            Statement::Return(expression) => {
                let value = if let Some(expr) = expression {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Null
                };
                return Err(FlowError::return_value(value));
            }
            
            Statement::Expression(expression) => {
                self.evaluate_expression(expression)?;
                Ok(())
            }
            
            Statement::Import { module_path, imports } => {
                self.handle_import(module_path, imports)
            }
        }
    }
    
    pub fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value> {
        match expression {
            Expression::Literal(literal) => Ok(self.literal_to_value(literal)),
            
            Expression::Identifier(name) => {
                self.environment.get_variable(name)
                    .cloned()
                    .ok_or_else(|| FlowError::undefined_variable(name.to_string()))
            }
            
            Expression::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_operator(&left_val, operator, &right_val)
            }
            
            Expression::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(operand)?;
                self.apply_unary_operator(operator, &operand_val)
            }
            
            Expression::FunctionCall { name, arguments } => {
                self.call_function(name, arguments)
            }
            
            Expression::MethodCall { object, method, arguments } => {
                let obj_val = self.evaluate_expression(object)?;
                self.call_method(&obj_val, method, arguments)
            }
            
            Expression::Array { elements } => {
                let mut array_elements = Vec::new();
                for element in elements {
                    let value = self.evaluate_expression(element)?;
                    array_elements.push(value);
                }
                Ok(Value::Array(FlowArray::from_values(array_elements)))
            }
            
            Expression::Object { properties } => {
                let mut object_properties = HashMap::new();
                for (key, value_expr) in properties {
                    let value = self.evaluate_expression(value_expr)?;
                    object_properties.insert(key.clone(), value);
                }
                Ok(Value::Object(FlowObject::from_map(object_properties)))
            }
            
            Expression::Index { object, index } => {
                let obj_val = self.evaluate_expression(object)?;
                let index_val = self.evaluate_expression(index)?;
                self.get_index(&obj_val, &index_val)
            }
            
            Expression::PropertyAccess { object, property } => {
                let obj_val = self.evaluate_expression(object)?;
                self.get_property(&obj_val, property)
            }
            
            Expression::Lambda { parameters, body } => {
                Ok(Value::Lambda {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                })
            }
        }
    }
    
    fn literal_to_value(&self, literal: &Literal) -> Value {
        match literal {
            Literal::String(s) => Value::String(s.clone()),
            Literal::Integer(i) => Value::Integer(*i),
            Literal::BigInteger(bi) => Value::BigInteger(bi.clone()),
            Literal::Float(f) => Value::Float(*f),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Null => Value::Null,
            Literal::Array(elements) => {
                let array_elements: Vec<Value> = elements
                    .iter()
                    .map(|lit| self.literal_to_value(lit))
                    .collect();
                Value::Array(FlowArray::from_values(array_elements))
            }
            Literal::Object(properties) => {
                let mut object_properties = HashMap::new();
                for (key, value_lit) in properties {
                    let value = self.literal_to_value(value_lit);
                    object_properties.insert(key.clone(), value);
                }
                Value::Object(FlowObject::from_map(object_properties))
            }
        }
    }
    
    fn apply_binary_operator(&self, left: &Value, operator: &BinaryOperator, right: &Value) -> Result<Value> {
        match operator {
            BinaryOperator::Add => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => {
                    match a.checked_add(*b) {
                        Some(result) => Ok(Value::Integer(result)),
                        None => {
                            // Overflow detected, use BigInt
                            let big_a = crate::bigint::BigInt::from_i64(*a);
                            let big_b = crate::bigint::BigInt::from_i64(*b);
                            Ok(Value::BigInteger(big_a + big_b))
                        }
                    }
                },
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::BigInteger(a), Value::BigInteger(b)) => Ok(Value::BigInteger(a.clone() + b.clone())),
                (Value::BigInteger(a), Value::Integer(b)) => Ok(Value::BigInteger(a.clone() + crate::bigint::BigInt::from_i64(*b))),
                (Value::Integer(a), Value::BigInteger(b)) => Ok(Value::BigInteger(crate::bigint::BigInt::from_i64(*a) + b.clone())),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b))),
                (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err(FlowError::type_error(format!("Cannot add {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::Subtract => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => {
                    match a.checked_sub(*b) {
                        Some(result) => Ok(Value::Integer(result)),
                        None => {
                            // Overflow detected, use BigInt
                            let big_a = crate::bigint::BigInt::from_i64(*a);
                            let big_b = crate::bigint::BigInt::from_i64(*b);
                            Ok(Value::BigInteger(big_a - big_b))
                        }
                    }
                },
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
                (Value::BigInteger(a), Value::BigInteger(b)) => Ok(Value::BigInteger(a.clone() - b.clone())),
                (Value::BigInteger(a), Value::Integer(b)) => Ok(Value::BigInteger(a.clone() - crate::bigint::BigInt::from_i64(*b))),
                (Value::Integer(a), Value::BigInteger(b)) => Ok(Value::BigInteger(crate::bigint::BigInt::from_i64(*a) - b.clone())),
                _ => Err(FlowError::type_error(format!("Cannot subtract {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::Multiply => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => {
                    match a.checked_mul(*b) {
                        Some(result) => Ok(Value::Integer(result)),
                        None => {
                            // Overflow, promote to BigInt
                            let big_a = crate::bigint::BigInt::from_i64(*a);
                            let big_b = crate::bigint::BigInt::from_i64(*b);
                            Ok(Value::BigInteger(big_a * big_b))
                        }
                    }
                },
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
                (Value::BigInteger(a), Value::BigInteger(b)) => Ok(Value::BigInteger(a.clone() * b.clone())),
                (Value::BigInteger(a), Value::Integer(b)) => Ok(Value::BigInteger(a.clone() * crate::bigint::BigInt::from_i64(*b))),
                (Value::Integer(a), Value::BigInteger(b)) => Ok(Value::BigInteger(crate::bigint::BigInt::from_i64(*a) * b.clone())),
                _ => Err(FlowError::type_error(format!("Cannot multiply {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::Divide => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => {
                    if *b == 0 {
                        Err(FlowError::DivisionByZero)
                    } else {
                        Ok(Value::Float(*a as f64 / *b as f64))
                    }
                },
                (Value::Float(a), Value::Float(b)) => {
                    if *b == 0.0 {
                        Err(FlowError::DivisionByZero)
                    } else {
                        Ok(Value::Float(a / b))
                    }
                },
                (Value::Integer(a), Value::Float(b)) => {
                    if *b == 0.0 {
                        Err(FlowError::DivisionByZero)
                    } else {
                        Ok(Value::Float(*a as f64 / b))
                    }
                },
                (Value::Float(a), Value::Integer(b)) => {
                    if *b == 0 {
                        Err(FlowError::DivisionByZero)
                    } else {
                        Ok(Value::Float(a / *b as f64))
                    }
                },
                _ => Err(FlowError::type_error(format!("Cannot divide {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::Modulo => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => {
                    if *b == 0 {
                        Err(FlowError::DivisionByZero)
                    } else {
                        Ok(Value::Integer(a % b))
                    }
                },
                _ => Err(FlowError::type_error(format!("Cannot modulo {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::Equal => Ok(Value::Boolean(self.values_equal(left, right))),
            BinaryOperator::NotEqual => Ok(Value::Boolean(!self.values_equal(left, right))),
            
            BinaryOperator::Greater => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a > b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Boolean(*a as f64 > *b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Boolean(*a > *b as f64)),
                _ => Err(FlowError::type_error(format!("Cannot compare {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::GreaterEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a >= b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Boolean(*a as f64 >= *b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Boolean(*a >= *b as f64)),
                _ => Err(FlowError::type_error(format!("Cannot compare {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::Less => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a < b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Boolean((*a as f64) < *b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Boolean(*a < (*b as f64))),
                _ => Err(FlowError::type_error(format!("Cannot compare {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::LessEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a <= b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Boolean((*a as f64) <= *b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Boolean(*a <= (*b as f64))),
                _ => Err(FlowError::type_error(format!("Cannot compare {} and {}", left.type_name(), right.type_name()))),
            },
            
            BinaryOperator::And => {
                let left_truthy = left.is_truthy();
                if !left_truthy {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            },
            
            BinaryOperator::Or => {
                let left_truthy = left.is_truthy();
                if left_truthy {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            },
        }
    }
    
    fn apply_unary_operator(&self, operator: &UnaryOperator, operand: &Value) -> Result<Value> {
        match operator {
            UnaryOperator::Not => Ok(Value::Boolean(!operand.is_truthy())),
            UnaryOperator::Minus => match operand {
                Value::Integer(i) => Ok(Value::Integer(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(FlowError::type_error(format!("Cannot negate {}", operand.type_name()))),
            },
        }
    }
    
    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Integer(a), Value::Float(b)) => (*a as f64 - b).abs() < f64::EPSILON,
            (Value::Float(a), Value::Integer(b)) => (a - *b as f64).abs() < f64::EPSILON,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
    
    fn call_function(&mut self, name: &str, arguments: &[Expression]) -> Result<Value> {
        // First check functions and variables for lambda values
        let function = if let Some(func) = self.environment.get_function(name) {
            func.clone()
        } else if let Some(var) = self.environment.get_variable(name) {
            var.clone()
        } else {
            // If not found in environment, check if it's a stdlib function
            if self.stdlib.has_function(name) {
                // Evaluate arguments for stdlib function
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expression(arg)?);
                }
                return self.stdlib.call_function(name, &args);
            }
            return Err(FlowError::undefined_function(name));
        };
        
        match function {
            Value::Function { name: func_name, parameters, body } => {
                // If this is a stdlib function wrapper, call the stdlib
                if self.stdlib.has_function(&func_name) {
                    let mut args = Vec::new();
                    for arg in arguments {
                        args.push(self.evaluate_expression(arg)?);
                    }
                    return self.stdlib.call_function(&func_name, &args);
                }
                // Otherwise call as user-defined function
                self.call_function_with_params(&parameters, &body, arguments)
            }
            
            Value::Lambda { parameters, body, closure } => {
                // Save current environment and switch to closure
                let saved_env = self.environment.clone();
                self.environment = closure.clone();
                
                let result = self.call_lambda_with_params(&parameters, &body, arguments);
                
                // Restore environment
                self.environment = saved_env;
                result
            }
            _ => Err(FlowError::runtime_error(format!("'{}' is not a function", name))),
        }
    }
    
    fn get_index(&self, object: &Value, index: &Value) -> Result<Value> {
        match (object, index) {
            (Value::Array(arr), Value::Integer(i)) => {
                let idx = *i as usize;
                if let Some(value) = arr.get(idx) {
                    Ok(value.clone())
                } else {
                    Err(FlowError::runtime_error(format!("Array index {} out of bounds", i)))
                }
            }
            (Value::Object(obj), Value::String(key)) => {
                if let Some(value) = obj.get(key) {
                    Ok(value.clone())
                } else {
                    Ok(Value::Null)
                }
            }
            _ => Err(FlowError::type_error(format!(
                "Cannot index {} with {}",
                object.type_name(),
                index.type_name()
            )))
        }
    }
    
    fn get_property(&self, object: &Value, property: &str) -> Result<Value> {
        match object {
            Value::Object(obj) => {
                if let Some(value) = obj.get(property) {
                    Ok(value.clone())
                } else {
                    Ok(Value::Null)
                }
            }
            Value::Array(arr) => {
                // Built-in array properties
                match property {
                    "length" => Ok(Value::Integer(arr.len() as i64)),
                    _ => Err(FlowError::runtime_error(format!("Array has no property '{}'", property)))
                }
            }
            _ => Err(FlowError::type_error(format!(
                "Cannot access property '{}' on {}",
                property,
                object.type_name()
            )))
        }
    }
    
    fn call_method(&mut self, object: &Value, method: &str, arguments: &[Expression]) -> Result<Value> {
        match object {
            Value::Array(arr) => {
                let mut arr_copy = arr.clone();
                match method {
                    "push" => {
                        if arguments.len() != 1 {
                            return Err(FlowError::runtime_error("push() expects exactly 1 argument".to_string()));
                        }
                        let value = self.evaluate_expression(&arguments[0])?;
                        arr_copy.push(value);
                        Ok(Value::Array(arr_copy))
                    }
                    "pop" => {
                        if !arguments.is_empty() {
                            return Err(FlowError::runtime_error("pop() expects no arguments".to_string()));
                        }
                        if let Some(value) = arr_copy.pop() {
                            Ok(value)
                        } else {
                            Ok(Value::Null)
                        }
                    }
                    "length" => {
                        if !arguments.is_empty() {
                            return Err(FlowError::runtime_error("length() expects no arguments".to_string()));
                        }
                        Ok(Value::Integer(arr.len() as i64))
                    }
                    "slice" => {
                        if arguments.len() != 2 {
                            return Err(FlowError::runtime_error("slice() expects exactly 2 arguments (start, end)".to_string()));
                        }
                        let start_val = self.evaluate_expression(&arguments[0])?;
                        let end_val = self.evaluate_expression(&arguments[1])?;
                        
                        if let (Value::Integer(start), Value::Integer(end)) = (start_val, end_val) {
                            match arr.slice(start as usize, end as usize) {
                                Ok(sliced_arr) => Ok(Value::Array(sliced_arr)),
                                Err(err) => Err(FlowError::runtime_error(err))
                            }
                        } else {
                            Err(FlowError::type_error("slice() arguments must be integers".to_string()))
                        }
                    }
                    _ => Err(FlowError::runtime_error(format!("Array has no method '{}'", method)))
                }
            }
            Value::Object(obj) => {
                // First check if the method exists as a property in the object
                if let Some(func_value) = obj.get(method) {
                    if let Value::Function { name, .. } = func_value {
                        // Evaluate arguments and call the stdlib function
                        let mut args = Vec::new();
                        for arg in arguments {
                            args.push(self.evaluate_expression(arg)?);
                        }
                        return self.stdlib.call_function(name, &args);
                    }
                }
                
                let mut obj_copy = obj.clone();
                match method {
                    "keys" => {
                        if !arguments.is_empty() {
                            return Err(FlowError::runtime_error("keys() expects no arguments".to_string()));
                        }
                        let keys: Vec<Value> = obj.keys()
                            .into_iter()
                            .map(|k| Value::String(k))
                            .collect();
                        Ok(Value::Array(FlowArray::from_values(keys)))
                    }
                    "has" => {
                        if arguments.len() != 1 {
                            return Err(FlowError::runtime_error("has() expects exactly 1 argument".to_string()));
                        }
                        let key_val = self.evaluate_expression(&arguments[0])?;
                        if let Value::String(key) = key_val {
                            Ok(Value::Boolean(obj.contains_key(&key)))
                        } else {
                            Err(FlowError::type_error("Object key must be a string".to_string()))
                        }
                    }
                    "remove" => {
                        if arguments.len() != 1 {
                            return Err(FlowError::runtime_error("remove() expects exactly 1 argument".to_string()));
                        }
                        let key_val = self.evaluate_expression(&arguments[0])?;
                        if let Value::String(key) = key_val {
                            if let Some(value) = obj_copy.remove(&key) {
                                Ok(value)
                            } else {
                                Ok(Value::Null)
                            }
                        } else {
                            Err(FlowError::type_error("Object key must be a string".to_string()))
                        }
                    }
                    _ => Err(FlowError::runtime_error(format!("Object has no method '{}'", method)))
                }
            }
            _ => Err(FlowError::type_error(format!(
                "Cannot call method '{}' on {}",
                method, object.type_name()
            )))
        }
    }
    
    fn call_function_with_params(
        &mut self,
        parameters: &[Parameter],
        body: &[Statement],
        arguments: &[Expression],
    ) -> Result<Value> {
        // Create new scope for function
        self.environment.push_scope();
        
        let mut arg_index = 0;
        let mut variadic_args = Vec::new();
        
        // Bind parameters
        for param in parameters {
            if param.is_variadic {
                // Collect remaining arguments into an array
                while arg_index < arguments.len() {
                    let arg_value = self.evaluate_expression(&arguments[arg_index])?;
                    variadic_args.push(arg_value);
                    arg_index += 1;
                }
                
                let array = crate::value::FlowArray::from_values(variadic_args.clone());
                self.environment.define_variable(param.name.clone(), Value::Array(array));
            } else if arg_index < arguments.len() {
                // Regular parameter with provided argument
                let arg_value = self.evaluate_expression(&arguments[arg_index])?;
                self.environment.define_variable(param.name.clone(), arg_value);
                arg_index += 1;
            } else if let Some(default) = &param.default_value {
                // Use default value
                let default_value = self.evaluate_expression(default)?;
                self.environment.define_variable(param.name.clone(), default_value);
            } else {
                // Missing required parameter
                self.environment.pop_scope();
                return Err(FlowError::runtime_error(format!(
                    "Missing required parameter '{}'",
                    param.name
                )));
            }
        }
        
        // Execute function body
        let mut result = Value::Null;
        for statement in body {
            match self.execute_statement(statement) {
                Ok(()) => {},
                Err(FlowError::Return { value }) => {
                    result = value;
                    break;
                },
                Err(e) => {
                    self.environment.pop_scope();
                    return Err(e);
                }
            }
        }
        
        self.environment.pop_scope();
        Ok(result)
    }
    
    fn call_lambda_with_params(
        &mut self,
        parameters: &[Parameter],
        body: &Expression,
        arguments: &[Expression],
    ) -> Result<Value> {
        // Create new scope for lambda
        self.environment.push_scope();
        
        let mut arg_index = 0;
        let mut variadic_args = Vec::new();
        
        // Bind parameters (same logic as functions)
        for param in parameters {
            if param.is_variadic {
                while arg_index < arguments.len() {
                    let arg_value = self.evaluate_expression(&arguments[arg_index])?;
                    variadic_args.push(arg_value);
                    arg_index += 1;
                }
                
                let array = crate::value::FlowArray::from_values(variadic_args.clone());
                self.environment.define_variable(param.name.clone(), Value::Array(array));
            } else if arg_index < arguments.len() {
                let arg_value = self.evaluate_expression(&arguments[arg_index])?;
                self.environment.define_variable(param.name.clone(), arg_value);
                arg_index += 1;
            } else if let Some(default) = &param.default_value {
                let default_value = self.evaluate_expression(default)?;
                self.environment.define_variable(param.name.clone(), default_value);
            } else {
                self.environment.pop_scope();
                return Err(FlowError::runtime_error(format!(
                    "Missing required parameter '{}'",
                    param.name
                )));
            }
        }
        
        // Evaluate lambda body
        let result = self.evaluate_expression(body);
        
        self.environment.pop_scope();
        result
    }
}