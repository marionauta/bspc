//! BSPC command to control BSPWM

extern crate libc;
extern crate unix_socket;

use std::env;
use std::io::{Read, Write};
use std::process;
use unix_socket::UnixStream;

/// Prints error message and quits with error code.
///
/// # Examples
///
/// ```
/// err("Failed to do something important.", 4);
/// ```
fn err(message: &str, code: i32) {
    println!("{}", message);
    process::exit(code);
}

const BUFSIZ: usize = libc::BUFSIZ as usize;

fn main() {
    let message = env::args().skip(1).collect::<Vec<String>>();
    if message.len() < 1 {
        err("No arguments given.", 1);
    }

    let mut i = 0;
    let mut buffer = [0; BUFSIZ];
    for string in message {
        for c in string.chars() {
            buffer[i] = c as u8;
            i += 1;
        }

        buffer[i] = 0;
        i += 1;
    }

    let stream = UnixStream::connect("/tmp/bspwm_0_0-socket");
    if stream.is_err() {
        err("Failed to connect to the socket.", 1);
    }

    let mut stream = stream.unwrap();

    stream.write(&buffer[..i]).unwrap();
    let size = stream.read(&mut buffer).unwrap();
    drop(stream);

    match buffer[0] as i32 {
        1 | 4 => {}
        3 => err("Unknown command.", 3),
        2 => err("Invalid syntax.", 2),
        _ => {
            let message = String::from_utf8_lossy(&buffer[0..size]);
            print!("{}", message);
        }
    }
}
