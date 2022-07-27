use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn macro_error(msg: &str) -> proc_macro::TokenStream {
    quote::quote! {
       compile_error!(#msg)
    }
    .into()
}

#[proc_macro_derive(EnumArray)]
pub fn derive_enum_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let vars = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return macro_error("Can only be derived on an enum"),
    };

    let (array_arms, array_len) = {
        let mut arms = Vec::<TokenStream>::new();

        for var in vars.iter() {
            let var_name = &var.ident;

            match var.fields {
                Fields::Unit => (),
                _ => return macro_error("Enum cannot have arguments"),
            };

            let arm = quote! { #name::#var_name };

            arms.push(arm);
        }
        let arms_len = arms.len();

        (arms, arms_len)
    };

    let (str_array, str_len) = {
        let mut arms = Vec::<TokenStream>::new();

        for var in vars.iter() {
            let var_name = &var.ident;

            match var.fields {
                Fields::Unit => (),
                _ => return macro_error("Enum cannot have arguments"),
            };

            let arm = quote! { stringify!(#var_name) };

            arms.push(arm);
        }

        let arms_len = arms.len();

        (arms, arms_len)
    };

    let v = quote! {
        impl #name {
            pub const fn to_array() -> [#name; #array_len] {
                [#(#array_arms),*]
            }
        }
    };

    v.into()
}
