use proc_macro2::Ident;
use quote::format_ident;
use syn::{parse_quote, Expr, ItemImpl};
use crate::builder::ItemIdents;

const BUILDER_SUBJECT_FIELD_NAME: &'static str = "inner";
const PARAMS_ARGUMENT_NAME: &'static str = "params";

pub struct InternalIdents {
    pub builder_subject_field_name: Ident,
    pub params_argument_name: Ident
}

impl Default for InternalIdents {
    fn default() -> Self {
        Self {
            builder_subject_field_name: format_ident!("{}", BUILDER_SUBJECT_FIELD_NAME),
            params_argument_name: format_ident!("{}", PARAMS_ARGUMENT_NAME)
        }
    }
}

pub fn subject_impl_from_item_expr(item: &Expr, item_idents: &ItemIdents) -> ItemImpl {
    let ItemIdents {
        subject_ident,
        params_ident,
        builder_ident
    } = &item_idents;
    let InternalIdents {
        builder_subject_field_name,
        params_argument_name
    } = Default::default();

    parse_quote! {
        impl #subject_ident {
            pub fn builder(#params_argument_name: #params_ident) -> #builder_ident {
                #builder_ident {
                    #builder_subject_field_name: #item
                }
            }
        }
    }
}
