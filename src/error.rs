use thiserror::Error;
use crate::value::Value;

#[derive(Error, Debug, Clone)]
pub enum FlowError {
    #[error("Lexer error at line {line}, column {column}: {message}")]
    LexerError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Parser error at line {line}: {message}")]
    ParserError { line: usize, message: String },

    #[error("Runtime error at line {line}: {message}")]
    RuntimeError { line: usize, message: String },

    #[error("Type error: {message}")]
    TypeError { message: String },

    #[error("Variable '{name}' is not defined")]
    UndefinedVariable { name: String },

    #[error("Function '{name}' is not defined")]
    UndefinedFunction { name: String },

    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Index out of bounds: {index}")]
    IndexOutOfBounds { index: i64 },

    #[error("IO error: {message}")]
    IoError { message: String },
    
    #[error("Return: {value:?}")]
    Return { value: Value },
}

impl FlowError {
    pub fn lexer_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        FlowError::LexerError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn parser_error(message: impl Into<String>) -> Self {
        FlowError::ParserError {
            line: 0, // Default to 0 when line info not available
            message: message.into(),
        }
    }

    pub fn parser_error_at_line(line: usize, message: impl Into<String>) -> Self {
        FlowError::ParserError {
            line,
            message: message.into(),
        }
    }

    pub fn runtime_error(message: impl Into<String>) -> Self {
        FlowError::RuntimeError {
            line: 0, // Default to 0 when line info not available
            message: message.into(),
        }
    }

    pub fn runtime_error_at_line(line: usize, message: impl Into<String>) -> Self {
        FlowError::RuntimeError {
            line,
            message: message.into(),
        }
    }

    pub fn type_error(message: impl Into<String>) -> Self {
        FlowError::TypeError {
            message: message.into(),
        }
    }

    pub fn undefined_variable(name: impl Into<String>) -> Self {
        FlowError::UndefinedVariable {
            name: name.into(),
        }
    }

    pub fn undefined_function(name: impl Into<String>) -> Self {
        FlowError::UndefinedFunction {
            name: name.into(),
        }
    }

    pub fn invalid_operation(message: impl Into<String>) -> Self {
        FlowError::InvalidOperation {
            message: message.into(),
        }
    }

    pub fn compilation_error(message: &str) -> Self {
        FlowError::RuntimeError {
            line: 0,
            message: format!("Compilation error: {}", message),
        }
    }
    
    pub fn return_value(value: Value) -> Self {
        FlowError::Return { value }
    }
}

impl From<std::io::Error> for FlowError {
    fn from(error: std::io::Error) -> Self {
        FlowError::IoError {
            message: error.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, FlowError>;