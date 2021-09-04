# Pixel Editor with Web Assembly (Rust and JavaScript)

- In-browser pixel editor with undo & redo using Webassembly with languages: Rust & Javascript.
- Rust-webpack-template kit was used for connecting JavaScript with Rust and the "im" immutable data structures library (Rust crate) to improve the storage of state in our re-do/un-do queue.
- **Author**: Artem Moshnin <br/>

<p align="center">
<img src="/img/work.gif" width="450" height="450"  />
</p>

## Why Rust and Web Assembly?

### Low-Level Control with High-Level Ergonomics

JavaScript Web applications struggle to attain and retain reliable performance. JavaScript's dynamic type system and garbage collection pauses don't help. Seemingly small code changes can result in drastic performance regressions if you accidentally wander off the JIT's happy path.

Rust gives programmers low-level control and reliable performance. It is free from the non-deterministic garbage collection pauses that plague JavaScript. Programmers have control over indirection, monomorphization, and memory layout.

### Small .wasm Sizes

Code size is incredibly important since the .wasm must be downloaded over the network. Rust lacks a runtime, enabling small .wasm sizes because there is no extra bloat included like a garbage collector. You only pay (in code size) for the functions you actually use.

### Plays Well With Others

Rust and WebAssembly integrates with existing JavaScript tooling. It supports ECMAScript modules and you can continue using the tooling you already love, like npm and Webpack.

## Reasons for using Immutable Datastructures

- `Immutable Datastructures` Rust's [Immutable Datastructures](https://docs.rs/im/15.0.0/im/) will be used to provide functionality of re-do and un-do drawings via a history of images. Benefit of using immutable datastructures is the fact that they are designed to be cloned.

- `Immutable Datastructures` are useful for our scenario because `struct Image {...}` instances will have to be cloned to create a history of images to allow for user to re-do/un-do and therefore using an immutable datastructure (`im::Vector`) will be much quicker & effective than a regular vector (`std::vec::Vec`)

## Resources used

- Wasm Bindgen Docs: https://rustwasm.github.io/docs/wasm-bindgen/
- Immutable Datastructures library: https://docs.rs/im/15.0.0/im/
- rust-webpack-template: https://github.com/rustwasm/rust-webpack-template
