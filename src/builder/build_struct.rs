use crate::builder::named_struct_builder::NamedStructBuilder;
use crate::builder::unit_struct_builder::UnitStructBuilder;
use crate::builder::unnamed_struct_builder::UnnamedStructBuilder;
use proc_macro2::Ident;
use syn::{Expr, Fields, ImplItemFn, ItemStruct};

/// Defines the interface used by the [ItemBuilder] to construct syntax based on the type of struct being derived from.
/// 
/// Structs that are defined with named fields vs. unnamed fields are different enough to warrant their own implementations, and this
/// trait unifies their interface so the item builder can be agnostic to the syntax of the struct being derived from.
/// 
pub trait BuildStruct {
    /// Stats on the struct, such as number of required/optional fields.
    /// 
    fn stats(&self) -> &BuildStructStats;

    /// An expression representing the instantiation of the item struct being derived from.
    /// 
    /// # Arguments
    /// 
    /// `ident` - Identifier of the item struct's type.
    /// `field_source` - Expression that defines the required fields of the item struct. This should used to initialize the required fields.
    /// 
    fn initialized_struct(&self, ident: Ident, field_source: Expr) -> Expr;

    /// An item struct that represents the set of required fields needed to construct the item struct being derived from.
    /// 
    /// # Arguments
    /// 
    /// `ident` - The identifier of the param struct's type.
    /// 
    fn params_struct(&self, ident: Ident) -> ItemStruct;

    /// A list of functions as defined in an impl block that represent the builder's functions to be called by the user.
    /// 
    /// # Arguments
    /// 
    /// `item_ident` - The identifier of the field in the builder used to access the fields of the item struct being derived from.
    /// 
    fn builder_functions(&self, item_ident: Ident) -> Vec<ImplItemFn>;
}

#[derive(Default)]
pub struct BuildStructStats {
    /// Number of required fields.
    pub required_count: usize,
    
    /// Number of optional fields (fields that have type [Option]).
    pub optional_count: usize
}

pub fn struct_builder_from_fields(fields: Fields) -> Box<dyn BuildStruct> {
    match fields {
        Fields::Named(named_fields) => Box::new(NamedStructBuilder::from(named_fields)),
        Fields::Unnamed(unnamed_fields) => Box::new(UnnamedStructBuilder::from(unnamed_fields)),
        Fields::Unit => Box::new(UnitStructBuilder::default())
    }
}
