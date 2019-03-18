# bfrun

> A brainfuck interpreter written in Rust.

[![Build Status](https://travis-ci.com/Janfel/bfrun.svg?branch=master)](https://travis-ci.com/Janfel/bfrun)

This program aims to be a straightforward interpreter for the brainfuck programming language. It is still under development and breaking changes are to be expected.

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

* 0.1.0
  * Work in progress

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

<!-- Markdown link & img dfn's
[travis-image]: https://img.shields.io/travis/dbader/node-datadog-metrics/master.svg?style=flat-square
[travis-url]: https://travis-ci.org/dbader/node-datadog-metrics
-->
