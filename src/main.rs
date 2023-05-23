// Copyright (c) 2023 Chaitanya Tata
// SPDX-License-Identifier: MIT

use env_logger;
use wfa_wts_sim::{parse_args, connect_to_ca, interactive_cli, file_input_cli};

fn main() {
    env_logger::init();

    let args = parse_args();

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
    if args.cmd_file == None {
        interactive_cli(stream);
    } else {
        file_input_cli(stream, args.cmd_file);
    }
}
