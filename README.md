### Course Project about Refactoring C library with Rust
#### mycjson
##### Decription
Manually refactor Rust code converted from a c library, CJSON with an automated tool, c2rust.
##### How to use
```
cargo build --release
move the library in target/release
gcc test.c -o test librust_cjson.a -pthread -ldl
./test
```
#### buffer-rust 
##### Decription
Rewrite a tiny c library, buffer.c using Rust and compatible with Rust interface.
##### How to use
```
cargo build --release
cargo run
```
#### buffer
##### Decription
Rewrite a tiny c library, buffer.c using Rust and compatible with c interface.
##### How to use
```
cargo build --release
move the library in target/release
gcc test.c -o test librust_buffer.a -pthread -ldl
./test
```