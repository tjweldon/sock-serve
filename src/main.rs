extern crate core;

use std::io::prelude::*;
use std::env;
use std::str;
use std::io::{self, Write};
use std::os::unix::net::{UnixListener, UnixStream};

const ADDRESS: &str = "/tmp/rst.sock";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let subcommand = &args[1];

    if subcommand.to_owned() == "client".to_owned() {
        client()
    } else if subcommand.to_owned() == "server".to_owned() {
        server()
    } else {
        Ok(())
    }
}

fn server() -> io::Result<()> {
    let listener = UnixListener::bind(ADDRESS)?;

    match listener.accept() {
        Ok((mut socket, addr)) => {
            println!("Got a client: {:?} - {:?}", socket, addr);
            let mut response = [0; 128];

            loop {
                socket.read(&mut response)?;
                if response.len() > 0 {
                    let output = match str::from_utf8(response.as_slice()) {
                        Ok(s) => s.to_owned(),
                        Err(_) => String::new()
                    };
                    print!("{}", output)
                }
                response = [0; 128];
            }
        },
        Err(e) => println!("accept function failed: {:?}", e),
    }
    Ok(())
}

fn get_socket() -> io::Result<UnixStream> {
    UnixStream::connect(ADDRESS)
}

fn client() -> io::Result<()> {
    let mut socket = get_socket()?;

    loop {
        let out = match read_stdin() {
            Ok(buf) => buf,
            Err(_) => String::new()
        };
        socket.write_all(out.as_ref())?;
    }
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}