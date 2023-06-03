/**
*
* rusty_md5
*
* The MD5 Message-Digest Algorithm, written in Rust.
* Implemented by michael dimovich (@mdimovich)
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
