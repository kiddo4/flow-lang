use flowlang::lexer::Lexer;
use flowlang::parser::Parser;
use flowlang::interpreter::Interpreter;
use flowlang::ast::*;
use flowlang::error::FlowError;

#[test]
fn test_basic_arithmetic() {
    let source = r#"
        let x be 5
        let y be 3
        let result be x + y
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_function_definition_and_call() {
    let source = r#"
        def add with a, b do
            return a + b
        end
        
        let result be add(5, 3)
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_if_statement() {
    let source = r#"
        let x be 10
        if x > 5 then
            let result be "greater"
        else
            let result be "lesser"
        end
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_for_loop() {
    let source = r#"
        let sum be 0
        for i from 1 to 5 do
            let sum be sum + i
        end
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_while_loop() {
    let source = r#"
        let i be 0
        let sum be 0
        while i < 5 do
            let sum be sum + i
            let i be i + 1
        end
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_string_operations() {
    let source = r#"
        let first be "Hello"
        let second be "World"
        let greeting be first + ", " + second + "!"
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_boolean_operations() {
    let source = r#"
        let a be true
        let b be false
        let and_result be a and b
        let or_result be a or b
        let not_result be not a
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_comparison_operations() {
    let source = r#"
        let x be 10
        let y be 5
        let greater be x > y
        let equal be x == y
        let less_equal be x <= y
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast).expect("Execution failed");
}

#[test]
fn test_undefined_variable_error() {
    let source = r#"
        let result be undefined_var + 5
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&ast);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        FlowError::UndefinedVariable { name } => {
            assert_eq!(name, "undefined_var");
        }
        _ => panic!("Expected UndefinedVariable error"),
    }
}

#[test]
fn test_division_by_zero_error() {
    let source = r#"
        let result be 10 / 0
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");
    
    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&ast);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        FlowError::DivisionByZero => {},
        _ => panic!("Expected DivisionByZero error"),
    }
}