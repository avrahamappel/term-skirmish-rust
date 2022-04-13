# Term Skirmish Rust 🦀

Term Skirmish Rust is a zero-player battle that takes place in your terminal. This project is a port of Tom Steven's excellent [term-skirmish](https://github.com/tom-on-the-internet/term-skirmish) to the Rust language.

## Why did you do this?

Partially because Rust [is faster than](https://github.com/avrahamappel/term-skirmish-rust/blob/master/src/game.rs#L84-L85) Go.

But mostly because Rust code just [looks so much better](screenshots/compare.md) in my editor.

## Installation

```bash
cargo install --git https://github.com/avrahamappel/term-skirmish-rust
term-skirmish-rust
```

Docker:

```bash
docker run --rm -it rust bash -c 'cargo install --git https://github.com/avrahamappel/term-skirmish-rust && term-skirmish-rust'
```
