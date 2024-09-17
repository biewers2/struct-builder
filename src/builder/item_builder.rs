use crate::builder::{struct_builder_from_fields, BuildStruct};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Ident, ItemStruct};

pub struct ItemBuilder {
    item_ident: Ident,
    item_ty: Ident,
    params_ident: Ident,
    params_ty: Ident,
    builder_ty: Ident,
    struct_builder: Box<dyn BuildStruct>
}

impl ItemBuilder {
    pub fn from(item: ItemStruct) -> Self {
        let ident = item.ident;
        let struct_builder = struct_builder_from_fields(item.fields);

        Self {
            item_ident: format_ident!("inner"),
            item_ty: ident.clone(),
            params_ty: format_ident!("{}Params", &ident),
            params_ident: format_ident!("params"),
            builder_ty: format_ident!("{}Builder", &ident),
            struct_builder
        }
    }

    pub fn new_item_impl(&self) -> TokenStream {
        let ItemBuilder {
            item_ident,
            item_ty,
            params_ident,
            params_ty,
            builder_ty,
            ..
        } = &self;

        let initialized_struct = self.struct_builder.initialized_struct(
            item_ty.clone(),
            parse_quote! { #params_ident }
        );
        
        quote! {
            impl #item_ty {
                pub fn builder(#params_ident: #params_ty) -> #builder_ty {
                    #builder_ty {
                        #item_ident: #initialized_struct
                    }
                }
            }
        }
    }

    pub fn new_params_struct(&self) -> TokenStream {
        self.struct_builder.params_struct(self.params_ty.clone()).into_token_stream()
    }

    pub fn new_builder_struct(&self) -> TokenStream {
        let ItemBuilder {
            item_ident,
            item_ty,
            builder_ty,
            ..
        } = &self;

        quote! {
            pub struct #builder_ty {
                #item_ident: #item_ty
            }
        }
    }

    pub fn new_builder_impl(&self) -> TokenStream {
        let ItemBuilder {
            item_ident,
            item_ty,
            builder_ty,
            ..
        } = &self;
        
        let functions = self.struct_builder.builder_functions(item_ident.clone());
        
        quote! {
            impl #builder_ty {
                #(#functions)*
                
                pub fn build(self) -> #item_ty {
                    self.#item_ident
                }
            }
        }
    }
    
    pub fn new_conversion_impl_from_params(&self) -> TokenStream {
        let ItemBuilder {
            item_ty,
            params_ident,
            params_ty,
            ..
        } = &self;
        
        if self.struct_builder.stats().required_count > 0 {
            quote! {
                impl From<#params_ty> for #item_ty {
                    fn from(value: #params_ty) -> Self {
                        #item_ty::builder(value).build()
                    }
                }
            }
        } else {
            let initialized_struct = self.struct_builder.initialized_struct(
                item_ty.clone(),
                parse_quote! { #params_ident }
            );
            
            quote! {
                impl Default for #item_ty {
                    fn default() -> Self {
                        #initialized_struct
                    }
                }
            }
        }
    }

    pub fn new_builder_from_item(&self) -> TokenStream {
        let ItemBuilder {
            item_ident,
            item_ty,
            builder_ty,
            ..
        } = &self;

        quote! {
            impl From<#item_ty> for #builder_ty {
                fn from(value: #item_ty) -> Self {
                    Self {
                       #item_ident: value
                    }
                }
            }
        }
    }

    pub fn new_item_from_builder(&self) -> TokenStream {
        let ItemBuilder {
            item_ident,
            item_ty,
            builder_ty,
            ..
        } = &self;

        quote! {
            impl From<#builder_ty> for #item_ty {
                fn from(value: #builder_ty) -> Self {
                    value.#item_ident
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::ItemBuilder;
    use quote::quote;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn test_item_builder_from_item_struct() {

    }

    #[test]
    fn test_new_item_impl() {

    }

    #[test]
    fn test_new_params_struct() {

    }

    #[test]
    fn test_new_builder_struct() {

    }

    #[test]
    fn test_new_builder_impl() {

    }

    #[test]
    fn test_new_conversion_impl_from_params_with_required_fields() {

    }

    #[test]
    fn test_new_conversion_impl_from_params_no_required_fields() {

    }

    #[test]
    fn test_new_builder_from_item() {
        let item_struct: ItemStruct = parse_quote! {
            pub struct Account {
                pub account_id: String,
                pub email: Option<String>
            }
        };

        let item_builder = ItemBuilder::from(item_struct);
        let builder_from_item = item_builder.new_builder_from_item();
        let expected = quote! {
            impl From<Account> for AccountBuilder {
                fn from(value: Account) -> Self {
                    Self {
                        inner: value
                    }
                }
            }
        };

        assert_eq!(builder_from_item.to_string(), expected.to_string());
    }

    #[test]
    fn test_new_item_from_builder() {
        let item_struct: ItemStruct = parse_quote! {
            pub struct Account {
                pub account_id: String,
                pub email: Option<String>
            }
        };

        let item_builder = ItemBuilder::from(item_struct);
        let item_from_builder = item_builder.new_item_from_builder();
        let expected = quote! {
            impl From<AccountBuilder> for Account {
                fn from(value: AccountBuilder) -> Self {
                    value.inner
                }
            }
        };

        assert_eq!(item_from_builder.to_string(), expected.to_string());
    }
}
