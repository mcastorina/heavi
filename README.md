# heavi
`heavi` is a CLI utility to make text processing up until a point easier.

## Use cases
* print a file until a matched line (exclusive)
* print a file starting at a matched line (exclusive)

## Examples
```
$ seq 5 | heavi 3
1
2

$ seq 5 | heavi -v 3
4
5
```

## Ideas
This is essentially the Heaviside function applied to text files. I
wonder how useful it would be to allow for other filters. `grep` would
essentially be the impulse function.

What would it mean to have f(t) be non-binary though? In other words,
if f(t) = 0.5, what does that mean in the context of text processing?

Perhaps this could use fuzzy searching instead of binary matching,
so each line gets a score given the input criteria. We now essentially
have two functions we can perform mathematical functions on. Describing
Heaviside in this way would be simplified to using the x coordinate
(the line number), however that defeats the usefulness of this tool.

Interesting ideas; for now `heavi` will remain a simple binary Heaviside
function.
