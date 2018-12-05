# Combine

## Overview
$web-only$
`combine` takes a similar approach to `nom` in that it asks its users to combine a series of parsers into something that itself is a parser though its approach to creating a parser does not involve macros. This departure makes the learning curve much shallower but if you want to get `nom`-like speeds it is somewhat difficult to learn how to do that.
$web-only-end$
$slides-only$
- Trait based combinators
- Quick to learn the basics
- Slow w/o advanced knowledge
$slides-only-end$

## Positives
$web-only$
After attempting to learn `nom`, `combine` feels like a breeze. The type annotations are a little verbose but there is no macro syntax to learn. Their documentation and examples make getting started very easy. While the naive approach to building a parser in `combine` will almost certainly not be blazing fast, *can* achieve some impressive performance.
$web-only-end$
$slides-only$
- No new syntax to learn
- Good documentation
- Can be as fast as `nom`
$slides-only-end$

## Negatives
$web-only$
The biggest positive also leads to the biggest negative, the naive implementation will never be as performant as a `nom` parser. This comes down to weighing performance vs productivity, you can be more productive more quickly with `combine` but that parser might not be the fastest. The other big negative is that when you start digging into the more advanced options, it can be difficult to correctly annotate the types for a parser. The compiler is good at inferring the types but if you wanted to break something into its own function, you need those type annotations.
$web-only-end$
$slides-only$
- Slow, w/o tuning
- Types can be hard
$slides-only-end$