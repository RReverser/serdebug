use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

#[proc_macro_derive(SerDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident,
        mut generics,
        ..
    } = syn::parse(input).unwrap();

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: ::serde::Serialize));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    (quote! {
        impl #impl_generics ::std::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::serdebug::fmt(self, f)
            }
        }
    })
    .into()
}
