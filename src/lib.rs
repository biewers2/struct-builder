extern crate proc_macro;

mod builder;

use builder::ItemBuilder;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(StructBuilder, attributes(builder))]
pub fn derive_builder(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let builder = ItemBuilder::from(item);

    proc_macro2::TokenStream::from_iter(vec![
        builder.new_params_struct(),
        builder.new_builder_struct(),
        builder.new_item_impl(),
        builder.new_builder_impl(),
        builder.new_conversion_impl_from_params()
    ]).into()
}
