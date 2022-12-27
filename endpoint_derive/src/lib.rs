extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate core;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{DataEnum, DeriveInput, Ident, Lit, Meta, MetaList, MetaNameValue, NestedMeta};

//TODO: replace attributes with single #[endpoint("/path", method = "GET", handler = "fn_name"

#[proc_macro_derive(Endpoints, attributes(endpoint))]
pub fn derive_endpoints(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).expect("Failed to parse AST");
    impl_endpoints(&ast)
}

fn impl_endpoints(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let output_struct_name = &Ident::new(&*format!("{}Route", name.to_string()), Span::call_site());
    let gen_impl_get_routes = match ast.data {
        syn::Data::Enum(ref data_enum) => impl_get_routes(data_enum, name, output_struct_name),
        _ => panic!("Must be an enum"),
    };
    let gen_impl_getters = impl_getters();

    quote!(
        /// Contains all information required to add a route for a new endpoint to
        /// an instance of `actix_web::App`
        struct #output_struct_name {
            /// Path for this endpoint
            path: &'static str,
            /// Request handler
            handler: Route,
        }

        trait Endpoints {
            fn get_path(&self) -> &'static str;
            fn get_handler(&self) -> Route;
            fn get_route(&self) -> #output_struct_name;
        }

        impl Endpoints for #name {

            #gen_impl_getters

            #gen_impl_get_routes
        }
    )
    .into()
}

fn impl_getters() -> TokenStream2 {
    quote!(
        /// Path for this request
        fn get_path(&self) -> &'static str {
            self.get_route().path
        }

        /// Request handler
        fn get_handler(&self) -> Route {
            self.get_route().handler
        }
    )
}

fn impl_get_routes(data_enum: &DataEnum, name: &Ident, output_struct_name: &Ident) -> TokenStream2 {
    let mut arms: Vec<TokenStream2> = Vec::new();
    for variant in data_enum.variants.iter() {
        let variant_name = &variant.ident;
        let mut arm: Option<TokenStream2> = None;
        for attr in variant.attrs.iter() {
            let attr = attr.parse_meta().expect("Failed to parse attr");
                if let Meta::List(MetaList {
                    ref path,
                    ref nested,
                    ..
                }) = attr {
                    if path.is_ident("endpoint") {
                        arm = Some(endpoint_helper_attr(nested, name, variant_name, output_struct_name));
                    }
                }
            }
        let arm = arm.expect("Failed to parse endpoint() attribute");
        arms.push(arm);
    }
    if arms.is_empty() {
        panic!("At least one variant must be annotated with #[endpoint(..)] attribute");
    }
    quote!(
        fn get_route(&self) -> #output_struct_name {
            match self {
                #(#arms)*
            }
        }
    )
}

fn endpoint_helper_attr(
    args: &Punctuated<NestedMeta, Comma>,
    name: &Ident,
    variant_name: &Ident,
    output_struct_name: &Ident,
) -> TokenStream2 {
    let args: Vec<&NestedMeta> = args.iter().collect();
    let mut endpoint_path: Option<String> = None;
    let mut handler: Option<Ident> = None;
    let mut method: Option<Ident> = Some(Ident::new("get", Span::call_site()));

    for arg in args.into_iter() {
        match arg {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) => {
                    let methods = vec!["get", "post", "put", "delete", "head"];
                    let ident = path.get_ident().expect("Invalid ident for method");
                    let ident_str = ident.to_string().to_lowercase();
                    if methods.iter().map(|x| String::from(*x)).any(|x| x == ident_str) {
                        method = Some(ident.clone());
                    }
                }
                Meta::NameValue(MetaNameValue {
                    ref path, ref lit, ..
                }) => {
                    if let Lit::Str(litstr) = lit {
                        if path.is_ident("handler") {
                            handler = Some(litstr.parse().expect("Failed to parse handler name"));
                        }
                    }
                }
                _ => (),
            },
            NestedMeta::Lit(Lit::Str(lit)) => {
                endpoint_path = Some(lit.value());
            }
            _ => (),
        }
    }
    let method = method.expect("Failed to parse method");
    let handler = handler.expect("Failed to parse handler name");
    let endpoint_path = endpoint_path.expect("Failed to parse path");
    quote!(
        #name::#variant_name => #output_struct_name {
            path: &*#endpoint_path,
            handler: web::#method().to(#handler),
        },
    )
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        println!("hello");
    }
}
