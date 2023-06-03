<h1 align="center">🦀 rusty md5 </h1>

## 📝 Description
This is a from scratch implementation of the md5 message digest algorithm using
rust. 

The purpose of this code is to act as a learning resource
for the md5 message digest algorithm. I wanted to provide
a straightforward walk-through of the md5 algorithm using
Rust.

This code was implemented using the original md5 message digest
RFC, written by R. Rivest iin April 1992.

For more information about the md5 algorithm, or the original C
source code implementation, please reference RFC 1321.

*(rfc source: https://datatracker.ietf.org/doc/html/rfc1321)*


## ⚙️ Build/Run Locally
1. download the repo
2. run `cargo build` inside the project
3. run `cargo run <string_to_hash>` to get a md5 
   message digest of a string
4. run `cargo test` to run unit tests
