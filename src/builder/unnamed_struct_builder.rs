use crate::builder::common_struct_building::{subject_impl_from_expr, InternalIdents};
use crate::builder::required::is_required;
use crate::builder::{BuildStruct, BuildStructStats, ItemIdents};
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Expr, Field, FieldsUnnamed, ImplItemFn, Index, ItemImpl, ItemStruct, Token};

pub struct UnnamedStructBuilder {
    stats: BuildStructStats,
    field_builders: Vec<UnnamedFieldBuilder>
}

struct UnnamedFieldBuilder {
    pub index: Index,
    pub field: Field,
    pub required: bool
}

impl BuildStruct for UnnamedStructBuilder {
    fn stats(&self) -> &BuildStructStats {
        &self.stats
    }

    fn subject_impl(&self, idents: &ItemIdents) -> Option<ItemImpl> {
        let ItemIdents {
            subject_ident,
            params_ident,
            builder_ident
        } = &idents;
        let InternalIdents {
            builder_subject_field: builder_subject_field_name,
            params_argument: params_argument_name
        } = Default::default();
        
        let mut next_index = 0;
        let punctuated_fields = self.field_builders
            .iter()
            .map::<Expr, _>(|fb|
                if fb.required {
                    let index = Index::from(next_index);
                    next_index += 1;
                    parse_quote! { #params_argument_name.#index }
                } else {
                    parse_quote! { ::std::option::Option::None }
                }
            )
            .collect::<Punctuated<Expr, Token![,]>>();
        
        Some(parse_quote! {
            impl #subject_ident {
                pub fn builder(#params_argument_name: #params_ident) -> #builder_ident {
                    #builder_ident {
                        #builder_subject_field_name: Self(#punctuated_fields)
                    }
                }
            }
        })
    }
    
    fn params_struct(&self, idents: &ItemIdents) -> Option<ItemStruct> {
        let params_ident = &idents.params_ident;
        
        let punctuated_fields = self.field_builders
            .iter()
            .filter(|fb| fb.required)
            .map(|fb| fb.field.clone())
            .collect::<Punctuated<Field, Token![,]>>();
    
        Some(parse_quote! {
            pub struct #params_ident(#punctuated_fields);
        })
    }

    fn builder_struct(&self, idents: &ItemIdents) -> Option<ItemStruct> {
        let ItemIdents {
            subject_ident,
            builder_ident,
            ..
        } = &idents;
        let builder_subject_field = InternalIdents::default().builder_subject_field;

        Some(parse_quote! {
            pub struct #builder_ident {
                #builder_subject_field: #subject_ident
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
            let field_index = &field_builder.index;
            let field_type = &field_builder.field.ty;
            let fn_ident = format_ident!("with_{}", &field_index);
    
            functions.push(parse_quote! {
                pub fn #fn_ident(mut self, value: #field_type) -> Self {
                    self.#subject_field_ident.#field_index = value;
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

impl From<FieldsUnnamed> for UnnamedStructBuilder {
    fn from(value: FieldsUnnamed) -> Self {
        let mut field_builders = Vec::<UnnamedFieldBuilder>::new();
        for (index, field) in value.unnamed.into_iter().enumerate() {
            let index = Index::from(index);
            let required = is_required(&field);
            field_builders.push(UnnamedFieldBuilder { index, field, required })
        }

        Self { 
            stats: From::from(&field_builders[..]),
            field_builders
        }
    }
}

impl From<&[UnnamedFieldBuilder]> for BuildStructStats {
    fn from(field_builders: &[UnnamedFieldBuilder]) -> Self {
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
    use crate::builder::unnamed_struct_builder::{UnnamedFieldBuilder, UnnamedStructBuilder};
    use crate::builder::{BuildStruct, ItemIdents};
    use quote::{format_ident, ToTokens};
    use syn::{parse_quote, FieldsUnnamed, Index, ItemImpl, ItemStruct};

    fn get_fields_unnamed() -> FieldsUnnamed {
        parse_quote! {
            (
                pub String,
                String,
                Option<usize>,
                pub std::option::Option<String>,
                option::Option<T>,
                pub Box<dyn Send>,
                pub Box<Option<dyn Send>>
            )
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
    fn test_unnamed_struct_builder_from_fields() {
        let fields_unnamed = get_fields_unnamed();
        let expected_values = vec![
            UnnamedFieldBuilder {
                index: Index::from(0),
                field: parse_quote!(pub String),
                required: true,
            },
            UnnamedFieldBuilder {
                index: Index::from(1),
                field: parse_quote!(String),
                required: true,
            },
            UnnamedFieldBuilder {
                index: Index::from(2),
                field: parse_quote!(Option<usize>),
                required: false,
            },
            UnnamedFieldBuilder {
                index: Index::from(3),
                field: parse_quote!(pub std::option::Option<String>),
                required: false,
            },
            UnnamedFieldBuilder {
                index: Index::from(4),
                field: parse_quote!(option::Option<T>),
                required: false,
            },
            UnnamedFieldBuilder {
                index: Index::from(5),
                field: parse_quote!(pub Box<dyn Send>),
                required: true,
            },
            UnnamedFieldBuilder {
                index: Index::from(6),
                field: parse_quote!(pub Box<Option<dyn Send>>),
                required: true,
            }
        ];

        let builder = UnnamedStructBuilder::from(fields_unnamed);

        assert_eq!(builder.field_builders.len(), 7);
        for (fbuilder, expected) in builder.field_builders.iter().zip(expected_values) {
            assert_eq!(
                fbuilder.field.to_token_stream().to_string(),
                expected.field.to_token_stream().to_string()
            );
            assert_eq!(fbuilder.index.index, expected.index.index);
            assert_eq!(fbuilder.required, expected.required);
        }
    }

    #[test]
    fn test_build_struct_stats_from_field_builders() {
        let fields_unnamed = get_fields_unnamed();

        let builder = UnnamedStructBuilder::from(fields_unnamed);

        assert_eq!(builder.stats.required_count, 4);
        assert_eq!(builder.stats.optional_count, 3);
    }

    #[test]
    fn test_subject_impl() {
        let builder = UnnamedStructBuilder::from(get_fields_unnamed());
        let idents = get_item_idents("MyStruct");
        
        let subject_impl = builder.subject_impl(&idents);
        let expected: ItemImpl = parse_quote! {
            impl MyStruct {
                pub fn builder(params: MyStructParams) -> MyStructBuilder {
                    MyStructBuilder {
                        inner: Self(
                            params.0,
                            params.1,
                            ::std::option::Option::None,
                            ::std::option::Option::None,
                            ::std::option::Option::None,
                            params.2,
                            params.3
                        )
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
        let builder = UnnamedStructBuilder::from(get_fields_unnamed());
        let idents = get_item_idents("MyStruct");
    
        let params_struct = builder.params_struct(&idents);
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructParams(
                pub String,
                String,
                pub Box<dyn Send>,
                pub Box<Option<dyn Send>>
            );
        };
    
        assert_eq!(
            params_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }


    #[test]
    fn test_builder_struct() {
        let builder = UnnamedStructBuilder::from(get_fields_unnamed());
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
    fn test_builder_functions() {
        let builder = UnnamedStructBuilder::from(get_fields_unnamed());
        let idents = get_item_idents("MyStruct");

        let builder_impl = builder.builder_impl(&idents);
        let expected: ItemImpl = parse_quote! {
            impl MyStructBuilder {
                pub fn with_0(mut self, value: String) -> Self {
                    self.inner.0 = value;
                    self
                }

                pub fn with_1(mut self, value: String) -> Self {
                    self.inner.1 = value;
                    self
                }

                pub fn with_2(mut self, value: Option<usize>) -> Self {
                    self.inner.2 = value;
                    self
                }

                pub fn with_3(mut self, value: std::option::Option<String>) -> Self {
                    self.inner.3 = value;
                    self
                }

                pub fn with_4(mut self, value: option::Option<T>) -> Self {
                    self.inner.4 = value;
                    self
                }

                pub fn with_5(mut self, value: Box<dyn Send>) -> Self {
                    self.inner.5 = value;
                    self
                }

                pub fn with_6(mut self, value: Box<Option<dyn Send>>) -> Self {
                    self.inner.6 = value;
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
        );
    }
}
