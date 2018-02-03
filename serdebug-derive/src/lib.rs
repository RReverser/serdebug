extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(SerDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let mut predicates_tokens = quote::Tokens::default();

    let empty_or_trailing;

    if let Some(where_clause) = ast.generics.where_clause.take() {
        let predicates = where_clause.predicates;
        empty_or_trailing = predicates.empty_or_trailing();
        predicates_tokens.append_all(predicates);
    } else {
        empty_or_trailing = true;
    };

    if empty_or_trailing {
        predicates_tokens.append_all(quote!(,));
    }

    predicates_tokens.append_all(quote!(Self: ::serde::Serialize));

    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics where #predicates_tokens {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&::serdebug::Serialize(self), f)
            }
        }
    };

    tokens.into()
}
