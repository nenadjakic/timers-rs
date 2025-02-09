# timers-rs

**timers-rs** is a Rust-based application designed to manage and track timers for various projects. It utilizes a terminal user interface (TUI) to provide an interactive experience for users.

## Overview

The **timers-rs** application is built to help users efficiently manage multiple timers across different projects. With its interactive TUI, users can easily start, stop, and monitor timers, ensuring accurate tracking of time. The application supports persistent storage, allowing users to save their timer data and resume their sessions seamlessly.

## Features

- Manage multiple projects and their associated timers.
- Interactive TUI using **ratatui** and **crossterm**.
- Persistent storage of project data using JSON.
- Configurable and extendable architecture.

## Installation

To install and run **timers-rs**, ensure you have Rust and Cargo installed on your system. Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/nenadjakic/timers-rs.git
cd timers-rs
cargo build --release
```

## Usage
Run the application with the following command:
```bash
cargo run --release
```

## Dependencies

- **ratatui**: For building the terminal user interface.
- **crossterm****: For handling terminal input and output.
- **serde** and **serde_json**: For serializing and deserializing project data.
- **chrono**: For handling date and time operations.

## Licence

This project is licensed under the Apache License.