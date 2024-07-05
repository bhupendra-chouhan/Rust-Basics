#### What is Rust?
- Statically typed language
- Systems programming language (lies betweeen LLP and HLP)
- No Garbage Collection -> Memory Management is Deterministic -> Predictible performance optimization and output

#### Rust documentation resources:
- The Rust book: https://doc.rust-lang.org/book/
- Rust by example: https://doc.rust-lang.org/rust-by-example/hello/print.html
- Rustlings: https://github.com/rust-lang/rustlings

#### Install Rust using command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Run Rust program using command:
```
<!-- To compile rust program -->
rustc filename.rs

<!-- To execute the executable file -->
./filename.exe
```
---
## Cargo package manager:

- Used to build Rust projects
- Help you easily use and mange Rust basd dependency/packages

#### Create a Project with Cargo:
```
cargo new project_name
cd project_name
```

#### Build and run a Cargo project:
```
cargo run
```
---


## The Rust Programming Language:

#### Data Types
Two types of data types:
- Four Scalar Types: Store Single Values
    - Signed & Unsigned Integers
    - Floating-Point Numbers
    - Booleans
    - Characters

- Compound Types: Store Multiple Values
    - Arrays
    - Vectors
    - Strings
    - Tuples
    - Maps
---


#### Writing Soroban Smartcontracts:
- #![no_std] : This Used to prevent rust from loading the standard library.




