pub mod cli {
    pub fn print_help() {
        println!("
        hostbeat CLI Program

        hostbeat     -h | --help                               show help
        hostbeat     -v | --version                            show program version

        hostbeat     heartbeat   config                        gets current stored configuration
        hostbeat     heartbeat   config     --get-token        gets current stored token
        hostbeat     heartbeat   config     --get-url          gets current stored url
        hostbeat     heartbeat   config     --get-interval     gets current stored interval
        hostbeat     heartbeat   config     --set-url          sets a new url in the configuration file
        hostbeat     heartbeat   config     --set-token        sets a new token in the configuration file
        hostbeat     heartbeat   config     --set-interval     sets a new token in the configuration file

        hostbeat     heartbeat   daemon                        send heartbeats as a daemon using the configuration file
        hostbeat     heartbeat   daemon     --use-url          send heartbeats as a daemon using custom url, overrides file value
        hostbeat     heartbeat   daemon     --use-token        send heartbeats as a daemon using custom token, overrides file value
        hostbeat     heartbeat   daemon     --use-interval     send heartbeats as a daemon using custom interval, overrides file value

        hostbeat     heartbeat   send                          send one heartbeat using data from file
        hostbeat     heartbeat   send     --use-url            send heartbeat to custom url, overrides file value
        hostbeat     heartbeat   send     --use-token          send heartbeat with custom token, overrides file value
        ");
    }
}