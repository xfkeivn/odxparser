extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse_macro_input;
use syn::ItemStruct;

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
            fn get_parent(&self)->&Option<Arc<RefCell<dyn TDataInstance >>>
            {
                return &self.instance_core.parent;
            }

            fn set_parent(&mut self,parent:Arc<RefCell<dyn TDataInstance>>)
            {
                self.instance_core.parent = Some(parent);
            }
        }
    };
    gen.into()
}



#[proc_macro_attribute]
pub fn instance_dt_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    
    let input_fn = parse_macro_input!(item as ItemStruct);
    // using newly created struct Args
        let args= parse_macro_input!(metadata as Args);
        TokenStream::from(quote!{fn dummy(){}})
    // Build the trait implementation
    impl_instance_dt_macro(&ast)
}

fn impl_instance_dt_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attr = &ast.attrs[0];
    
    let gen = quote! {
        impl DataType for #name
{
    type InstanceType = StaticFieldInstance;
    fn create_instance(datatype:Arc<RefCell<StaticField>>,name:&str,byte_postion:u32,bit_position:u32)->Arc<RefCell<StaticFieldInstance>>
    {
        let di =  StaticFieldInstance{
            instance_core:DataInstanceCore{datatype:datatype ,..Default::default()},
        };
        return Arc::new(RefCell::new(di));

    }
}
    };
    gen.into()
}












#[cfg(test)]
mod tests {
    use super::*; 
}
