use oath::{diagnostics::*, source::*, tokenization::TokenIterator};

const SRC: &'static str = r#"
    pub mod f {
        pub func g() -> (int, dyn Int + Signed + !Goofy)

        pub func g() pub
    }
    pub func hello(a: int, b: int) -> int {
        a
        var c = b
        mut var d: uint

        var e = {
            b
        }
        (a, b, 2, "", { func })
    }

    struct F {
        f: int,
        g: bool,
        d: 5
    }
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
