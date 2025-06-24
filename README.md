# dbg-rs 🦀

![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![crate](https://img.shields.io/crates/v/dbg-rs.svg)
![docs](https://docs.rs/dbg-rs/badge.svg)
![Forks](https://img.shields.io/github/forks/joaoviictorti/dbg-rs)
![Stars](https://img.shields.io/github/stars/joaoviictorti/dbg-rs)
![License](https://img.shields.io/github/license/joaoviictorti/dbg-rs)

Safe Rust bindings for the COM interfaces of the Windows debugging engine.

## Features

- ✅ Safe Rust bindings for Windows debugging interfaces.
- ✅ Easy-to-use macros for logging to the debugger.
- ✅ Abstractions for managing symbols, memory, and CPU registers.
- ✅ Works seamlessly with the Windows COM-based debugging system.

## Getting started

Add `dbg` to your project by updating your `Cargo.toml`:
```bash
cargo add dbg-rs
```

## Usage

The `dbg-rs` library provides utilities to interact with the Windows debugging engine, such as executing commands, logging messages, and inspecting debug symbols. Below are some common use cases:

### Logging to the Debugger

Use the `dprintln!` macro to send formatted messages to the debugger output:

```rs
use dbg_rs::dprintln;

// Example usage
dprintln!(dbg, "Hello, {}!", "Debugger");
dprintln!(dbg, "Number: {}", 42);
```

### Executing Commands

Running commands in the debugger:

```rs
use dbg_rs::Dbg;

dbg.exec(".echo Hello, Debugger!")?;
```

### Reading Virtual Memory

Access specific regions of the debugged process's memory:

```rs
use dbg_rs::Dbg;

let mut buffer = vec![0u8; 128];
dbg.read_vaddr(0x7FFEBEEF0000, &mut buffer)?;
println!("Read memory: {:?}", &buffer[..16]); // Print first 16 bytes
```

## Additional Resources

For more examples, check the [examples](/examples) folder in the repository.

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.
