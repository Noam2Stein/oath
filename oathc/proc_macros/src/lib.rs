mod interner;
mod span;
mod tokens;

// Interner

#[proc_macro_derive(InternedDisplay, attributes(display))]
pub fn derive_interned_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    interner::interned_display_derive_macro(input)
}

// Span

#[proc_macro_derive(Spanned, attributes(span, option_spanned))]
pub fn spanned_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    span::spanned_derive_macro(input)
}

#[proc_macro_derive(OptionSpanned, attributes(span, option_spanned))]
pub fn option_spanned_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    span::option_spanned_derive_macro(input)
}

// Tokens

#[proc_macro]
pub fn keyword(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens::keyword(input)
}

#[proc_macro]
pub fn punct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens::punct(input)
}

#[proc_macro]
pub fn delims(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens::delims(input)
}

#[proc_macro]
pub fn open(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens::open(input)
}

#[proc_macro]
pub fn close(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens::close(input)
}
