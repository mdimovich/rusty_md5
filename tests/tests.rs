use rusty_md5;

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

/**
* For these tests, I'm naming them such that the first portion of each
* test name is the string I'm attempting to test against, unless that is
* impossible (such as empty string), in which case I'm describing what I'm
* testing against instead.
*/
#[test]
fn a_correct_hash() {
    assert_eq!("0cc175b9c0f1b6a831c399e269772661", rusty_md5::md5("a"));
}

#[test]
fn empty_string_correct_hash() {
    assert_eq!("d41d8cd98f00b204e9800998ecf8427e", rusty_md5::md5(""));
}

#[test]
fn abc_correct_hash() {
    assert_eq!("900150983cd24fb0d6963f7d28e17f72", rusty_md5::md5("abc"));
}

#[test]
fn message_digest_correct_hash() {
    assert_eq!(
        "f96b697d7cb7938d525a2f31aaf161d0",
        rusty_md5::md5("message digest")
    );
}

#[test]
fn lowercase_alphabet_correct_hash() {
    assert_eq!(
        "c3fcd3d76192e4007dfb496cca67e13b",
        rusty_md5::md5("abcdefghijklmnopqrstuvwxyz")
    )
}

#[test]
fn all_alphanumeric_correct_hash() {
    let alpha_num_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    assert_eq!(
        "d174ab98d277d9f5a5611c2c9f419d9f",
        rusty_md5::md5(alpha_num_str)
    );
}

#[test]
fn repeated_numbers_correct_hash() {
    let repeated_number_str =
        "12345678901234567890123456789012345678901234567890123456789012345678901234567890";

    assert_eq!(
        "57edf4a22be3c955ac49da2e2107b67a",
        rusty_md5::md5(repeated_number_str)
    );
}
