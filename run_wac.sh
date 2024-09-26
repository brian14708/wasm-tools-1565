#!/bin/sh
cargo b --target wasm32-wasip1
wasm-tools component new ./target/wasm32-wasip1/debug/wasm_compose_test.wasm \
    -o component.wasm --adapt ./wasm/wasi_snapshot_preview1.reactor.wasm
wac plug --plug ./wasm/virt.wasm component.wasm
