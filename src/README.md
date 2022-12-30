# Getting Started

```zsh
$ cargo install lunatic-runtime
$ rustup target add wasm32-wasi
$ cargo build --release --target=wasm32-wasi
$ lunatic target/wasm32-wasi/release/example-lunatic-submillisecond.wasm

# open new terminal
$ curl 0.0.0.0:3000
```

