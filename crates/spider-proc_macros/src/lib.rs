use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse_macro_input, Attribute, Error, FnArg, Ident, ItemFn, LitInt, LitStr, Result};

struct AllTuples {
    fake_variadic: bool,
    macro_ident: Ident,
    start: usize,
    end: usize,
    idents: Vec<Ident>,
}

impl Parse for AllTuples {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fake_variadic = input.call(parse_fake_variadic_attr)?;
        let macro_ident = input.parse::<Ident>()?;
        input.parse::<Comma>()?;
        let start = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;
        let end = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;
        let mut idents = vec![input.parse::<Ident>()?];
        while input.parse::<Comma>().is_ok() {
            idents.push(input.parse::<Ident>()?);
        }

        if start > 1 && fake_variadic {
            return Err(Error::new(
                input.span(),
                "#[doc(fake_variadic)] only works when the tuple with length one is included",
            ));
        }

        Ok(AllTuples {
            fake_variadic,
            macro_ident,
            start,
            end,
            idents,
        })
    }
}

/// Generates all tuple implementations for a type
#[proc_macro]
pub fn all_tuples(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AllTuples);
    let len = 1 + input.end - input.start;
    let mut ident_tuples = Vec::with_capacity(len);
    for i in 0..=len {
        let idents = input
            .idents
            .iter()
            .map(|ident| format_ident!("{}{}", ident, i));
        ident_tuples.push(to_ident_tuple(idents, input.idents.len()));
    }

    let macro_ident = &input.macro_ident;
    let invocations = (input.start..=input.end).map(|i| {
        let ident_tuples = choose_ident_tuples(&input, &ident_tuples, i);
        let attrs = if input.fake_variadic {
            fake_variadic_attrs(len, i)
        } else {
            TokenStream2::default()
        };
        quote! {
            #macro_ident!(#attrs #ident_tuples);
        }
    });
    TokenStream::from(quote! {
        #(
            #invocations
        )*
    })
}
fn choose_ident_tuples(input: &AllTuples, ident_tuples: &[TokenStream2], i: usize) -> TokenStream2 {
    // `rustdoc` uses the first ident to generate nice
    // idents with subscript numbers e.g. (F₁, F₂, …, Fₙ).
    // We don't want two numbers, so we use the
    // original, unnumbered idents for this case.
    if input.fake_variadic && i == 1 {
        let ident_tuple = to_ident_tuple(input.idents.iter().cloned(), input.idents.len());
        quote! { #ident_tuple }
    } else {
        let ident_tuples = &ident_tuples[..i];
        quote! { #(#ident_tuples),* }
    }
}
fn to_ident_tuple(idents: impl Iterator<Item = Ident>, len: usize) -> TokenStream2 {
    if len < 2 {
        quote! { #(#idents)* }
    } else {
        quote! { (#(#idents),*) }
    }
}

/// Parses the attribute `#[doc(fake_variadic)]`
fn parse_fake_variadic_attr(input: ParseStream) -> Result<bool> {
    let attribute = match input.call(Attribute::parse_outer)? {
        attributes if attributes.is_empty() => return Ok(false),
        attributes if attributes.len() == 1 => attributes[0].clone(),
        attributes => {
            return Err(Error::new(
                input.span(),
                format!("Expected exactly one attribute, got {}", attributes.len()),
            ));
        }
    };

    if attribute.path().is_ident("doc") {
        let nested = attribute.parse_args::<Ident>()?;
        if nested == "fake_variadic" {
            return Ok(true);
        }
    }

    Err(Error::new(
        attribute.meta.span(),
        "Unexpected attribute".to_string(),
    ))
}
fn fake_variadic_attrs(len: usize, i: usize) -> TokenStream2 {
    let cfg = quote! { any(docsrs, docsrs_dep) };
    match i {
        // An empty tuple (i.e. the unit type) is still documented separately,
        // so no `#[doc(hidden)]` here.
        0 => TokenStream2::default(),
        // The `#[doc(fake_variadic)]` attr has to be on the first impl block.
        1 => {
            let doc = LitStr::new(
                &format!("This trait is implemented for tuples up to {len} items long."),
                Span2::call_site(),
            );
            quote! {
                #[cfg_attr(#cfg, doc(fake_variadic))]
                #[cfg_attr(#cfg, doc = #doc)]
            }
        }
        _ => quote! { #[cfg_attr(#cfg, doc(hidden))] },
    }
}
