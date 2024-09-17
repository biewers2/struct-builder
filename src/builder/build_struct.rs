use crate::builder::named_struct_builder::NamedStructBuilder;
use crate::builder::unit_struct_builder::UnitStructBuilder;
use crate::builder::unnamed_struct_builder::UnnamedStructBuilder;
use proc_macro2::Ident;
use syn::{Expr, Fields, ImplItemFn, ItemStruct};

pub trait BuildStruct {
    fn stats(&self) -> &BuildStructStats;

    fn initialized_struct(&self, ident: Ident, field_source: Expr) -> Expr;

    fn params_struct(&self, ident: Ident) -> ItemStruct;

    fn builder_functions(&self, item_ident: Ident) -> Vec<ImplItemFn>;
}

#[derive(Default)]
pub struct BuildStructStats {
    pub required_count: usize,
    pub optional_count: usize
}

pub fn struct_builder_from_fields(fields: Fields) -> Box<dyn BuildStruct> {
    match fields {
        Fields::Named(named_fields) => Box::new(NamedStructBuilder::from(named_fields)),
        Fields::Unnamed(unnamed_fields) => Box::new(UnnamedStructBuilder::from(unnamed_fields)),
        Fields::Unit => Box::new(UnitStructBuilder::default())
    }
}
