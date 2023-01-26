pub mod cli {
    pub fn print_help() {
        println!("
        DontDie CLI Program

        dontdie     -h | --help                               show help
        dontdie     -v | --version                            show program version

        dontdie     heartbeat   config     --set-url          sets a new url in the configuration file
        dontdie     heartbeat   config     --set-token        sets a new token in the configuration file
        dontdie     heartbeat   config     --set-interval     sets a new token in the configuration file

        dontdie     heartbeat   daemon                        send heartbeats as a daemon using the configuration file
        dontdie     heartbeat   daemon     --use-url          send heartbeats as a daemon using custom url, overrides file value
        dontdie     heartbeat   daemon     --use-token        send heartbeats as a daemon using custom token, overrides file value
        dontdie     heartbeat   daemon     --use-interval     send heartbeats as a daemon using custom interval, overrides file value

        dontdie     heartbeat   send                          send one heartbeat using data from file
        dontdie     heartbeat   send     --use-url            send heartbeat to custom url, overrides file value
        dontdie     heartbeat   send     --use-token          send heartbeat with custom token, overrides file value
        ");
    }
}