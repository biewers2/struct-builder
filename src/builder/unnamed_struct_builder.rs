use crate::builder::{BuildStruct, BuildStructStats};
use crate::builder::required::is_required;
use proc_macro2::Ident;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Expr, Field, FieldsUnnamed, ImplItemFn, Index, ItemStruct, Token, Visibility};

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
    
    fn initialized_struct(&self, ident: Ident, field_source: Expr) -> Expr {
        let mut punctuated_fields = Punctuated::<Expr, Token![,]>::new();

        let mut next_index = 0;
        for field_builder in &self.field_builders {
            let expr: Expr = if field_builder.required {
                let index = Index::from(next_index);
                next_index += 1;
                parse_quote! { #field_source.#index }
            } else {
                parse_quote! { ::std::option::Option::None }
            };
            punctuated_fields.push(expr);
        }

        parse_quote! {
            #ident(#punctuated_fields)
        }
    }

    fn params_struct(&self, ident: Ident) -> ItemStruct {
        let mut punctuated_fields = Punctuated::<Field, Token![,]>::new();

        for field_builder in &self.field_builders {
            if field_builder.required {
                let mut field = field_builder.field.clone();
                field.vis = Visibility::Public(Default::default());
                punctuated_fields.push(field);
            }
        }

        parse_quote! {
            pub struct #ident(#punctuated_fields);
        }
    }

    fn builder_functions(&self, item_ident: Ident) -> Vec<ImplItemFn> {
        let mut functions = Vec::<ImplItemFn>::new();

        for field_builder in &self.field_builders {
            let field_index = &field_builder.index;
            let field_type = &field_builder.field.ty;
            let fn_ident = format_ident!("with_{}", &field_index);

            functions.push(parse_quote! {
                pub fn #fn_ident(mut self, value: #field_type) -> Self {
                    self.#item_ident.#field_index = value;
                    self
                }
            });
        }

        functions
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
    use crate::builder::BuildStruct;
    use quote::{format_ident, ToTokens};
    use syn::{parse_quote, Expr, FieldsUnnamed, ImplItemFn, Index, ItemStruct};

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
    fn test_initialized_struct() {
        let builder = UnnamedStructBuilder::from(get_fields_unnamed());
        let ident = format_ident!("MyStruct");
        let field_source = parse_quote!(field.source);

        let initialized_struct = builder.initialized_struct(ident, field_source);
        let expected: Expr = parse_quote! {
            MyStruct(
                field.source.0,
                field.source.1,
                ::std::option::Option::None,
                ::std::option::Option::None,
                ::std::option::Option::None,
                field.source.2,
                field.source.3
            )
        };

        assert_eq!(
            initialized_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_params_struct() {
        let builder = UnnamedStructBuilder::from(get_fields_unnamed());
        let ident = format_ident!("MyStructParams");

        let params_struct = builder.params_struct(ident);
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructParams(
                pub String,
                pub String,
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
    fn test_builder_functions() {
        let builder = UnnamedStructBuilder::from(get_fields_unnamed());
        let item_ident = format_ident!("inner");

        let builder_functions = builder.builder_functions(item_ident);
        let expected: Vec<ImplItemFn> = vec![
            parse_quote! {
                pub fn with_0(mut self, value: String) -> Self {
                    self.inner.0 = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_1(mut self, value: String) -> Self {
                    self.inner.1 = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_2(mut self, value: Option<usize>) -> Self {
                    self.inner.2 = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_3(mut self, value: std::option::Option<String>) -> Self {
                    self.inner.3 = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_4(mut self, value: option::Option<T>) -> Self {
                    self.inner.4 = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_5(mut self, value: Box<dyn Send>) -> Self {
                    self.inner.5 = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_6(mut self, value: Box<Option<dyn Send>>) -> Self {
                    self.inner.6 = value;
                    self
                }
            }
        ];

        for (function, expected) in builder_functions.iter().zip(expected) {
            assert_eq!(
                function.to_token_stream().to_string(),
                expected.to_token_stream().to_string()
            );
        }
    }
}
