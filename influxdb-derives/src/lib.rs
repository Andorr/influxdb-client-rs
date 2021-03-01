extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

use proc_macro_roids::{namespace_parameter, DeriveInputStructExt, FieldExt};

#[proc_macro_derive(PointSerialize, attributes(point))]
pub fn point_serialize_derive(input: TokenStream) -> TokenStream {
    // TODO: Finish serialize_with_timestamp
    // TODO: Add a check for [field] value and make sure to add qouble quotes
    // Paths
    let namespace: syn::Path = syn::parse_quote!(point);
    let field_path: syn::Path = syn::parse_quote!(field);
    let tag_path: syn::Path = syn::parse_quote!(tag);
    let timestamp_path: syn::Path = syn::parse_quote!(timestamp);

    // Struct-level
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    // eprintln!("{:#?}", ast.attrs);

    let measurement: String = if let syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue{
        path, lit, ..
    })) =
        namespace_parameter(&ast.attrs, &namespace).expect("Missing measurement tag, use #[point(measurement = \"something\")] before struct declaration")
     {
        if path.segments[0].ident == "measurement" {
            if let syn::Lit::Str(lit_str) = lit {
                lit_str.value()
            } else {
                return (quote! { compile_error!("Measurement should be a string") }).into();
            }
        } else {
            return (quote! { compile_error!("Top attribute is not measurement, which was expected") }).into();
        }
    } else {
        return (quote! { compile_error!("Did not find a suitable measurement tag should be in format '#[point(measurement = \"name\")]'"); }).into();
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
                        return (quote! { compile_error!("Attribute must be a string type"); })
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
    for field in ast_fields
        .iter()
        .filter(|field| field.contains_tag(&namespace, &field_path))
    {
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

    let complete_text = format!("{{}},{} {}", tag_names_combined, field_names_combined);

    let struct_timestamp = ast_fields
        .iter()
        .find(|field| field.contains_tag(&namespace, &timestamp_path))
        .expect("Missing timestamp field!")
        .ident
        .as_ref()
        .unwrap();

    // Output
    (quote! {
        impl PointSerialize for #name {
            fn serialize(&self) -> String {
                return format!(#complete_text, #measurement, #(self.#tag_tokens),*, #(self.#field_tokens),*).to_string();
            }
            fn serialize_with_timestamp(&self, timestamp: Option<Timestamp>) -> String {
                match timestamp {
                    Some(timestamp) => format!("{} {}", self.serialize(), timestamp.to_string()),
                    None => format!("{} {}", self.serialize(), self.#struct_timestamp.to_string())
                }
            }
        }
    })
    .into()
}

/* // TODO: Proper error handling, with compiler feedback
#[proc_macro_derive(PointSerialize, attributes(measurement, tag, field))]
pub fn point_serialize_derive(input: TokenStream) -> TokenStream {
    // Ast is top level struct definition
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let mut measurement: Option<String> = None;
    // Find top-level attributes, currently only measurement
    ast.attrs.into_iter().for_each(|attr| {
        match meta_matched_to_metanamevalue(attr.parse_meta().unwrap()) {
            // Match '#[ident = lit]' attributes, more specifically the only one defined for this use, measurement
            Some(syn::MetaNameValue { path, lit, .. })
                if path.segments[0].ident == "measurement" =>
            {
                if let syn::Lit::Str(lit) = lit {
                    measurement = Some(lit.value());
                }
            }
            _ => { /* Unknown attribute, could be some other macros */ }
        }
    });

    if measurement.is_none() {
        eprintln!("Missing measurement attribute! Make sure it looks like '#[measurement = \"Some measurement\"]'");
        panic!()
    }

    // Find struct field level attributes, currently only tag & field
    match data_matched_to_datastruct(ast.data)
        .and_then(|datastruct| fields_matched_to_fieldsnamed(datastruct.fields))
    {
        None => {
            eprintln!("Couldn't extract named fields from struct, do they exist?");
            panic!()
        }
        Some(syn::FieldsNamed { named, .. }) => {
            named
                .iter()
                .filter(|field| field.attrs.len() > 0) // Check that the field actually has a attribute
                .for_each(|field| {
                    /* eprintln!("{:#?}", field);
                    eprintln!(); */
                    // Check if the named field has a attribute
                    /* if (field) */
                    let ident = &field.attrs[0].path.segments[0].ident.to_string() as &str;

                    let field_name = field.ident.as_ref().map(|it| it.to_string());

                    match ident {
                        "tag" => {
                            let mut tag_name: Option<String> = None;
                            // Check if attribute has a value or field name should be used
                            if field.attrs[0].tokens.is_empty() {
                                tag_name = field_name;
                            } else {
                                match meta_matched_to_metanamevalue(
                                    field.attrs[0].parse_meta().unwrap(),
                                ) {
                                    Some(syn::MetaNameValue { lit, .. }) => {
                                        if let syn::Lit::Str(lit) = lit {
                                            tag_name = Some(lit.value());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            eprintln!("Found tag value! {} = {:?}", ident, tag_name.expect("FAK"));
                        }
                        "field" => {
                            let mut protocol_field_name: Option<String> = None;
                            // Check if attribute has a value or field name should be used
                            if field.attrs[0].tokens.is_empty() {
                                protocol_field_name = field_name;
                            } else {
                                match meta_matched_to_metanamevalue(
                                    field.attrs[0].parse_meta().unwrap(),
                                ) {
                                    Some(syn::MetaNameValue { lit, .. }) => {
                                        if let syn::Lit::Str(lit) = lit {
                                            protocol_field_name = Some(lit.value());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            eprintln!(
                                "Found field value! {} = {:?}",
                                ident,
                                protocol_field_name.expect("FAK")
                            );
                        }
                        _ => { /* Unknown field, could be some other macros */ }
                    }
                })
        }
    }

    let gen = quote! {
        impl PointSerialize for #name {
            fn serialize() -> String {
                return format!("{} ", #measurement).to_string();
            }
        }
    };
    gen.into()
} */

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let name = ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
