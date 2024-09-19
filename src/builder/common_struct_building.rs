use proc_macro2::Ident;
use quote::format_ident;
use syn::{parse_quote, Expr, ItemImpl};
use crate::builder::ItemIdents;

const BUILDER_SUBJECT_FIELD_NAME: &str = "inner";
const PARAMS_ARGUMENT_NAME: &str = "params";

pub struct InternalIdents {
    pub builder_subject_field: Ident,
    pub params_argument: Ident
}

impl Default for InternalIdents {
    fn default() -> Self {
        Self {
            builder_subject_field: format_ident!("{}", BUILDER_SUBJECT_FIELD_NAME),
            params_argument: format_ident!("{}", PARAMS_ARGUMENT_NAME)
        }
    }
}
