## Introduction

This program spawns multiple threads to calculate the *Fibonacci sequence*. We use [samply](https://github.com/mstange/samply) to run performance profiling on it.

## Prerequisites

### Install samply

```bash
cargo install --locked samply
```

## Usage

### Profile the execution binary

Instrunctions:

 1. Create a global cargo profile called `profiling`, see below how.
 2. Compile with `cargo build --profile profiling`.
 3. Record with `samply record target/profiling/multithreading-profiling-demo --duration 1 --threads 10 --sleep-ms 50 --start-index 10000`.

To create the `profiling` cargo profile, create a text file at `~/.cargo/config.toml` with the following content:

```toml
[profile.profiling]
inherits = "release"
debug = true
```

Available command line arguments:

```bash
Command-line arguments configuration

Usage: multithreading-profiling-demo [OPTIONS]

Options:
  -d, --duration <DURATION>        Total execution time in seconds [default: 10]
  -t, --threads <THREADS>          Number of worker threads [default: 10]
  -s, --sleep-ms <SLEEP_MS>        Sleep duration in milliseconds for even numbers [default: 50]
  -i, --start-index <START_INDEX>  Starting index for Fibonacci calculations [default: 10000]
  -h, --help                       Print help
```

### Profile the running process

```bash
samply setup # macOS
samply record -p [PID]
```