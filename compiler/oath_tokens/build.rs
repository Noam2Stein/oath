use std::fs;
use std::path::Path;

use oath_token_definitions::with_tokens_expr;

fn main() {
    let lang_config = with_tokens_expr!(stringify!( {
        "comments": {
            "lineComment": "//",
            "blockComment": ["/*", "*/"]
        },
        "brackets": [
            $([$delim_open, $delim_close]), *
        ],
        "autoClosingPairs": [
            $([$delim_open, $delim_close],)*
            ["\"", "\""],
            ["'", "'"]
        ],
        "surroundingPairs": [
            $([$delim_open, $delim_close],)*
            ["\"", "\""],
            ["'", "'"]
        ]
    }));

    let tm_lang = with_tokens_expr!(
        stringify!( {
            "$schema": "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
            "name": "Oath",
            "patterns": [
                { "include": "#keywords" },
                { "include": "#strings" }
            ],
            "repository": {
                "keywords": {
                    "patterns": [
                        {
                            "name": "keyword.control.oath",
                            "match": "\\b(~pink_keywords~)\\b"
                        },
                        {
                            "name": "keyword.other.oath",
                            "match": "\\b(~blue_keywords~)\\b"
                        }
                    ]
                },
                "strings": {
                    "name": "string.quoted.double.oath",
                    "begin": "\"",
                    "end": "\"",
                    "patterns": [
                        {
                            "name": "constant.character.escape.oath",
                            "match": "\\\\."
                        }
                    ]
                }
            },
            "scopeName": "source.oath"
        })
        .replace("~blue_keywords~", &[$($blue_keyword), *].join("|"))
        .replace("~pink_keywords~", &[$($pink_keyword), *].join("|"))
    );

    fs::write(
        Path::new("../../vscode-extension/language-configuration.json"),
        lang_config,
    )
    .expect("Failed to write to language-configuration.json");

    fs::write(
        Path::new("../../vscode-extension/syntaxes/oath.tmLanguage.json"),
        tm_lang,
    )
    .expect("Failed to write to oath.tmLanguage.json");
}
