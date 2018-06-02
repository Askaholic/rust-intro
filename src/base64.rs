// base64.rs
// Rohan Weeden
// Created: June 1, 2018

// Base64 encode some bytes

use std;
use std::io::prelude::Read;
use std::io::Write;

pub fn read_and_encode() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut buf = Vec::new();
    let mut input_stream = stdin.lock();
    input_stream.read_to_end(&mut buf)
        .expect("Failed to read from stdin");

    let mut output_stream = stdout.lock();
    output_stream.write(encode(&buf).as_bytes())
        .expect("Failed to write to stdout");
    output_stream.write(&[b'\n'])
        .expect("Failed to write to stdout");
}

pub fn encode(input: &Vec<u8>) -> std::borrow::Cow<str> {
    String::from_utf8_lossy(input)
}
