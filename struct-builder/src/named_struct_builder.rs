use crate::build_struct::{BuildStruct, BuildStructStats};
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{parse, parse_quote, Expr, ExprStruct, Field, FieldValue, FieldsNamed, ImplItemFn, ItemStruct, Token, Type};

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

    fn initialized_struct(&self, ident: Ident, field_source: Expr) -> ExprStruct {
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
                punctuated_fields.push(field_builder.field.clone());
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
            let fn_ident = format_ident!("with_{}", &field_builder.ident);

            let function_token_stream = quote! {
                pub fn #fn_ident(mut self, value: #field_type) -> Self {
                    self.#item_ident.#field_ident = value;
                    self
                }
            };
            let function = parse(function_token_stream.into()).expect("function impl syntax invalid");
            functions.push(function);
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
