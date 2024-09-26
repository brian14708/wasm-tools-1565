#!/bin/sh
cargo b --target wasm32-wasip1
wasm-tools component new ./target/wasm32-wasip1/debug/wasm_compose_test.wasm \
    -o component.wasm --adapt ./wasm/wasi_snapshot_preview1.reactor.wasm
wasm-tools compose component.wasm -d ./wasm/virt.wasm -o component.virt.wasm
