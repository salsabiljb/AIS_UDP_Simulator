
# AIS Messages Simulator

This project is an AIS (Automatic Identification System) message simulator written in Rust. The simulator reads AIS messages from a specified file and sends them over UDP to a target address. It also includes an optional receiver that can capture these messages and save them to a file. The sending order of the AIS messages can be randomized, and the sending delay is randomized within a specified range.

## Features

- Send AIS messages from a file to a specified target address over UDP.
- Randomize the order of AIS messages.
- Randomize the delay between sending messages within a specified range.
- Optional receiver to capture and save received messages to a file.
- Continuous streaming of messages until stopped manually.

## Prerequisites

- Rust programming language (install from [rust-lang.org](https://www.rust-lang.org/))
- `cargo` package manager

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/salsabiljb/AIS.git
    cd AIS/ais_messages_simulator
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

### Running the Sender

To run the sender that reads AIS messages from a file and sends them over UDP:

```sh
cargo run -- --sender-file-path /path/to/AIS_messages.txt --target-addr 000.000.0.0:0 
```

### Running Both Sender and Receiver

To run both the sender and receiver:

```sh
cargo run -- --sender-file-path /path/to/AIS_messages.txt --target-addr 000.000.0.0:0 --bind-addr 000.000.0.0:0 --receiver-file-path /path/to/udp_messages.txt
```

### Command-line Arguments

- `--sender-file-path`: Path to the file containing AIS messages.
- `--target-addr`: Target address (IP:port) to send UDP messages.
- `--bind-addr` (optional): Address (IP:port) to bind the receiver.
- `--receiver-file-path` (optional): Path to the file where received messages will be saved.

## Example

To send AIS messages from `AIS_messages.txt` to `000.000.0.0:0`:

```sh
cargo run -- --sender-file-path /home/user/AIS_messages.txt --target-addr 000.000.0.0:0
```

To send AIS messages and simultaneously receive and save them to `udp_messages.txt`:

```sh
cargo run -- --sender-file-path /home/user/AIS_messages.txt --target-addr 000.000.0.0:0 --bind-addr 000.000.0.0:0 --receiver-file-path /home/user/udp_messages.txt
```
