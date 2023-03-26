# Minimal Allocation Mutator

This library exposes a mutator for fuzzing purposes using minimal memory allocations, implemented in Zig with Rust bindings! Rust calls out to the mutator via a single, static
library via FFI, as Zig has C equivalence in its FFI boundaries. This also means the Zig code can interact with the Rust code, inspired by `[rustiffy](https://github.com/DutchGhost/rustiffy)`.

The mutator takes in a single, mutable bytes buffer and mutates it N times
according to different strategies with _minimal_ memory allocation and runtime 
safety checks provided by Zig itself. Using a custom allocation strategy allows the
mutator to be lightning fast, and the allocator of choice can be customized
when interacting with the libraries.

Inspired by honggfuzz's mutator.

## Example

``` rust
let mut buf = vec![0; 6];
let num_iterations = 1_000_000;
mutate(&mut buf, num_iterations);
```

## TODOs

- [ ] Support coverage guided fuzzing
- [ ] Support taint tracking
- [ ] Implement all mutations strategies available
- [ ] Criterion benchmarks from Rust
- [ ] Make it a usable Rust library. How to ship the zig static lib?
