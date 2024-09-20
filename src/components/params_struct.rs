use crate::struct_builder::BuilderContext;
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
        let idents = BuilderContext::from(value);
        let fields = value.fields.clone();

        Self { ctx: idents, fields }
    }
}

impl ToTokens for ParamsStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BuilderContext {
            params,
            generics,
            ..
        } = &self.ctx;
        
        match &self.fields {
            Fields::Named(_) => {
                let punctuated_fields = self.punctuated_fields();
                let item_struct: ItemStruct = parse_quote! {
                    pub struct #params #generics {
                        #punctuated_fields
                    }
                };
                
                item_struct.to_tokens(tokens);
            },
            
            Fields::Unnamed(_) => {
                let punctuated_fields = self.punctuated_fields();
                let item_struct: ItemStruct = parse_quote! {
                    pub struct #params #generics ( #punctuated_fields );
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
            .map(|f| f.clone())
            .collect::<Punctuated<Field, Token![,]>>()
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, ItemStruct};
    use crate::components::params_struct::ParamsStruct;

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
            pub struct MyStructParams {
                pub public_field: String,
                private_field: String,
                pub dynamic: Box<dyn Send>,
                pub dynamic2: Box<Option<dyn Send>>
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
            pub struct MyStructParams(
                pub String,
                String,
                pub Box<dyn Send>,
                pub Box<Option<dyn Send>>
            );
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
