# Mahampreter: A Minimalistic LISP Interpreter

Welcome to Mahampreter, a lightweight and user-friendly LISP interpreter
designed for educational purposes and rapid prototyping. The primary goal of
Mahampreter is to provide a simple yet powerful environment for those who are
eager to learn the fundamentals of LISP programming language and its
concepts.

## Features
- Minimalistic and easy-to-understand Rust codebase
- Support for fundamental LISP operations and data structures
- Interactive REPL (Read-Eval-Print Loop) for quick experimentation
- Extensible and customizable design for further development
- Available anywhere Rust is (MacOS, Linux, Windows, FreeBSD, etc.)

## Getting Started

### Prerequisites
To get started with Mahampreter, you'll need to have Rust installed.

### Building Mahampreter
To build Mahampreter, follow these steps:

1. Clone the repository:
```
git clone https://github.com/aidenfoxivey/mahampreter.git
```

2. Change to the project directory:
```
cd mahampreter
```

3. Build the project with `cargo`:
```
cargo build --release
```
It will automatically pull all of the dependencies.

### Running Mahampreter
To start the Mahampreter REPL, simply execute the compiled binary:

```
./mahampreter
```

Now you can start experimenting with LISP expressions!

## Example Usage
Here's a simple example of using Mahampreter to perform basic arithmetic operations:

```
> (+ 1 2 3)
6
> (* 2 3)
6
> (/ 9 3)
3
```

You can also use Mahampreter to define functions and work with more complex expressions:

```
> (define square (lambda (x) (* x x)))
> (square 5)
25
> (define sum-of-squares (lambda (x y) (+ (square x) (square y))))
> (sum-of-squares 3 4)
25
```

## License
Mahampreter is released under the MIT License. For more information, please see the [LICENSE](LICENSE) file.