# Combine (revisited)

$web-only$
I wanted to revisit `combine`, while the original works, I was able to create an implementation that is actually faster than `nom`'s. Doing so required quite a bit of additional work in understanding how the "zero-copy" tools in `combine` work. With the following implementation the `combine` parser, the whole set of benchmarks are:

$web-only-end$
$slides-only$
- Learning Curve gets steeper
- Performance gets better
$slides-only-end$

| crate       | parse 1 (+/-)      | parse 1000 (+/-)   | build time | bin size  |
| ----------- | ------------------ | ------------------ | ---------- | --------- |
| combine (0) | 2.72ms (7.08ms)    | 2.54s (6.72s)      | 35.15s     | 723.99 kb |
| combine     | 7.46ms (3.26ms)    | 4.14s (12.14s)     | 32.13s     | 739.68 kb |
| nom         | 1.33ms (69.00ns)   | 833.53ms (37.55ms) | 10.66s     | 727.08 kb |
| pest        | 3.66ms (342.00ns)  | 3.89s (279.92ms)   | 31.31s     | 767.86 kb |
| hand_rolled | 694.00ns (93.00ns) | 551.86ms (77.66ms) | 10.81s     | 718.87 kb |