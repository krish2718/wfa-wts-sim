// Copyright (c) 2023 Chaitanya Tata
// License: MIT
use clap::Parser;
use env_logger;
use inquire::Text;
use log;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

const CONN_TIMEOUT_S: u64 = 5;
const RESP_TIMEOUT_S: u64 = 30;

/// IP address and port of CA
#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    ca: String,
    #[clap(short, long)]
    port: u16,
}

/// Connect to CA and return TcpStream
fn connect_to_ca(ca: String, port: u16) -> Result<TcpStream, std::io::Error> {
    let addr = format!("{}:{}", ca, port);
    // Connect to CA with timeout
    let stream = TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        std::time::Duration::from_secs(CONN_TIMEOUT_S),
    );
    return stream;
}

fn interactive_cli(mut stream: TcpStream)
{
   // Open an interactive prompt in a loop
   let cmd = Text::new("Enter command: ");
   // Send command to CA, if exit or ctrl-c break
   loop {
       let mut resp = [0; 1024];
       let input = cmd.clone().prompt();
       match input {
           Ok(_) => {}
           Err(e) => {
               println!("Error reading input: {}", e);
               return;
           }
       }
       let input = input.unwrap();
       log::debug!("Input: {}", input);
       if input.to_lowercase() == "exit" {
           break;
       }
       // Append 3 dummy bytes to the end of the input (required by CA)
       let input = format!("{}{}", input, "   ");
       let bytes_sent = stream.write(input.as_bytes()).unwrap();
       log::debug!("Bytes sent: {}", bytes_sent);

       loop {
           let bytes_read = stream.read(&mut resp);
           match bytes_read {
               Ok(bytes_read) => {
                   if bytes_read == 0 {
                       println!("Connection closed by CA");
                       return;
                   }
               }
               Err(e) => {
                   println!("Error reading from CA: {}", e);
                   return;
               }
           }
           let bytes_read = bytes_read.unwrap();
           log::debug!("Bytes read: {}", bytes_read);
           let resp = String::from_utf8_lossy(&resp);
           println!("Response: {}", resp);
           if resp.contains("COMPLETE") {
               break;
           }
       }
   }
}

fn main() {
    env_logger::init();

    let args = Cli::parse();
    println!("CA: {}: {}", args.ca, args.port);

    let stream = connect_to_ca(args.ca, args.port);
    match stream {
        Ok(_) => {
            println!("Connected to CA");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
            return;
        }
    }
    let stream = stream.unwrap();
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(RESP_TIMEOUT_S)))
        .unwrap();

    interactive_cli(stream);

}
