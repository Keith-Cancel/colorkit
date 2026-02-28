# Color Kit

`colorkit` is a lightweight `#[no_std]` color crate for Rust.

It provides an easy to use and strongly typed conversions between color spaces. Color Kit also provides layout and quantization tools for working with with pixel data. One can also implement additional/custom color spaces and layouts.

## Color Kit Overview

- `#[no_std]` friendly and dependency-free.
- Typed color spaces: `Srgb`, `LinSrgb`, `OkLab`, `Xyz<WhitePoint>` etc...
- Conversion API via `FromColor` / `IntoColor`.
- Alpha wrappers for normal and premultiplied color: `Alpha<T>`, `AlphaPre<T>`.
- Layout and quantization primitives: `Planar`, `MappedLayout`, `Packed565`.
- Built-in rounding and optional dithering hooks for scalar/layout conversions.

## Getting Started

Add `colorkit` to your `Cargo.toml`:

```toml
[dependencies]
colorkit = "0.1.0"
```