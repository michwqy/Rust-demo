## Course Project about Refactoring C library with Rust
### 1 mycjson
#### Decription
Manually refactor Rust code translated from a C library, *CJSON* with an automated tool, *c2rust*.
#### How to use
```
cargo build --release
move the library in target/release
gcc test.c -o test librust_cjson.a -pthread -ldl
./test
```
### 2 buffer-rust 
#### Decription
Rewrite a tiny C library, *Buffer* using Rust and compatible with Rust interface. The Rust code has the same semantics as the original C library, but it cannot be called in C code.
#### How to use
```
cargo build --release
cargo run
```
### 3 buffer
#### Decription
Rewrite a tiny C library, *Buffer* using Rust and compatible with C interface. The Rust code has the same semantics as the original C library, and it can be called in C code.
#### How to use
```
cargo build --release
move the library in target/release
gcc test.c -o test librust_buffer.a -pthread -ldl
./test
```
