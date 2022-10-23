/*
open a port and wait to receive data
when data is received, convert it to a string and print it

the program has two modes,
passive listener mode, and sender mode
the mode will be determined by the command line arguments
*/

use clap::Parser;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// parse the command line arguments
//  -p, --port <port>    port to listen on or connect to
//  -s, --send <ip>      ip address to send data to
//  -h, --help           Prints help information
//  -V, --version        Prints version information
//  -l, --listen         listen for incoming connections
#[derive(Parser)]
#[command(name = "file-mail-box")]
#[command(version = "0.1.0")]
#[command(about = "send and receive like a retard", long_about = None)]
struct Cli {
    #[arg(short, long)]
    listen: bool,
    #[arg(short, long, default_value_t = ("127.0.0.1").to_string())]
    address: String,
    #[arg(short, long, default_value_t = ("8080").to_string())]
    port: String,
}

fn main() {
    let cli = Cli::parse();
    let address: String = format!("{}:{}", cli.address, cli.port);
    if cli.listen {
        let listener = TcpListener::bind(&address).unwrap();
        println!("Listening on {}", &address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    } else {
        let stream = TcpStream::connect(&address);
        match stream {
            Ok(mut stream) => {
                println!("Sending to {}", &address);
                let mut total_bytes_sent = 0;
                loop {
                    let message = "dog";
                    let sent = stream.write(message.as_bytes());
                    match sent {
                        Ok(num_bytes_sent) => {
                            total_bytes_sent += num_bytes_sent;
                        }
                        Err(e) => {
                            println!("total bytes sent: {}", total_bytes_sent);
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // let mut total_num_bytes_received = 0;
    loop {
        let mut buffer = [0; 512];
        let num_bytes_received = stream.read(&mut buffer).unwrap();
        if num_bytes_received > 0 {
            println!("{}", String::from_utf8_lossy(&buffer[..]));
        }
    }
}