This is a test of `wasm-tools compose`/`wac` when composing wasi-virt with custom component.

The custom component has a simple `my-sleep` func that returns a custom `pollable`.

To run:

```
./run_wasm_tool.sh
# or ./run_wac.sh
```

Expected result in two files `component.wasm` and `component.virt.wasm`.

To test the generated output:

```
cargo r -p runner component.wasm        # should print "before sleep" and "after sleep"
cargo r -p runner component.virt.wasm   # should panic (wasi-virt has clock disabled)
```

---

Actual result:

`wasm-tools compose` fails with:

```
[2024-09-26T18:01:25Z WARN ] instance `pollable` will be imported because a dependency named `pollable` could not be found
[2024-09-26T18:01:25Z WARN ] instance `my-sleep` will be imported because a dependency named `my-sleep` could not be found
thread 'main' panicked at /Users/brian/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/wasm-compose-0.217.0/src/encoding.rs:616:21:
internal error: entered unreachable code: should have been handled in `TypeEncoder::component_entity_type`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`wac` fails with:

```
error: encoding produced a component that failed validation

Caused by:
    type mismatch for import `pollable`
    resource types are not the same (ResourceId { globally_unique_id: 2, contextually_unique_id: 1 } vs. ResourceId { globally_unique_id: 2, contextually_unique_id: 110 }) (at offset 0x1e58e5)
```

Test Environment:

- wasm-tools 1.217.0
- wac-cli 0.6.0
- wasi-virt 02de7b49
- `./wasm/host.wasm` is generated using: `wasi-virt --out wasm/virt.wasm  --allow-stdio`
- [`./wasm/wasi_snapshot_preview1.reactor.wasm`](https://github.com/bytecodealliance/wasmtime/releases/download/v25.0.1/wasi_snapshot_preview1.reactor.wasm)
