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


## 🔍 Basic Overview of this MD5 Implementation

1. Take in a command line string and convert into a series of unsigned 8 bit ints (`Vec<u8>`).
2. Pad the message with a single 1, followed by `n` 0's, such that `message length in bits % 512 = 448`.
3. Pad the last 64 bits with a little-endian representation of the original message length.
4. Convert our vector of unsigned 8-bit integers into blocks of 32 bit unsigned ints (`u32`).
5. Using the 4 auxiliary functions described in rfc 1321, begin a series of 64 bitwise operations on each 512 bit
   block of our message.
6. Once we are done, combine our 4 32 bit registers used during our bitwise operations to retrieve our message digest.
   
   (This is a simplification, but describes the core functionality)
