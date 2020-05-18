Benchmark code can be found at https://github.com/iotaledger/iota.rs/blob/feature/benchmark/benchmark/src/main.rs and https://github.com/iotaledger/iota.rs/blob/feature/wasm-binding/bindings/wasm/benchmark.js.

# Results

Benchmark results using [hyperfine](https://github.com/sharkdp/hyperfine).

 | Command                                        |            Mean [s] | Min [s] | Max [s] |         Relative |
 | :--------------------------------------------- | ------------------: | ------: | ------: | ---------------: |
 | `./iota.rs/benchmark/target/release/benchmark` | 11.482        0.063 |  11.354 |  11.681 |             1.00 |
 | `node iota-rs-wasm/benchmark.js`               | 13.406        0.168 |  13.158 |  14.487 | 1.17        0.02 |
Summary
  './iota.rs/benchmark/target/release/benchmark' ran
    1.17 ± 0.02 times faster than 'node iota-rs-wasm/benchmark.js'
