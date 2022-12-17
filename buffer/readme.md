### Course Porject
Rewrite a tiny c library, buffer.c using Rust and compatible with c interface
### How to use
#### c
```
gcc test.c buffer.c -o test
./test
```
#### rust
```
gcc test.c -o test librust_buffer.a -pthread -ldl
./test
```
