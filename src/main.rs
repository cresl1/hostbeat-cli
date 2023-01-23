mod libs;
mod services;

use crate::libs::utils;
use std::{env, collections::HashMap};
use exitcode::{self, ExitCode};
use services::settingsservice::SettingsService;

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

                if args.len() < 4 {
                    exit(exitcode::USAGE, false, "> The config command needs parameters, please read help");
                }

                let parameters = vec!["--set-url", "--set-token", "--set-interval"];
                let config_parameters = get_command_parameters(&parameters, &args[3..]);

                if config_parameters == None {
                    exit(exitcode::DATAERR, false, "> Invalid parameters for config command, please read help")
                }

                SettingsService::new()
                    .load_settings()
                    .set_from(config_parameters.unwrap());

                // Do send
                exit(exitcode::OK, false, &format!("> Config set"));

            }

            if third_arg == "daemon" {

                // TODO: remove this for final version
                let mut has_params = false;

                if args.len() >= 4 {

                    let parameters = vec!["--use-url", "--use-token", "--use-interval"];
                    let send_parameters = get_command_parameters(&parameters, &args[3..]);

                    if send_parameters == None {
                        exit(exitcode::DATAERR, false, "> Invalid parameters for daemon command, please read help");
                    }

                    // TODO: Inject parameters
                    println!("> Daemon... parameters are checked and correct");
                    has_params = true;
                }

                // Do send
                exit(exitcode::OK, false, &format!("> Running daemon, has params {}", has_params));

            }

            if third_arg == "send" {

                // TODO: remove this for final version
                let mut has_params = false;

                if args.len() >= 4 {

                    let parameters = vec!["--use-url", "--use-token"];
                    let send_parameters = get_command_parameters(&parameters, &args[3..]);

                    if send_parameters == None {
                        exit(exitcode::DATAERR, false, "> Invalid parameters for send command, please read help");
                    }

                    // TODO: Inject parameters
                    println!("> Sending... parameters are checked and correct");
                    has_params = true;
                }

                // Do send
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