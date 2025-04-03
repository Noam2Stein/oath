use proc_macro2::TokenStream;
use syn::{Attribute, DataEnum, Error, Variant};

pub fn option_fallback_variant(data: &DataEnum) -> (Option<&Variant>, Vec<&Variant>) {
    let mut fallback_variant = None;
    let mut non_fallback_variants = Vec::with_capacity(data.variants.len());

    for variant in &data.variants {
        if has_attr(&variant.attrs, "fallback") && fallback_variant.is_none() {
            fallback_variant = Some(variant);
        } else {
            non_fallback_variants.push(variant);
        }
    }

    (fallback_variant, non_fallback_variants)
}
pub fn try_fallback_variant(data: &DataEnum) -> Result<(&Variant, Vec<&Variant>), TokenStream> {
    let (fallback_variant, non_fallback_variants) = option_fallback_variant(data);

    match fallback_variant {
        Some(fallback_variant) => Ok((fallback_variant, non_fallback_variants)),
        None => Err(
            Error::new(data.enum_token.span, "expected a fallback variant").into_compile_error(),
        ),
    }
}

fn has_attr<'a>(obj_attrs: impl IntoIterator<Item = &'a Attribute>, attr: &'a str) -> bool {
    obj_attrs
        .into_iter()
        .any(|obj_attr| obj_attr.path().is_ident(attr))
}
