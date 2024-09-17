use crate::builder::build_struct::{BuildStruct, BuildStructStats};
use proc_macro2::Ident;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Expr, Field, FieldValue, FieldsNamed, ImplItemFn, ItemStruct, Token, Type, Visibility};

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

    fn initialized_struct(&self, ident: Ident, field_source: Expr) -> Expr {
        let mut punctuated_fields = Punctuated::<FieldValue, Token![,]>::new();

        for field_builder in &self.field_builders {
            let field_ident = &field_builder.ident;

            let expr: Expr = if field_builder.required {
                parse_quote! { #field_source.#field_ident }
            } else {
                parse_quote! { ::std::option::Option::None }
            };

            punctuated_fields.push(parse_quote! {
                #field_ident: #expr
            });
        }

        parse_quote! {
            #ident {
                #punctuated_fields
            }
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
            pub struct #ident {
                #punctuated_fields
            }
        }
    }

    fn builder_functions(&self, item_ident: Ident) -> Vec<ImplItemFn> {
        let mut functions = Vec::<ImplItemFn>::new();

        for field_builder in &self.field_builders {
            let field_ident = &field_builder.ident;
            let field_type = &field_builder.field.ty;
            let fn_ident = format_ident!("with_{}", &field_ident);

            functions.push(parse_quote! {
                pub fn #fn_ident(mut self, value: #field_type) -> Self {
                    self.#item_ident.#field_ident = value;
                    self
                }
            });
        }

        functions
    }
}

impl From<FieldsNamed> for NamedStructBuilder {
    fn from(value: FieldsNamed) -> Self {
        let mut field_builders = Vec::<NamedFieldBuilder>::new();
        for field in value.named.into_iter() {
            if let Type::Path(path_type) = &field.ty {
                let ident = field.ident.clone().expect("named field is missing identifier");
                let required = path_type.path.segments.last()
                    .map(|seg| seg.ident != "Option")
                    .unwrap_or(true);

                field_builders.push(NamedFieldBuilder { ident, field, required });
            }
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
    use crate::builder::BuildStruct;
    use quote::{format_ident, ToTokens};
    use syn::{parse_quote, Expr, FieldsNamed, ImplItemFn, ItemStruct};

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
    fn test_initialized_struct() {
        let builder = NamedStructBuilder::from(get_fields_named());
        let ident = format_ident!("MyStruct");
        let field_source = parse_quote!(field.source);

        let initialized_struct = builder.initialized_struct(ident, field_source);
        let expected: Expr = parse_quote! {
            MyStruct {
                public_field: field.source.public_field,
                private_field: field.source.private_field,
                optional: ::std::option::Option::None,
                test: ::std::option::Option::None,
                test2: ::std::option::Option::None,
                dynamic: field.source.dynamic,
                dynamic2: field.source.dynamic2
            }
        };

        assert_eq!(
            initialized_struct.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
    
    #[test]
    fn test_params_struct() {
        let builder = NamedStructBuilder::from(get_fields_named());
        let ident = format_ident!("MyStructParams");

        let params_struct = builder.params_struct(ident);
        let expected: ItemStruct = parse_quote! {
            pub struct MyStructParams {
                pub public_field: String,
                pub private_field: String,
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
    fn test_builder_functions() {
        let builder = NamedStructBuilder::from(get_fields_named());
        let item_ident = format_ident!("inner");

        let builder_functions = builder.builder_functions(item_ident);
        let expected: Vec<ImplItemFn> = vec![
            parse_quote! {
                pub fn with_public_field(mut self, value: String) -> Self {
                    self.inner.public_field = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_private_field(mut self, value: String) -> Self {
                    self.inner.private_field = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_optional(mut self, value: Option<usize>) -> Self {
                    self.inner.optional = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_test(mut self, value: std::option::Option<String>) -> Self {
                    self.inner.test = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_test2(mut self, value: option::Option<T>) -> Self {
                    self.inner.test2 = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_dynamic(mut self, value: Box<dyn Send>) -> Self {
                    self.inner.dynamic = value;
                    self
                }
            },
            parse_quote! {
                pub fn with_dynamic2(mut self, value: Box<Option<dyn Send>>) -> Self {
                    self.inner.dynamic2 = value;
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
