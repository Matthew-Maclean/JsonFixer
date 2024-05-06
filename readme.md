# Permissive JSON Fixer

!Only for use with utf-8 or ASCII streams!

A `Read`er shim to remove c-style comments and trailing commas from utf-8 JSON
files. Featuring:

- Single-line `//` comments, which cannot start inside quotes.
- Multi-line `/*` to `*/` comments, which cannot start, but can end inside
quotes.
- Trailing commas, which are not emittited if they precede immediately a
closing square bracket or brace.

## Structs
```rust
struct JsonFixer<R>
```

A `Read`er that contains another `Read`er and fixes the
JSON it emits.

## Functions

```rust
fn JsonFixer::new(reader: R) -> JsonFixer<R>
```

Create an new `JsonFixer` with a `Read`er.

## Notes

This library only works with utf-8 or ASCII-compatable streams. I tried to keep
the memory overhead low (2 bytes!), but the trade-off here is many (many) calls
to `read` in the contained `Read`er. Using a `BufReader` would make this more
efficient (as it would make most things).
