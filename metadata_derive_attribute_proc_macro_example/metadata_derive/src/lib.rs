use proc_macro::{TokenStream};
use std::collections::HashMap;
use quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(MetaData, attributes(metadata))]
pub fn metadata_derive_macro(input: TokenStream) -> TokenStream {
    metadata_derive_macro2(input.into()).unwrap().into()
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataStructAttributes {
    author: String,
    #[deluxe(default = "0".into())]
    version: String,
}

fn metadata_derive_macro2(input: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    // parse tokenstream into AST
    let mut ast = syn::parse2::<syn::DeriveInput>(input)?;

    // get struct attributes
    let MetaDataStructAttributes { author, version } = deluxe::extract_attributes(&mut ast)?;

    // get field attributes
    let field_attributes: HashMap<String, MetaDataFieldAttributes> = get_metadata_field_attrs(&mut ast)?;
    let (fields, field_authors): (Vec<String>, Vec<String>) = field_attributes
        .into_iter()
        .map(|(field_name, field_attrs)| (field_name, field_attrs.author))
        .unzip();

    // define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // generate modified tokenstream
    Ok(quote::quote! {
        impl #impl_generics MetaData for #ident #type_generics #where_clause {
            fn author(&self) -> &'static str {
                #author
            }

            fn version(&self) -> &'static str {
                #version
            }

            fn field_authors(&self) -> std::collections::HashMap<&'static str, &'static str> {
                let fields = [#(#fields,)*];
                let field_authors = [#(#field_authors,)*];
                let hashmap: std::collections::HashMap<&'static str, &'static str> = fields.iter()
                    .zip(field_authors.iter())
                    .map(|(&field, &author)| (field, author))
                    .collect();
                hashmap
            }
        }
    })
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataFieldAttributes {
    author: String,
}


fn get_metadata_field_attrs(ast: &mut DeriveInput) -> deluxe::Result<HashMap<String, MetaDataFieldAttributes>> {
    let mut field_attrs = HashMap::new();
    match &mut ast.data {
        Data::Struct(data) => {
            for field in data.fields.iter_mut() {
                let field_name = match field.ident.as_ref() {
                    Some(ident) => ident.to_string(),
                    None => {
                        // ignore fields without names
                        continue;
                        // return Err(deluxe::Error::new(
                        //     field.ident.as_ref().unwrap().span(),
                        //     "field must have an identifier"
                        // ));
                    }
                };

                let attrs: MetaDataFieldAttributes = match deluxe::extract_attributes(field) {
                    Ok(attrs) => attrs,
                    Err(e) => {
                        // return Err(e);
                        // skip if the attribute is not present on the field
                        continue;
                    }
                };

                field_attrs.insert(field_name, attrs);
            }
        }
        _ => {}
    }
    Ok(field_attrs)
}
