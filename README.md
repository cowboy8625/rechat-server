# Rusty Chat

[![docs.rs](https://docs.rs/rechat-server/badge.svg)](https://docs.rs/rechat-server)
[![crates.io](https://img.shields.io/crates/v/rechat-server.svg)](https://crates.io/crates/rechat-server)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![lastupdated](https://img.shields.io/github/last-commit/cowboy8625/rechat-server)
![GitHub repo size](https://img.shields.io/github/repo-size/cowboy8625/rechat-server)
![issuse](https://img.shields.io/github/issues/cowboy8625/rechat-server)
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

[Rechat (GUI Rust)](https://github.com/cowboy/rechat-gui-client)
[Chat Client (GUI Java)](https://github.com/BJTMastermind/ChatClient)

## TODO

- [ ] Change Server from data serialization from just a byte array to
Json or [Erlang](http://www1.erlang.org/doc/apps/erts/erl_ext_dist.html)
- [ ] Work on TUI for Client.
- [ ] Work on GUI for Client. (Python or Rust).
