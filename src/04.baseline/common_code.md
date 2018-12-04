# Common Code Items
$web-only$
Before we dig into each implementation, I want to cover a few things that are common
to most of the examples. The first thing they share is the following enum.
$web-only-end$
```rust
pub struct Duration {
    years: Option<f32>,
    months: Option<f32>,
    weeks: Option<f32>,
    days: Option<f32>,
    hours: Option<f32>,
    minutes: Option<f32>,
    seconds: Option<f32>,
}
```

```rust
enum DurationPart {
    Years(f32),
    Months(f32),
    Weeks(f32),
    Days(f32),
    Hours(f32),
    Minutes(f32),
    Seconds(f32),
}
```
$web-only$
This will represent a successfully parsed number/unit pair. so `100Y` would translate to `DurationPart::Years(100.0)`

The next thing that pops up across multiple crates is the need to parse a string into a float.

> quick note:, the values are not actual floating point numbers as the spec does not allow for scientific notation (`1.2e-3`), they would be more akin to the decimal data type provided in some languages.

For this we are going to lean on the implementation in the standard library so it will typically just look like this.
$web-only-end$

```rust
let value_str = "1.222";
let value: f32 = value_str.parse();
```

$web-only$
Here the `parse` method on `&str` returns a result, so we would need to deal with that as well.
$web-only-end$

$web-only$
The last thing to cover here is that each parser will need to deal with the fact that `M` can mean either month or minute. While there isn't a shared code solution for this, it does pop up a few times.
$web-only-end$