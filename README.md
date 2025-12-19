# `libriscv_sys`

Low-level Rust FFI bindings to [libriscv](https://github.com/libriscv/libriscv), a fast RISC-V sandbox emulator. This is a `-sys` crate intended to be used by higher-level Rust wrappers.

## Usage

Add the dependency:

```toml
[dependencies]
libriscv_sys = { version = "0.1" }
```

Call the C API:

```rust
use libriscv_sys::*;

fn main() {
    unsafe {
        let mut options: RISCVOptions = std::mem::zeroed();
        libriscv_set_defaults(&mut options);
    }
}
```

## Features

- `bindgen`: regenerate bindings at build time (requires libclang).
- `binary-translation`: enable libriscv binary translation support.
