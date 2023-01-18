mod libs;

use crate::libs::utils;
use std::{env, collections::HashMap};
use exitcode::{self, ExitCode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        exit(exitcode::USAGE, true, "");
    }

    let second_arg: &str = args[1].trim();

    if !is_command(&second_arg) {
        
        if second_arg == "--help" || second_arg == "-h" {
            exit(exitcode::OK, true, "");
        }

        if second_arg == "--version" || second_arg == "-v" {
            exit(exitcode::OK, false, &format!("dontdie {}", env!("CARGO_PKG_VERSION")))
        }
        
        exit(exitcode::DATAERR, false, "> Invalid parameter, please read help");
    }

    if second_arg == "heartbeat" {

        if args.len() <= 2 {
            exit(exitcode::USAGE, true, "");
        }

        let third_arg: &str = args[2].trim();

        if is_command(&third_arg) {

            if third_arg == "config" {

            }
            
            if third_arg == "daemon" {

            }

            if third_arg == "send" {
                
                // TODO: remove this for final version
                let mut has_params = false;

                if args.len() >= 4 {
                    
                    let send_parameters = get_send_parameters(&args[3..]);

                    if send_parameters.is_empty() {
                        exit(exitcode::DATAERR, false, "> Invalid parameters for send command, please read help");
                    }

                    // TODO: Inject parameters
                    println!("> Sending... parameters are checked and correct");
                    has_params = true;
                }

                // Do send
                println!("> Sending heartbeat, has params {}", has_params);
                exit(exitcode::OK, false, &format!("> Sending heartbeat, has params {}", has_params));
            }

            exit(exitcode::DATAERR, false, "> Command for heartbeat not found, please read help");
        }

        exit(exitcode::DATAERR, false, "> Heartbeat command does not have parameters, please read help");
    }

    exit(exitcode::DATAERR, false, "> Invalid command, please read help");
}

fn exit(code: ExitCode, show_help: bool, message: &str) {
    if show_help {
        utils::cli::print_help();
    }

    if !show_help && !message.is_empty() {
        println!("{}", message);
    }

    std::process::exit(code);
}

fn is_command(arg: &str) -> bool {
    let mut is_command = true;

    if arg.starts_with("-") {
        is_command = false;
    }

    return is_command;
}

fn get_send_parameters(args: &[String]) -> HashMap<String, &String> {
    let use_url = "--use-url";
    let use_token = "--use-token";

    if args.len() == 2 {

        if args[0] == use_url {
            let use_url_map = (use_url.to_string(), &args[1]);
            return HashMap::from([use_url_map]);
        }

        if args[0] == use_token {
            let use_token_map = (use_token.to_string(), &args[1]);
            return HashMap::from([use_token_map]);
        }
    }

    if args.len() == 4 {

        if args[0] == use_url && args[2] == use_token {
            let use_url_map = (use_url.to_string(), &args[1]);
            let use_token_map = (use_token.to_string(), &args[3]);
            return HashMap::from([use_url_map, use_token_map]);
        }

        if args[0] == use_token && args[2] == use_url {
            let use_token_map = (use_token.to_string(), &args[1]);
            let use_url_map = (use_url.to_string(), &args[3]);
            return HashMap::from([use_token_map, use_url_map]);
        }

    }

    return HashMap::new();
}