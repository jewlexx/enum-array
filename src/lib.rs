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

    let mut arms = Vec::<TokenStream>::new();

    for var in vars.iter() {
        let var_name = &var.ident;

        let params = match var.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        let arm = quote! { #name::#var_name #params => stringify!(#var_name) };

        arms.push(arm);
    }

    let arms_len = arms.len();

    let v = quote! {
        impl #name {
            pub const fn to_array(&self) -> [#name; #arms_len] {
                [#(#arms),*]
            }
        }
    };

    v.into()
}

#[cfg(test)]
mod tests {}
