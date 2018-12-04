# Pest

$web-only$
Before we start digging into the rust code, we should first cover the grammar file.

It looks a lot like our BNF grammar, the biggest difference is that we have the opportunity to use some more flexible notation. For example the instead of having one rule for optional values and another for non-optional, we can use the `?` to say that any existing rule is optional. When noting that values repeat, we can use `+` to indicate 1 or more and the `*` to indicate 0 or more. These might be familiar to you if you have used regular expressions.

Some other things to keep in mind when using the `pest` grammar syntax, the right hand side of a rule needs to be wrapped in curly braces and each segment should be separated with `~`. There are some more advanced things you can do with this style but we don't need them here.

Starting from the bottom again, first we define our `decimal` rule, this is really just an alias for the `ASCII_DIGIT` rule provided by `pest`. Next we have `integer` which is 1 or more `decimal`s. Then remainder, a period followed by an integer, notice that strings need to be wrapped in double quotes. Now we can define our `number` rule as either an `integer` with an optional `remainder` or an optional `integer` followed by a `remainder`. Above that is all of our unit/value pairs followed by the `time_section` and `date_section` rules. All the way at the top we have the `Duration` rule.

```
{{#include duration.pest}}
```
Now for the rust part, to start we are going to use a derive provided by pest for their trait `Parser`. The derive allows for an attribute `grammar` which should be assigned the relative plath to the grammar file. We apply these to a unit struct, I called my `DurationParser`. This will create an enum called `Rule` that will have one variant for each of the rules in our grammar file. For us this would look something like this.

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
Inside of the `parse` function, the first thing we do is call `DurationParser::parse`, providing the rule we are looking to parse, in this case `Rule::Duration` and the `&str`. This is going to return a `Result` with a `Pairs` in the success position. `Pairs` is an `iterator` over `Pair`. For our case, we just need to first `Pair` so we can call `next` to get that. Once we have that we can pass it off to `assemble_parts`, which will take the `Pair` and pull out the inner rules. You can think about that in the same way our grammar is layed out, the `Duration` rule had `DateSection` and `TimeSection` in its definition, so the inner pairs would be one of these two variants of the `Rule` enum.

Once we have the inner values we are going to loop over them and pass it off to `assemble_part`. This is again going to pull out the inner `Pair` which should be one of the unit value rules. Once it has pulled that out it passes that pair off to `update_duration`. Here we are going to first get the float value from the pair, we do this by calling `as_str` on the `Pair` which gives the full slice of the original, we know the last character is the unit wo we call `parse` on the sub string not including that. Now that we have the value, we can just match on the `Pair::as_rule` which will be one of our unit variants. At each stage we have passed down a mutable reference to the `Duration` we are assembling, making it easier to update it as needed. That is it, we off loaded quite a bit of the logic to the parser generator.

```rust
{{#include lib.rs}}
```
$web-only-end$
$slides-only$

# D-E-M-O
# demo!
# demo!

$slides-only-end$