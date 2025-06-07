use crate::ast::*;
use crate::error::{FlowError, Result};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            // Skip newlines at the top level
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            
            statements.push(self.statement()?);
        }
        
        Ok(Program { statements })
    }
    
    fn statement(&mut self) -> Result<Statement> {
        match &self.peek() {
            Token::Let => self.variable_declaration(),
            Token::Def => self.function_declaration(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::For => self.for_statement(),
            Token::Show => self.show_statement(),
            Token::Return => self.return_statement(),
            Token::Import => self.import_statement(),
            Token::Export => self.export_statement(),
            Token::Try => self.try_catch_statement(),
            _ => {
                let expr = self.expression()?;
                self.consume_newline_or_eof()?;
                Ok(Statement::Expression(expr))
            }
        }
    }
    
    fn variable_declaration(&mut self) -> Result<Statement> {
        self.consume(&Token::Let, "Expected 'let'")?;
        
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(FlowError::parser_error("Expected variable name")),
        };
        
        self.consume(&Token::Be, "Expected 'be' after variable name")?;
        
        let value = self.expression()?;
        self.consume_newline_or_eof()?;
        
        Ok(Statement::VariableDeclaration { name, value })
    }
    
    fn function_declaration(&mut self) -> Result<Statement> {
        self.consume(&Token::Def, "Expected 'def'")?;
        
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(FlowError::parser_error("Expected function name")),
        };
        
        let mut parameters = Vec::new();
        
        if self.check(&Token::With) {
            self.advance(); // consume 'with'
            
            loop {
                // Check for variadic parameter
                let is_variadic = if self.check(&Token::Ellipsis) {
                    self.advance();
                    true
                } else {
                    false
                };
                
                let param_name = match self.advance() {
                    Token::Identifier(param) => param.clone(),
                    _ => return Err(FlowError::parser_error("Expected parameter name")),
                };
                
                // Check for default value
                let default_value = if self.check(&Token::Assign) {
                    self.advance(); // consume '='
                    Some(self.expression()?)
                } else {
                    None
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    default_value,
                    is_variadic,
                });
                
                if self.check(&Token::Comma) {
                    self.advance();
                    // Skip any newlines after the comma
                    while self.check(&Token::Newline) {
                        self.advance();
                    }
                } else {
                    break;
                }
            }
        }
        
        self.consume(&Token::Do, "Expected 'do' after function parameters")?;
        self.consume_newline()?;
        
        let mut body = Vec::new();
        while !self.check(&Token::End) && !self.is_at_end() {
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            body.push(self.statement()?);
        }
        
        self.consume(&Token::End, "Expected 'end' to close function")?;
        self.consume_newline_or_eof()?;
        
        Ok(Statement::FunctionDeclaration { name, parameters, body })
    }
    
    fn if_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::If, "Expected 'if'")?;
        
        let condition = self.expression()?;
        
        self.consume(&Token::Then, "Expected 'then' after if condition")?;
        self.consume_newline()?;
        
        let mut then_branch = Vec::new();
        while !self.check(&Token::Else) && !self.check(&Token::End) && !self.is_at_end() {
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            then_branch.push(self.statement()?);
        }
        
        let else_branch = if self.check(&Token::Else) {
            self.advance(); // consume 'else'
            
            // Check for 'else if'
            if self.check(&Token::If) {
                let else_if = self.if_statement()?;
                Some(vec![else_if])
            } else {
                self.consume_newline()?;
                let mut else_stmts = Vec::new();
                while !self.check(&Token::End) && !self.is_at_end() {
                    if self.check(&Token::Newline) {
                        self.advance();
                        continue;
                    }
                    else_stmts.push(self.statement()?);
                }
                Some(else_stmts)
            }
        } else {
            None
        };
        
        self.consume(&Token::End, "Expected 'end' to close if statement")?;
        self.consume_newline_or_eof()?;
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn while_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::While, "Expected 'while'")?;
        
        let condition = self.expression()?;
        
        self.consume(&Token::Do, "Expected 'do' after while condition")?;
        self.consume_newline()?;
        
        let mut body = Vec::new();
        while !self.check(&Token::End) && !self.is_at_end() {
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            body.push(self.statement()?);
        }
        
        self.consume(&Token::End, "Expected 'end' to close while loop")?;
        self.consume_newline_or_eof()?;
        
        Ok(Statement::While { condition, body })
    }
    
    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::For, "Expected 'for'")?;
        
        let variable = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(FlowError::parser_error("Expected variable name in for loop")),
        };
        
        self.consume(&Token::From, "Expected 'from' in for loop")?;
        let start = self.expression()?;
        
        self.consume(&Token::To, "Expected 'to' in for loop")?;
        let end = self.expression()?;
        
        self.consume(&Token::Do, "Expected 'do' after for range")?;
        self.consume_newline()?;
        
        let mut body = Vec::new();
        while !self.check(&Token::End) && !self.is_at_end() {
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            body.push(self.statement()?);
        }
        
        self.consume(&Token::End, "Expected 'end' to close for loop")?;
        self.consume_newline_or_eof()?;
        
        Ok(Statement::For {
            variable,
            start,
            end,
            body,
        })
    }
    
    fn show_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Show, "Expected 'show'")?;
        let expr = self.expression()?;
        self.consume_newline_or_eof()?;
        Ok(Statement::Show(expr))
    }
    
    fn return_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Return, "Expected 'return'")?;
        
        let value = if self.check(&Token::Newline) || self.is_at_end() {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume_newline_or_eof()?;
        Ok(Statement::Return(value))
    }
    
    fn import_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Import, "Expected 'import'")?;
        
        let module = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(FlowError::parser_error("Expected module name")),
        };
        
        self.consume_newline_or_eof()?;
        Ok(Statement::Import(module))
    }
    
    fn export_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Export, "Expected 'export'")?;
        let statement = self.statement()?;
        Ok(Statement::Export(Box::new(statement)))
    }
    
    fn try_catch_statement(&mut self) -> Result<Statement> {
        self.consume(&Token::Try, "Expected 'try'")?;
        self.consume_newline()?;
        
        let mut try_block = Vec::new();
        while !self.check(&Token::Catch) && !self.is_at_end() {
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            try_block.push(self.statement()?);
        }
        
        self.consume(&Token::Catch, "Expected 'catch'")?;
        
        let catch_variable = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(FlowError::parser_error("Expected error variable name")),
        };
        
        self.consume_newline()?;
        
        let mut catch_block = Vec::new();
        while !self.check(&Token::End) && !self.is_at_end() {
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            catch_block.push(self.statement()?);
        }
        
        self.consume(&Token::End, "Expected 'end'")?;
        self.consume_newline_or_eof()?;
        
        Ok(Statement::TryCatch {
            try_block,
            catch_variable,
            catch_block,
        })
    }
    
    fn expression(&mut self) -> Result<Expression> {
        self.logical_or()
    }
    
    fn logical_or(&mut self) -> Result<Expression> {
        let mut expr = self.logical_and()?;
        
        while self.check(&Token::Or) {
            self.advance();
            let right = self.logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn logical_and(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;
        
        while self.check(&Token::And) {
            self.advance();
            let right = self.equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;
        
        while matches!(self.peek(), Token::Equal | Token::NotEqual) {
            let operator = match self.advance() {
                Token::Equal => BinaryOperator::Equal,
                Token::NotEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;
        
        while matches!(self.peek(), Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual) {
            let operator = match self.advance() {
                Token::Greater => BinaryOperator::Greater,
                Token::GreaterEqual => BinaryOperator::GreaterEqual,
                Token::Less => BinaryOperator::Less,
                Token::LessEqual => BinaryOperator::LessEqual,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;
        
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let operator = match self.advance() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;
        
        while matches!(self.peek(), Token::Multiply | Token::Divide | Token::Modulo) {
            let operator = match self.advance() {
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                Token::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expression> {
        if matches!(self.peek(), Token::Not | Token::Minus) {
            let operator = match self.advance() {
                Token::Not => UnaryOperator::Not,
                Token::Minus => UnaryOperator::Minus,
                _ => unreachable!(),
            };
            let operand = self.unary()?;
            Ok(Expression::Unary {
                operator,
                operand: Box::new(operand),
            })
        } else {
            self.call()
        }
    }
    
    fn call(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;
        
        loop {
            if self.check(&Token::LeftParen) {
                self.advance();
                let mut arguments = Vec::new();
                
                if !self.check(&Token::RightParen) {
                    loop {
                        arguments.push(self.expression()?);
                        if self.check(&Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                
                self.consume(&Token::RightParen, "Expected ')' after arguments")?;
                
                match expr {
                    Expression::Identifier(name) => {
                        expr = Expression::FunctionCall { name, arguments };
                    }
                    _ => {
                        return Err(FlowError::parser_error("Invalid function call"));
                    }
                }
            } else if self.check(&Token::Dot) {
                self.advance();
                let property = match self.advance() {
                    Token::Identifier(name) => name.clone(),
                    _ => return Err(FlowError::parser_error("Expected property name after '.'")),
                };
                
                if self.check(&Token::LeftParen) {
                    // Method call
                    self.advance();
                    let mut arguments = Vec::new();
                    
                    if !self.check(&Token::RightParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if self.check(&Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    
                    self.consume(&Token::RightParen, "Expected ')' after arguments")?;
                    
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: property,
                        arguments,
                    };
                } else {
                    // Property access
                    expr = Expression::PropertyAccess {
                        object: Box::new(expr),
                        property,
                    };
                }
            } else if self.check(&Token::LeftBracket) {
                // Index access
                self.advance();
                let index = self.expression()?;
                self.consume(&Token::RightBracket, "Expected ']' after index")?;
                
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn primary(&mut self) -> Result<Expression> {
        match self.advance() {
            Token::String(s) => Ok(Expression::Literal(Literal::String(s.clone()))),
            Token::Integer(i) => Ok(Expression::Literal(Literal::Integer(*i))),
            Token::Float(f) => Ok(Expression::Literal(Literal::Float(*f))),
            Token::Boolean(b) => Ok(Expression::Literal(Literal::Boolean(*b))),
            Token::Identifier(name) => Ok(Expression::Identifier(name.clone())),
            Token::LeftParen => {
                // Check if this is a lambda expression
                if self.is_lambda_expression() {
                    self.parse_lambda()
                } else {
                    self.advance();
                    let expr = self.expression()?;
                    self.consume(&Token::RightParen, "Expected ')' after expression")?;
                    Ok(expr)
                }
            }
            Token::LeftBracket => self.parse_array(),
            Token::LeftBrace => self.parse_object(),
            _ => Err(FlowError::parser_error("Expected expression")),
        }
    }

    fn parse_array(&mut self) -> Result<Expression> {
        let mut elements = Vec::new();
        
        if !self.check(&Token::RightBracket) {
            loop {
                // Skip any newlines before the element
                while self.check(&Token::Newline) {
                    self.advance();
                }
                
                // Check if we've reached the end of the array
                if self.check(&Token::RightBracket) {
                    break;
                }
                
                elements.push(self.expression()?);
                if self.check(&Token::Comma) {
                    self.advance();
                    // Skip any newlines after the comma
                    while self.check(&Token::Newline) {
                        self.advance();
                    }
                } else {
                    break;
                }
            }
        }
        
        // Skip any newlines before the closing bracket
        while self.check(&Token::Newline) {
            self.advance();
        }
        
        self.consume(&Token::RightBracket, "Expected ']' after array elements")?;
        Ok(Expression::Array { elements })
    }

    fn parse_object(&mut self) -> Result<Expression> {
        let mut properties = Vec::new();
        
        if !self.check(&Token::RightBrace) {
            loop {
                // Skip any newlines before the key
                while self.check(&Token::Newline) {
                    self.advance();
                }
                
                // Check if we've reached the end of the object
                if self.check(&Token::RightBrace) {
                    break;
                }
                
                let key = match self.peek() {
                    Token::String(s) => {
                        let key = s.clone();
                        self.advance();
                        key
                    }
                    Token::Identifier(name) => {
                        let key = name.clone();
                        self.advance();
                        key
                    }
                    _ => return Err(FlowError::parser_error("Expected string or identifier as object key")),
                };
                
                self.consume(&Token::Colon, "Expected ':' after object key")?;
                let value = self.expression()?;
                properties.push((key, value));
                
                if self.check(&Token::Comma) {
                    self.advance();
                    // Skip any newlines after the comma
                    while self.check(&Token::Newline) {
                        self.advance();
                    }
                } else {
                    break;
                }
            }
        }
        
        // Skip any newlines before the closing brace
        while self.check(&Token::Newline) {
            self.advance();
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after object properties")?;
        Ok(Expression::Object { properties })
    }
    
    fn is_lambda_expression(&self) -> bool {
        // Look ahead to see if this is a lambda: (params) => expr
        let mut pos = self.current + 1; // Skip the '('
        let mut paren_count = 1;
        
        // Skip to the matching ')'
        while pos < self.tokens.len() && paren_count > 0 {
            match &self.tokens[pos] {
                Token::LeftParen => paren_count += 1,
                Token::RightParen => paren_count -= 1,
                _ => {}
            }
            pos += 1;
        }
        
        // Check if the next token after ')' is '=>'
        pos < self.tokens.len() && matches!(self.tokens[pos], Token::Arrow)
    }
    
    fn parse_lambda(&mut self) -> Result<Expression> {
        self.consume(&Token::LeftParen, "Expected '(' for lambda parameters")?;
        
        let mut parameters = Vec::new();
        
        if !self.check(&Token::RightParen) {
            loop {
                // Check for variadic parameter
                let is_variadic = if self.check(&Token::Ellipsis) {
                    self.advance();
                    true
                } else {
                    false
                };
                
                let param_name = match self.advance() {
                    Token::Identifier(name) => name.clone(),
                    _ => return Err(FlowError::parser_error("Expected parameter name")),
                };
                
                // Check for default value
                let default_value = if self.check(&Token::Assign) {
                    self.advance(); // consume '='
                    Some(self.expression()?)
                } else {
                    None
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    default_value,
                    is_variadic,
                });
                
                if self.check(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        self.consume(&Token::RightParen, "Expected ')' after lambda parameters")?;
        self.consume(&Token::Arrow, "Expected '=>' after lambda parameters")?;
        
        let body = Box::new(self.expression()?);
        
        Ok(Expression::Lambda { parameters, body })
    }
    
    fn consume(&mut self, expected: &Token, message: &str) -> Result<&Token> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(FlowError::parser_error(message))
        }
    }
    
    fn consume_newline(&mut self) -> Result<()> {
        if self.check(&Token::Newline) {
            self.advance();
            Ok(())
        } else {
            Err(FlowError::parser_error("Expected newline"))
        }
    }
    
    fn consume_newline_or_eof(&mut self) -> Result<()> {
        if self.check(&Token::Newline) || self.check(&Token::Eof) {
            if self.check(&Token::Newline) {
                self.advance();
            }
            Ok(())
        } else {
            Err(FlowError::parser_error("Expected newline or end of file"))
        }
    }
    
    fn check(&self, token_type: &Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}