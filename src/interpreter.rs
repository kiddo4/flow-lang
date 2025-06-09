use crate::ast::*;
use crate::error::{FlowError, Result};
use crate::value::{Value, FlowArray, FlowObject, Environment};
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
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            environment: Environment::new(),
            return_value: None,
        };
        
        // Add built-in functions
        interpreter.add_builtins();
        interpreter
    }
    
    fn add_builtins(&mut self) {
        // Built-in math functions can be added here
        // For now, we'll keep it simple
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
    
    fn execute_statement(&mut self, statement: &Statement) -> Result<()> {
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
            
            Statement::Import(module) => {
                // For now, just acknowledge the import
                // In a full implementation, this would load external modules
                println!("Importing module: {}", module);
                Ok(())
            }
        }
    }
    
    fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value> {
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
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
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
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
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
        let function = self.environment.get_function(name)
            .cloned()
            .ok_or_else(|| FlowError::undefined_function(name))?;
        
        match function {
            Value::Function { parameters, body, .. } => {
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
                    _ => Err(FlowError::runtime_error(format!("Array has no method '{}'", method)))
                }
            }
            Value::Object(obj) => {
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