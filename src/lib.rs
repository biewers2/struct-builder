extern crate proc_macro;
mod item_builder;
mod build_struct;
mod named_struct_builder;
mod unnamed_struct_builder;
mod unit_struct_builder;

use crate::item_builder::ItemBuilder;
use syn::{parse_macro_input, Item};

#[proc_macro_derive(StructBuilder, attributes(builder))]
pub fn derive_builder(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = match parse_macro_input!(item as Item) {
        Item::Struct(item) => item,
        _ => panic!("StructBuilder can only be used on structs")
    };

    let builder = ItemBuilder::from(item);

    proc_macro2::TokenStream::from_iter(vec![
        builder.new_params_struct(),
        builder.new_builder_struct(),
        builder.new_item_impl(),
        builder.new_builder_impl(),
        builder.new_conversion_impl_from_params()
    ]).into()
}
