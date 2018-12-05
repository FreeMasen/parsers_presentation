# Pest

$web-only$
Before we start digging into the rust code, we should first cover the grammar file.

It looks a lot like our BNF grammar, the biggest difference is that we have the opportunity to use some more flexible notation. For example the instead of having one rule for optional values and another for non-optional, we can use the `?` to say that any existing rule is optional. When noting that values repeat, we can use `+` to indicate 1 or more and the `*` to indicate 0 or more. These might be familiar to you if you have used regular expressions.

Some other things to keep in mind when using the `pest` grammar syntax, the right hand side of a rule needs to be wrapped in curly braces and each segment should be separated with `~`. There are some more advanced things you can do with this style but we don't need them here.

Starting from the bottom again, first we define our `Decimal` rule, this is really just an alias for the `ASCII_DIGIT` rule provided by `pest`.

```
{{#include duration.pest:28}}
```
Next we have `Integer` which is 1 or more `decimal`s.

```
{{#include duration.pest:27}}
```
Then `Remainder`, a period followed by an integer, notice that strings need to be wrapped in double quotes.

```
{{#include duration.pest:26}}
```

Now we can define our `Number` rule as either an `Integer` with an optional `Remainder` or an optional `Integer` followed by a `Remainder`.

```
{{#include duration.pest:23:25}}
```

Above that is all of our unit/value pairs.

```
{{#include duration.pest:17:22}}
```
Followed by the `time_section` and `date_section` rules.

```
{{#include duration.pest:5:16}}
```

All the way at the top we have the `Duration` rule.

```
{{#include duration.pest:2:4}}
```
Now for the rust part, to start we are going to use a derive provided by pest for their trait `Parser`. The derive allows for an attribute `grammar` which should be assigned the relative plath to the grammar file. We apply these to a unit struct, I called mine `DurationParser`.

```rust
{{#include lib.rs:12:14}}
```

This will create an enum called `Rule` that will have one variant for each of the rules in our grammar file. Here it would look something like this.

```rust
enum Rule {
    Duration,
    DateSection,
    TimeSection,
    Year,
    Week,
    Day,
    MinuteOrMonth,
    Second,
    Number,
    Remainder,
    Integer,
    Decimal,
}
```

Inside of the `parse` function, the first thing we do is call `DurationParser::parse`, providing the rule we are looking to parse, in this case `Rule::Duration` and the `&str`.

```rust
{{#include lib.rs:16:23}}
```
This is going to return a `Result` with a `Pairs` in the success position. `Pairs` is an `iterator` over `Pair`. For our case, we just need to first `Pair` so we can call `next` to get that. Once we have that we can pass it off to `assemble_parts`, which will take the `Pair` and pull out the inner rules. You can think about that in the same way our grammar is layed out, the `Duration` rule had `DateSection` and `TimeSection` in its definition, so the inner pairs would be one of these two variants of the `Rule` enum.

```rust
{{#include lib.rs:25:38}}
```

Once we have the inner values we are going to loop over them and pass it off to `assemble_part`.

```rust
{{#include lib.rs:41:46}}
```

This is again going to pull out the inner `Pair` which should be one of the unit value rules. Once it has pulled that out it passes that pair off to `update_duration`.

```rust
{{#include lib.rs:48:84}}
```

Here we are going to first get the float value from the pair, we do this by calling `as_str` on the `Pair` which gives the full slice of the original, we know the last character is the unit so we call `parse` on the sub string not including that. Now that we have the value, we can just match on the `Pair::as_rule` which will be one of our unit variants. At each stage we have passed down a mutable reference to the `Duration` we are assembling, making it easier to update it as needed. That is it, we off loaded quite a bit of the logic to the parser generator.

Here are the full grammar and rust files.

```
{{#include duration.pest}}
```

```rust
{{#include lib.rs}}
```
$web-only-end$
$slides-only$

# D-E-M-O
# demo!
# demo!

$slides-only-end$