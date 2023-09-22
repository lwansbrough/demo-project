# Build

```
cargo build --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/debug/test_game.wasm -o test-game.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
```

Snapshot preview is from here: https://github.com/bytecodealliance/wasmtime/releases/tag/dev