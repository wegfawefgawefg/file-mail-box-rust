/*
as listener: read in bytes, then write to file, and go back to waiting for more connections
as sender: open a file, read in bytes, send to listener, then close connection on completion
*/

use clap::Parser;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

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
        listen(address);
    } else {
        send(address);
    }
}

fn handle_listen_client(mut stream: TcpStream) {
    let mut total_num_bytes_received = 0;
    loop {
        let mut buffer = [0; 512];
        let num_bytes_received = stream.read(&mut buffer).unwrap();
        total_num_bytes_received += num_bytes_received;
        // quit on rcv 10 mb
        if total_num_bytes_received > (usize::pow(1024, 2) * 10) {
            break;
        }
        // if num_bytes_received > 0 {
        //     println!("{}", String::from_utf8_lossy(&buffer[..]));
        // }
    }
}

fn listen(address: String) {
    let listener = TcpListener::bind(&address);
    match listener {
        Ok(listener) => {
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
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn send(address: String) {
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
