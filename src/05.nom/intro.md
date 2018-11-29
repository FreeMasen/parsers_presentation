# nom

## Overview
$web-only$
By far the most popular option, `nom` heavily utilizes macros to build parsing workflows.
This allows it to be performant yet flexible but significantly increases the learning curve for users.

$web-only-end$
$slides-only$
- Most popular
- Performant
- Flexible
- High learning curve
$slides-only-end$

## Positives

$web-only$
`nom` has really set the bar high for the performance of any parsing library. Once you have a working parser, you can feel pretty confident that it will be just about as fast as you can make it. It's popularity makes it the most battle tested option.
$web-only-end$
$slides-only$
- Performance
- Battle Tested
$slides-only-end$

## Negatives
$web-only$
Working with `nom` lead me to a headache's worth of frustration. While it is well documented, it can be difficult to know where to look when getting started. When putting together the code samples for this, I spent the better part of a day and a half trying to implement the `nom` version, at that point I put that version aside to come back to it later. The inclusion of new operators, the requirement of using nested macros and the stream focus of the project were all big ergonomic barriers for me. At one point I had a working version of that parser that kept failing because the input I was providing wasn't wrapped up as a `CompleteStr`, a type defined by `nom` that indicates all input has been received. I needed to reach out to someone in the `nom` gitter channel to know that was my problem. The feature of being stream focused I think is a plus overall for `nom` as more experienced users would leverage that feature but it does make the process of learning quite confusing.
$web-only-end$
$slides-only$
- Learning curve
    - Custom operators
    - Nested macro calls
    - Stream focused
$slides-only-end$