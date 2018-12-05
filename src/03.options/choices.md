# What We'll Cover
$slides-only$
- `nom`
    - Macro based
- `combine`
    - `impl Trait` based
- `pest`
    - External Grammar Files
$slides-only-end$
$web-only$
The first choice to make when building a parser is which of the parser
crates to use, if one at all. In my search for a parsing crate I found
3 options that I wanted to investigate.

The three crates were `nom`, `pest`, and `combine`.
Three popular options that take different approaches.
`nom`'s approach is to use macros to define parsing pipelines called combinators.
`combine` also leverages combinators but instead of using macros,
it uses regular rust functions.
`pest` is a PEG parser generator, meaning it utilizes an external grammar
file to generate a parser for you.
$web-only-end$