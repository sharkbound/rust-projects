use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Ident};

#[proc_macro_derive(Reflective)]
pub fn reflective_derive_macro(input: TokenStream) -> TokenStream {
    // step 1: parse the input
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    // step 2: generate new TokenStream with modifications, and return new TokenStream
    impl_reflective_trait(ast)
}

fn impl_reflective_trait(input: DeriveInput) -> TokenStream {
    // step 1: get identifier of the struct
    let ident = input.ident;
    let ident_str = ident.to_string();

    // step 2: get field names
    let field_idents = match input.data {
        Data::Struct(data) => {
            data.fields.into_iter().filter_map(|f| f.ident).collect::<Vec<Ident>>()
        }
        Data::Enum(_) => { panic!("Enums are not supported by reflective."); }
        Data::Union(_) => { panic!("Unions are not supported by reflective."); }
    };
    let field_names = field_idents.iter().map(|i| i.to_string()).collect::<Vec<String>>();

    // step 3: generate additional functions in the struct (using quote to convert ast to TokenStream)
    quote::quote! {
        impl Reflective for #ident {
            fn name(&self) -> &'static str {
                #ident_str
            }

            fn field_names(&self) -> Vec<&'static str> {
                vec![#(#field_names),*]
            }
        }
    }
    .into()
}