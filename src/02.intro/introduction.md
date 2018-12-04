## Introduction

$slides-only$
### Why me?
- Rusty ECMAScript Scanner (RESS)
    - https://github.com/FreeMasen/RESS
- Rusty ECMAScript Syntax Analyzer (RESSA)
    - https://github.com/FreeMasen/RESSA

### Where to find me?
- @FreeMasen
    - twitter
    - irc
    - discord
- https://wiredforge.com
    - infrequent blogging
    - random JS things
- https://robertmasen.com
    - My resume online
$slides-only-end$
$web-only$
In early 2018, I found myself looking for a crate that would parse javascript into an AST. While there are a few options out there for parsing javascript, they are all tied to some greater development tool. What I was looking for was `syn` but for javascript. Since this didn't exist, I thought it would be a good idea to build it.

The first step in that process was to decide which of parsing crate to use under the hood, if one at all. To decide, I tried out three options `nom`, `combine`, and `pest`. Each have different ergonomics and performance considerations, here is what I found.
$web-only-end$