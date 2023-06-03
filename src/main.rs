/**
*
* The MD5 Message-Digest Algorithm, implemented in Rust.
* implemented by michael dimovich (@mdimovich)
*
* The purpose of this code is to act as a learning resource
* for the md5 message digest algorithm. I wanted to provide
* a straightforward walk-through of the md5 algorithm using
* Rust.
*
* This code was implemented using the original md5 message digest
* RFC, written by R. Rivest iin April 1992.
*
* For more information about the md5 algorithm, or the original C
* source code implementation, please reference RFC 1321.
*
* (source: https://datatracker.ietf.org/doc/html/rfc1321)
*
*/

/*
MD5 Test Suite From RFC Implementation
MD5 ("") = d41d8cd98f00b204e9800998ecf8427e
MD5 ("a") = 0cc175b9c0f1b6a831c399e269772661
MD5 ("abc") = 900150983cd24fb0d6963f7d28e17f72
MD5 ("message digest") = f96b697d7cb7938d525a2f31aaf161d0
MD5 ("abcdefghijklmnopqrstuvwxyz") = c3fcd3d76192e4007dfb496cca67e13b
MD5 ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789") =
d174ab98d277d9f5a5611c2c9f419d9f
MD5 ("123456789012345678901234567890123456789012345678901234567890123456
78901234567890") = 57edf4a22be3c955ac49da2e2107b67a
*/

fn remove_first(vec: &mut Vec<String>) -> Option<String> {
    if vec.is_empty() {
        return None;
    }
    Some(vec.remove(0))
}

fn main() {
    let mut input_args: Vec<String> = std::env::args().collect();

    remove_first(&mut input_args);

    // join multi-word strings together by spaces
    let input = input_args.join(" ");

    let result = rusty_md5::md5(input.as_str());
    println!("{}", result);
}
