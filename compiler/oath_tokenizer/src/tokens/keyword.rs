use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, SpanLengthed, Spanned};

use crate::Seal;

macro_rules! declare_keywords {
    ($($keyword:ident($keyword_len:literal $keyword_variant:ident $keyword_type:ident), )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Keyword {$(
            $keyword_variant($keyword_type),
        )*}

        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $keyword_type(pub SpanLengthed<$keyword_len>);
        )*

        #[macro_export]
        macro_rules! keyword {$(
            ($keyword) => {
                $crate::$keyword_type
            };
            ($keyword($span:expr)) => {
                $crate::$keyword_type($span)
            };
        )*}

        #[allow(private_bounds)]
        pub trait KeywordType: Seal + Send + Sync + Debug + Copy + Eq + Ord + Hash + Spanned {}

        impl KeywordType for Keyword {}
        impl Seal for Keyword {}
        impl Spanned for Keyword {
            #[inline(always)]
            fn span(&self) -> Span {
                match self {$(
                    Self::$keyword_variant(keyword) => keyword.span(),
                )*}
            }
        }
        impl Keyword {
            pub const KEYWORDS: &[&str] = &[$(stringify!($keyword)), *];

            pub fn is_keyword(s: &str) -> bool {
                match s {
                    $(
                        stringify!($keyword) => true,
                    )*
                    _ => false,
                }
            }

            pub fn from_str(s: &str, span: Span) -> Option<Self> {
                match s {
                    $(
                        stringify!($keyword) => span.lengthed().map(|span| Self::$keyword_variant($keyword_type(span))),
                    )*
                    _ => None,
                }
            }
        }

        $(
            impl KeywordType for $keyword_type {}
            impl Seal for $keyword_type {}
            impl Spanned for $keyword_type {
                #[inline(always)]
                fn span(&self) -> Span {
                    self.0.unlined()
                }
            }
        )*
    };
}
declare_keywords!(
    mod(3 Mod ModKeyword),
    pub(3 Pub PubKeyword),
    use(3 Use UseKeyword),
    package(7 Package PackageKeyword),
    super(5 Super SuperKeyword),
    trait(5 Trait TraitKeyword),
    value(5 Value ValueKeyword),
    type(4 Type TypeKeyword),
    struct(6 Struct StructKeyword),
    union(5 Union UnionKeyword),
    untagged(8 Untagged UntaggedKeyword),
    fn(2 Fn FnKeyword),
    alias(5 Alias AliasKeyword),
    impl(4 Impl ImplKeyword),
    with(4 With WithKeyword),
    requires(8 Requires RequiresKeyword),
    promises(8 Promises PromisesKeyword),
    may(3 May MayKeyword),
    will(4 Will WillKeyword),
    wont(4 Wont WontKeyword),
    static(6 Static StaticKeyword),
    const(5 Const ConstKeyword),
    async(5 Async AsyncKeyword),
    mem(3 Mem MemKeyword),
    panic(5 Panic PanicKeyword),
    dlock(5 DLock DLockKeyword),
    con(3 Con ConKeyword),
    raw(3 Raw RawKeyword),
    var(3 Var VarKeyword),
    mut(3 Mut MutKeyword),
    smut(4 Smut SmutKeyword),
    excl(4 Excl ExclKeyword),
    if(2 If IfKeyword),
    else(4 Else ElseKeyword),
    match(5 Match MatchKeyword),
    loop(4 Loop LoopKeyword),
    while(5 While WhileKeyword),
    for(3 For ForKeyword),
    in(2 In InKeyword),
    break(5 Break BreakKeyword),
    continue(8 Continue ContinueKeyword),
    return(6 Return ReturnKeyword),
);
