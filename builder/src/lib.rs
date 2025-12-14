use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    macro_impl::builder_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

mod macro_impl {
    use proc_macro2::TokenStream;
    use quote::format_ident;
    pub fn builder_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
        let ident = input.ident.clone();
        let builder_struct = format_ident!("{}Builder", ident);
        Ok(quote::quote! {
            struct #builder_struct {
                hello: String,
            }
            impl #builder_struct {
                pub fn print_derive_input(&self) {
                    println!("Hello Builder");
                }
            }
            impl #ident {
                pub fn builder() -> #builder_struct {
                    #builder_struct {
                        hello: "Hello".to_string(),
                    }
                }
            }
        })
    }
}
