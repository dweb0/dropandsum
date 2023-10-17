# `dropandsum`

Drop duplicates and sum by column. Simple command line utility to collapse fields by a numeric column.

For example

```
Giraffe Lion    40
Fox     Lion    30
Giraffe Lion    20
Giraffe Lion    10
```

Becomes

```
Giraffe Lion    70
Fox     Lion    30
```

## Examples

From file

```
dropandsum -i 3 animals.tsv
```

You can also pipe from stdin (useful for multiple files)

```
cat farm/animal*.tsv | dropandsum -i 3
```

Different delimiter

```
dropandsum -i 3 
```

## Installation

Via [cargo](https://www.rust-lang.org/tools/install) (Recommended)

```
cargo install --git https://github.com/dweb0/dropandsum
```
