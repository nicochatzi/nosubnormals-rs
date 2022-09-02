# Floating Point Subnormal Remover

[![ci](https://github.com/nicochatzi/nosubnormals-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/nicochatzi/nosubnormals-rs/actions/workflows/ci.yml)

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
    assert_eq!(1.0e-40, 0.);
}
```

Testing is done with [cross-rs](https://github.com/cross-rs/cross)


## Targets

### Supported

* `x86_64`: Can flush subnormals and change rounding modes.
* `aarch64`: Can flush subnormals and change rounding modes.
* `riscv64`: FPCR on RISCV cannot configure subnormal operations and it does not default to flushing to zero. Rounding modes are configurable. https://lists.riscv.org/g/tech/topic/76445971?p=Created,,,20,2,0,0::recentpostdate%2Fsticky,,,20,2,20,76445971

### Future Support

* `wasm`: still not ready to support control over IEEE-754 denormal flush to zero, relevant [PR](https://github.com/WebAssembly/design/pull/271) and [issue](https://github.com/WebAssembly/design/issues/1429)

## Stability

Unfortunately, many `core::instrinsics` in Rust are stuck behind a catch-all feature flag called `stdsimd` even though they are not technically unstable. This requires some targets to use inline assembly which use the same instructions are the corresponding `core::instrinsics` instructions.

Some tracking links:

* https://github.com/rust-lang/rust/issues/98253
* https://github.com/rust-lang/stdarch/issues/1268
* https://github.com/rust-lang/rust/issues/90972

## Issues

NEON vmsr/vmrs instructions should be used but does not seem to be recognised when compiling with cross..
https://developer.arm.com/documentation/dui0489/i/neon-and-vfp-programming/vmrs?lang=en