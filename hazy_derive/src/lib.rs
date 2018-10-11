extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

use self::proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(OpaqueDebug, attributes(debug))]
pub fn derive_hazy_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let ident = name.to_string();
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let ((start, end), fields) = match input.data {
        Data::Struct(ref data) => {
            let len = data.fields.iter().count() - 1;
            let f = data.fields.iter().enumerate().map(|(i, ref field)| {
                let mut hidden = false;
                let mut visible = false;
                for attr in &field.attrs {
                    let path = &attr.path;
                    if format!("{}", quote!(#path)) == "debug" {
                        if format!("{}", attr.tts) == "( Clear )" {
                            visible = true;
                        }
                        if format!("{}", attr.tts) == "( Hidden )" {
                            hidden = true;
                        }
                    }
                }
                field.ident.as_ref().map(|ident| {
                    let field = format!("{}: ", ident);
                    let after = if i != len { ", " } else { " " };
                    if hidden {
                        quote! {
                            write!(f, #field)?;
                            write!(f, "_")?;
                            write!(f, #after)?;
                        }
                    } else if visible {
                        quote! {
                            write!(f, #field)?;
                            ::std::fmt::Debug::fmt(&self.#ident, f)?;
                            write!(f, #after)?;
                        }

                    } else {
                        quote! {
                            write!(f, #field)?;
                            ::hazy::OpaqueDebug::fmt(&self.#ident, f)?;
                            write!(f, #after)?;
                        }
                    }
                }).unwrap_or_else(|| {
                    quote! {
                        write!(f, "_")?;
                    }
                })
            }).collect::<Vec<_>>();
            (match data.fields {
                Fields::Named(_) => {
                    (" {{ ", "}}")
                }
                Fields::Unnamed(_) => {
                    ("(", ")")
                }
                Fields::Unit => {
                    ("", ";")
                }
            }, f)
        }
        // Data::Enum(ref data) => {
        //     for variant in &data.variants {
        //         let ident = variant.ident;
        //         quote! {
        //             write!(f, #field)?;
        //             write!(f, "_")?;
        //             write!(f, #after)?;
        //         }
        //     }
        // }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let output = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generics ::hazy::OpaqueDebug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use ::hazy::OpaqueDebug;  // get the default implementations for builtin types

                write!(f, #ident)?;
                write!(f, #start)?;
                #(
                    #fields
                )*
                write!(f, #end)
            }
        }

        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::hazy::OpaqueDebug::fmt(self, f)
            }
        }
    };
    output.into()
}

// #[proc_macro_attribute]
// pub fn hazy(_args: TokenStream, _input: TokenStream) -> TokenStream {
//     (quote! {}).into()
// }