mod libs;
mod services;

use crate::libs::utils;
use std::{env, collections::HashMap};
use exitcode::{self, ExitCode};
use services::{settingsservice::SettingsService, heartbeatservice::HeartbeatService};

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
            exit(exitcode::OK, false, &format!("hostbeat {}", env!("CARGO_PKG_VERSION")))
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

                if args.len() < 4 {
                    exit(exitcode::USAGE, false, "> The config command needs parameters, please read help");
                }

                let parameters = vec!["--set-url", "--set-token", "--set-interval"];
                let config_parameters = get_command_parameters(&parameters, &args[3..]);

                if config_parameters == None {
                    exit(exitcode::DATAERR, false, "> Invalid parameters for config command, please read help")
                }

                match SettingsService::new().load_settings().set_to_file_from(config_parameters.unwrap()) {
                    Some(string_error) => exit(exitcode::DATAERR, false, &string_error),
                    None => exit(exitcode::OK, false, "")
                }
            }

            if third_arg == "daemon" {

                let mut settings_service = SettingsService::new().load_settings();

                if args.len() >= 4 {

                    let parameters = vec!["--use-url", "--use-token", "--use-interval"];
                    let send_parameters = get_command_parameters(&parameters, &args[3..]);

                    if send_parameters == None {
                        exit(exitcode::DATAERR, false, "> Invalid parameters for daemon command, please read help");
                    }

                    let result = settings_service.set_to_memory_from(send_parameters.unwrap());
                    
                    if result.is_some() {
                        exit(exitcode::DATAERR, false, &result.unwrap())
                    }
                }
                
                HeartbeatService::new().send_as_daemon(&settings_service.settings);
            }

            if third_arg == "send" {
                
                if args.len() >= 4 {

                    let parameters = vec!["--use-url", "--use-token"];
                    let send_parameters = get_command_parameters(&parameters, &args[3..]);

                    if send_parameters == None {
                        exit(exitcode::DATAERR, false, "> Invalid parameters for send command, please read help");
                    }
                    
                    let mut service = SettingsService::new();
                    let result = service.set_to_memory_from(send_parameters.unwrap());                  
                    
                    if result.is_some() {
                        exit(exitcode::DATAERR, false, &result.unwrap())
                    }
                    
                    match HeartbeatService::new().send(&service.settings) {
                        Some(error) => exit(exitcode::DATAERR, false, &error),
                        None => exit(exitcode::OK, false, "")
                    };
                }
                
                let settings_service = SettingsService::new().load_settings();

                match HeartbeatService::new().send(&settings_service.settings) {
                    Some(error) => exit(exitcode::DATAERR, false, &error),
                    None => exit(exitcode::OK, false, "")
                };
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

fn is_flag(arg: &str) -> bool {
    let mut is_flag = false;

    if arg.starts_with("--") {
        is_flag = true;
    }

    return is_flag;
}

fn get_command_parameters(parameters: &Vec<&str>, args: &[String]) -> Option<HashMap<String, String>> {
    let mut result: HashMap<String, String> = HashMap::new();

    let mut param_index: usize = 0;
    let mut error = false;

    while param_index < parameters.len() && !error {

        let mut arg_index: usize = 0;
        let mut found = false;

        while arg_index < args.len() && !found && !error {

            if is_flag(&args[arg_index]) && parameters[param_index] == args[arg_index] {

                if (arg_index + 1) >= args.len() {
                    error = true;
                }
                else {
                    result.insert(parameters[param_index].to_string(), args[arg_index + 1].trim().to_string());
                    found = true;
                }

            }

            arg_index += 1;
        }

        param_index += 1;
    }

    if error || result.is_empty() {
        return None;
    }

    return Some(result);

}