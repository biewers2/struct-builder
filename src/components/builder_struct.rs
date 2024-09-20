use crate::struct_builder::BuilderContext;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemStruct};

pub struct BuilderStruct {
    idents: BuilderContext
}

impl From<&ItemStruct> for BuilderStruct {
    fn from(value: &ItemStruct) -> Self {
        let idents = BuilderContext::from(value);

        Self { idents }
    }
}

impl ToTokens for BuilderStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            subject,
            builder,
            builder_subject_field,
            ..
        } = &self.idents;

        let builder_struct: ItemStruct = parse_quote! {
            pub struct #builder {
                #builder_subject_field: #subject
            }
        };
        
        builder_struct.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::{parse_quote, ItemStruct};
    use crate::components::BuilderStruct;

    #[test]
    fn test_with_named_fields() {
        let item_struct: ItemStruct = parse_quote! {
            pub struct MyStruct {
                pub public_field: String,
                private_field: String,
                optional: Option<usize>,
                pub test: std::option::Option<String>,
                test2: option::Option<T>,
                pub dynamic: Box<dyn Send>,
                pub dynamic2: Box<Option<dyn Send>>
            }
        };
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructBuilder {
                inner: MyStruct
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
        let item_struct: ItemStruct = parse_quote! {
            pub struct MyStruct(
                pub String,
                String,
                Option<usize>,
                pub std::option::Option<String>,
                option::Option<T>,
                pub Box<dyn Send>,
                pub Box<Option<dyn Send>>
            );
        };
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructBuilder {
                inner: MyStruct
            }
        };

        let builder_struct = BuilderStruct::from(&item_struct);

        assert_eq!(
            builder_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}