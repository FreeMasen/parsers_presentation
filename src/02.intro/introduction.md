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
In early 2018, I found myself looking for a crate that would parse javascript into an AST. While there are a few options out there for parsing javascript, they are all tied to some greater development tool. What I was looking for was [`syn`](https://github.com/dtolnay/syn) but for javascript. Since this didn't exist, I thought it would be a good idea to build it. After looking over the options available, I wanted to share my experience.
$web-only-end$