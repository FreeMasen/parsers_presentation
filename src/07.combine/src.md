# Combine
$web-only$
To start, I want to point out how `combine` is able to create combinators w/o using macros. It heavily leverages the `impl Trait` feature released this year. looking the signature of the `float` parser:

```rust
fn float<I>() -> impl Parser<Input = I, Output = f32>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
```
We are saying that this function will return a parser who's input is of type `I` and output is an `f32`. Additionally we constrain `I` saying it needs to implement `Stream`, where the `Item` type is a `char`, and the `Error` type of `I` needs to implement `ParseError`. `ParseError` is also generic so we pass along the properties of `I`, `Item`, `Range`, and `Position`. This is pretty verbose, but I found that building a simple parser I didn't need to worry a ton about what it meant, instead just using it as a blueprint for all of my parser functions. That is to say, I just copied and pasted this whenever I wanted to add another parser to a source file.

One thing to keep in mind is that these functions are not parsing the input but returning a struct that will be able to parse the input. We define these structs by combining them together and mapping over the result if successful. To indicate that parsers are chained together in a sequence we wrap them in either an array if they are all the exact same type otherwise we would use a tuple. Looking at the body of `float` we are using a tuple, with two parers inside it. The first parser is `many1` this takes a parser as an argument and applies it a minimum of 1 times but will collect the results until in argument parser fails, we passed in the result of calling `digit` which is a parser that will get us a single digit number. Next we have `optional` this takes in a parser and wraps the result in an `Option`, we pass it a tuple of 2 parsers the `char` parser, this is a single character and another `many1` with `digit` as its argument. Notice in this second call to `many1` we included a type annotation, `many1` operates similar to the `collect` method on an `iterator`, the result could be a number of different collections we are just telling it it should be a `String`.

We now call `map` on our tuple, this essentially is saying if you find the pattern we defined, call this closure to generate the result. In this closure we are going to check and see if the remainder exists and coalesce that into a string with the integer portion if not just use the integer portion. We then call `str::parse` on that coalesced value.

Moving up the file we next see `value_pair` the signature is similar to `float` though the `Output` type is `DurationPart` and it takes in an argument `time` to determine if `M` is a minute or a month. Again here we have a tuple, the first parser is the `float` parser we just defined, next is a `choice` parser, this takes in an array or tuple of parsers and tries each one starting at the top, stopping at the first successful. We have passed `choice` an array of `char` parsers with our unit characters. We map our tuple, this time our closure has the `move` keyword to take ownership of the `time` argument. We have annotated the argument to our closure to help the compiler figure out what we are trying to do here. `map` on a parser always takes 1 argument, this will be a tuple of the results, for us that is `f32` and `char`. We simply match on the `char` and generate the correct `DurationPart` variant as per the letter and the time flag.

Next we have the `time_part` parser, here we have another tuple, the first parser is a call to `char` with our `T` indicator that this is time based values, the second is a `many1` call to the `value_pair` parser we just defined, passing `true` for the `time` flag. The map here is simply discarding the `T` character, that make the `Output` type annotation work for `p` which means it also work for `many1`.

After `time_part` we have `date_part` this one is just the `many1` call to `value_pair` passing `false` for the `time` flag.

The last of the parsers we are going to define is `duration`, this is another tuple indicating the sequence of a `char` of `P` followed by an `optional` `date_part` followed by an `optional` `time_part`. We `map` that, the argument to the `map` closure would be `(char, Option<Vec<DurationPart>>, Option<Vec<DurationPart>>)`. In the body we loop over those two `Vec`s if they exist, adding them to a `Duration` returning the result.

In our `parse` function we create a `Duration` parser by calling `duration` and then pass a `&str` to the `parse` method, this is one of the methods defined on the `Parser` trait. `Parse` returns a `Result` and in the `Ok` position we have a tuple, the first item has the same type as `Output` and the second is the remaining input to parse. For us the `Output` is going to be a `Duration` so we return `Ok(d.0)`

{{#playpen lib.rs}}
$web-only-end$
$slides-only$
# Demo Time!
$slides-only-end$