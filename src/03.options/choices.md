# What We'll Cover
$slides-only$
- `nom`
    - Macro based
- `pest`
    - External Grammar Files
- `combine`
    - `impl Trait` based
$slides-only-end$
$web-only$
First, I wanted to leverage the tools that already existed to make my life a little easier. I searched for a lower level byte parsing library, and found 3 options that I wanted to investigate.

The three crates were `nom`, `pest`, and `combine`.
Three popular options that take different approaches.
`nom`'s approach is to use macros to define parsing pipelines called combinators.
`combine` also leverages combinators but instead of using macros, it uses regular rust functions.
`pest` is a PEG parser generator, meaning it utilizes an external grammar file to generate a parser struct for you.
$web-only-end$