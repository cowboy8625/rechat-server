# Rusty Chat

![lastupdated](https://img.shields.io/github/last-commit/cowboy8625/rechat-server)
![issuse](https://img.shields.io/github/issues/cowboy8625/rechat-server)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![GitHub repo size](https://img.shields.io/github/repo-size/cowboy8625/rechat-server)
![Discord](https://img.shields.io/discord/509849754155614230)
![Lines of Code](https://tokei.rs/b1/github/cowboy8625/rechat-server)

A simple rust server chat program
Run `cargo run --release` to run the program.

## Data Protocol

We are sending the data is a `byte array` encoded `json` string.

```json
{
    "username": "Mr. AWESOME",
    "message": "I AM SO COOL"
}
```

Messages have a max size of `2048` characters, while the data has a hole is not limited.

## Clients

- [Rechat (GUI Rust)](https://github.com/cowboy8625/rechat-gui-client)
- [Chat Client (GUI Java)](https://github.com/BJTMastermind/ChatClient)

## TODO

- [ ] server keeps message log.
- [ ] send to client when users log get online.
- [ ] Swap from spawning threads to asynchronous behavior like tokei.
- [ ] Work on TUI for Client.
- [ ] Work on GUI for Client in Rust.
- [ ] Work on GUI for Client in Python.
