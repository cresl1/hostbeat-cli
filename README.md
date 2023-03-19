
![GitHub](https://img.shields.io/github/license/ruben69695/hostbeat-cli?color=purple)
![GitHub last commit](https://img.shields.io/github/last-commit/ruben69695/hostbeat-cli)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/ruben69695/hostbeat-cli?color=purple)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ruben69695/hostbeat-cli?color=purple)

# Hostbeat CLI
The command line hostbeat client for Windows, MacOS and Linux. Made with 🦀 Rust and 💙 love.

## 📦 Languages and Dev-Tools
- Rust 1.65.0
- Cargo 1.65.0

## 🔨 Install debug
This installation version is only for development purposes, be sure to have principal language and dev-tools installed: Rust and Cargo 
with the minimal version specified.

1. Clone the respository
    ```zsh
    git clone https://github.com/ruben69695/hostbeat.git
    ```

2. Build the project usign cargo cli tool
    ```zsh
    cargo build
    ```

3. Run the project
    ```zsh
    cargo run -- --version
    ```
    
## 🚀 Install release
This installation version is for production use, be sure to have installed the dependencies for each 
platform to be able to install the CLI tool.

### 🐧 Linux -  MacOS
#### Dependencies
To be able to use the installation script `ìnstall.sh`, is required to have installed the next dependencies:
- bash
- curl
- unzip

#### Installation
Execute the script `ìnstall.sh`, root is not necessary

1. Give execute permission
    ```zsh
    chmod u+x install.sh
    ```

2. Execute the installer
    ```zsh
    ./install.sh
    ```
    > If it's the first time installing the tool, re-open a new terminal to be able to use it
    
### 🪟 Windows
#### Dependencies
To be able to use the installation script `ìnstall.ps1`, is required to have installed the next dependencies:
- powershell

#### Installation
Execute the script `ìnstall.ps1` in Windows, admin is required to make the installation, if is not executed as admin it 
will be elevated to do it automatically.

1. Execute the installer
    ```zsh
    .\install.ps1
    ```
    > If it's the first time installing the tool, re-open a new terminal to be able to use it
   

## ✏️ Getting started

- Parameters
    ```zsh
    hostbeat     -h | --help                               show help
    hostbeat     -v | --version                            show program version

    hostbeat     heartbeat   config     --set-url          sets a new url in the configuration file
    hostbeat     heartbeat   config     --set-token        sets a new token in the configuration file
    hostbeat     heartbeat   config     --set-interval     sets a new interval in the configuration file

    hostbeat     heartbeat   daemon                        send heartbeats as a daemon using the configuration file
    hostbeat     heartbeat   daemon     --use-url          send heartbeats as a daemon using custom url, overrides file value
    hostbeat     heartbeat   daemon     --use-token        send heartbeats as a daemon using custom token, overrides file value
    hostbeat     heartbeat   daemon     --use-interval     send heartbeats as a daemon using custom interval, overrides file value

    hostbeat     heartbeat   send                          send one heartbeat using data from file
    hostbeat     heartbeat   send       --use-url          send heartbeat to custom url, overrides file value
    hostbeat     heartbeat   send       --use-token        send heartbeat with custom token, overrides file value
    ```
- Usage
    ```zsh

    ```
    
- Examples
    ```zsh

    ```

## 🏭 Building release
1. Adding platform targets if not added: MacOS, Linux, Windows
    ```zsh
    rustup target add aarch64-apple-darwin
    rustup target add x86_64-apple-darwin
    rustup target add x86_64-unknown-linux-gnu
    rustup target add x86_64-pc-windows-gnu
    ```

2. Build release for every platform
- Multiple way
    ```zsh
    cargo build --release --target=aarch64-apple-darwin --target=x86_64-apple-darwin --target=x86_64-unknown-linux-gnu --target=x86_64-pc-windows-gnu
    ```

- Individual way
    ```zsh
    cargo build --target=aarch64-apple-darwin --release
    cargo build --target=x86_64-apple-darwin --release
    cargo build --target=x86_64-unknown-linux-gnu --release
    cargo build --target=x86_64-pc-windows-gnu --release
    ```


## 💻 Development environment
- Fedora 37 and MacOS Monterey
- Lapce 0.2.5
- Codium 1.74

## 🚀 Runs on
- Linux
- MacOS (Apple Silicon and Intel)
- Windows
