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
    use syn::{spanned::Spanned, Data, DeriveInput, Fields, Index};
    pub fn builder_impl(input: syn::DeriveInput) -> syn::Result<TokenStream> {
        let span = input.span();
        let DeriveInput { data, ident, .. } = input;
        let Data::Struct(ds) = data else {
            return Err(syn::Error::new(span, "expected struct"));
        };
        let builder_struct = format_ident!("{}Builder", ident);
        let fields = match ds.fields {
            Fields::Named(f) => f.named,
            _ => {
                return Err(syn::Error::new(span, "expected named field"));
            }
        };
        let builder_fields = fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote::quote! {
                #ident: ::core::option::Option<#ty>
            }
        });
        let builder_fields_init = fields.iter().map(|f| {
            let ident = &f.ident;
            quote::quote! {
                #ident: None
            }
        });
        Ok(quote::quote! {
            struct #builder_struct {
                #(#builder_fields,)*
            }
            impl #builder_struct {
                pub fn print_derive_input(&self) {
                    println!("Hello Builder");
                }
            }
            impl #ident {
                pub fn builder() -> #builder_struct {
                    #builder_struct {
                        #(#builder_fields_init,)*
                    }
                }
            }
        })
    }
}
