
# OKX zkdex on wasm

This is POC of zkDex on wasm.

## Build

### Requirements

- ubuntu 22
- Rust 1.68
- wasm-pack
- wasm-opt
- clang 14

if you use MacOS, you should install docker. (https://github.com/rust-bitcoin/rust-secp256k1/issues/283)

### Build

```bash
$ make build
```

### Build with docker

```bash
$ make env-docker
$ make build-in-docker
```




