# Wi-Fi Connection Manager

This Rust-based Wi-Fi connection manager handles sessions for the MEO-WiFi network. It supports one-time connections or continuous reconnection attempts, using nmcli for network management and automatic login handling.

## Project Structure

- **`connections`**: Handles network-related operations such as connecting to and disconnecting from networks, checking active connections, and managing network states using `nmcli`.
- **`session`**: Manages user login sessions for the Wi-Fi, including authentication and session lifecycle.
- **`log`**: Provides logging capabilities for monitoring execution flow and capturing events.
- **`args`**: Handles parsing of command-line arguments such as username, password, execution mode, and help/version information.
- **`executor`**: Defines different execution strategies:
  - **`Oneshot`**: Executes a single connection attempt.
  - **`Continuous`**: Continuously attempts reconnection until manually stopped or an error occurs.

## Build

To build the project, you need to have **Rust** and **Cargo** installed.

1. **Clone the repository**:
   ```sh
   git clone <repository-url>
   cd <repository-directory>

2. Build the project: Run the following command to build the project in release mode:
   ```sh
   cargo build --release
   ```

   Alternatively, you can use the provided Makefile to build the project:
   ```sh
   make build
   ```

   The binary will be created in the target/release/ directory by default.

## Usage

**Command-line Arguments**:
   - `-u` | `--username`: Wi-Fi login username.
   - `-p` | `--password`: Wi-Fi login password.
   - `-c` | `--continuous`: Defines the execution mode. When defined, the program runs in `continuous` mode for continuous reconnection attempts.
   - `-h` | `--help`: Displays usage instructions.
   - `-v` | `--version`: Displays the project version.