## Nom
$web-only$
A good place to start when learning to use `nom` is the `named!` macro, this macro will create a function for you with a specific name and behavior. `named!` takes two arguments, the first is the function's name, this needs to include 2 generic arguments, the second is the function's body. This can be a little difficult to get used to but it isn't too much of a pain. The generic arguments provided with the name define the data type of the argument and return type respectively.

This is already starting to feel difficult to explain, I will re-iterate one last time that the learning curve for `nom` is pretty steep.

Just like with our grammar, let's start at the bottom and move up. The first `named!` entry we have in that direction is called `float`, which takes `CompleteStr` and returns `f32`.
```rust
{{#include lib.rs:84:86}}
```

The second argument is two nested macro calls. To help understand this conceptually here is a very loose interpretation of what this might look like as a function.

```rust
fn float(i: CompleteStr) -> Result<f32, Error> {
    map_res!(take_while1!(i, digit), parse_float)
}
```

While the above is an extreme simplification, I hope it illustrates what is going on when we use the `named!` macro.

To dig into the function body a little, we are using macros provided by `nom`, the first is `map_res!` which takes two arguments. The first argument is going to be a parser, this will capture some portion of the input, the second argument is a function that will take the result of the first argument's parsing and return a `Result`. The basic idea here is that we are going to map the output of a parser with the function, since the function might fail `map_res!` will convert the function's error case into a `nom` error. The first argument is `take_while1!`, another `nom` macro, this takes a function that takes 1 argument, this argument needs to be the smallest part of the input which for this would be a `char` the function then needs to return a boolean, indicating if that `char` matches what we are expecting. To flesh that our a little we should probably cover the two helper functions we are using `digit` and `parse_float`.
```rust
{{#include lib.rs:88:94}}
```
If we had the input `111.111H`, we would first use `take_while1` to pass each character to `digit`.

| 1 | 1 | 1 | . | 1 | 1 | 1 | H |
|---|---|---|---|---|---|---|---|
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |

that would leave us with `111.111`, this would get passed along to `parse_float`, which would just call the `parse` method on `&str`. Since the `parse` method returns a result, this would work for the requirement of `map_res!`. Now that we have a way to parse
the number part of our number + letter pairs, we can move on to the next use of `named!`.

```rust
{{#include lib.rs:75:81}}
```

Here we have defined `duration_part` this will take us from a `CompleteStr` to a tuple with the type `(f32, CompleteStr)`, it does this by using the `do_parse!` macro. `do_parse!` is our primary way to declare a parsing pipeline, we provide it with a list of parsers and the remaining input should be applied to them in order. Here we see the use of a couple of custom syntax items. The first is how we store a parser's result in a variable, we do that by putting the variable name and a colon before the parser (`value: float`). The second bit of custom syntax is the `>>` operator, this essentially means, "and then". To break down what we are doing here, first we are using the `float` parser defined above, once done with that pass any remaining input on to the parser `take!`, which doesn't look at the input, just takes the number of characters requested, in our case 1.

At this point, you may be asking yourself "why does take! need to be called but float doesn't?", that is a great question. One of the things that is hardest to get used to is that all of the `nom` macros return functions, not values. To break this down a little `take!` doesn't return the character, it returns a function that returns the character, it might look something like this.

```rust
fn take(i: CompleteStr) -> Result<CompleteStr, Error> {
    if i.len() >= 1 {
        Ok(&i[0..1].into())
    } else {
        Err(...)
    }
}
```
That means the result of `take!` can be called, just like the `float`. To put it another way, we are not parsing the input, instead we are building a parser out of other parsers that will parse the input.

To finish the explanation of this `do_parse!`, the last step in any use of `do_parse!` we need to have a tuple, this will be the return value, for us that is the tuple of `(value, flag)` which is of type `(f32, CompleteStr)`.

At this point we should be able to parse a duration's smallest part. Now we need to figure out how to get that into our `DurationPart` enum. Then next item moving up the file is the helper function `combine_duration_part`.

```rust
{{#include lib.rs:59:73}}
```
This is going to take in the two parts the `f32` value and the `flag` as well as a third argument to indicate if an `M` means minutes or months.

We are going to use this as we are building the two halves. The function itself is relatively simple, just a `match` statement on the flag, matching the appropriate letter to the duration part.

As we move up the file, we have two more `named!` macros `date_part` and `time_part`.

```rust
{{#include lib.rs:44:57}}
```

These are almost identical, essentially they use the `duration_part` to get first two arguments of `combine_duration_part`, it then provides the appropriate value for the third.

As we continue up we have a `named!` item to parse the `time_parts`,

```rust
{{#include lib.rs:36:42}}
```

Here we have another call to `do_parse!` the first item here is is a call to `tag!`, another `nom` macro. This one simply recognizes whatever string you provide to it. In our case we are matching against the `T` that would begin the time section. It then moves on to another provided macro `many_m_n!`, this one takes three arguments, the first is the minimum number of times you expect something, the second is the maximum number of times you expect something, and the third is what you expect. In our case we expect a `time_part` at least 1 time, up to 3 times. We are going to assign the result of that to the variable `time_parts`, this value will be a `Vec<DurationPart>`. Since the last item in `do_parse!` needs to be a tuple, we wrap `time_parts` in parentheses.

Up next is another helper function, `combine_duration_parts`

```rust
{{#include lib.rs:20:34}}
```

This takes in two `Option<Vec<DurationPart>>`, the first will represent the date part and the second will represent the time part of our duration. Again this helper is mostly straight forward, it loops over both, sets of parts (if they exist) and adds them all up into a `Duration`.

Our last `named!` is the final step for combining all of our previous work.

```rust
{{#include lib.rs:11:18}}
```
Again we start with a `do_parse!`, this one begins with a `tag!` for the first letter `P`, we pass the rest on to another `nom` macor `opt!`, this converts a standard parser into a one that returns an `Option`, we pass the first one another `many_m_n!` that will generate a `Vec<DurationPart>` with a length from one to four, assigning the result to a variable `date_parts`. It then passes the remainder of the text to another `opt!`, this one around `time_parts` and assigns the result to a variable of the same name. Now we have the two `Option<Vec<DurationPart>>`s that we need to pass them along to the `combine_duration_parts` helper function.

With all that we have finally built a `nom` parser that would take in our duration string and return a `Duration`. You will notice in the `parse` function that we actually call the last `named!` item manually providing the argument. `nom` parsers always return a `Result`, the `Ok` case for this result is going to be a tuple, the first index will be the remainder of the input, the second will be a `Duration`.

You can find the whole parser file below.

{{#playpen lib.rs}}

$web-only-end$
$slides-only$

# Now for a demo!

$slides-only-end$

