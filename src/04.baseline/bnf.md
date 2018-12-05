# BNF Grammar

$web-only$
At this point I want to take a moment and cover the concept of language grammars.
In my experience most parsing resources will assume that you have at least
a little knowledge about the concept making these resources difficult to use if
you don't. If you already know a bit about language grammars, feel free to skip
this page.

Essentially grammars are a way to write a definition for a language with the
goal of making it easier to talk about that language. I am using the term
language here pretty broadly to mean any agreed upon data format. That means that
the 8601 Duration format could be thought of as a language though most people would
not consider it one.

There are many options to choose from when trying to document a grammar,
just like there are many language options to choose from when building software.
I am going to use the Backus-Naur Form (BNF). We will going to walk through
a BNF grammar describing the ISO 8601 Duration format piece by piece but
if you wanted to look at the full thing you can scroll to the bottom of this page.

For those unfamiliar with grammar forms, they consist of a series of rules,
the left side of a rule is a name, the right side is a description of what
that rule means, in BNF the sides are separated by `::=`.

I find it easiest to read a grammar from the bottom up. For our duration format
we would start with the rule `digit`

```xml
{{#include duration.bnf:26}}
```
On the right side of this rule we see each number from 0 to 9 separated by a
`|`, which here means *or*, so a digit is a single digit number from 0
through 9. Next we have the `integer` rule.

```xml
{{#include duration.bnf:25}}
```
An `integer` is either a single digit or an `integer` followed by a `digit`.
This is where things can get a little confusing as this style of
notation might feel backwards, at least it does to me. Lets use the
example `999`, if you think about it starting with the right most `9`,
you would assign that position `<integer><digit>`, the `<integer>` here
would point to the middle `9`, this would also be assigned `<integer><digit>`
and again the `<integer>` would represent the left most `9`, this would
finally be assigned `<digit>`. Here is a little flow chart to hopefully
help visualize what I am trying to say.

```xml
<integer> = 9
    ┗━━━━━━━━━━━┓
<integer> = <integer>9
    ┗━━━━━━━━━━━┓
<integer> = <integer>9
```

When the right side of a rule looks like this it is referred to as a
"left recursive" rule. In this case we could also write it as `<digit><integer>` making it "right recursive". The more important take away is that if you see a
rule in its own definition then it *could* go on forever in one direction or the other.

Moving up the grammar, next we have the `remainder` rule.

```xml
{{#include duration.bnf:24}}
```
This rule is defined by a `.` followed by an `integer`. At this point it
should become clear that as we move up the grammar, each rule will combine
the previous rules, possibly with some additions which is why I like to
start from the bottom. Next up we have `number`.

```xml
{{#include duration.bnf:23}}
```

A `number` is either an `integer` or a `remainder` or an `integer` followed
by a `remainder`. This is a bit verbose but essentially we need a way
to say that both are optional but at least one must exist. So `0` would
work, also `.877` would work and finally `0.877` would work.

The next few rules finally start to get into the specifics of the format.
There are two rules for each of our number + letter pairs. Since we are
using BNF the only operator we get is the `|` for or, some other grammar
notations use other operators to make things a little more concise. If you have
ever written a regular expression, you would be familiar with the `+` or `?`
operators for declaring recursion or optional values. In BNF we are required
to create a new rule for the optional case if we want to have both optional
and non-optional. Take the `seconds` and `seconds-opt` rules as an example.

```xml
{{#include duration.bnf:21:22}}
```
The bottom one fits with what we went over on the previous page, a number
followed by the capital letter `S`. The top one is just a way to make the
previous rule optional. There is an entry like the above for each of our
duration parts. After those we get to the `time` rule, this rule will
hopefully make it clear why we needed those optional rules.

```xml
{{#include duration.bnf:8:10}}
```
> minth is used here as both `M`onth and `M`inute

This rule has 3 options, all three start with the letter `T` and each is
followed by all 3 of the time rules, in each case one of the time rules
is not optional. This is saying that each of these parts can be optional
but not all of them can be absent. So `T1H` or `T2M1S` are okay but `T`
is not a valid `time`. Skipping ahead to the `date` rule we see a similar
pattern.

```xml
{{#include duration.bnf:3:6}}
```

So a `date` can be `1D` or `1M1W` but couldn't just be empty. You might
have noticed that we again have an optional version of `time`, just
like we did for the duration unit rules. When we look at the `duration`
rule, the top rule in the grammar, it should become clears as to why.

```xml
{{#include duration.bnf:1:2}}
```
As we covered in the previous page, durations can have from 1 to 7
number letter pairs. So this rule is saying that we can have a duration
only with a date part (`P1M`) or a date part and a time part time part
(`P1DT1H`) or just a time part(`PT1S`) but it cannot be empty (`P`).

If you haven't already below is the full BNF grammar I wrote for this
the ISO 8601 Duration format.
$web-only-end$

```xml
{{#include duration.bnf}}
```