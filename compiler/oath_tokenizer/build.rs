use std::{fs::File, io::Write};

use oath_keywords_puncts::{
    with_control_keywords, with_delimiters, with_keywords, with_other_keywords,
};

fn main() {
    update_vscode_lang_config();
    update_vscode_tmlang();
}

fn update_vscode_lang_config() {
    const PATH: &str = "../../vscode-extension/language-configuration.json";

    const CONTENT: &str = {
        with_delimiters! {
            stringify!(
                {
                    "comments": {
                        "lineComment": "//",
                        "blockComment": ["/*", "*/"]
                    },
                    "brackets": [
                        $([$open_delim, $close_delim],)*
                    ],
                    "autoClosingPairs": [
                        $([$open_delim, $close_delim],)*
                        ["\"", "\""],
                        ["'", "'"],
                    ],
                    "surroundingPairs": [
                        $([$open_delim, $close_delim],)*
                        ["\"", "\""],
                        ["'", "'"],
                    ]
                }
            )
        }
    };

    File::create(PATH)
        .unwrap()
        .write_all(CONTENT.as_bytes())
        .unwrap();
}

fn update_vscode_tmlang() {
    const PATH: &str = "../../vscode-extension/syntaxes/oath.tmLanguage.json";

    let control_keywords = {
        with_control_keywords! {
            stringify!(
                $($keyword)|*
            )
        }
    }
    .replace(" ", "");

    let other_keywords = {
        with_other_keywords! {
            stringify!(
                $($keyword)|*
            )
        }
    }
    .replace(" ", "");

    let control_keyword_regex = format!("\"\\\\b({control_keywords})\\\\b\"");

    let other_keyword_regex = format!("\"\\\\b({other_keywords})\\\\b\"");

    let content = {
        with_keywords! {
            stringify!(
                {
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
                                    "match": control_keyword_regex
                                },
                                {
                                    "name": "keyword.other.oath",
                                    "match": other_keyword_regex
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
                }
            )
        }
    }.replace("control_keyword_regex", &control_keyword_regex).replace("other_keyword_regex", &other_keyword_regex).replace("\n", "");

    File::create(PATH)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}
