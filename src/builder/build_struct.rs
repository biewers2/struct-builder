use crate::builder::named_struct_builder::NamedStructBuilder;
use crate::builder::unit_struct_builder::UnitStructBuilder;
use crate::builder::unnamed_struct_builder::UnnamedStructBuilder;
use proc_macro2::Ident;
use syn::{parse_quote, Expr, Fields, ImplItemFn, ItemImpl, ItemStruct, Type};
use crate::builder::ItemIdentifiers;

/// Defines the interface used by the [ItemBuilder] to construct syntax based on the type of struct being derived from.
/// 
/// Structs that are defined with named fields vs. unnamed fields are different enough to warrant their own implementations, and this
/// trait unifies their interface so the item builder can be agnostic to the syntax of the struct being derived from.
/// 
pub trait BuildStruct {
    /// Stats on the struct, such as number of required/optional fields.
    /// 
    fn stats(&self) -> &BuildStructStats;

    /// An implementation of the subject item.
    /// 
    /// # Arguments
    /// 
    /// `idents` - The builder-related identifiers
    /// 
    /// # Returns
    /// 
    /// An item implementation of the subject item that implements any required methods to implement the
    /// builder pattern. If [None] is returned, then no implementation is needed to implement the builder pattern.
    /// 
    fn subject_impl(&self, idents: ItemIdentifiers) -> Option<ItemImpl> {
        None
    }

    /// The definition of the params item.
    /// 
    /// The params item defines required fields found in the subject item (those that don't have the [Option] type).
    ///
    /// # Arguments
    ///
    /// `idents` - The builder-related identifiers
    /// 
    /// # Returns
    /// 
    /// A struct definition containing required fields matching those in the subject item. If [None] is returned, then
    /// no params struct is needed to implement the builder pattern.
    ///
    fn params_struct(&self, idents: ItemIdentifiers) -> Option<ItemStruct> {
        None
    }
    
    /// An implementation of the params item.
    /// 
    /// # Arguments
    /// 
    /// `idents` - The builder-related identifiers
    /// 
    /// # Returns
    /// 
    /// An item implementation of the params item that implements any required methods to implement the
    /// builder pattern. If [None] is returned, then no implementation is needed to implement the builder pattern.
    /// 
    fn params_impl(&self, idents: ItemIdentifiers) -> Option<ItemImpl> {
        None
    }
    
    /// The definition of the builder item.
    /// 
    /// # Arguments
    ///
    /// `idents` - The builder-related identifiers
    /// 
    /// # Returns
    /// 
    /// A struct definition of the builder item. If [None] is returned, then no params struct is needed to implement
    /// the builder pattern.
    ///
    fn builder_struct(&self, idents: ItemIdentifiers) -> Option<ItemStruct> {
        None
    }

    /// An implementation of the builder item.
    /// 
    /// The builder item defines functions used to build fields found in the subject item.
    /// 
    /// # Arguments
    /// 
    /// `idents` - The builder-related identifiers
    ///
    /// # Returns
    ///
    /// An item implementation of the builder item that implements any required methods to implement the
    /// builder pattern. If [None] is returned, then no implementation is needed to implement the builder pattern.
    ///
    fn builder_impl(&self, idents: ItemIdentifiers) -> Option<ItemImpl> {
        None
    }
}

#[derive(Default)]
pub struct BuildStructStats {
    /// Number of required fields.
    pub required_count: usize,
    
    /// Number of optional fields (fields that have type [Option]).
    pub optional_count: usize
}

impl From<Fields> for Box<dyn BuildStruct> {
    fn from(fields: Fields) -> Self {
        match fields {
            Fields::Named(named_fields) => Box::new(NamedStructBuilder::from(named_fields)),
            Fields::Unnamed(unnamed_fields) => Box::new(UnnamedStructBuilder::from(unnamed_fields)),
            Fields::Unit => Box::new(UnitStructBuilder::default())
        }
    }
}
