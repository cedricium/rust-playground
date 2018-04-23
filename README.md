# rust-playground :gear: <img src="https://img.shields.io/badge/language-rust-orange.svg"> <img src="https://img.shields.io/badge/license-MIT-blue.svg">

A collection of simple but complete and executable Rust applications. These examples were primarily taken from *[The Book - The Rust Programming Language (2nd ed.)](https://doc.rust-lang.org/book/second-edition/index.html)* which I am reading in order to learn Rust.

## How to use "rust-playground"

> In order to run any of these applications, you will need Rust correctly installed on your computer. To install Rust, [follow the "Installation" instructions outlined here](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html).

To use the repository, first clone it. Each example is in its own top-level directory and contains its own short README explaining what it does.

All of these applications were written using the `stable` build of Rust, and therefore should work across all builds, unless otherwise noted.

With the exception of the `hello_rust/` directory, all applications can be ran using [Cargo](https://doc.rust-lang.org/cargo/), Rust's package manager. Simply `cd` into an application's top-level directory and run `cargo run`.

For example, to run the `guessing_game` application:

```sh
# assuming you're in rust-playground/
$ cd guessing_game/
$ cargo run
Compiling guessing_game v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 3.35 secs
     Running `target/debug/guessing_game`
Guess the number between 0 and 50.
Enter your guess:
…
```

## Problems?

If you find a problem, please [file a bug](https://github.com/cedricium/rust-playground/issues/new).

## Contributing

Contributions are warmly welcomed, whether they are whole new examples, new features,
or bug fixes to existing example code. Simply clone this repository, make your changes, then open a pull-request.

## License

[MIT](https://github.com/cedricium/rust-playground/blob/master/LICENSE.md)
