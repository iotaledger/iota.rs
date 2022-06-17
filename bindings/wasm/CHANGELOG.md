# Changelog

## \[0.4.1]

- Fix primary_pow_node usage.
  - [2ab756f](https://github.com/iotaledger/iota.rs/commit/2ab756ffe884bda87cad0029077863389f8cdddb) Bump neon from 0.8.3 to 0.10.1 in /bindings/nodejs/native ([#1070](https://github.com/iotaledger/iota.rs/pull/1070)) on 2022-06-17

## \[0.4.0]

- Added `getEssenceHash()` to calculate the hash of a transaction essence.
  - [86259ee](https://github.com/iotaledger/iota.rs/commit/86259ee292a570cc9beb85db08aecc6153b28470) Add getEssenceHash() ([#940](https://github.com/iotaledger/iota.rs/pull/940)) on 2022-04-27

## \[0.3.1]

- Updated dependencies to fix compilation.
  - [0838d48e](https://github.com/iotaledger/iota.rs/commit/0838d48e4683ab8d10f685c444f4ed097cf76493) Update dependencies ([#767](https://github.com/iotaledger/iota.rs/pull/767)) on 2021-12-09

## \[0.3.0]

- Changed "discovered" to "autopeered" in the result from getPeers().
  - [f43b9dea](https://github.com/iotaledger/iota.rs/commit/f43b9deac867cc52bff92a6c400ff54cbd06add2) add change file and updated covector workflow on 2021-11-03

## \[0.2.2]

- Fixed reattach function
  - [64dcf583](https://github.com/iotaledger/iota.rs/commit/64dcf583fc53b2e6ad80103ee3aaf2485dfee51c) Fix reattach for wasm binding ([#713](https://github.com/iotaledger/iota.rs/pull/713)) on 2021-10-27

## \[0.2.1]

- Select another reqwest version, so posting a message doesn't fail.
  - [f3692e13](https://github.com/iotaledger/iota.rs/commit/f3692e1385aa5e45b61e6222f738415af0a264ba) Lock dependencies to a specific version ([#719](https://github.com/iotaledger/iota.rs/pull/719)) on 2021-10-21

## \[0.2.0]

- Add getTransactionId() method
  - [fe441d79](https://github.com/iotaledger/iota.rs/commit/fe441d791450ee77d10bfd842ff7e250c971dfca) Add get_transaction_id ([#714](https://github.com/iotaledger/iota.rs/pull/714)) on 2021-10-13

## \[0.1.3]

- Reduced WASM build size
  - [8163754b](https://github.com/iotaledger/iota.rs/commit/8163754be208c24471e45e66020b0cd333ecb0dc) Reduce WASM build size ([#708](https://github.com/iotaledger/iota.rs/pull/708)) on 2021-10-07

## \[0.1.2]

- Cleaned installation instructions and fixed the link for the API reference
  - [4b825fe2](https://github.com/iotaledger/iota.rs/commit/4b825fe271f3eb6fd07fd59d7087c144290b68e8) Clean readme and revert commit 191c7b2 ([#698](https://github.com/iotaledger/iota.rs/pull/698)) on 2021-09-28
  - [4de0e96d](https://github.com/iotaledger/iota.rs/commit/4de0e96dd399886c4de10b0846d429b3389755ab) Apply Version Updates From Current Changes ([#699](https://github.com/iotaledger/iota.rs/pull/699)) on 2021-09-28

## \[0.1.1]

- Publish wasm binding
  - [5eba5806](https://github.com/iotaledger/iota.rs/commit/5eba5806840d652f77aac5f1845c9e3801729b9f) fix wasm-binding path ([#694](https://github.com/iotaledger/iota.rs/pull/694)) on 2021-09-27
