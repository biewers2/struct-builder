use crate::components::{is_required, BuilderStruct, ImplBuilderFns, ImplFromBuilderForSubject, ImplFromParamsForSubject, ImplFromSubjectForBuilder, ImplSubjectFnBuilder, ParamsStruct};
use crate::generic_resolution::field_has_generic;
use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::punctuated::Punctuated;
use syn::{parse_quote, Attribute, ConstParam, GenericParam, Generics, Ident, ItemStruct, LifetimeParam, Token, TypeParam, WhereClause};

const PARAMS_ARGUMENT_NAME: &str = "params";
const BUILDER_SUBJECT_FIELD_NAME: &str = "inner";

pub struct StructBuilder(pub ItemStruct);

pub struct BuilderContext {
    pub subject: Ident,
    pub params: Ident,
    pub params_argument: Ident,
    pub builder: Ident,
    pub builder_subject_field: Ident,
    pub attributes: AttributesContext,
    pub generics: GenericsContext,
    pub fields_metadata: FieldsMetadata
}

pub struct AttributesContext {
    pub outer_attrs: Vec<Attribute>,
}

pub struct GenericsContext {
    pub generics_def: Generics,
    pub generics_expr: Generics,
    pub where_clause: Option<WhereClause>
}

pub struct FieldsMetadata {
    pub required_fields_count: usize,
    pub optional_fields_count: usize,
    pub generic_required_fields_count: usize,
    pub generic_optional_fields_count: usize
}

impl From<&ItemStruct> for BuilderContext {
    fn from(item: &ItemStruct) -> Self {
        BuilderContext {
            subject: format_ident!("{}", &item.ident),
            params: format_ident!("{}Params", &item.ident),
            params_argument: format_ident!("{}", PARAMS_ARGUMENT_NAME),
            builder: format_ident!("{}Builder", &item.ident),
            builder_subject_field: format_ident!("{}", BUILDER_SUBJECT_FIELD_NAME),
            attributes: item.into(),
            generics: item.into(),
            fields_metadata: item.into()
        }
    }
}

impl From<&ItemStruct> for AttributesContext {
    fn from(item: &ItemStruct) -> Self {
        let outer_attrs = item.attrs.to_owned();
        Self { outer_attrs } 
    }
}

impl From<&ItemStruct> for GenericsContext {
    fn from(item: &ItemStruct) -> Self {
        let generics = &item.generics;

        // Definitions of generic params, including bounds
        let mut generics_def = generics.to_owned();
        generics_def.where_clause = None;

        // Expression of generic params, just the identifiers (no bounds)
        let mut generics_expr = generics.to_owned();
        generics_expr.params = generics_expr.params
            .into_iter()
            // Strip everything but the identifier from each param
            .map(|p| match p {
                GenericParam::Lifetime(LifetimeParam { lifetime, .. }) =>
                    GenericParam::Lifetime(parse_quote! { #lifetime }),
                
                GenericParam::Type(TypeParam { ident, .. }) =>
                    GenericParam::Type(parse_quote! { #ident }),
                
                GenericParam::Const(ConstParam { ident, .. }) =>
                    GenericParam::Const(parse_quote! { #ident })
            })
            .collect::<Punctuated<GenericParam, Token![,]>>();
        
        // Separate where clause
        let where_clause = generics.where_clause.to_owned();
        
        GenericsContext {
            generics_def,
            generics_expr,
            where_clause
        }
    }
}

impl From<&ItemStruct> for FieldsMetadata {
    fn from(value: &ItemStruct) -> Self {
        let mut meta = Self {
            required_fields_count: 0,
            optional_fields_count: 0,
            generic_required_fields_count: 0,
            generic_optional_fields_count: 0,
        };

        for field in &value.fields {
            let generic = field_has_generic(&value.generics, &field);
            let required = is_required(&field);

            if generic && required {
                meta.generic_required_fields_count += 1;
            } else if generic && !required {
                meta.generic_optional_fields_count += 1;
            } else if !generic && required {
                meta.required_fields_count += 1;
            } else {
                meta.optional_fields_count += 1;
            }
        }

        meta
    }
}

impl ToTokens for StructBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(item) = &self;
        
        let token_streams: Vec<Box<dyn ToTokens>> = vec![
            Box::new(ImplSubjectFnBuilder::from(item)),
            Box::new(ParamsStruct::from(item)),
            Box::new(BuilderStruct::from(item)),
            Box::new(ImplBuilderFns::from(item)),
            Box::new(ImplFromBuilderForSubject::from(item)),
            Box::new(ImplFromParamsForSubject::from(item)),
            Box::new(ImplFromSubjectForBuilder::from(item)),
        ];

        token_streams.iter().for_each(|ts| ts.to_tokens(tokens));
    }
}
