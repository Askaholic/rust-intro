// base64.rs
// Rohan Weeden
// Created: June 1, 2018

// Base64 encode some bytes

use std;
use std::io::Write;

static ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

pub fn read_and_encode() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    writeln(&mut stdout.lock(), encode_stream(stdin.lock()).as_bytes());
}

pub fn encode_stream<T>(mut input_stream: T) -> String
    where T: std::io::Read {

    let mut output = String::new();
    let mut buf: [u8; 3] = [0; 3];

    loop {
        let num_read = match input_stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => continue
        };

        encode_chunk(&buf, num_read + 1, &mut output);
        if num_read < buf.len() {
            for _ in 0..3-num_read {
                output += &ALPHABET[64..65];
            }
        }
        clear_buf(&mut buf);
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

fn clear_buf(buf: &mut [u8]) {
    for item in buf.iter_mut() {
        *item = 0;
    }
}

fn writeln(out: &mut std::io::StdoutLock, buf: &[u8]) {
    out.write(&buf)
        .expect("Failed to write to stdout");
    out.write(&[b'\n'])
        .expect("Failed to write to stdout");
}
