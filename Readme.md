# Solana NoStd Sha256

A more efficient implementation of Sha256 for SVM.

# Installation

```cargo add solana-nostd-sha256```

# Features

- `#![no_std]` — first-class no-std support, on and off Solana.
- Adds `hash_ref` which takes in any type that implements `AsRef<[u8]>`
- No `Hash` struct. Returns `[u8;32]` directly.
- Makes use of MaybeUninit to skip zero allocations
- Adds `hash_into` to let you hash directly into a mutable buffer.

# Performance

| library        | function          | CU cost |
|----------------|-------------------|---------|
| nostd-sha256   | hashv(&[b"test"]) | 100     |
| nostd-sha256   | hash(b"test")     | 105     |
| nostd-sha256   | hash_ref("test")  | 105     |
| solana-program | hashv(&[b"test"]) | 120     |
| solana-program | hash(b"test")     | 123     |

# Benchmarking

CU usage is tracked under [svm-unit-test](https://github.com/blueshift-gg/svm-unit-test). With `cargo build-sbf` on `$PATH`:

```sh
cargo test --test sbpf -- --nocapture
```

Sample output (includes the SBPF entrypoint wrapper, ~12 CUs above the raw call):

```
svm_test `bench_hashv`    => 112 CUs
svm_test `bench_hash`     => 113 CUs
svm_test `bench_hash_ref` => 113 CUs
```
