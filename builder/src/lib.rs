use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    gen_builder_impl(&ast).into()
}

fn generate_builder_method() -> TokenStream {
    todo!()
}

fn gen_builder_impl(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let base_struct_ident = ast.ident.clone();
    // let struct_data = match &ast.data {
    //     syn::Data::Struct(data) => data,
    //     syn::Data::Enum(_) => panic!("Macro does not support enum types"),
    //     syn::Data::Union(_) => panic!("Macro does not support union types"),
    // };
    // let struct_fields

    todo!()
}

fn gen_builder_setter() {}
