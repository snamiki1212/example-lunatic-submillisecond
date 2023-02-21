# Getting Started

```zsh
$ cargo install lunatic-runtime
$ rustup target add wasm32-wasi
$ cargo build --release --target=wasm32-wasi && \
  RUST_LOG=lunatic=debug WASMTIME_BACKTRACE_DETAILS=1 lunatic target/wasm32-wasi/release/example-lunatic-submillisecond.wasm

# open new terminal
$ curl 0.0.0.0:3000
```

```zsh
$ PREFIX="0.0.0.0:3000"
$ curl ${PREFIX}/parent/child
$ curl ${PREFIX}/hoge-hoge # expect to find no endpoint
$ curl ${PREFIX}/user/hoge-san/99
```

## LICENSE

MIT
