# Pest
## Overview
$web-only$

`pest` is a completely different approach from the other two options discussed here. Instead of building your own parser you write out your grammar, in their format, and `pest` does all the building for you. Since it is designed to handle anything you throw at it you ultimately are sacrificing speed for convenience which can be a very valid sacrifice.

$web-only-end$
$slides-only$
- Provide Grammar
- Generate Parser
$slides-only-end$

## Positives
$web-only$
The biggest advantage that `pest` has over the other options is the barrier to entry. The learning curve is pinned almost entirely learning to the grammar format. If you don't already have this skill it is a pretty valuable one to have so the cost of that is nominal. Another perk of this system is that you might be able to find a similar grammar format published online somewhere. A good example of this is the es5 grammar I found when I was getting started with my parsers, you can see it [here](http://boshi.inimino.org/3box/PanPG/grammars/ECMAScript_5.peg). It won't fit exactly but often a simple find/replace can do almost all of the work for you. Lastly, you are off loading a large amount of the solution development to a tool, this makes it much faster to get up and running.
$web-only-end$
$slides-only$
- Low barrier to entry
- Grammar might already exist?
- Lots of work offloaded
$slides-only-end$

## Negatives
$web-only$
The biggest negative is that you give up a lot of control, you have little to no ability to adjust the parser for performance reason. Another big issue in the same vein is you are stuck with what is offered by the library maintainer, when I was testing out pest to use in my js parser, they did not yet have unicode support meaning I needed to include in my grammar any unicode categories manually which bloated my binary size and compilation times. More recently they have baked that into the library but if you wanted to express something complicated that is not yet implemented, you are stuck. The next issue is that it outputs the larges binary size, this isn't a metric that is always important but it can be. The last issue is speed, it is not the fastest option but it also isn't the slowest. I bring this up last because I think the speed is easily offset by the east of use depending on your intentions.
$web-only-end$
$slides-only$
- Limited control
- Largest Binary Size
- Speed, to some extent
$slides-only-end$