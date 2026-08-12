# [WIP](docs/WIP.md)

# What is imvoxexamples?
This is a collection of simple examples of using imvox and writing libraries on it  

# This collection includes:
- hello_world
- and... something other later!

# This collection excludes:
- Your modules/plugins
- Responsibility for modules loaded into the core

# Build projects:
For example "hello_world":  
from this directory: `cd hello_world`  
`cargo build ` (or with "` --release`" if you want lightweight version)  
Done!  

# How to use builded library?
You need to load the library you've compiled into your loader (example: imvoxloader) via your interface (example: imvoxcli).
I'll show you an example using the cli (imvoxcli) and the hello_world compiled above:  
Build workspace from root of project: `cargo build --workspace`  
Switch to target: `cd target/debug` or `cd target/release` if you build workspace with `--release` flag  
Check imvoxcli: `./imvoxcli --help`  
And try to load your library to Loader: `./imvoxcli load ../../examples/hello_world/target/debug/libimvox_hello.so `, again, if you build it with ` --release` flag then use "/target/" instead "/debug/"  
Done! After this you can see how your plugin loaded to imvoxcore!  