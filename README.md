# Pixel Editor with Web Assembly (Rust and JavaScript)

## Why Rust and Web Assembly?

## Functionality explained

- `Immutable Datastructures` Rust's [Immutable Datastructures](https://docs.rs/im/15.0.0/im/) will be used to provide functionality of re-do and un-do drawings via a history of images. Benefit of using immutable datastructures is the fact that they are designed to be cloned.

- `Immutable Datastructures` They are useful for our scenario becasuee `struct Image {...}` instances will have to be cloned to create a history of images to allow for user to re-do/un-do and therefore using an immutable datastructure (`im::Vector`) will be much quicker and effective than a regular vector (`std::vec::Vec`)
