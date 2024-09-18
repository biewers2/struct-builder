use crate::builder::named_struct_builder::NamedStructBuilder;
use crate::builder::unit_struct_builder::UnitStructBuilder;
use crate::builder::unnamed_struct_builder::UnnamedStructBuilder;
use proc_macro2::Ident;
use syn::{Expr, Fields, ImplItemFn, ItemImpl, ItemStruct, Type};

/// Defines the interface used by the [ItemBuilder] to construct syntax based on the type of struct being derived from.
/// 
/// Structs that are defined with named fields vs. unnamed fields are different enough to warrant their own implementations, and this
/// trait unifies their interface so the item builder can be agnostic to the syntax of the struct being derived from.
/// 
pub trait BuildStruct {
    /// Stats on the struct, such as number of required/optional fields.
    /// 
    fn stats(&self) -> &BuildStructStats;

    /// The implementation of the subject item.
    /// 
    /// # Arguments
    /// 
    /// `ident` - The identifier of the subject item.
    /// 
    fn subject_impl(&self, subject_ident: Ident) -> ItemImpl;

    /// The definition of the params item.
    /// 
    /// The params item defines required fields found in the subject item (those that don't have the [Option] type).
    ///
    /// # Arguments
    ///
    /// `ident` - The identifier of the param struct's type.
    ///
    fn params_struct(&self, ident: Ident) -> ItemStruct;
    
    /// The implementation of the params item.
    /// 
    fn params_impl(&self, ident: Ident) -> ItemImpl;
    
    /// The definition of the builder item.
    /// 
    /// 
    fn builder_struct(&self, ident: Ident) -> ItemStruct;
    
    /// A list of functions as defined in an impl block that represent the builder's functions to be called by the user.
    /// 
    /// # Arguments
    /// 
    /// `item_ident` - The identifier of the field in the builder used to access the fields of the item struct being derived from.
    /// 
    fn builder_impl(&self, item_ident: Ident) -> ItemImpl;
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
