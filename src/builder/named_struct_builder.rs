use crate::builder::required::is_required;
use crate::builder::{BuildStruct, BuildStructStats, ItemIdents};
use proc_macro2::Ident;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Expr, Field, FieldValue, FieldsNamed, ImplItemFn, ItemImpl, ItemStruct, Token};
use crate::builder::common_struct_builder::{subject_impl_from_item_expr, InternalIdents};

pub struct NamedStructBuilder {
    stats: BuildStructStats,
    field_builders: Vec<NamedFieldBuilder>,
}

struct NamedFieldBuilder {
    pub ident: Ident,
    pub field: Field,
    pub required: bool,
}

impl BuildStruct for NamedStructBuilder {
    fn stats(&self) -> &BuildStructStats {
        &self.stats
    }

    fn subject_impl(&self, idents: &ItemIdents) -> Option<ItemImpl> {
        let params_argument_name = InternalIdents::default().params_argument_name;

        let punctuated_fields = self.field_builders 
            .iter()
            .map::<FieldValue, _>(|fb| {
                let field_ident = &fb.ident;
                if fb.required {
                    parse_quote! { #field_ident: #params_argument_name.#field_ident }
                } else {
                    parse_quote! { #field_ident: ::std::option::Option::None }
                }
            })
            .collect::<Punctuated<FieldValue, Token![,]>>();

        let item: Expr = parse_quote! { Self { #punctuated_fields } };
        let item_impl = subject_impl_from_item_expr(&item, &idents);
        Some(item_impl)
    }
    
    fn params_struct(&self, idents: &ItemIdents) -> Option<ItemStruct> {
        let params_ident = &idents.params_ident;

        let punctuated_fields = self.field_builders
            .iter()
            .filter(|fb| fb.required)
            .map(|fb| fb.field.clone())
            .collect::<Punctuated<Field, Token![,]>>();

        Some(parse_quote! {
            pub struct #params_ident {
                #punctuated_fields
            }
        })
    }
    
    fn builder_struct(&self, idents: &ItemIdents) -> Option<ItemStruct> {
        let ItemIdents {
            subject_ident,
            builder_ident,
            ..
        } = &idents;
        let subject_field_ident = self.builder_subject_field_ident();

        Some(parse_quote! {
            pub struct #builder_ident {
                #subject_field_ident: #subject_ident
            }
        })
    }

    fn builder_impl(&self, idents: &ItemIdents) -> Option<ItemImpl> {
        let ItemIdents {
            subject_ident,
            builder_ident,
            ..
        } = &idents;
        let subject_field_ident = self.builder_subject_field_ident();

        let mut functions = Vec::<ImplItemFn>::new();
        for field_builder in &self.field_builders {
            let field_ident = &field_builder.ident;
            let field_type = &field_builder.field.ty;
            let fn_ident = format_ident!("with_{}", &field_ident);

            functions.push(parse_quote! {
                pub fn #fn_ident(mut self, value: #field_type) -> Self {
                    self.#subject_field_ident.#field_ident = value;
                    self
                }
            });
        }

        Some(parse_quote! {
            impl #builder_ident {
                #(#functions)*
                
                pub fn build(self) -> #subject_ident {
                    self.#subject_field_ident
                }
            }
        })
    }
}

impl From<FieldsNamed> for NamedStructBuilder {
    fn from(value: FieldsNamed) -> Self {
        let mut field_builders = Vec::<NamedFieldBuilder>::new();
        for field in value.named.into_iter() {
            let ident = field.ident.clone().expect("named field is missing identifier");
            let required = is_required(&field);
            
            field_builders.push(NamedFieldBuilder { ident, field, required });
        }

        Self {
            stats: From::from(&field_builders[..]),
            field_builders,
        }
    }
}

