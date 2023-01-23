extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate core;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_error::{abort, proc_macro_error, SpanRange};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, ItemFn, Lit, Meta, MetaList, NestedMeta};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn add_path_const(args: TokenStream, item: TokenStream) -> TokenStream {
    add_path_const_attr(args, item)
}

fn add_path_const_attr(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn: ItemFn = syn::parse(item).unwrap_or_else(|_| {
        abort!(
            SpanRange::call_site(),
            "Endpoint attribute can only be applied to a function."
        )
    });
    let fn_ident = &item_fn.sig.ident;
    let mut endpoint_path: Option<String> = None;
    let methods: Vec<Ident> = vec!["get", "post", "put", "delete", "head"]
        .into_iter()
        .map(|x| Ident::new(x, Span::call_site()))
        .collect();
    let mut valid_method_attr_exists = false;
    for attr in item_fn.attrs.iter() {
        let attr = attr.parse_meta().expect("Failed to parse attr");
        if let Meta::List(MetaList {
            ref path,
            ref nested,
            ..
        }) = attr
        {
            if let Some(attr_ident) = path.get_ident() {
                if methods.contains(attr_ident) {
                    valid_method_attr_exists = true;
                    endpoint_path = get_endpoint_path_from_attr_args(
                        nested,
                        &SpanRange::from_tokens(&attr),
                    );
                    break;
                }
            }
        }
    }

    if !valid_method_attr_exists {
        abort!(
            &SpanRange::from_tokens(&fn_ident),
            "No valid method attribute exists for this function. \n\te.g. #[get(\"/path\")]"
        )
    }

    let impl_path = impl_path_for_struct(endpoint_path, &fn_ident);

    quote!(
        #item_fn

        #impl_path
    )
    .into()
}

fn get_endpoint_path_from_attr_args(
    args: &Punctuated<NestedMeta, Comma>,
    span_range: &SpanRange,
) -> Option<String> {
    for arg in args.iter() {
        if let NestedMeta::Lit(Lit::Str(endpoint_path)) = arg {
            return Some(endpoint_path.value());
        }
    }
    abort!(span_range, "No endpoint path in method attribute.");
}

fn impl_path_for_struct(
    endpoint_path: Option<String>,
    fn_ident: &Ident,
) -> TokenStream2 {
    let endpoint_path = &*endpoint_path.unwrap_or_else(|| {
        abort!(
            SpanRange::call_site(),
            "No endpoint path in method attribute."
        )
    });

    quote!(
        impl #fn_ident {
            pub const PATH: &str = #endpoint_path;
        }
    )
}
