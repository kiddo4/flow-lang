use clap::{Arg, Command};
use colored::*;
use std::fs;
use std::path::Path;

mod ast;
mod lexer;
mod parser;
mod interpreter;
mod error;
mod collections;
mod bytecode;
mod compiler;
mod stdlib;
mod value;
mod bigint;
mod optimized_vm;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::bytecode::VirtualMachine;
use crate::compiler::compile_program;
use crate::stdlib::StandardLibrary;
use crate::error::FlowError;

fn main() {
    let matches = Command::new("FlowLang")
        .version("0.1.0")
        .author("FlowLang Team")
        .about("A modern, human-friendly programming language")
        .arg(
            Arg::new("file")
                .help("The FlowLang source file to execute")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("repl")
                .short('r')
                .long("repl")
                .help("Start interactive REPL mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("vm")
                .long("vm")
                .help("Use bytecode virtual machine (default: tree-walking interpreter)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let use_vm = matches.get_flag("vm");

    if matches.get_flag("repl") {
        start_repl();
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        execute_file(file_path, use_vm);
    } else {
        println!("{}", "Welcome to FlowLang!".bright_blue().bold());
        println!("Usage: flowlang <file.flow> [--vm] or flowlang --repl");
    }
}

fn execute_file(file_path: &str, use_vm: bool) {
    if !Path::new(file_path).exists() {
        eprintln!("{}: File '{}' not found", "Error".red().bold(), file_path);
        return;
    }

    let source = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{}: Failed to read file '{}': {}", "Error".red().bold(), file_path, err);
            return;
        }
    };

    if use_vm {
        execute_source_vm(&source, file_path);
    } else {
        execute_source(&source, file_path);
    }
}

fn execute_source(source: &str, file_name: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}: {}", "Lexer Error".red().bold(), err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}: {}", "Parser Error".red().bold(), err);
            return;
        }
    };

    let mut interpreter = Interpreter::new();
    if let Err(err) = interpreter.execute(&ast) {
        eprintln!("{}: {}", "Runtime Error".red().bold(), err);
    }
}

fn execute_source_vm(source: &str, file_name: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}: {}", "Lexer Error".red().bold(), err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}: {}", "Parser Error".red().bold(), err);
            return;
        }
    };

    let chunk = match compile_program(&ast.statements) {
        Ok(chunk) => chunk,
        Err(err) => {
            eprintln!("{}: {}", "Compiler Error".red().bold(), err);
            return;
        }
    };

    let mut vm = VirtualMachine::new();
    vm.load_chunk(chunk);
    if let Err(err) = vm.run() {
        eprintln!("{}: {}", "VM Error".red().bold(), err);
    }
}

fn start_repl() {
    println!("{}", "FlowLang REPL v0.1.0".bright_blue().bold());
    println!("Type 'exit' to quit\n");

    let mut interpreter = Interpreter::new();
    
    loop {
        print!("{} ", "flow>".bright_green().bold());
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "exit" {
                    println!("Goodbye!");
                    break;
                }
                if input.is_empty() {
                    continue;
                }

                execute_repl_line(input, &mut interpreter);
            }
            Err(err) => {
                eprintln!("{}: {}", "Input Error".red().bold(), err);
                break;
            }
        }
    }
}

fn execute_repl_line(input: &str, interpreter: &mut Interpreter) {
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            return;
        }
    };

    if let Err(err) = interpreter.execute(&ast) {
        eprintln!("{}: {}", "Error".red().bold(), err);
    }
}