use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// implement the Validator trait which calls validate on each property by default
#[proc_macro_derive(Validated)]
pub fn derive_validated(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, data, .. } = &input;

    let obj = match data {
        syn::Data::Struct(obj) => obj,
        _ => panic!("Only structs supported in From macro"),
    };

    let fields = obj
        .fields
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect::<Vec<_>>();

    quote!(
        impl Validator for #ident {
            fn validate(&self, ctx: &ValidateContext) -> Result<(), ValidateError> {
                #(self.#fields.validate(ctx)?;)*
                Ok(())
            }
        }
    )
    .into()
}
