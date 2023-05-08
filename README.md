# mahampreter

Welcome to mahampreter, a lightweight and user-friendly Scheme interpreter
designed for educational purposes.

This, for me at least, is *baby's first interpreter*. I have zero formal training in building 
interpreters and I have a questionable understanding of computer science. Please do
email me at aidenfoxivey@disroot.org if you have criticism. Alternatively, open an issue or 
"toot" me [on Mastodon](https://mastodon.social/@aidenfoxivey). Sorry, it bothers me too!

More specifically, mahampreter implements the version of Scheme R7RS-small standard,
which can be most thoroughly understood through reading [the formal specification][1].

## Features
- [ ] Full support for Scheme R7RS-small standard (not yet)
- [x] Interactive REPL (Read-Eval-Print Loop) for quick experimentation
- [x] Fancy shmancy cat face emoji prompt!

## Getting Started

### Prerequisites
To get started with mahampreter, you'll need to have Rust installed.
If you're unaware of how to install Rust, [go here][2]. If you're unwilling
to install Rust, then maybe ask a trusted computer wizard friend to run this code
for you? That's my best advice at least.

### Building mahampreter
To build mahampreter, follow these steps in your local terminal:

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

### Running mahampreter
To start the mahampreter REPL, simply execute the compiled binary:

```
./mahampreter
```

## Example Usage
Here's a simple example of using mahampreter to perform basic arithmetic:

```
> (+ 1 (2 3))
6
> (* 2 3)
6
> (/ 9 3)
3
```

You can also use mahampreter to define functions and work with more complex expressions:

```
> (define square (lambda (x) (* x x)))
> (square 5)
25
> (define sum-of-squares (lambda (x y) (+ (square x) (square y))))
> (sum-of-squares 3 4)
25
```

## License
mahampreter is released under the MIT License. For more information, please see the [LICENSE](LICENSE) file.


[1]: https://small.r7rs.org/attachment/r7rs.pdf
[2]: https://www.rust-lang.org/learn/get-started