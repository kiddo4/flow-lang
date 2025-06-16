//! FlowLang - A modern, human-friendly programming language
//!
//! FlowLang is designed to make software development easy for everyone,
//! from absolute beginners to expert developers.
//!
//! # Features
//!
//! - Easy to learn syntax that feels natural
//! - Memory safe and secure by design
//! - Interactive REPL for quick experimentation
//! - Cross-platform support
//!
//! # Example
//!
//! ```rust
//! use flowlang::{lexer::Lexer, parser::Parser, interpreter::Interpreter};
//!
//! let source = r#"
//!     let name be "FlowLang"
//!     show "Hello, " + name + "!"
//! "#;
//!
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.tokenize().unwrap();
//!
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse().unwrap();
//!
//! let mut interpreter = Interpreter::new();
//! interpreter.execute(&ast).unwrap();
//! ```

pub mod ast;
pub mod collections;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod value;
pub mod bytecode;
pub mod bigint;
pub mod optimized_vm;
pub mod jit;
pub mod memory;
pub mod specialized_instructions;
pub mod stdlib;
pub mod stdlib_modules;
pub mod compiler;

pub use ast::*;
pub use error::*;
pub use interpreter::*;
pub use lexer::*;
pub use parser::*;

/// Execute FlowLang source code directly
///
/// This is a convenience function that handles the entire pipeline:
/// lexing, parsing, and interpreting.
///
/// # Arguments
///
/// * `source` - The FlowLang source code as a string
///
/// # Returns
///
/// * `Result<()>` - Ok if execution succeeds, Err with FlowError if it fails
///
/// # Example
///
/// ```rust
/// use flowlang::execute;
///
/// let result = execute(r#"
///     let x be 5
///     let y be 3
///     show x + y
/// "#);
///
/// assert!(result.is_ok());
/// ```
pub fn execute(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast)
}

/// Parse FlowLang source code into an AST without executing it
///
/// This function is useful for syntax checking or AST analysis.
///
/// # Arguments
///
/// * `source` - The FlowLang source code as a string
///
/// # Returns
///
/// * `Result<Program>` - The parsed AST or an error
///
/// # Example
///
/// ```rust
/// use flowlang::parse;
///
/// let ast = parse("let x be 42").unwrap();
/// assert_eq!(ast.statements.len(), 1);
/// ```
pub fn parse(source: &str) -> Result<Program> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Tokenize FlowLang source code
///
/// This function converts source code into a sequence of tokens.
///
/// # Arguments
///
/// * `source` - The FlowLang source code as a string
///
/// # Returns
///
/// * `Result<Vec<Token>>` - The tokenized source or an error
///
/// # Example
///
/// ```rust
/// use flowlang::tokenize;
///
/// let tokens = tokenize("let x be 42").unwrap();
/// assert!(!tokens.is_empty());
/// ```
pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}