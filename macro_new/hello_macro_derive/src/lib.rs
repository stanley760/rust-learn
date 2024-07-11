use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let parse = syn::parse(input).unwrap();

    impl_hello_macro(&parse)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let token_stream = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("hello macro! it's {}", stringify!(#name))
            }
        }
    };
    token_stream.into()
}
