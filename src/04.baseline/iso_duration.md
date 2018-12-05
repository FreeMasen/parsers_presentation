# Our Format

$web-only$
As we investigate these options, we should probably have a format that we want to parse to keep each example consistent.

One format that I think is simple enough to be digestible while being complicated
enough to be interesting is the ISO 8601 Duration format. ISO 8601 is the date/time specification that aids in making dates work across cultures, sometimes
referred to as I18n. You may be familiar time libraries that advertize their 8601
support, [chrono](https://github.com/chronotope/chrono#readme) for example.

> As an aside, I18n is an abbreviation for Internationalization, it is used because there are 18 letters between the I and the last n

The duration format is dynamic, yet simple. Each duration will start with the character `P`, a list of number letter pairs. The number can be an integer or decimal value, the letter is a single capital letter. The units are broken into two sections separated by a `T`. The first section includes `Y`ear, `M`onth, `W`eek, and `D`ay and the second section include `H`our, `M`inute, and `S`econd.
$web-only-end$

$slides-only$
## ISO 8601 Durations
- A standard way to encode a length of time into a string
- I18n?

### Basic Description
- Start with a capital letter `P`
- up to 7 Decimal number + letter pairs
- Date half & Time half (separator: `T`)
- letters: `Y`, `M`, `W`, `D`, `H`, `M`, `S`
- Each is optional, minimum of 1 is required
- If at least one unit that comes after Days is present they must be preceded by `T`

### Examples
$slides-only-end$
$web-only$
Here are some examples of different lengths of times as ISO 8601 Durations.
$web-only-end$

- One Day: `P1D`, `PT24H`, `P0.14285W`
- One Hour: `PT1H`, `P0.041666D`
- One of Everything: `P1Y1M1W1DT1H1M1.1S`

$web-only$
With any data format, there are a few things here that could be problematic in a
real world scenario. First, the size of each number is unspecified meaning
deserializing a duration with two large a value could lead to overflow. Second,
the larger of the units don't always have a consistent or clear meaning, a good
example of this would a `M`onth, using the gregorian calendar a month could
be anywhere from 28 to 31 days. I bring this up because theses are important
things to think about when approaching deserialization or parsing process.
So long as both the serializer and deserializer agree on the meaning of
any given symbol, everything should work out just fine.
$web-only-end$