## ISO 8601 Duration Parser
$web-only$
Below you will find my attempt at using `nom` to create a parser for our duration format.

A good place to start when learning to use `nom` is the `named!` macro, this macro will create a function for you with a specific name and behavior. `named!` takes two arguments, the first is the function's name, this needs to include 2 generic arguments, the second is the function's body. This can be a little difficult to get used to but it isn't too much of a pain. The generic arguments provided with the name define the data type of the argument and return type respectively.

This is already starting to feel difficult to explain, I will re-iterate one last time that the learning curve for `nom` is pretty steep.

Just like with our grammar, let's start at the bottom and move up. The first `named!` entry we have in that direction is called `float`, which takes `CompleteStr` and returns `f32`. The second argument is two nested macro calls. To help understand this conceptually here is a very loose interpretation of what this might look like as a function.

```rust
fn float(i: CompleteStr) -> Result<f32, Error> {
    map_res!(take_while!(digit), parse_float)
}
```
While the above is an extreme simplification, I hope it illustrates what is going on when we use the `named!` macro.

To dig into the function body a little, we are using two macros provided by `nom`, `map_res!` which takes a parser and a `Fn(I) -> Result<T, Error>` where `I` is the input type, for us that would be `CompleteStr` and `T` is the return type which for this would be `f32`. The first argument we are using `take_while1!`, this is another `nom` macro the only argument it takes is a `Fn(B) -> bool`, where `B` is the smallest part of the input, in our case that would be `char` since `CompleteStr` is a collection of `char`s. What this is going to do look at each `char` in the input and pass that to our test function, which checks to see if it is `0-9` or `.`, if it is one of those the `char` is added to the output, if not it stops and returns all of the `char`s it found as a `CompleteStr`. The last thing to note about this is that `take_whle1!` requires at least the first character it tries to be successful otherwise it fails. If we had the string `111.111D`, `take_while1!` would pull the first 7 characters ('111.111') off the front and return that.

When we combine these two parts, `float` is going to pull 1 or more characters off the front of our input and then attempt to convert that into an `f32`, this will give us the value part of our `DurationUnit`.

Moving up to the next `named!` we have `duration_part`, this takes a `CompleteStr` and returns a `(f32, CompleteStr)`. Looking at the body of `duration_part` we have another new `nom` macro `do_parse!`. This is a very powerful tool in the `nom` tool shed, `do_parse!` will take any number of parsers and apply them in sequence. We indicate the flow of the parsers using the `>>` operator, we can assign the result of any parse to a variable by putting that variable name and a colon before it. `do_parse!` requires the final item in your flow to be a tuple with the expected values. For the `duration_part` body, we first use the `float` parser, that will take all of the leading numbers off the front and convert them into an `f32` like we went over before. notice you are not calling `float()`, I'll get to that in just a moment. we are going to assign that `f32` to the variable `value`, once that is done we take the rest of our input and pass it to `nom`'s macro `take!`, which takes 1 argument of a `usize`. Similar to `take_while1!`, `take!` pulls stuff off the front of the input in the smallest parts possible, for us again that will be `char` but instead of using a function to test if we want a character it just takes the number of characters you passed in. For us this `take!` is just going to take 1 character.

At this point, you may be asking yourself "why does take! need to be called but float doesn't?", that is a great question. One of the things that is hardest to get used to is that all of the `nom` macros return functions, not values. To break this down a little `take!` doesn't return the character, it returns a function that returns the character.

```rust
fn take(i: CompleteStr) -> Result<CompleteStr, Error> {
    if i.len() >= 1 {
        Ok(&i[0..1].into())
    } else {
        Err(...)
    }
}
```
That means the result of `take!` can be called, just like the `float`.

Moving along, we are assigning that single character string to the variable `flag`, we then end with the tuple `(value, flag)` which is of type `(f32, CompleteStr)`.

So now we should be able to successfully to parse our duration's smallest parts. Now we need to figure out how to get that into our `DurationPart` enum. Then next item moving up the chain is the helper function `combine_duration_part` this is going to take in the two parts the `f32` value and the `flag` as well as a third argument to indicate if an `M` means minutes or months.

We are going to use this as we are building the two halves. The function itself is relatively simple, just a `match` statement on the flag, matching the appropriate letter to the duration part.

As we move up the file, we have two more `named!` macros `date_part` and `time_part`. These are almost identical, essentially they use the `duration_part` to get first two arguments of `combine_duration_part`, it then provides the appropriate value for the third.

As we continue up we have a `named!` item to parse the `time_parts`, here we have another call to `do_parse!` the first item here is is a call to `tag!`, another `nom` macro. This one simply recognizes whatever string you provide to it. In our case we are matching against the `T` that would begin the time section. It then moves on to another provided macro `many_m_n!`, this one takes three arguments, the first is the minimum number of times you expect something, the second is the maximum number of times you expect something, and the third is what you expect. In our case we expect a `time_part` at least 1 time, up to 3 times. We are going to assign the result of that to the variable `time_parts`, this value will be a `Vec<DurationPart>`. Since the last item in `do_parse!` needs to be a tuple, we wrap `time_parts` in one.

Up next is another helper function, `combine_duration_parts`, this takes in two `Option<Vec<DurationPart>>`, the first will represent the date part and the second will represent the time part of our duration. Again this helper is mostly straight forward, it loops over both of, sets of parts (if they exist) and adds them all up into a `Duration`.

Our last `named!` is the final step for combining all of our previous work. Again we start with a `do_parse!`, this one begins with a `tag!` for the first letter `P`, we pass the rest on to another `nom` macor `opt!`, this converts a standard parser into a one that returns an `Option`, we pass the first one another `many_m_n!` that will generate a `Vec<DurationPart>` with a length from one to four, assigning the result to a variable `date_parts`. It then passes the remainder of the text to another `opt!`, this one around `time_parts` ad assigns the result to a variable of the same name. Now we have the two `Option<Vec<DurationPart>>`s that we need to pass them along to the `combine_duration_parts` helper function.

With all that we have finally built a `nom` parser that would take in our duration string and return a `Duration`. You will notice in the `parse` function that we actually call the last `named!` item manually providing the argument. This will always result in a `Result`, the `Ok` case for this result is going to be a pair, the first will be the remainder of the input, the second will be a `Duration`.

$web-only-end$
$slides-only$

### Now for a demo!

$slides-only-end$

{{#playpen lib.rs}}

<script>
(function() {
    var pp = getPlaypen();
    var mode = localStorage.getItem('presentation_mode');
    if (!pp) return;
    pp.classList.add('article-content');
    if (mode == 1) {
        pp.classList.add('not-presenting');
    } else if (mode == 0) {
        pp.classList.add('presenting');
    }

    function getPlaypen() {
        var cls = '.playpen';
        var pre = document.querySelectorAll(cls);
        if (!pre || pre.length < 1) return console.error('failed to find', cls);
        pre = pre[pre.length - 1];
        if (!pre || !pre.parentElement) return console.error('unknown structure of', cls);
        return pre.parentElement;
    }
})()
</script>