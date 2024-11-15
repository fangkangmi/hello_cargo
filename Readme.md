# Rust Learning Journey

Welcome to my Rust learning journey! This document serves as a log of my progress and experiences as I delve into the Rust programming language.

## Why Rust?

Rust is known for its performance, reliability, and productivity. It empowers developers to write safe and efficient code, making it an excellent choice for system-level programming.

## Goals

- Understand Rust's ownership and borrowing system
- Master Rust's concurrency model
- Build efficient and safe applications
- Contribute to open-source Rust projects

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [The Rust Programming Language Book - CN](https://kaisery.github.io/trpl-zh-cn/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

## Progress

- [ ] Completed Chapter 1 of The Rust Programming Language Book
- [ ] Solved exercises in Rustlings
- [ ] Built a simple CLI application

Stay tuned for more updates as I continue to explore and learn Rust!


### Cargo
Cargo is Rust's package manager and build system. It simplifies a lot of tasks associated with Rust development, such as managing dependencies, running tests, and building projects. With Cargo, you can easily create new projects, add libraries, and ensure your code is up-to-date with the latest versions of your dependencies.

To check your Cargo version, you can run the following command in your terminal:
```cargo --version```

### Creating a New Project with Cargo

To create a new Rust project using Cargo, you can use the following command:

```sh
cargo new hello_cargo
```

This command will create a new directory called `hello_cargo` with the necessary files and folders for a Rust project. You can navigate into this directory and start building your application:

```sh
cd hello_cargo
```

To build and run your new project, use the following commands:

```sh
cargo build
cargo run
```

For a release build, which is optimized for performance, use:

```sh
cargo build --release
```

This will compile your project and execute the resulting binary, displaying any output from your `main.rs` file.

### Note about Cargo

Cargo also handles running tests for your Rust projects. You can write tests in your `src` files and run them using the following command:

```sh
cargo test
```

This will compile your tests and run them, providing feedback on which tests passed or failed. It's a powerful tool to ensure your code works as expected.

### Variables and Mutability

Constant cannot use mut. Declear const need to use 