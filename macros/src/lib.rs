use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ApiResponse)]
pub fn api_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl ApiResponse for #struct_name {
            fn deserialize_response(bytes: Bytes) -> Result<Self, ApiError> {
                serde_json::from_slice(&bytes)
                    .map_err(|e| ApiError::Deserialization(format!("Deserialization error: {}", e)))
            }
        }
    };

    expanded.into()
}
