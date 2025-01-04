mod tokenize;
mod tokens;
pub use tokenize::*;
pub use tokens::*;

mod raw_tokenizer;

trait Seal {}

#[macro_export(local_inner_macros)]
macro_rules! with_keywords {
    ($dollar:tt($keyword:ident $keyword_len:ident $keyword_variant:ident $keyword_type:ident): $($tt:tt)*) => {
        macro_rules! $keyword {
            ($dollar($dollar$keyword:ident($dollar$keyword_len:literal $dollar$keyword_variant:ident $dollar$keyword_type:ident), )*) => {
                $($tt)*
            }
        }
        $keyword!(
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
    };
}

#[macro_export(local_inner_macros)]
macro_rules! with_puncts {
    ($dollar:tt($punct:ident $punct_len:ident $punct_variant:ident $punct_type:ident): $($tt:tt)*) => {
        macro_rules! $punct {
            ($dollar($dollar$punct:literal($dollar$punct_len:literal $dollar$punct_variant:ident $dollar$punct_type:ident), )*) => {
                $($tt)*
            }
        }
        $punct!(
            ">>="(3 ShiftRAssign ShiftRAssignPunct),
            "<<="(3 ShiftLAssign ShiftLAssignPunct),
            "..."(3 DotDotDot DotDotDotPunct),
            "&&"(2 AndAnd AndAndPunct),
            "||"(2 OrOr OrOrPunct),
            "<<"(2 ShiftL ShiftLPunct),
            ">>"(2 ShiftR ShiftRPunct),
            "+="(2 PlusAssign PlusAssignPunct),
            "-="(2 MinusAssign MinusAssignPunct),
            "*="(2 StarAssign StarAssignPunct),
            "/="(2 SlashAssign SlashAssignPunct),
            "%="(2 PercentAssign PercentAssignPunct),
            "&="(2 AndAssign AndAssignPunct),
            "|="(2 OrAssign OrAssignPunct),
            "^="(2 CaretAssign CaretAssignPunct),
            "=="(2 EqEq EqEqPunct),
            "<="(2 LessEq LessEqPunct),
            ">="(2 MoreEq MoreEqPunct),
            "::"(2 ColonColon ColonColonPunct),
            ".."(2 DotDot DotDotPunct),
            "+"(1 Plus PlusPunct),
            "-"(1 Minus MinusPunct),
            "*"(1 Star StarPunct),
            "/"(1 Slash SlashPunct),
            "%"(1 Percent PercentPunct),
            "="(1 Eq EqPunct),
            "<"(1 Less LessPunct),
            ">"(1 More MorePunct),
            "&"(1 And AndPunct),
            "|"(1 Or OrPunct),
            "^"(1 Caret CaretPunct),
            "~"(1 Tilde TildePunct),
            "?"(1 Question QuestionPunct),
            "$"(1 Dollar DollarPunct),
            ","(1 Comma CommaPunct),
            ";"(1 Semi SemiPunct),
            ":"(1 Colon ColonPunct),
            "."(1 Dot DotPunct),
            "`"(1 Backtick BacktickPunct),
        );
    };
}
