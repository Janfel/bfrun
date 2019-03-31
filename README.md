# bfrun

> A brainfuck interpreter written in Rust.

[![Build Status](https://travis-ci.com/Janfel/bfrun.svg?branch=master)](https://travis-ci.com/Janfel/bfrun)

This program aims to be a straightforward and easily embeddable interpreter for the brainfuck programming language. It is still under development and breaking changes are to be expected.

## About

This program is the result of a school project I did. Therefore I tried to use best coding practices wherever I could, including, but not limited to, the following:

- Consistent use of documentation, especially concerning the public API of my library.
- This README.
- Adherence to the guidelines laid out in the official book [The Rust Programming Language](https://doc.rust-lang.org/book/index.html).
- An easy-to-use command line interface.
- A clear and easily readable git history.

Furthermore, I set the following challenges for myself:

- The use of a low-level language such as Rust.
- Adherence to POSIX conventions.
- Support for shell pipes.
- Support for custom input and output streams for the brainfuck program.
- Embeddability.
- Continuous Integration using [Travis CI](https://travis-ci.com).
- Use of the [Git-flow-Workflow](https://nvie.com/posts/a-successful-git-branching-model/).
- Use of a text editor instead of an IDE.

This repository exists to show that I am capable of writing good code and adhering to common best practices at the same time.

## Installation

```sh
cargo install --git "https://github.com/janfel/bfrun/"
```

## Usage example

```sh
bfrun my_program.b
```

## Development setup

```sh
git clone "https://github.com/janfel/bfrun/"
cd bfrun
cargo build
cargo test
```

## Release History

- 0.2.1
  - ADD: More documentation for the public API
  - REMOVE: Unnecessary module `types.rs`
- 0.2.0
  - ADD: New commandline interface
  - ADD: Support for multiple inputs
  - ADD: Support for stdin
  - ADD: API helper functions
  - CHANGE: API of the internal lib
  - FIX: Number wrapping errors
- 0.1.0
  - Work in progress

## Meta

Jan Felix Langenbach

Distributed under the GPLv3. See ``LICENSE`` for more information.

[https://github.com/janfel/](https://github.com/janfel/)

## Contributing

1. Fork it (<https://github.com/janfel/bfrun/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
