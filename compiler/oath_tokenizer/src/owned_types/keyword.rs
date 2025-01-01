use std::{fmt::Debug, hash::Hash};

use oath_src::{Span, Spanned};

use crate::Seal;

macro_rules! declare_keywords {
    ($($keyword:ident($keyword_variant:ident $keyword_type:ident), )*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Keyword {$(
            $keyword_variant($keyword_type),
        )*}

        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $keyword_type(pub Span);
        )*

        #[macro_export]
        macro_rules! Keyword {$(
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
                        stringify!($keyword) => Some(Self::$keyword_variant($keyword_type(span))),
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
                    self.0
                }
            }
        )*
    };
}
declare_keywords!(
    mod(Mod ModKeyword),
    pub(Pub PubKeyword),
    use(Use UseKeyword),
    package(Package PackageKeyword),
    super(Super SuperKeyword),
    trait(Trait TraitKeyword),
    value(Value ValueKeyword),
    type(Type TypeKeyword),
    struct(Struct StructKeyword),
    union(Union UnionKeyword),
    untagged(Untagged UntaggedKeyword),
    fn(Fn FnKeyword),
    alias(Alias AliasKeyword),
    impl(Impl ImplKeyword),
    with(With WithKeyword),
    where(Where WhereKeyword),
    static(Static StaticKeyword),
    const(Const ConstKeyword),
    async(Async AsyncKeyword),
    unsafe(Unsafe UnsafeKeyword),
    safe(Safe SafeKeyword),
    mem(Mem MemKeyword),
    panic(Panic PanicKeyword),
    dlock(DLock DLockKeyword),
    con(Con ConKeyword),
    raw(Raw RawKeyword),
    var(Var VarKeyword),
    mut(Mut MutKeyword),
    smut(Smut SmutKeyword),
    excl(Excl ExclKeyword),
    if(If IfKeyword),
    else(Else ElseKeyword),
    match(Match MatchKeyword),
    loop(Loop LoopKeyword),
    while(While WhileKeyword),
    for(For ForKeyword),
    in(In InKeyword),
    break(Break BreakKeyword),
    continue(Continue ContinueKeyword),
    return(Return ReturnKeyword),
);
