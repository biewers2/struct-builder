use syn::{parse_quote, ItemStruct};

pub fn sample_named_item_struct() -> ItemStruct {
    parse_quote! {
        pub struct MyStruct<T, I: Send, W>
        where
            W: Sync
        {
            pub public_field: String,
            private_field: String,
            optional: Option<usize>,
            pub test: std::option::Option<String>,
            test2: option::Option<T>,
            pub dynamic: Box<dyn Send>,
            pub dynamic2: Box<Option<dyn Send>>,
            pub generic: T,
            pub generic_inline: I,
            pub generic_where: W
        }
    }
}

pub fn sample_unnamed_item_struct() -> ItemStruct {
    parse_quote! {
        pub struct MyStruct<T, I: Send, W>(
            pub String,
            String,
            Option<usize>,
            pub std::option::Option<String>,
            option::Option<T>,
            pub Box<dyn Send>,
            pub Box<Option<dyn Send>>,
            pub T,
            pub I,
            pub W
        )
        where
            W: Sync;
    }
}

pub fn sample_unit_item_struct() -> ItemStruct {
    parse_quote! { pub struct MyStruct; }
}
