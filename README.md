<p align="center">
  <img src="assets/imvox.png" alt="imvox" width="200">
</p>

# IMVOX

## What is imvox?
Is a modular runtime with plugins  
This project, it's a cargo workspace for official imvoxcore/loader/cli projects  
__Our git hostings:__  
[Codeberg (main)](https://codeberg.org/bhorolsky/imvox)  
[GitHub (mirror)](https://github.com/bhorolsky/imvox)  

## This repository includes:
- official imvoxcore/loader/cli implementations projects
- imvox documentation (README, .md files) understandable for beginners
- cargo deny list, with doesn't allow you use code with not compatible with MIT License

## This repository excludes:
- cargo tests - it's will be added in the future // TODO
- plugins/modules - no, only examples + other repos which are independent of this repository

## Getting Started: (from imvoxcli README)
### __How to start:__  
- Clone this repo: `git clone https://codeberg.org/bhorolsky/imvox.git && cd imvox` or from [github.com](https://github.com/bhorolsky/imvox.git)  
- Build all workspace: `cargo build --workspace` or `--release` flag for optimized release version  
- Now you can run imvoxcli: `cargo run -p imvoxcli`  

### __Hello world:__  
- Build your hello world module: `cd examples/hello/ && cargo build`  
- And you can load our module in imvoxcli with command: `/path/to/imvoxcli load /path/to/libimvox_hello.so`  

See more about imvoxcli [there](cli/README.md)  

## Q&A:
- Q: "Is this app an editor? For music, media, or something like that?"  
- A: __No, this is a runtime for future applications and tools.__  
- Q: "What is the goal of the project?"  
- A: __The goal of the project is to create a future application where all applications are modules that communicate via a single bus, communicate with other modules, and where there is no hard API, the API is minimal, the rest will be behind the implementation of modules and third-party clients.__  
- Q: "What's next for the project?"  
- A: __In the future, we want to create a framebuffer plugin that will occupy a bus, like "framebuffer," but the implementation for each interface will be different, like "framebuffertui" and "framebuffergui," and they will all occupy the same bus and interpret the data differently.
We also need to polish the project itself and make this fully possible while it's still WIP. If you're even reading this, you're among the first. Don't use this project yet, as it's still in development and things could change soon...__  
- Q: "How is imvox different from other runtimes?"  
- A: __Nothing. You can use mature and more convenient runtimes than imvox if you want, this is a learning project for myself, it should not kill or replace the existing one, it is simply there as a more modular and lightweight alternative, I am not saying that there are no alternatives to my project, I am just doing this for study, and with the kind of architecture and minimalism that only I need__  
- Q: "Can I use AI?"  
- A: __For assistance, yes; for blind generation, no. Using AI is super useful, but make sure the comments are meaningful and the code isn't hardcoded, then everything will be perfect. I use AI to write code every day myself__  

## Contacts:
- [Discord.gg](https://discord.gg/c7amwQf4KJ/)
- [GitHub.io](https://bhorolsky.github.io/projects/imvox/)

## License
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details