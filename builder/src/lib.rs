use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    generate_builder_def(&ast).into()
}

fn generate_builder_def(ast: &DeriveInput) -> TokenStream {
    let base_struct_ident_str = ast.ident.clone().to_string();
    let base_struct_ident = ast.ident.clone();
    let builder_ident = Ident::new(
        (base_struct_ident_str + "Builder").as_str(),
        Span::call_site(),
    );

    let orig_struct_data = match &ast.data {
        syn::Data::Struct(data) => data,
        syn::Data::Enum(_) => panic!("Macro does not support enum types"),
        syn::Data::Union(_) => panic!("Macro does not support union types"),
    };
    let orig_struct_fields = orig_struct_data.fields.clone();

    //Define the Builder Struct (Not the impl)
    let builder_field_defs = orig_struct_fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let field_type = field.ty.clone();
        quote! {
            #field_ident: Option<#field_type>
        }
    });
    let builder_struct_def = quote! {
        pub struct #builder_ident{
            #(#builder_field_defs),*
        }
    };

    //Define the builder() fn in the base struct
    let base_builder_fn = generate_base_builder_fn(builder_ident.clone(), &orig_struct_fields);

    //Define the setters for the builder
    let builder_setters = orig_struct_fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let field_type = field.ty.clone();
        generate_builder_default_setter(field_ident, field_type)
    });

    let build_fn_impl = generate_build_fn(base_struct_ident.clone(), &orig_struct_fields);

    quote! {
        use std::error::Error;

        #builder_struct_def

        impl #builder_ident{
            #(#builder_setters)*

            #build_fn_impl
        }

        impl #base_struct_ident{
            #base_builder_fn
        }
    }
}

fn generate_base_builder_fn(builder_ident: Ident, orig_struct_fields: &Fields) -> TokenStream {
    let builder_init_fields = orig_struct_fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        quote! {#field_ident: None}
    });

    quote! {
            pub fn builder() -> #builder_ident{
                #builder_ident{
                    #(#builder_init_fields),*
                }
            }
    }
}

fn generate_build_fn(base_struct_ident: Ident, builder_fields: &Fields) -> TokenStream {
    //Let Unwrap throw error state if a field is uninit.
    let local_field_copy = builder_fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        quote! {
            let #field_ident = self.#field_ident.clone().expect("Field #field_ident was not constructed");
        }
    });

    //create the actual struct
    let struct_field_set = builder_fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        quote! {
            #field_ident
        }
    });

    quote! {
        pub fn build(&mut self)-> Result<#base_struct_ident, Box<dyn Error>>{
            #(#local_field_copy)*

            Ok(
                #base_struct_ident{
                    #(#struct_field_set),*
                }
            )
        }
    }
}

fn generate_builder_default_setter(field_ident: Ident, field_type: Type) -> TokenStream {
    quote! {
    pub fn #field_ident(&mut self, #field_ident: #field_type) -> &mut Self{
        self.#field_ident = Some(#field_ident);
        self
    }
    }
}
