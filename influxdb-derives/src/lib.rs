extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn;
use syn::spanned::Spanned;

use proc_macro_roids::{namespace_parameter, DeriveInputStructExt, FieldExt};

#[proc_macro_derive(PointSerialize, attributes(point))]
pub fn point_serialize_derive(input: TokenStream) -> TokenStream {
    // Paths
    let namespace: syn::Path = syn::parse_quote!(point);
    let field_path: syn::Path = syn::parse_quote!(field);
    let tag_path: syn::Path = syn::parse_quote!(tag);
    let timestamp_path: syn::Path = syn::parse_quote!(timestamp);

    // Struct-level
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let measurement: String = if let syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue{
        path, lit, ..
    })) =
        namespace_parameter(&ast.attrs, &namespace).expect("Missing measurement tag, use #[point(measurement = \"something\")] before struct declaration")
     {
        if path.segments[0].ident == "measurement" {
            if let syn::Lit::Str(lit_str) = lit {
                lit_str.value()
            } else {
                let span = lit.span();
                return (quote_spanned! { span => compile_error!("Measurement should be a string"); }).into();
            }
        } else {
            let span = path.segments[0].ident.span();
            return (quote_spanned! { span => compile_error!("Top attribute is not measurement, which was expected") }).into();
        }
    } else {
        let span = ast.attrs[0].path.segments[0].ident.span();
        return (quote_spanned! { span => compile_error!("Did not find a suitable measurement tag should be in format '#[point(measurement = \"name\")]'"); }).into();
    };

    let ast_fields = ast.fields();

    macro_rules! field_splitter {
        ($names:ident, $tokens:ident, $field:ident) => {
            let ident: &syn::Ident = &$field.ident.as_ref().unwrap();
            let field_name: String =
                if let syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                    lit, ..
                })) = namespace_parameter(&$field.attrs, &namespace).unwrap()
                {
                    if let syn::Lit::Str(lit_str) = lit {
                        lit_str.value()
                    } else {
                        let span = lit.span();
                        return (quote_spanned! { span => compile_error!("Attribute must be a string type"); })
                            .into();
                    }
                } else {
                    ident.to_string()
                };
            $names.push(field_name);
            $tokens.push(ident);
        };
    }

    macro_rules! string_vec_joiner {
        ($vec:ident, $quotes:expr) => {
            $vec.iter()
                .map(|it| {
                    if $quotes {
                        format!("{}={{:?}}", it)
                    } else {
                        format!("{}={{}}", it)
                    }
                })
                .collect::<Vec<String>>()
                .join(",")
        };
    }

    // Field-level
    let mut field_names: Vec<String> = Vec::new();
    let mut field_tokens: Vec<&syn::Ident> = Vec::new();
    let fields = ast_fields
        .iter()
        .filter(|field| field.contains_tag(&namespace, &field_path));
    if fields.clone().count() == 0 {
        return (quote!{ compile_error!("Fields are not optional, there needs to be atleast one!"); }).into();
    }
    for field in fields {
        field_splitter!(field_names, field_tokens, field);
    }
    let field_names_combined = string_vec_joiner!(field_names, true);

    let mut tag_names: Vec<String> = Vec::new();
    let mut tag_tokens: Vec<&syn::Ident> = Vec::new();
    for field in ast_fields
        .iter()
        .filter(|field| field.contains_tag(&namespace, &tag_path))
    {
        field_splitter!(tag_names, tag_tokens, field);
    }
    let tag_names_combined = string_vec_joiner!(tag_names, false);

    eprintln!("Tag_names_combined: {}", tag_names_combined);
    let complete_text = if tag_names_combined != "" {
        format!("{{}},{} {}", tag_names_combined, field_names_combined)
    } else {
        format!("{{}} {}", field_names_combined)
    };

    let timestamp = ast_fields
        .iter()
        .find(|field| field.contains_tag(&namespace, &timestamp_path))
        .expect("Missing timestamp field! Use #[point(timestamp)] over the timestamp field");

    let timestamp_field = &timestamp.ty;
    if "Timestamp" != quote! { #timestamp_field }.to_string() {
        let span = timestamp.ty.span();
        return (quote_spanned! { span => compile_error!("Timestamp field must have 'Timestamp' type!"); })
            .into();
    }

    let struct_timestamp = timestamp.ident.as_ref().unwrap();

    let tag_tokens_length = tag_tokens.len();


    eprintln!("Complete text: {}", complete_text);
    eprintln!("Tags: {}", (quote!{ #(self.#tag_tokens),* }).to_string());
    eprintln!("Fields: {}", (quote!{ #(self.#field_tokens),* }).to_string());

    let serialize_with_timestamp = quote! {
        fn serialize_with_timestamp(&self, timestamp: Option<Timestamp>) -> String {
            match timestamp {
                Some(timestamp) => format!("{} {}", self.serialize(), timestamp.to_string()),
                None => format!("{} {}", self.serialize(), self.#struct_timestamp.to_string())
            }
        }
    };

    // Output
    (if tag_tokens_length != 0 {
        quote! {
            impl PointSerialize for #name {
                fn serialize(&self) -> String {
                    format!(#complete_text, #measurement, #(self.#tag_tokens),*, #(self.#field_tokens),*).to_string() 
                }
                #serialize_with_timestamp
            }
        } 
    } else {
        quote! {
            impl PointSerialize for #name {
                fn serialize(&self) -> String {
                    format!(#complete_text, #measurement, #(self.#field_tokens),*).to_string()
                }
                #serialize_with_timestamp
            }
        }
    })
    .into()
}
