# godfishbot_rust
This is intended to be a complete rewrite of my older project, the
[GodfishBot](https://github.com/Follpvosten/GodfishBot), which was
written in Java. `godfishbot_rust` will be written in Rust, as the
name suggests.

The development of this bot is happening entirely on FreeBSD, and i
am not doing any testing on other platforms.

## Building and running it
1. Install the latest nightly toolchain of Rust
2. Clone this repository
3. Copy the `BotConfig.example.toml` file to a new one called
   `BotConfig.toml` and put in your bot API token. (the file can
   also be called something else, but then you'll have to pass
   its filename/path as a command line argument.
4. Run `cargo build` inside the project directory to compile it,
   or `cargo run` to (compile and) run it.
