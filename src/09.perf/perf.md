# Performance
$web-only$
Below you will find a table of benchmark information for each of the 4
implementations. There are two benchmarks for each implementation, the first is parsing just 1 duration, the second is parsing 1000 durations, these are all created using `lazy_static` so each parser is provided the same input. For the [build time](#build-time) and [build size](#build-size) benchmarks, I used the following binary application with feature flags to conditionally use each of the implementations.

```rust
{{#include main.rs}}
```

### Build time
This is the time it took to run `cargo build` on each implementation. With the increasing improvement of incremental compilation, it isn't the most important metric but some people might care about it.

### Bin size
This is the size of the program when built using `cargo build --release`.

### Parse 1
This is the time `cargo +nightly bench` reported for this parser to parse 1 duration.

### Parse 1000
This is the time `cargo +nightly bench` reported for this parser to parse 1000 durations.

### Benches
```rust
{{#include benches.rs}}
```

$web-only-end$

{{#include benches.md}}
