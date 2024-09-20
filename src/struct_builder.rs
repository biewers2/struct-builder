use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, ConstParam, GenericParam, Generics, Ident, ItemStruct, LifetimeParam, Token, TypeParam, WhereClause};
use syn::punctuated::Punctuated;
use crate::components::{BuilderStruct, ImplBuilderFns, ImplFromParamsForSubject, ImplFromSubjectForBuilder, ImplSubjectFnBuilder, ParamsStruct};

const PARAMS_ARGUMENT_NAME: &str = "params";
const BUILDER_SUBJECT_FIELD_NAME: &str = "inner";

pub struct StructBuilder(pub ItemStruct);

pub struct BuilderContext {
    pub subject: Ident,
    pub params: Ident,
    pub params_argument: Ident,
    pub builder: Ident,
    pub builder_subject_field: Ident,
    pub generics: GenericsContext
}

pub struct GenericsContext {
    pub generics_def: Generics,
    pub generics_expr: Generics,
    pub where_clause: Option<WhereClause>
}

impl From<&ItemStruct> for BuilderContext {
    fn from(item: &ItemStruct) -> Self {
        BuilderContext {
            subject: format_ident!("{}", &item.ident),
            params: format_ident!("{}Params", &item.ident),
            params_argument: format_ident!("{}", PARAMS_ARGUMENT_NAME),
            builder: format_ident!("{}Builder", &item.ident),
            builder_subject_field: format_ident!("{}", BUILDER_SUBJECT_FIELD_NAME),
            generics: GenericsContext::from(&item.generics)
        }
    }
}

impl From<&Generics> for GenericsContext {
    fn from(value: &Generics) -> Self {
        let mut generics_def = value.clone();
        generics_def.where_clause = None;

        let mut generics_expr = value.clone();
        generics_expr.params = generics_expr.params
            .into_iter()
            .map(|p| match p {
                GenericParam::Lifetime(LifetimeParam { lifetime, .. }) =>
                    GenericParam::Lifetime(parse_quote! { #lifetime }),
                
                GenericParam::Type(TypeParam { ident, .. }) =>
                    GenericParam::Type(parse_quote! { #ident }),
                
                GenericParam::Const(ConstParam { ident, .. }) =>
                    GenericParam::Const(parse_quote! { #ident })
            })
            .collect::<Punctuated<GenericParam, Token![,]>>();
        
        let where_clause = value.where_clause.clone();
        
        GenericsContext {
            generics_def,
            generics_expr,
            where_clause
        }
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
            Box::new(ImplFromParamsForSubject::from(item)),
            Box::new(ImplFromSubjectForBuilder::from(item)),
        ];

        token_streams.iter().for_each(|ts| ts.to_tokens(tokens));
    }
}
