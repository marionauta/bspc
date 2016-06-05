//! BSPC command to control BSPWM

extern crate libc;
extern crate unix_socket;

use libc::{EXIT_FAILURE, EXIT_SUCCESS};
use std::env;
use std::io::{Read, Write};
use std::process;
use unix_socket::UnixStream;

const BUFSIZ: usize = libc::BUFSIZ as usize;
const FAILURE_MESSAGE: i32 = 7;
const SOCKET_ENV_VAR: &'static str = "BSPWM_SOCKET";

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

fn socket_file() -> String {
    format!("/tmp/bspwm{}_{}_{}-socket", "", 0, 0)
}

fn main() {
    let mut i = 0;
    let mut buffer = [0; BUFSIZ];

    // We add all the command line arguments to 'buffer', separated by a null
    // character (\0). At the end, 'i' will store the buffer's length, so we
    // can send it (and nothing more) to the socket stream.
    //
    // We skip the first argument because in most platforms it contains the
    // executable's name.
    for string in env::args().skip(1) {
        for c in string.chars() {
            buffer[i] = c as u8;
            i += 1;
        }

        buffer[i] = 0;
        i += 1;
    }

    if i == 0 {
        err("No arguments given.", 1);
    }

    // An environment variable can be set by bspwm (or the user) to tell bspc
    // where the socket file is. If the variable is set, use it. If not, use
    // the new scheme.
    let stream_file = env::var(SOCKET_ENV_VAR)
                          .unwrap_or_else(|_| socket_file());

    let mut stream = {
        let maybe_stream = UnixStream::connect(stream_file);

        if let Err(_) = maybe_stream {
            err("Failed to connect to the socket.", 1);
        }

        maybe_stream.unwrap()
    };

    stream.write(&buffer[..i]).unwrap();
    let size = stream.read(&mut buffer).unwrap();
    drop(stream);

    // Force a null character at the end of the message.
    buffer[size] = 0;

    let (status, offset) = match buffer[0] as i32 {
        FAILURE_MESSAGE => (EXIT_FAILURE, 1),
        _ => (EXIT_SUCCESS, 0),
    };

    let message = String::from_utf8_lossy(&buffer[offset..size]);
    print!("{}", message);

    process::exit(status);
}
