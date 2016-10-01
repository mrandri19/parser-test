# Rust calculator parser
A simple calculator built to experiment writing a parser in [rust](rust-lang.org)

> More features implemented in [second-parser-test](https://github.com/mrandri19/second-parser-test)

## Example
```
cargo run 1+1
```

```
Running `target/debug/parser-test 1+1`
is_is_digit #0: 0 -> 1
calling finished() with args 1, 3
is_operator: 1 -> 2
calling finished() with args 2, 3
is_digit: 2 -> 3
calling finished() with args 3, 3
Did we finish?
yes!
Ok(2)
```

## TODO
- Multiple digit numbers
- Negative numbers
- Decimal numbers