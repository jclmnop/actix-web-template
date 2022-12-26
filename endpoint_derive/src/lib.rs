extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate core;

use proc_macro::{TokenStream};
use syn::{DataEnum, DeriveInput, Ident, Lit, Meta, MetaNameValue};
use proc_macro2;

#[proc_macro_derive(Endpoints, attributes(get, post, put, patch, delete, handler, endpoint_path))]
pub fn derive_endpoints(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).expect("Failed to parse AST");
    impl_endpoints(&ast)
}

fn impl_endpoints(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen_impl_get_routes = match ast.data {
        syn::Data::Enum(ref data_enum) => {
            impl_get_routes(data_enum, name)
        },
        _ => panic!("Must be an enum")
    };
    let gen_impl_getters = impl_getters();

    quote!(
        /// Contains all information required to add a route for a new endpoint to
        /// an instance of `actix_web::App`
        struct EndpointRoute {
            /// Path for this endpoint
            path: &'static str,
            /// Request handler
            handler: Route,
        }

        trait Endpoints {
            fn get_path(&self) -> &'static str;
            fn get_handler(&self) -> Route;
            fn get_route(&self) -> EndpointRoute;
        }

        impl Endpoints for #name {

            #gen_impl_getters

            #gen_impl_get_routes
        }
    ).into()
}

fn impl_getters() -> proc_macro2::TokenStream {
    quote!(
        /// Path for this request
        fn get_path(&self) -> &'static str {
            self.get_route().path
        }

        /// Request handler
        fn get_handler(&self) -> Route {
            self.get_route().handler
        }
    ).into()
}

fn impl_get_routes(data_enum: &DataEnum, name: &Ident) -> proc_macro2::TokenStream {
    let mut arms: Vec<proc_macro2::TokenStream> = Vec::new();
    for variant in data_enum.variants.iter() {
        let variant_name = &variant.ident;
        let mut endpoint_path: Option<String> = None;
        let mut handler: Option<Ident> = None;
        let mut method: Option<Ident> = None;
        for attr in variant.attrs.iter() {
            let attr = attr.parse_meta().expect("Failed to parse attr");
            match attr {
                Meta::Path(ref path) => {
                    method = Some(path.get_ident().expect("Failed to parse method").clone())
                }
                Meta::NameValue(MetaNameValue{ref path, ref lit, ..}) => {
                    if path.is_ident("endpoint_path") {
                        if let Lit::Str(lit) = lit {
                            endpoint_path = Some(lit.value());
                        }
                    } else if path.is_ident("handler") {
                        if let Lit::Str(lit) = lit {
                            handler = Some(lit.parse().expect("Failed to parse handler name"));
                        }
                    }
                }
                _ => panic!("Unrecognised attribute value")
            }
        }
        let method = method.expect("Failed to parse method");
        let handler = handler.expect("Failed to parse handler name");
        let endpoint_path = endpoint_path.expect("Failed to parse path");
        let arm: proc_macro2::TokenStream = quote!(
            #name::#variant_name => EndpointRoute {
                path: &*#endpoint_path,
                handler: web::#method().to(#handler),
            },
        );
        arms.push(arm);
    }
    quote!(
        fn get_route(&self) -> EndpointRoute {
            match self {
                #(#arms)*
            }
        }
    ).into()
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        println!("hello");
    }
}
