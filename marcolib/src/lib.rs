extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse_macro_input;

#[proc_macro_derive(Instance)]
pub fn instance_derive_macro(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_instance_derive_macro(&ast)
}

fn impl_instance_derive_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        
        impl TDataInstance for #name {
           
          
           
        } 
    };
    gen.into()
}

