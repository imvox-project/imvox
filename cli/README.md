# What is imvoxcli?
Implementation of a cli-type interface for imvox plugin system, needed to interact with imvoxcore via imvoxloader  

# This crate includes:
- The logic of the interface interaction with imvoxloader
- User-oriented features
- Minimal implementation of imvox interface

# This crate excludes:
- Core logic
- Plugins

# Getting Started:
__How To Start:__  
- Clone this repo: `git clone https://codeberg.org/bhorolsky/imvox.git && cd imvox` or from [github.com](https://github.com/bhorolsky/imvox)  
- Build all workspace (run it from root of project): `cargo build --workspace` or `--release` flag for optimized release version  
- Now you can run cli: `cargo run -p imvoxcli`  

__Hello world:__  
- Build your hello world module: `cd examples/hello/ && cargo build`  
- And you can load our module in cli with command: `/path/to/imvoxcli load /path/to/libimvox_hello.so`  
