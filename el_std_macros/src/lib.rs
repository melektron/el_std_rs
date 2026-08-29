/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
26.06.26, 18:48
All rights reserved.

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree. 
*/

//! Proc-macros for [`el_std`]. Please refer to [`el_std`] for documentation.

mod repl;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};


#[proc_macro_derive(AutoDeref)]
pub fn auto_derive_deref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let field_ty = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed[0].ty
            }
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "AutoDeref can only be derived for structs with exactly one unnamed field",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input,
                "AutoDeref can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::core::ops::Deref for #name #ty_generics #where_clause {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
    .into()
}


#[proc_macro_derive(AutoDerefMut)]
pub fn auto_derive_deref_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {}
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "AutoDerefMut requires a struct with exactly one unnamed field",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input,
                "AutoDerefMut requires a struct",
            )
            .to_compile_error()
            .into();
        }
    }

    let (impl_generics, ty_generics, where_clause) =
        generics.split_for_impl();

    quote::quote! {
        impl #impl_generics ::core::ops::DerefMut
            for #name #ty_generics #where_clause
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
    .into()
}
