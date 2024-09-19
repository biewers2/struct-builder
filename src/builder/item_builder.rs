use crate::builder::BuildStruct;
use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{Ident, ItemStruct};

pub struct StructBuilder {
    idents: ItemIdents,
    struct_builder: Box<dyn BuildStruct>
}

pub struct ItemIdents {
    pub subject_ident: Ident,
    pub params_ident: Ident,
    pub builder_ident: Ident,
}

impl From<StructBuilder> for TokenStream {
    fn from(value: StructBuilder) -> Self {
        let struct_builder = value.struct_builder;
        
        TokenStream::from_iter(
            vec![
                struct_builder.subject_impl(&value.idents).to_token_stream(),
                struct_builder.params_struct(&value.idents).to_token_stream(),
                struct_builder.params_impl(&value.idents).to_token_stream(),
                struct_builder.builder_struct(&value.idents).to_token_stream(),
                struct_builder.builder_impl(&value.idents).to_token_stream()
            ]
        )
    }
}

impl StructBuilder {
    pub fn from(item: ItemStruct) -> Self {
        let subject_ident = item.ident;
        let params_ident = format_ident!("{}Params", &subject_ident);
        let builder_ident = format_ident!("{}Builder", &subject_ident);
        let idents = ItemIdents { subject_ident, params_ident, builder_ident };
        
        let struct_builder = item.fields.into();
        Self { idents, struct_builder }
    }

    // pub fn new_item_impl(&self) -> TokenStream {
    //     let Self {
    //         item_ident,
    //         item_ty,
    //         params_ident,
    //         params_ty,
    //         builder_ty,
    //         ..
    //     } = &self;
    // 
    //     let initialized_struct = self.struct_builder.initialized_struct(
    //         item_ty.clone(),
    //         parse_quote! { #params_ident }
    //     );
    // 
    //     let doc = format!(r#"
    //         Create a new builder.
    // 
    //         # Arguments
    // 
    //         `{params_ident}` - The fields required by {item_ty}
    //     "#);
    //     quote! {
    //         impl #item_ty {
    //             #[doc=#doc]
    //             pub fn builder(#params_ident: #params_ty) -> #builder_ty {
    //                 #builder_ty {
    //                     #item_ident: #initialized_struct
    //                 }
    //             }
    //         }
    //     }
    // }
    // 
    // pub fn new_params_struct(&self) -> TokenStream {
    //     let Self {
    //         item_ty,
    //         params_ty,
    //         ..
    //     } = &self;
    // 
    //     let params_struct = self.struct_builder.params_struct(params_ty.clone());
    // 
    //     let doc = format!(r#"
    //         Represents the fields required by {item_ty} to be constructed.
    //     "#);
    //     quote! {
    //         #[doc=#doc]
    //         #params_struct
    //     }
    // }
    // 
    // pub fn new_builder_struct(&self) -> TokenStream {
    //     let Self {
    //         item_ident,
    //         item_ty,
    //         builder_ty,
    //         ..
    //     } = &self;
    // 
    //     let doc = format!(r#"
    //         Represents a builder for the {item_ty} struct.
    //     "#);
    //     quote! {
    //         #[doc=#doc]
    //         pub struct #builder_ty {
    //             #item_ident: #item_ty
    //         }
    //     }
    // }
    // 
    // pub fn new_builder_impl(&self) -> TokenStream {
    //     let Self {
    //         item_ident,
    //         item_ty,
    //         builder_ty,
    //         ..
    //     } = &self;
    //     
    //     let functions = self.struct_builder.builder_functions(item_ident.clone());
    // 
    //     let doc = format!(r#"
    //         Consume this builder and build {item_ty}
    //     "#);
    //     quote! {
    //         impl #builder_ty {
    //             #(#functions)*
    // 
    //             #[doc=#doc]
    //             pub fn build(self) -> #item_ty {
    //                 self.#item_ident
    //             }
    //         }
    //     }
    // }
    // 
    // pub fn new_conversion_impl_from_params(&self) -> TokenStream {
    //     let Self {
    //         item_ty,
    //         params_ident,
    //         params_ty,
    //         ..
    //     } = &self;
    //     
    //     if self.struct_builder.stats().required_count > 0 {
    //         quote! {
    //             impl From<#params_ty> for #item_ty {
    //                 fn from(value: #params_ty) -> Self {
    //                     #item_ty::builder(value).build()
    //                 }
    //             }
    //         }
    //     } else {
    //         let initialized_struct = self.struct_builder.initialized_struct(
    //             item_ty.clone(),
    //             parse_quote! { #params_ident }
    //         );
    //         
    //         quote! {
    //             impl Default for #item_ty {
    //                 fn default() -> Self {
    //                     #initialized_struct
    //                 }
    //             }
    //         }
    //     }
    // }
    // 
    // pub fn new_builder_from_item(&self) -> TokenStream {
    //     let Self {
    //         item_ident,
    //         item_ty,
    //         builder_ty,
    //         ..
    //     } = &self;
    // 
    //     quote! {
    //         impl From<#item_ty> for #builder_ty {
    //             fn from(value: #item_ty) -> Self {
    //                 Self {
    //                    #item_ident: value
    //                 }
    //             }
    //         }
    //     }
    // }
    // 
    // pub fn new_item_from_builder(&self) -> TokenStream {
    //     let Self {
    //         item_ty,
    //         builder_ty,
    //         ..
    //     } = &self;
    // 
    //     quote! {
    //         impl From<#builder_ty> for #item_ty {
    //             fn from(value: #builder_ty) -> Self {
    //                 value.build()
    //             }
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::builder::StructBuilder;
    use quote::quote;
    use syn::{parse_quote, ItemStruct};