impl From<&[NamedFieldBuilder]> for BuildStructStats {
    fn from(field_builders: &[NamedFieldBuilder]) -> Self {
        let mut stats = BuildStructStats::default();
        for builder in field_builders.iter() {
            if builder.required {
                stats.required_count += 1;
            } else {
                stats.optional_count += 1;
            }
        }
        stats
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::named_struct_builder::{NamedFieldBuilder, NamedStructBuilder};
    use crate::builder::{BuildStruct, ItemIdents};
    use quote::{format_ident, ToTokens};
    use syn::{parse_quote, FieldsNamed, ItemImpl, ItemStruct};

    fn get_fields_named() -> FieldsNamed {
        parse_quote! {
            {
                pub public_field: String,
                private_field: String,
                optional: Option<usize>,
                pub test: std::option::Option<String>,
                test2: option::Option<T>,
                pub dynamic: Box<dyn Send>,
                pub dynamic2: Box<Option<dyn Send>>
            }
        }
    }
    
    fn get_item_idents(base: impl AsRef<str>) -> ItemIdents {
        ItemIdents {
            subject_ident: format_ident!("{}", base.as_ref()),
            params_ident: format_ident!("{}Params", base.as_ref()),
            builder_ident: format_ident!("{}Builder", base.as_ref()),
        }
    }

    #[test]
    fn test_named_struct_builder_from_fields() {
        let fields_named = get_fields_named();
        let expected_values = vec![
            NamedFieldBuilder {
                ident: format_ident!("public_field"),
                field: parse_quote!(pub public_field: String),
                required: true,
            },
            NamedFieldBuilder {
                ident: format_ident!("private_field"),
                field: parse_quote!(private_field: String),
                required: true,
            },
            NamedFieldBuilder {
                ident: format_ident!("optional"),
                field: parse_quote!(optional: Option<usize>),
                required: false,
            },
            NamedFieldBuilder {
                ident: format_ident!("test"),
                field: parse_quote!(pub test: std::option::Option<String>),
                required: false,
            },
            NamedFieldBuilder {
                ident: format_ident!("test2"),
                field: parse_quote!(test2: option::Option<T>),
                required: false,
            },
            NamedFieldBuilder {
                ident: format_ident!("dynamic"),
                field: parse_quote!(pub dynamic: Box<dyn Send>),
                required: true,
            },
            NamedFieldBuilder {
                ident: format_ident!("dynamic2"),
                field: parse_quote!(pub dynamic2: Box<Option<dyn Send>>),
                required: true,
            }
        ];

        let builder = NamedStructBuilder::from(fields_named);

        assert_eq!(builder.field_builders.len(), 7);
        for (fbuilder, expected) in builder.field_builders.iter().zip(expected_values) {
            assert_eq!(
                fbuilder.field.to_token_stream().to_string(),
                expected.field.to_token_stream().to_string()
            );
            assert_eq!(fbuilder.ident, expected.ident);
            assert_eq!(fbuilder.required, expected.required);
        }
    }

    #[test]
    fn test_build_struct_stats_from_field_builders() {
        let fields_named = get_fields_named();

        let builder = NamedStructBuilder::from(fields_named);

        assert_eq!(builder.stats.required_count, 4);
        assert_eq!(builder.stats.optional_count, 3);
    }

    #[test]
    fn test_subject_impl() {
        let builder = NamedStructBuilder::from(get_fields_named());
        let idents = get_item_idents("MyStruct");

        let subject_impl = builder.subject_impl(&idents);
        let expected: ItemImpl = parse_quote! {
            impl MyStruct {
                pub fn builder(params: MyStructParams) -> MyStructBuilder {
                    MyStructBuilder {
                        inner: Self {
                            public_field: params.public_field,
                            private_field: params.private_field,
                            optional: ::std::option::Option::None,
                            test: ::std::option::Option::None,
                            test2: ::std::option::Option::None,
                            dynamic: params.dynamic,
                            dynamic2: params.dynamic2
                        }
                    }
                }
            }
        };

        assert_eq!(
            subject_impl.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_params_struct() {
        let builder = NamedStructBuilder::from(get_fields_named());
        let idents = get_item_idents("MyStruct");
    
        let params_struct = builder.params_struct(&idents);
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructParams {
                pub public_field: String,
                private_field: String,
                pub dynamic: Box<dyn Send>,
                pub dynamic2: Box<Option<dyn Send>>
            }
        };
    
        assert_eq!(
            params_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_builder_struct() {
        let builder = NamedStructBuilder::from(get_fields_named());
        let idents = get_item_idents("MyStruct");

        let builder_struct = builder.builder_struct(&idents);
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructBuilder {
                inner: MyStruct
            }
        };

        assert_eq!(
            builder_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );       
    }
    
    #[test]
    fn test_builder_impl() {
        let builder = NamedStructBuilder::from(get_fields_named());
        let idents = get_item_idents("MyStruct");

        let builder_impl = builder.builder_impl(&idents);
        let expected: ItemImpl = parse_quote! {
            impl MyStructBuilder {
                pub fn with_public_field(mut self, value: String) -> Self {
                    self.inner.public_field = value;
                    self
                }

                pub fn with_private_field(mut self, value: String) -> Self {
                    self.inner.private_field = value;
                    self
                }

                pub fn with_optional(mut self, value: Option<usize>) -> Self {
                    self.inner.optional = value;
                    self
                }

                pub fn with_test(mut self, value: std::option::Option<String>) -> Self {
                    self.inner.test = value;
                    self
                }

                pub fn with_test2(mut self, value: option::Option<T>) -> Self {
                    self.inner.test2 = value;
                    self
                }

                pub fn with_dynamic(mut self, value: Box<dyn Send>) -> Self {
                    self.inner.dynamic = value;
                    self
                }

                pub fn with_dynamic2(mut self, value: Box<Option<dyn Send>>) -> Self {
                    self.inner.dynamic2 = value;
                    self
                }
                
                pub fn build(self) -> MyStruct {
                    self.inner
                }
            }
        };
        
        assert_eq!(
            builder_impl.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        )
    }
}
