use oath::{diagnostics::*, source::*, tokenization::TokenIterator};

const SRC: &'static str = r#"
    pub func neg(value: int : != int::MIN) -> int {
    }


    pub func try_neg(value: int) -> Option<int>
    where
        if value == int::MIN { -> None } else { Some(_) }
    { }
"#;

fn main() {
    let src = SrcFile::new(SrcFileId::new(0), SRC);

    let mut errors = Vec::new();
    let mut errors_handle = ErrorsHandle::new(&mut errors);

    let tokens = src.tokenize().parse_syntax(&mut errors_handle);

    println!("{}", prettyprint(&src, &errors));

    println!();
    println!("---------------------------------------------------------------------");
    println!();

    println!("{tokens:?}");

    if errors.len() > 0 {
        println!();
        println!("\x1b[31m---------------------------------------------------------------------");
        println!();

        for error in &errors {
            println!("line {}: {error}", src.line_number(error.span().end()));
        }

        println!();
    }
}
