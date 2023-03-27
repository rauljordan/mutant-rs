# Mutant-rs

This library exposes a **simple, bytes mutator** for fuzzing purposes, implemented in [Zig](https://ziglang.org/) with **Rust bindings**! Rust calls out to the mutator via a single, static
library via FFI, as Zig has C equivalence in its FFI boundaries. This also means the Zig code can interact with the Rust code, inspired by [rustiffy](https://github.com/DutchGhost/rustiffy).

The mutator takes in a single bytes buffer and mutates it N times
according to different strategies with **minimal** memory allocation and runtime 
safety checks provided by Zig itself. Using a custom allocation strategy allows the
mutator to be lightning fast, and the allocator of choice can be customized
when interacting with the libraries.

Eventually, **coverage-guided mutation via an input DB will be supported for the library**.

Inspired by [honggfuzz](https://github.com/google/honggfuzz)'s mutator.

## Example

The mutator can perform **33 million mutations per second** on a simple laptop on sample input bytes using a variety of strategies using Rust to interact with the Zig mutator via FFI.

``` rust
let mut buf = vec![0; 6];
let num_iterations = 1_000_000;
mutate(&mut buf, num_iterations);
```

In zig:

```zig
export fn mutate_ffi(
    noalias buf: [*]u8,
    len: usize,
    num_mutations: usize,
) callconv(.C) void {
    mutate(buf[0..len], num_mutations)
}

fn mutate(buf: []u8, num_mutations: usize) !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    var alloc: std.mem.Allocator = arena.allocator();
    defer _ = arena.deinit();

    var mutator = try Mutator.init(MAX_SIZE, alloc);
    mutator.input(buf);

    try mutator.mutate(num_mutations);
}
```

## TODOs

- [ ] Support coverage guided fuzzing
- [ ] Support taint tracking
- [ ] Implement all mutations strategies available
- [ ] Criterion benchmarks from Rust
- [ ] Make it a usable Rust library. How to ship the zig static lib?

## License

MIT
