# Floating Point Subnormal Remover

Stable, `no_std` and dependency-free crate to set and clear the Denormals As Zero (DAZ) and Flush To Zero (FTZ) flags for the FPU.

Comes with a RAII-style "guard" to set-and-reset the flags within a given scope.

Works on x86_64 and AArch64.

## Subnormals/Denormals

The terms denormals and subnormals are used interchangeably. They are defined by IEEE-754 as
the range of floating point values that have a 0 for an exponent. This requires hardware to handle
the special case with microcode/alternate methods which can be slight performance hit.

## Usage

```rust
fn temporarily_disable_subnormals() {
    nosubnormals::guard!{}
    let x = 1.0e-40;
    assert_eq!(x, 0.);
}
```

Testing is done with [cross-rs](https://github.com/cross-rs/cross)


## Next Steps

Some of the `core::instrinsics` could have been used but they seem to be behind a "catch-all" unstable
library feature flag call `std::simd`.

* https://github.com/rust-lang/rust/issues/98253
* https://github.com/rust-lang/stdarch/issues/1268
* https://github.com/rust-lang/rust/issues/90972


Some targets are not yet supported (RISCV, MIPS), [Rust targets](https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools)

WASM is still not ready to support control over IEEE-754 denormal flush to zero.

* https://github.com/WebAssembly/design/pull/271
* https://github.com/WebAssembly/design/issues/1429

NEON vmsr/vmrs instructions should be used.
https://developer.arm.com/documentation/dui0489/i/neon-and-vfp-programming/vmrs?lang=en