# citro2d-rs

A work in prorgress Rust binding and safe wrapper to the citro2d library, to write 2d graphics for homebrew programs on 3DS.

This is inspired from [citro3d-rs](https://github.com/rust3ds/citro3d-rs).

> This library is in early development. The API may change between versions.

## What is this?

citro2d is a 2D graphics library for the Nintendo 3DS, built on top of citro3d and the PICA200 GPU. This crate provides:

- **`citro2d-sys`**: raw unsafe bindings generated from the citro2d C headers via bindgen
- **`citro2d`**: a safe, idiomatic Rust API on top of the raw bindings

The safe wrapper uses the Rust type system to make incorrect usage impossible, for example drawing outside a frame or forgetting to end a frame.

## Requirements

- [devkitPro](https://devkitpro.org/) with 3ds-dev installed (using the package-manager)
- Rust nightly
- [cargo-3ds](https://github.com/rust3ds/cargo-3ds)

The following environment variables must be set
```
DEVKITPRO=/opt/devkitpro
DEVKITARM=/opt/devkitpro/devkitARM
```

The easiest way to do this is to use the development flake with nix. This still needs the `devkitPro` packages to be installed.

## License

This project is licensed under the [zlib license](LICENSE).
