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
    #[clap(short = 'c', long = "ca")]
    ca: String,
    #[clap(short = 'p', long = "port")]
    port: u16,
    #[clap(short = 'f', long = "cmd-file")]
    cmd_file: Option<std::path::PathBuf>,
}

struct InternalCmd {
    key: String,
    value: String,
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

fn parse_internal_cmd(cmd: &String, mut parsed_cmd: &mut InternalCmd) -> bool
{
    let cmd = cmd.to_lowercase();
    // Format is: !key!value
    if cmd.starts_with("!") && cmd.ends_with("!") {
        let cmd = cmd.trim_start_matches("!");
        let cmd = cmd.trim_end_matches("!");
        let cmd: Vec<&str> = cmd.split("!").collect();
        if cmd.len() == 2 {
            parsed_cmd.key = cmd[0].to_string();
            parsed_cmd.value = cmd[1].to_string();
            return true;
        }
    }

    return false;
}

fn process_internal_cmd(cmd: &String) -> bool
{
    let mut int_cmd: InternalCmd = InternalCmd {
        key: String::new(),
        value: String::new(),
    };

    let parsed: bool = parse_internal_cmd(&cmd, &mut int_cmd);

    if !parsed {
        return false;
    }

    log::debug!("Internal command: {} {}", int_cmd.key, int_cmd.value);
    match  int_cmd.key.as_str() {
        "sleep" => {
            println!("Internal command: Sleeping for {} seconds\n", int_cmd.value);
            let sleep_time: u64 = int_cmd.value.parse().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(sleep_time));
            return true;
        }
        _ => {
            log::warn!("Unknown internal command: {}", int_cmd.key);
            return false;
        }
    }
}


fn send_one_cmd(mut stream: TcpStream, input: &String)
{
       // Append 3 dummy bytes to the end of the input (required by CA)
       let input = format!("{}{}", input, "   ");
       let mut resp = [0; 1024];
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


fn interactive_cli(stream: TcpStream)
{
   // Open an interactive prompt in a loop
   let cmd = Text::new("Enter command: ");
   // Send command to CA, if exit or ctrl-c break
   loop {
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

       let int_cmd: bool = process_internal_cmd(&input);
       if int_cmd {
         continue;
       }
       send_one_cmd(stream.try_clone().unwrap(), &input);
   }
}

fn file_input_cli(stream: TcpStream, file: Option<std::path::PathBuf>)
{
    let input ;
    match &file {
        Some(file) => {
            input = std::fs::read_to_string(file);
            match input {
                Ok(_) => {}
                Err(e) => {
                    println!("Error reading file: {}", e);
                    return;
                }
            }
        }
        None => {
            println!("No file specified");
            return;
        }
    }

    // Read each line from the file and send to CA
    for line in input.unwrap().lines() {
        let int_cmd: bool = process_internal_cmd(&line.to_string());
        if int_cmd {
            continue;
        }
        send_one_cmd(stream.try_clone().unwrap(), &line.to_string());
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

    if args.cmd_file == None {
        interactive_cli(stream);
    } else {
        file_input_cli(stream, args.cmd_file);
    }

}
