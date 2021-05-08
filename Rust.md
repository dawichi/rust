# Rust

This is a summary of the official [Rust docs](https://doc.rust-lang.org/book/title-page.html) for personal use.

[TOC]

## 1.  Instaling Rust

The easiest way to install Rust is use `rustup`, a command line tool to manage Rust versions and associated tools.

`````bash
# Linux / Mac / WSL
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ rustc --version
`````

This provides usefull tools to update or uninstall Rust for example

````bash
$ rustup update
$ rustup self uninstall
````

And it comes with a copy of the docs

````bash
$ rustup doc
````



## 2. Hello World

Create a `hello.rs` file and place this code inside:

````rust
fn main() {
    println!("Hello, world!");
}
````

As on C or C++, compiling and running are separate steps. First you need to compile the source Rust code to a executable binary. And then you will be able to execute the binary in your operative system. This is done with:

````bash
$ rustc main.rs
````

After that, if the compilation is successful, you'll be able to see the output in a new file:

````bash
$ ls
	hello
	hello.rs
````

If we execute the file `hello` with `./hello`, the program will be executed and we will get a printed line with our string "Hello, world!"

This behavior, the compiling and then running the file, could feels strange to programmers coming from dynamic languages such as Ruby, Python or JavaScript. The pros of the separate steps is that you can compile the code and then execute it in any other device without need Rust installed, meanwhile you need always python or node to execute .py or .js code. But the cons is that extra step you must do to run your code.



## 3. Cargo

Rust provides a build system and package manager called **Cargo**. Mostly projects use it.

````bash
# check cargo version and create a new project
$ cargo --version
$ cargo new project_name
````

This creates a basic project structure

````bash
$ tree project_name
project_name/
├── Cargo.toml
└── src
    └── main.rs
````

* `Cargo.toml`

This file is in [*TOML*](https://toml.io/) (*Tom’s Obvious, Minimal Language*) format. 

````rust
[package]
name = "project_name"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
````



Running the cargo project

```bash
# Compile
$ cargo build

# Checks if it compiles, but without building the output (faster than 'build')
$ cargo check

#  Compile and run the output
$ cargo run
```

Cargo will output the result into a `target/` directory instead of the current one. So our small cargo project, if we run `cargo build` will look like:

````bash
$ cargo build
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        │   ├── project_name-8cdecc78e28a704f
        │   └── project_name-8cdecc78e28a704f.d
        ├── examples
        ├── incremental
        │   └── project_name-2urmu7x7zmtlz
        │       └── (...more code) 
        ├── project_name
        └── project_name.d
````

So our executable will be placed in `./target/project_name` 



Production build: to compile with optimizations for production, run:

````bash
$ cargo build --release
````





## 4. Learn more: Rustlings

[rust-lang/rustlings: Small exercises to get you used to reading and writing Rust code! (github.com)](https://github.com/rust-lang/rustlings)







