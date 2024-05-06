use serde_json::{Value, json, from_reader};

use super::*;

use std::io::Cursor;

#[test]
fn remove_line_comment()
{
    let input = r#"
    {
        // the first name of the person
        "name": "Joe",
        // the last name of the person
        "surname": "Smith",
        // the address of the person
        "address": {
            "number": "123",
            // address street name
            "street": "Mulberry Ln"
        }
    }
    "#;

    let output = json!({
        "name": "Joe",
        "surname": "Smith",
        "address": {
            "number": "123",
            "street": "Mulberry Ln"
        }
    });

    let mut fixer = JsonFixer::new(Cursor::new(input.as_bytes()));

    let parsed: Value = from_reader(&mut fixer).unwrap();

    assert_eq!(parsed, output);
}

#[test]
fn remove_multi_comment()
{
    let input = r#"
    {
        /*
         * Customer list
         */
        "customers": [
            {
                "name": "Joe Smith",
                "id": "001",
                "debt": "1000"
            },
            {
                "name": "Jane Smith",
                "id": "002",
                "debt": "7500"
            },
            /* Alice is no longer a customer
            {
                "name": "Alice A",
                "id": "003",
                "debt": "200"
            },
            */
            {
                "name": "Bob B",
                "id": "004",
                "debt": "3600"
            }
            /* Eve was removed
            {
                "name": "Eve E",
                "id": "005",
                "debt": "2400"
            }
            */
        ]
    }
    "#;

    let output = json!({
        "customers": [
            {
                "name": "Joe Smith",
                "id": "001",
                "debt": "1000"
            },
            {
                "name": "Jane Smith",
                "id": "002",
                "debt": "7500"
            },
            {
                "name": "Bob B",
                "id": "004",
                "debt": "3600"
            }
        ]
    });

    let mut fixer = JsonFixer::new(Cursor::new(input.as_bytes()));

    let parsed: Value = from_reader(&mut fixer).unwrap();

    assert_eq!(parsed, output);
}

#[test]
fn remove_trailing_commas()
{
    let input = r#"
    {
        "numbers": [
            1,
            2,
            3,
        ],
        "letters": [
            "a",
            "b",
            "c",
        ],
        "objects": [
            {
                "number": 1,
                "letter": "a",
            },
            {
                "number": 2,
                "letter": "b",
            },
            {
                "number": 3,
                "letter": "c",
            },
        ],
    }
    "#;

    let output = json!({
        "numbers": [
            1,
            2,
            3
        ],
        "letters": [
            "a",
            "b",
            "c"
        ],
        "objects": [
            {
                "number": 1,
                "letter": "a"
            },
            {
                "number": 2,
                "letter": "b"
            },
            {
                "number": 3,
                "letter": "c"
            }
        ]
    });

    let mut fixer = JsonFixer::new(Cursor::new(input.as_bytes()));

    let parsed: Value = from_reader(&mut fixer).unwrap();

    assert_eq!(parsed, output);
}

#[test]
fn whole_json()
{
    let input = r#"
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
    "#;

    let output = json!({
        "glossary": {
            "title": "example glossary",
            "GlossDiv": {
                "GlossList": {
                    "GlossEntry": {
                        "ID": "SGML",
                        "SortAs": "SGML",
                        "GlossTerm": "Standard Generalized Markup Language",
                        "Acronym": "SGML",
                        "Abbrev": "ISO 8879:1986",
                        "GlossDef": {
                            "GlossSeeAlso": ["GML", "XML"]
                        },
                        "GlossSee": "markup"
                    }
                }
            }
        }
    });

    let mut fixer = JsonFixer::new(Cursor::new(input.as_bytes()));

    let parsed: Value = from_reader(&mut fixer).unwrap();

    assert_eq!(parsed, output);
}

#[test]
fn fixer_size()
{
    let size = std::mem::size_of::<JsonFixer::<()>>();

    assert_eq!(size, 2);
}
