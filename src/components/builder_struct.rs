use crate::struct_builder::{BuilderContext, GenericsContext};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Fields, ItemStruct};

pub struct BuilderStruct {
    ctx: BuilderContext,
    unit: bool
}

impl From<&ItemStruct> for BuilderStruct {
    fn from(value: &ItemStruct) -> Self {
        let ctx: BuilderContext = value.into();
        let unit = matches!(value.fields, Fields::Unit);

        Self { ctx, unit }
    }
}

impl ToTokens for BuilderStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            subject,
            builder,
            builder_subject_field,
            generics,
            ..
        } = &self.ctx;
        let GenericsContext {
            generics_def,
            generics_expr,
            where_clause,
        } = &generics;

        if !self.unit {
            let builder_struct: ItemStruct = parse_quote! {
                pub struct #builder #generics_def #where_clause {
                    #builder_subject_field: #subject #generics_expr
                }
            };

            builder_struct.to_tokens(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::BuilderStruct;
    use crate::test_util::{sample_named_item_struct, sample_unit_item_struct, sample_unnamed_item_struct};
    use proc_macro::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn test_with_named_fields() {
        let item_struct = sample_named_item_struct();
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructBuilder<T, I: Send, W>
            where
                W: Sync
            {
                inner: MyStruct<T, I, W>
            }
        };
        
        let builder_struct = BuilderStruct::from(&item_struct);

        assert_eq!(
            builder_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unnamed_fields() {
        let item_struct = sample_unnamed_item_struct();
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructBuilder<T, I: Send, W>
            where
                W: Sync
            {
                inner: MyStruct<T, I, W>
            }
        };

        let builder_struct = BuilderStruct::from(&item_struct);

        assert_eq!(
            builder_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unit_struct() {
        let item_struct = sample_unit_item_struct();

        let builder_struct = BuilderStruct::from(&item_struct);

        assert_eq!(
            builder_struct.to_token_stream().to_string(),
            TokenStream::new().to_string()
        );
    }
}