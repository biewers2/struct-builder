use crate::struct_builder::{BuilderContext, GenericsContext};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Field, Fields, ItemStruct, Token};
use syn::punctuated::Punctuated;
use crate::components::is_required;

pub struct ParamsStruct {
    ctx: BuilderContext,
    fields: Fields
}

impl From<&ItemStruct> for ParamsStruct {
    fn from(value: &ItemStruct) -> Self {
        let ctx: BuilderContext = value.into();
        let fields = value.fields.clone();

        Self { ctx, fields }
    }
}

impl ToTokens for ParamsStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            params,
            generics,
            fields_metadata,
            attributes,
            ..
        } = &self.ctx;
        let GenericsContext {
            generics_def,
            where_clause,
            ..
        } = &generics;
        let attrs = &attributes.outer_attrs;
        
        let include_generics = fields_metadata.generic_required_fields_count > 0;
        
        match &self.fields {
            Fields::Named(_) => {
                let punctuated_fields = self.punctuated_fields();
                let item_struct: ItemStruct = if include_generics {
                    parse_quote! {
                        #(#attrs)*
                        pub struct #params #generics_def #where_clause {
                            #punctuated_fields
                        }
                    }
                } else {
                    parse_quote! {
                        #(#attrs)*
                        pub struct #params {
                            #punctuated_fields
                        }
                    }
                };

                item_struct.to_tokens(tokens);
            },
            
            Fields::Unnamed(_) => {
                let punctuated_fields = self.punctuated_fields();
                let item_struct: ItemStruct = if include_generics {
                    parse_quote! {
                        #(#attrs)*
                        pub struct #params #generics_def ( #punctuated_fields ) #where_clause;
                    }
                } else {
                    parse_quote! {
                        #(#attrs)*
                        pub struct #params ( #punctuated_fields );
                    }
                };

                item_struct.to_tokens(tokens);
            },
            
            Fields::Unit => ()
        }
    }
}

impl ParamsStruct {
    fn punctuated_fields(&self) -> Punctuated<Field, Token![,]> {
        self.fields
            .iter()
            .filter(|f| is_required(f))
            .cloned()
            .collect::<Punctuated<Field, Token![,]>>()
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, ItemStruct};
    use crate::components::params_struct::ParamsStruct;
    use crate::test_util::{sample_named_item_struct, sample_unnamed_item_struct};

    #[test]
    fn test_with_named_fields() { 
        let item_struct = sample_named_item_struct();
        let expected: ItemStruct = parse_quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub struct MyStructParams<T, I: Send, W>
            where
                W: Sync
            {
                pub public_field: String,
                private_field: String,
                pub dynamic: Box<dyn Send>,
                pub dynamic2: Box<Option<dyn Send>>,
                #[serde(rename = "simpleGeneric")]
                pub generic: T,
                pub generic_inline: I,
                pub generic_where: W
            }
        };
        
        let params_struct = ParamsStruct::from(&item_struct);

        assert_eq!(
            params_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unnamed_fields() {
        let item_struct = sample_unnamed_item_struct();
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructParams<T, I: Send, W>(
                pub String,
                String,
                pub Box<dyn Send>,
                pub Box<Option<dyn Send>>,
                #[inline_required]
                pub T,
                pub I,
                pub W
            )
            where
                W: Sync;
        };

        let params_struct = ParamsStruct::from(&item_struct);

        assert_eq!(
            params_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_with_unit_struct() {
        let item_struct = parse_quote! { pub struct MyStruct; };

        let params_struct = ParamsStruct::from(&item_struct);

        assert_eq!(
            params_struct.to_token_stream().to_string(),
            TokenStream::new().to_string()
        );
    }
}
