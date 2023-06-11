# CrabSlumber

A server management tool that keeps an eye on your Minecraft server during its well-deserved rest.

## Progress

- [x] Logger (print to stdout and log file, supports various log levels and is adjustable via flags at runtime)

- [x] Settings file (create, read, and parse)

- [x] Helper functions & constants

- [x] Discord webhook functions

- [x] MOTD Parser ([@sfirew/minecraft-motd-parser](https://npmjs.com/package/@sfirew/minecraft-motd-parser) implementation)

- [ ] Web interface (will probably use some js framework)

- [ ] Redstone (inhouse node_minecraft_protocol replacement)

- [ ] Copper (prismarine replacement)

- [ ] Java server implementation

- [ ] Bedrock server implementation

- [ ] Container (main server management)

- [ ] Plugin API (for adding custom functionality)

## Installation

While this project is still in pre-alpha stage, it is not runnable yet. But once it is, you will be able to get it running by downloading the [latest release](/releases/latest) binary and running it. A configuration file will be generated in the same directory as the binary, with options and examples as comments.

## Motivation

I started this project primarily with the goal of learning more about Rust. I also wanted to provide a smaller binary and footprint than the original project, and more reliable because of rust's compile-time checks.

## Contributing

Contributions more than welcome! If you want to contribute, you will only need to have [Rust](https://www.rust-lang.org/) and Cargo installed. You can then clone this repository and run `cargo run` to get started. If you want to contribute but don't know where to start, check out the [issues](/issues) page. Feel free to open a new issue if you have any questions or suggestions, or create a PR if you have a fix or feature to add. ❤️❤️

## License

All credit goes to the original author of this project [@Vincss](https://github.com/Vincss). ❤️

Outsite of the original author, this project is licensed under the [MIT License](https://opensource.org/licenses/MIT). See `LICENSE` for more information.
