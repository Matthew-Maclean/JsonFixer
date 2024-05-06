# Permissive JSON Fixer

!Only for use with utf-8 or ASCII streams!

A `Read`er shim to remove c-style comments and trailing commas from utf-8 JSON
files. Featuring:

- Single-line `//` comments, which cannot start inside quotes.
- Multi-line `/*` to `*/` comments, which cannot start, but can end inside
quotes.
- Trailing commas, which are not emitted if they precede immediately a
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

## Example usage

input:

```javascript
{
    /*
     * Sample JSON file from the JSON website
     * with added comments and trailing commas
     */
    "glossary": {
        // line comment
        "title": "example glossary",
        "GlossDiv": {
            /* removed element with multiline comment
            "title": "S",
            */
            "GlossList": {
                "GlossEntry": {
                    "ID": "SGML",
                    "SortAs": "SGML",
                    // another line comment
                    "GlossTerm": "Standard Generalized Markup Language",
                    "Acronym": "SGML",
                    "Abbrev": "ISO 8879:1986",
                    "GlossDef": {
                        /* this line removed because it's too long! 
                        "para": "A meta-markup language, used to create
                                 markup languages such as DocBook.",
                        */
                        "GlossSeeAlso": ["GML", "XML",],
                    },
                    "GlossSee": "markup",
                },
            },
        },
    },
}
```

output:

```json
{

    "glossary": {
                    "title": "example glossary"
        ,"GlossDiv": {

            "GlossList": {
                "GlossEntry": {
                    "ID": "SGML"
                    ,"SortAs": "SGML"
                                            ,"GlossTerm": "Standard Generalized Markup Language"
                    ,"Acronym": "SGML"
                    ,"Abbrev": "ISO 8879:1986"
                    ,"GlossDef": {

                        "GlossSeeAlso": ["GML" ,"XML"]
                    }
                    ,"GlossSee": "markup"
                }
            }
        }
    }
}
```

As demonstrated, whitespace and the location of commmas is not preserved
exactly. JSON whitespace is not signifigant so this should not affect parsing
or deserialization.
