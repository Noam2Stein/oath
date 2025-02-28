use crate::*;

impl PeekOk for TokenTree {}

impl PeekOk for Ident {}

impl PeekOk for Keyword {}
with_token_set! {$(
    impl PeekOk for $keyword_type {}
)*}

impl PeekOk for Punct {}
with_token_set! {$(
    impl PeekOk for $punct_type {}
)*}

impl PeekOk for Literal {}
impl PeekOk for IntLiteral {}
impl PeekOk for FloatLiteral {}
impl PeekOk for CharLiteral {}
impl PeekOk for StrLiteral {}

impl PeekOk for Group {}
with_token_set! {$(
    impl PeekOk for Group<$delim_type> {}
)*}
