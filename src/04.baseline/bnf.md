# BNF Grammar

$web-only$
Another thing that will prove useful if you are interested in getting started with parsing
is to understand that there are languages out there for talking about languages. The overall structure
of any format is called a grammar, when explaining the structure of a format it is helpful to have
method to communicate it. One option is to use the Backus-Naur Form (BNF), if you scroll down you will find the ISO 8601 Duration specification written in the BNF.

For those unfamiliar with grammar forms, they consist of a series of rules, the left side of a rule is a name, the right side is a description of what that rule means, they are separated by `::=` in BNF.

I find it easiest to read a grammar from the bottom up. Using the example below we have the `digit` rule, on the right side of this rule we see each number from 0 to 9 separated by a `|`, which here means *or*, so a digit is a single digit number from 0 through 9. Next we have the `integer` rule, this is either an `integer` followed by a `digit` or "". This is where things can get a little confusing, To illustrate that something has a possibly, infinite length, grammars typically use this kind of notation.

To me, this style of notation feels backwards, lets use the example `999`. If you think about it starting with the right most `9`, you would assign that position `<integer><digit>`, the `<integer>` here would point to the middle `9`, this would also be assigned `<integer><digit>` and again the first `<integer>` would represent the left most `9`, this would also be assigned `<integer><digit>`, but the first `<integer>` in this case would be assigned `""`. Here is a little flow chart to hopefully help visualize what I am trying to say.

```xml
<integer> = ""
    ┗━━━━━━┓
<integer> = <integer>9
    ┗━━━━━━┓
<integer> = <integer>9
    ┗━━━━━━┓
<integer> = <integer>9
```

When the right side of a rule looks like this it is referred to as a "recursive" rule. Moving up the grammar, next we have the `remainder` rule, this is either a `.` followed by an `integer` or nothing. Next up we have `number` which either an `integer` or a `remainder` or an `integer` followed by a `remainder`.

The next few rules are our units, you will notice that there are two rules for each unit this is because need to have two versions, one non-optional and one optional. Each of the rules is a `number` followed by the specific unit's character, the optional version is just the normal version or nothing.

After our units we have the two halves of our duration, the `time-length` and the `date-length`, notice that we again have the optional version of each. These rules are almost identical in structure, the only difference is the units that are included. Using the `date-length` as an example, we have 4 options in each option one of the units is not optional. so a date-length is one of the following
- first
    - optional year
    - optional month
    - optional week
    - day
- second
    - optional year
    - optional month
    - week
    - optional day
- third
    - optional year
    - month
    - optional week
    - optional day
- fourth
    - year
    - optional month
    - optional week
    - optional day
This is because one unit is always required for a date-length. The same applies to the `time-length` rule, the only difference is that the `time-length` starts with a `T`.

Finally we have the `duration` rule, this is the letter `P` followed by either an optional `date-length` and a `time-length` or a `date-length` followed by an optional `time-length`, again both items are optional but at least one must be included.

With that out of the way, let's dig into each of our options a little more.

$web-only-end$

```xml
<duration>              ::= P <date-length-opt><time-length> |
                            P <date-length><time-length-opt>
<date-length-opt>       ::= <date-length> | ""
<date-length>           ::= <years-opt><months-or-minutes-opt><weeks-opt><days> |
                            <years-opt><months-or-minutes-opt><weeks><days-opt> |
                            <years-opt><months-or-minutes><weeks-opt><days-opt> |
                            <years><months-or-minutes-opt><weeks-opt><days-opt>
<time-length-opt>       ::= <time-length> | ""
<time-length>           ::= T<hours-opt><months-or-minutes-opt><seconds> |
                            T<hours-opt><months-or-minutes><seconds-opt> |
                            T<hours><months-or-minutes-opt><seconds-opt>
<years-opt>             ::= <years> | ""
<years>                 ::= <number>Y
<months-or-minutes-opt> ::= <months-or-minutes> | ""
<months-or-minutes>     ::= <number>M
<weeks-opt>             ::= <weeks> | ""
<weeks>                 ::= <number>W
<days-opt>              ::= <days> | ""
<days>                  ::= <number>D
<hours-opt>             ::= <hours> | ""
<hours>                 ::= <number>H
<seconds-opt>           ::= <seconds> | ""
<seconds>               ::= <number>S
<number>                ::= <integer> | <remainder> | <integer><remainder>
<remainder>             ::= .<integer> | ""
<integer>               ::= <integer><digit> | ""
<digit>               ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9

```