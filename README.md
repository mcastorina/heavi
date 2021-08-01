# heavi
`heavi` is a CLI utility to make text filtering up until a point
easier. It is named after the Heaviside step function.

## Use cases
* print a file starting at a matched line
* print a file until a matched line
* print a file starting at a match
* print a file until a match

## Examples
```
$ seq 5 | heavi 3
4
5

$ seq 5 | heavi -v 3
1
2
```

## Flags

| Flag | Description |
| ---- | ----------- |
| `-v` | In**v**ert the Heaviside function; Output the file up until the match |
| `-b` | **B**yte mode; Byte processing instead of line processing |
| `-i` | **I**nclusive; Output the match in addition to other output |

