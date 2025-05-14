# Caching Proxy Server 

A high-performance caching proxy server implementation in Rust that intercepts HTTP requests, caches responses, and serves cached content when available.

## Features

- HTTP request proxying with automatic response caching
- In-memory cache with thread-safe implementation
- Cache clearing functionality
- Multiple user interfaces:
  - Command Line Interface (CLI)
  - Terminal User Interface (TUI)
- Async/await based implementation for high performance
- Comprehensive error handling and logging

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

## Project Structure

The project is organized as a Rust workspace with three main components:

- `caching_proxy_sever/` - Core server implementation
- `cli_caching_proxy/` - Command line interface
- `tui_caching_proxy/` - Terminal user interface

## Installation

Clone the repository:

```bash
git clone https://github.com/Hemenguelbindi/caching_proxy.git
cd caching_proxy
```

## Usage 

### Command Line Interface

Start the server with specific port and origin:
```bash
cd cli_caching_proxy 
cargo run -- --port <PORT> --origin <ORIGIN>
```

Clear the cache:
```bash
cd cli_caching_proxy
cargo run -- --clear-cache
```

### Terminal User Interface

Launch the TUI application:
```bash
cd tui_caching_proxy
cargo run
```

The TUI provides an interactive interface for:
- Configuring server port and origin URL
- Starting/stopping the server
- Viewing server logs and status

## Development

### Building

Build all components:
```bash
cargo build --workspace
```

### Testing

Run the test suite:
```bash
cargo test --workspace
```

### Documentation

Generate and view the documentation:
```bash
cargo doc --workspace --open
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Project Challenge

This project was created as a solution to the challenge from: https://roadmap.sh/projects/caching-server
