use oath_keywords_puncts::{with_delimiters, with_keywords, with_puncts};

fn main() {}

fn update_vscode_lang_config() {
    const PATH: &str = "../../vscode-extension/language-configuration.json";

    let content = with_keywords! { with_puncts! { with_delimiters! {
        stringify!(
            {
                "comments": {
                    "lineComment": "//",
                    "blockComment": [ "/*", "*/" ]
                },
                "brackets": [$(

                ), *],
                "autoClosingPairs": [
                    [
                        "{",
                        "}"
                    ],
                    [
                        "[",
                        "]"
                    ],
                    [
                        "(",
                        ")"
                    ],
                    [
                        "\"",
                        "\""
                    ],
                    [
                        "'",
                        "'"
                    ],
                    [
                        "<#",
                        "#>"
                    ],
                ],
                "surroundingPairs": [
                    [
                        "{",
                        "}"
                    ],
                    [
                        "[",
                        "]"
                    ],
                    [
                        "(",
                        ")"
                    ],
                    [
                        "\"",
                        "\""
                    ],
                    [
                        "'",
                        "'"
                    ],
                    [
                        "<#",
                        "#>"
                    ],
                ]
            }
        )
    } } };

    const CONTENT: &str = with_keywords! { with_puncts! { with_delimiters! {
        stringify!(
            {
                "comments": {
                    "lineComment": "//",
                    "blockComment": [ "/*", "*/" ]
                },
                "brackets": [$(

                ), *],
                "autoClosingPairs": [

                ],
                "surroundingPairs": [

                ]
            }
        )
    } } };

    let f = {
        with_keywords! { with_puncts! { with_delimiters! {
            format!(r#"
                {{
                    "comments": {
                        "lineComment": "//",
                        "blockComment": [ "/*", "*/" ]
                    },
                    "brackets": ,
                    "autoClosingPairs": [

                    ],
                    "surroundingPairs": [

                    ]
                }}
            "#)
        } } }
    };
}
