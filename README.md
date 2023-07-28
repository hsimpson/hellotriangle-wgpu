# hellotriangle-wgpu

## Introduction

Hello Triangle with a Rust backend based on [wgpu](https://wgpu.rs/)
The purpose of this project is to learn how to use rendering backend based on WebGPU for different platforms.

The directory `renderer` contains the Rust backend.
The directory `frontend` contains all frontends.

## Setup

### Install Rust and wasm-pack

- On macOS:

```shell
brew install rustup-init
rustup-init
```

- On Windows:

```shell
scoop install main/rustup
# was not in PATH after installation:
rustup-init 
```

- All:

```shell
cargo install wasm-pack
```

## Frontends

### Web

The Web frontend uses WebAssembly to run the Rust backend in the browser.
Build the Rust backend for WebAssembly:

```shell
cd renderer
wasm-pack build --out-dir ../frontend/web/src/wasm --target web --no-pack --no-typescript
```

After the build, the Rust backend is located in `frontend/web/src/wasm`. And the page can be served.
