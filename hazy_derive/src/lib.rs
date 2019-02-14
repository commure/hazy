extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

use self::proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::spanned::Spanned;

fn mk_pat(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(named_fields) => {
            let pats = named_fields.named.iter().filter_map(|ref field| {
                field.ident.as_ref().map(|ident| {
                    let new_ident = syn::Ident::new(&format!("__field_{}", ident), ident.span());
                    quote! { #ident: ref #new_ident }
                })
            }).collect::<Vec<_>>();
            quote! { { #( #pats ),* } }
        }
        Fields::Unnamed(unnamed_fields) => {
            let pats = unnamed_fields.unnamed.iter().enumerate().map(|(i, ref field)| {
                syn::Ident::new(&format!("__field_{}", i), field.ty.span())
            }).collect::<Vec<_>>();
            quote! { (#( ref #pats ),*) }
        }
        Fields::Unit => quote! {},
    }
}

enum Opacity {
    Visible,
    Hidden,
    Default,
}

fn opacity(attrs: &[syn::Attribute]) -> Opacity {
    let mut hidden = false;
    let mut visible = false;
    for attr in attrs {
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
    if hidden {
        Opacity::Hidden
    } else if visible {
        Opacity::Visible
    } else {
        Opacity::Default
    }
}

fn mk_body(fields: &Fields, adt_ident: &syn::Ident) -> proc_macro2::TokenStream {
    let adt_ident_str = adt_ident.to_string();
    match fields {
        Fields::Named(named_fields) => {
            let pats = named_fields.named.iter().enumerate().map(|(i, ref field)| {
                let opacity = opacity(&field.attrs);
                let ident = field.ident.as_ref().unwrap();
                let new_ident = syn::Ident::new(&format!("__field_{}", ident), ident.span());
                let ty_sp = field.ty.span();
                let field = format!("{}: ", ident);
                let after = if i == named_fields.named.len() - 1 { "" } else { ", " };
                match opacity {
                    Opacity::Hidden => quote! {
                        write!(f, #field)?;
                        write!(f, "_")?;
                        write!(f, #after)?;
                    },
                    Opacity::Visible => quote_spanned! { ty_sp =>
                        write!(f, #field)?;
                        ::std::fmt::Debug::fmt(&#new_ident, f)?;
                        write!(f, #after)?;
                    },
                    Opacity::Default => quote_spanned! { ty_sp =>
                        write!(f, #field)?;
                        ::hazy::OpaqueDebug::fmt(&#new_ident, f)?;
                        write!(f, #after)?;
                    },
                }
            }).collect::<Vec<_>>();
            quote! {{
                write!(f, #adt_ident_str)?;
                write!(f, " {{ ")?;
                #( #pats ) *
                write!(f, " }}")?;
                Ok(())
            } }
        }
        Fields::Unnamed(unnamed_fields) => {
            let pats = unnamed_fields.unnamed.iter().enumerate().map(|(i, ref field)| {
                let new_ident = syn::Ident::new(&format!("__field_{}", i), field.ty.span());
                let after = if i == unnamed_fields.unnamed.len() - 1 { "" } else { ", " };
                let opacity = opacity(&field.attrs);
                let ty_sp = field.ty.span();
                match opacity {
                    Opacity::Hidden => quote! {
                        write!(f, "_")?;
                        write!(f, #after)?;
                    },
                    Opacity::Visible => quote_spanned! { ty_sp =>
                        ::std::fmt::Debug::fmt(&#new_ident, f)?;
                        write!(f, #after)?;
                    },
                    Opacity::Default => quote_spanned! { ty_sp =>
                        ::hazy::OpaqueDebug::fmt(&#new_ident, f)?;
                        write!(f, #after)?;
                    },
                }
            }).collect::<Vec<_>>();
            quote! {{
                write!(f, #adt_ident_str)?;
                write!(f, "(")?;
                #( #pats )*
                write!(f, ")")?;
                Ok(())
            }}
        }
        Fields::Unit => quote! { write!(f, #adt_ident_str) },
    }
}

#[proc_macro_derive(OpaqueDebug, attributes(debug))]
pub fn derive_hazy_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let adt_ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let pats = match input.data {
        Data::Struct(ref data) => {
            let pat = mk_pat(&data.fields);
            let body = mk_body(&data.fields, &adt_ident);
            vec![quote!{
                #adt_ident #pat => #body
            }]
        }
        Data::Enum(ref data) => {
            data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let pat = mk_pat(&variant.fields);
                let body = mk_body(&variant.fields, &variant.ident);
                quote!{ #adt_ident::#variant_ident #pat => #body }
            }).collect::<Vec<_>>()
        }
        _ => {
            unimplemented!("`OpaqueDebug` for `union` is not yet implemented");
        }
    };
    let output = quote!{
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generics ::hazy::OpaqueDebug for #adt_ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use ::hazy::OpaqueDebug;  // get the default implementations for builtin types
                match &*self {
                    #(#pats)*
                }
            }
        }

        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generics ::std::fmt::Debug for #adt_ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::hazy::OpaqueDebug::fmt(self, f)
            }
        }
    };
    output.into()
}
