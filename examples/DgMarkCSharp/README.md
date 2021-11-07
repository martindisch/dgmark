# DgMarkCSharp demo

## DgMarkFfi

In order for this to run, you have to compile the Rust library natively first.

```console
cargo build --release
```

## DgMarkWasm

This one requires a WASM module.

```console
cargo build --release --target wasm32-unknown-unknown
```
