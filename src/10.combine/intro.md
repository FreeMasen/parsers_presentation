# Combine (revisited)

$web-only$
I wanted to revisit `combine`, while the original works, I was able to create an implementation that is actually faster than `nom`'s. Doing so required quite a bit of additional work in understanding how the "zero-copy" tools in `combine` work. In truth I still haven't been able to figure out how the type annotations need to be set for this implementation to work, instead each parser is assigned to a variable inside of a function body. With the following implementation the `combine` parser, the whole set of benchmarks are:

$web-only-end$
$slides-only$
- Learning Curve gets steeper
- Performance gets better
$slides-only-end$

| crate       | parse 1 (+/-)      | parse 1000 (+/-)   | build time | bin size  |
| ----------- | ------------------ | ------------------ | ---------- | --------- |
| combine     | 893.00ns (37.00ns) | 1.17s (73.91ms)    | 25.41s     | 723.99 kb |
| nom         | 1.33ms (69.00ns)   | 833.53ms (37.55ms) | 10.66s     | 727.08 kb |
| pest        | 3.66ms (342.00ns)  | 3.89s (279.92ms)   | 31.31s     | 767.86 kb |
| hand_rolled | 694.00ns (93.00ns) | 551.86ms (77.66ms) | 10.81s     | 718.87 kb |

$web-only$
```rust
{{#include lib.rs}}
```
$web-only-end$