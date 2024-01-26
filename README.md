# Rust WebSocket Client

A Rust WebSocket client application for processing real-time data from a WebSocket server.

## Introduction

This Rust WebSocket client allows you to connect to a WebSocket server and process real-time data. It consists of two modes: 'cache' and 'average'. The 'cache' mode fetches data from the WebSocket server and stores it in a file, while the 'average' mode computes the average of data received from the server.

## Features

- Two operational modes: 'cache' and 'average'.
- Real-time data processing from a WebSocket server.
- Data caching with timestamp in 'cache' mode.
- Computation of average data values in 'average' mode.

## Getting Started

Follow these instructions to get started with the project.

### Prerequisites

Before running the application, make sure you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/tools/install) (Rust programming language)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust package manager)

### Installation

1. Clone the repository to your local machine:

   ```sh
   git clone https://github.com/yourusername/rust-websocket-client.git
   ```

2. Change to the project directory:

   ```sh
   cd rust-websocket-client
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

## Usage

To use the Rust WebSocket client, you can choose between two modes:

### Cache Mode

In 'cache' mode, the client fetches data from the WebSocket server and caches it in a file with a timestamp.

```sh
./target/release/rust-websocket-client --mode cache --times 10
```

- `--mode cache`: Specifies the 'cache' mode.
- `--times 10`: Optional. Number of times to fetch data (default is 10).

### Average Mode

In 'average' mode, the client computes the average of data received from the WebSocket server.

```sh
./target/release/rust-websocket-client --mode average
```

- `--mode average`: Specifies the 'average' mode.

## Project Structure

The project includes the following Rust source files:

- `main.rs`: Entry point of the application.
- `cache.rs`: Implements the 'cache' mode functionality.
- `compute_average.rs`: Implements the 'average' mode functionality.

 