pub mod cli {
    pub fn print_help() {
        println!("
        hostbeat CLI Program

        hostbeat     -h | --help                               show help
        hostbeat     -v | --version                            show program version

        hostbeat     client   config                        gets current stored configuration
        hostbeat     client   config     --get-token        gets current stored token
        hostbeat     client   config     --get-url          gets current stored url
        hostbeat     client   config     --get-interval     gets current stored interval
        hostbeat     client   config     --get-monitoring   gets current stored monitoring value
        hostbeat     client   config     --set-url          sets a new url in the configuration file
        hostbeat     client   config     --set-token        sets a new token in the configuration file
        hostbeat     client   config     --set-interval     sets a new interval in the configuration file
        hostbeat     client   config     --set-monitoring   sets value to enable or disable monitoring in the configuration file

        hostbeat     client   daemon                        send heartbeats as a daemon using the configuration file
        hostbeat     client   daemon     --use-url          send heartbeats as a daemon using custom url, overrides file value
        hostbeat     client   daemon     --use-token        send heartbeats as a daemon using custom token, overrides file value
        hostbeat     client   daemon     --use-interval     send heartbeats as a daemon using custom interval, overrides file value
        hostbeat     client   daemon     --use-monitoring   send heartbeats as a daemon enabling or disabling monitoring, overrides file value

        hostbeat     client   send                          send one heartbeat using data from file
        hostbeat     client   send     --use-url            send heartbeat to custom url, overrides file value
        hostbeat     client   send     --use-token          send heartbeat with custom token, overrides file value
        hostbeat     client   send     --use-monitoring     send heartbeat and include host performance data
        ");
    }
}