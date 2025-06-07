# FlowLang 🌊

A modern, human-friendly, and secure programming language designed to make software development easy for everyone — from absolute beginners to expert developers.

## Features

- **Easy to Learn**: Natural syntax that feels like human thinking
- **Memory Safe**: Built with Rust for security and performance
- **Human-Friendly**: Clean, minimal syntax inspired by natural language
- **Interactive REPL**: Test code snippets instantly
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

### Prerequisites
- Rust 1.70+ installed on your system

### Build from Source

```bash
git clone <repository-url>
cd flowlang
cargo build --release
```

The compiled binary will be available at `target/release/flowlang`.

## Usage

### Running FlowLang Files

```bash
# Run a FlowLang program
./target/release/flowlang program.flow

# Or if installed globally
flowlang program.flow
```

### Interactive REPL

```bash
# Start the interactive REPL
./target/release/flowlang --repl
# or
./target/release/flowlang -r
```

## Language Syntax

### Variables

```flowlang
let name be "Alice"
let age be 25
let isActive be true
let score be 98.7
```

### Functions

```flowlang
def greet with name, age do
    return "Hi, " + name + ", you are " + age + " years old"
end

let message be greet("Bob", 30)
show message
```

### Output

```flowlang
show "Hello, world!"
show name
show age + 5
```

### Conditionals

```flowlang
let age be 18

if age >= 18 then
    show "You can vote"
else
    show "You are too young"
end

# Else-if chains
if score > 80 then
    show "Great"
else if score > 50 then
    show "Average"
else
    show "Poor"
end
```

### Loops

```flowlang
# For loop
for i from 1 to 5 do
    show i
end

# While loop
let i be 0
while i < 5 do
    show i
    let i be i + 1
end
```

### Comments

```flowlang
# This is a comment
let name be "Jane"  # Variable for user's name
```

### Operators

- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Logical**: `and`, `or`, `not`

## Example Programs

### Hello World

```flowlang
show "Hello, FlowLang!"
```

### Calculator

```flowlang
def add with a, b do
    return a + b
end

def multiply with a, b do
    return a * b
end

let result be add(5, 3)
show "5 + 3 = " + result

let product be multiply(4, 7)
show "4 * 7 = " + product
```

### Fibonacci Sequence

```flowlang
def fibonacci with n do
    if n <= 1 then
        return n
    else
        return fibonacci(n - 1) + fibonacci(n - 2)
    end
end

for i from 0 to 10 do
    show "fib(" + i + ") = " + fibonacci(i)
end
```

### FizzBuzz

```flowlang
for i from 1 to 100 do
    if i % 15 == 0 then
        show "FizzBuzz"
    else if i % 3 == 0 then
        show "Fizz"
    else if i % 5 == 0 then
        show "Buzz"
    else
        show i
    end
end
```

## Project Structure

```
flowlang/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lexer.rs         # Tokenization
│   ├── parser.rs        # AST generation
│   ├── interpreter.rs   # Code execution
│   ├── ast.rs          # Abstract syntax tree definitions
│   └── error.rs        # Error handling
├── examples/           # Example FlowLang programs
├── tests/             # Test suite
├── Cargo.toml         # Rust dependencies
└── README.md          # This file
```

## Development

### Running Tests

```bash
cargo test
```

### Development Build

```bash
cargo run -- examples/hello.flow
```

### REPL Development

```bash
cargo run -- --repl
```

## Roadmap

### Current Features (v0.1)
- [x] Basic syntax (variables, functions, conditionals, loops)
- [x] Arithmetic and logical operations
- [x] String manipulation
- [x] Interactive REPL
- [x] Error handling and reporting

### Planned Features
- [ ] Static & dynamic typing support
- [ ] Native async/await
- [ ] Pattern matching
- [ ] Class-based & functional programming
- [ ] Package management system
- [ ] Standard library
- [ ] WebAssembly compilation
- [ ] IDE integration
- [ ] Debugger support

## Contributing

We welcome contributions! Please see our contributing guidelines for more information.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Design Philosophy

FlowLang is inspired by:
- **Python** – Clean and readable syntax
- **Dart** – Modern tooling and package support
- **Rust** – Safety-first, compiled and efficient
- **Swift** – Human-friendly and expressive
- **JavaScript** – Flexibility and ease of use

Our goal is to create the most learnable, buildable, and secure language of the modern era.