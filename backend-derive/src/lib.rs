use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput, Lit, Meta, MetaNameValue, NestedMeta};

/// Attribute macro for implementing the NamedStruct trait.
///
/// # Examples
///
/// ```rust
/// #[named_struct]
/// struct User {
///     // fields...
/// }
/// ```
///
/// You can also customize the singular and plural names:
///
/// ```rust
/// #[named_struct(singular = "person", plural = "people")]
/// struct User {
///     // fields...
/// }
/// ```
#[proc_macro_attribute]
pub fn named_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input struct
    let input = parse_macro_input!(item as DeriveInput);

    // Extract struct name
    let struct_name = &input.ident;

    // Parse attributes for custom names
    let attrs = syn::punctuated::Punctuated::<syn::NestedMeta, syn::token::Comma>::parse_terminated
        .parse(attr)
        .unwrap_or_default();

    // Default names based on the struct name (convert to snake_case)
    let struct_name_str = struct_name.to_string();
    let default_singular = struct_name_str.to_case(Case::Snake);
    let default_plural = format!("{}s", default_singular);

    // Check for custom names in attributes
    let mut singular_name = default_singular;
    let mut plural_name = default_plural;

    for nested_meta in attrs {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = nested_meta {
            let ident = path.get_ident();
            if let Some(ident) = ident {
                if ident == "singular" {
                    if let Lit::Str(lit_str) = lit {
                        singular_name = lit_str.value();
                    }
                } else if ident == "plural" {
                    if let Lit::Str(lit_str) = lit {
                        plural_name = lit_str.value();
                    }
                }
            }
        }
    }

    // Generate implementation
    let expanded = quote! {
        #input

        impl crate::repository::traits::NamedStruct for #struct_name {
            fn singular_name() -> &'static str {
                #singular_name
            }

            fn plural_name() -> &'static str {
                #plural_name
            }
        }
    };

    // Convert back to token stream
    expanded.into()
}