    fn get_item_struct() -> ItemStruct {
        parse_quote! {
            pub struct MyStruct {
                pub public_field: String,
                private_field: String,
                optional: Option<usize>,
                pub test: std::option::Option<String>,
                test2: option::Option<T>,
                pub dynamic: Box<dyn Send>,
                pub dynamic2: Box<Option<dyn Send>>,
                pub tuple: (u8, u16, String)
            }
        }
    }

    // #[test]
    // fn test_item_builder_from_item_struct() {
    //     let item_struct = get_item_struct();
    // 
    //     let builder = ItemBuilder::from(item_struct);
    // 
    //     assert_eq!(builder.item_ident.to_string(), "inner");
    //     assert_eq!(builder.item_ty.to_string(), "MyStruct");
    //     assert_eq!(builder.params_ident.to_string(), "params");
    //     assert_eq!(builder.params_ty.to_string(), "MyStructParams");
    //     assert_eq!(builder.builder_ty.to_string(), "MyStructBuilder");
    // }
    // 
    // #[test]
    // fn test_new_item_impl() {
    //     let item_struct = get_item_struct();
    //     let builder = ItemBuilder::from(item_struct);
    // 
    //     let item_impl = builder.new_item_impl();
    //     let expected = quote! {
    //         impl MyStruct {
    //             # [doc = "\n            Create a new builder.\n\n            # Arguments\n\n            `params` - The fields required by MyStruct\n        "]
    //             pub fn builder(params: MyStructParams) -> MyStructBuilder {
    //                 MyStructBuilder {
    //                     inner: MyStruct {
    //                         public_field: params.public_field,
    //                         private_field: params.private_field,
    //                         optional: ::std::option::Option::None,
    //                         test: ::std::option::Option::None,
    //                         test2: ::std::option::Option::None,
    //                         dynamic: params.dynamic,
    //                         dynamic2: params.dynamic2,
    //                         tuple: params.tuple
    //                     }
    //                 }
    //             }
    //         }
    //     };
    // 
    //     assert_eq!(item_impl.to_string(), expected.to_string());
    // }
    // 
    // #[test]
    // fn test_new_params_struct() {
    // 
    // }
    // 
    // #[test]
    // fn test_new_builder_struct() {
    // 
    // }
    // 
    // #[test]
    // fn test_new_builder_impl() {
    // 
    // }
    // 
    // #[test]
    // fn test_new_conversion_impl_from_params_with_required_fields() {
    // 
    // }
    // 
    // #[test]
    // fn test_new_conversion_impl_from_params_no_required_fields() {
    // 
    // }
    // 
    // #[test]
    // fn test_new_builder_from_item() {
    //     let item_struct: ItemStruct = parse_quote! {
    //         pub struct Account {
    //             pub account_id: String,
    //             pub email: Option<String>
    //         }
    //     };
    // 
    //     let item_builder = ItemBuilder::from(item_struct);
    //     let builder_from_item = item_builder.new_builder_from_item();
    //     let expected = quote! {
    //         impl From<Account> for AccountBuilder {
    //             fn from(value: Account) -> Self {
    //                 Self {
    //                     inner: value
    //                 }
    //             }
    //         }
    //     };
    // 
    //     assert_eq!(builder_from_item.to_string(), expected.to_string());
    // }
    // 
    // #[test]
    // fn test_new_item_from_builder() {
    //     let item_struct: ItemStruct = parse_quote! {
    //         pub struct Account {
    //             pub account_id: String,
    //             pub email: Option<String>
    //         }
    //     };
    // 
    //     let item_builder = ItemBuilder::from(item_struct);
    //     let item_from_builder = item_builder.new_item_from_builder();
    //     let expected = quote! {
    //         impl From<AccountBuilder> for Account {
    //             fn from(value: AccountBuilder) -> Self {
    //                 value.build()
    //             }
    //         }
    //     };
    // 
    //     assert_eq!(item_from_builder.to_string(), expected.to_string());
    // }
}
