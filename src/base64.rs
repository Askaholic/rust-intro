// base64.rs
// Rohan Weeden
// Created: June 1, 2018

// Base64 encode some bytes

use std;
use std::io::prelude::Read;
use std::io::Write;

static ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

pub fn read_and_encode() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut buf = Vec::new();
    let mut input_stream = stdin.lock();
    input_stream.read_to_end(&mut buf)
        .expect("Failed to read from stdin");

    writeln(&mut stdout.lock(), encode(&buf).as_bytes());
}

pub fn encode(input: &[u8]) -> String {
    // let mut output = String::with_capacity();
    let mut output = String::new();
    // Break into 3 byte chunks
    let mut i: usize = 0;
    while i < input.len() {
        if i+3 <= input.len() {
            encode_chunk(&input[i..i+3], 4, &mut output);
        }
        else {
            let num_bytes = input.len() % 3;
            let mut chunk = [0; 4];

            copy_to_chunk(&mut chunk, &input[i..input.len()]);
            encode_chunk(&chunk, num_bytes + 1, &mut output);

            for _ in 0..3-num_bytes {
                output += &ALPHABET[64..65];
            }
        }
        i += 3;
    }
    output
}

fn encode_chunk(chunk: &[u8], num_bytes: usize, output: &mut String) {
    for c in 0..num_bytes {
        let ind = six_bit_index(&chunk, c);
        *output += &ALPHABET[ind..ind+1];
    }
}

fn six_bit_index(chunk: &[u8], index: usize) -> usize {
    match index {
        0 => (chunk[0] as usize & 0xFC) >> 2,
        1 => ((chunk[0] as usize & 0x3) << 4 | (chunk[1] as usize & 0xF0) >> 4),
        2 => ((chunk[1] as usize & 0xF) << 2 | (chunk[2] as usize & 0xC0) >> 6),
        3 => (chunk[2] as usize & 0x3F),
        _ => 64
    }
}

fn copy_to_chunk(chunk: &mut [u8], input: &[u8]) {
    for c in 0..input.len() {
        chunk[c] = input[c];
    }
}

fn writeln(out: &mut std::io::StdoutLock,buf: &[u8]) {
    out.write(&buf)
        .expect("Failed to write to stdout");
    out.write(&[b'\n'])
        .expect("Failed to write to stdout");
}
