use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, GenericParam, Generics, Ident, ItemStruct, TypeParam};
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
    pub generics: Generics
}

pub struct GenericsContext {
    generics_def: Generics,
    generics_expr: Generics,
    where_expr: Generics
}

impl From<&ItemStruct> for BuilderContext {
    fn from(item: &ItemStruct) -> Self {
        BuilderContext {
            subject: format_ident!("{}", &item.ident),
            params: format_ident!("{}Params", &item.ident),
            params_argument: format_ident!("{}", PARAMS_ARGUMENT_NAME),
            builder: format_ident!("{}Builder", &item.ident),
            builder_subject_field: format_ident!("{}", BUILDER_SUBJECT_FIELD_NAME),
            generics: item.generics.clone()
        }
    }
}

impl From<&Generics> for GenericsContext {
    fn from(value: &Generics) -> Self {
        let mut generics_def = value.clone();
        generics_def.where_clause = None;

        let mut generics_expr = value.clone();
        // generics_expr.params.into_iter().map(|p| match p {
        //     GenericParam::Lifetime(_) => todo!(),
        //     GenericParam::Type(TypeParam { ident, .. }) => parse_quote! { #ident }),
        //     GenericParam::Const(_) => panic!("not supported!")
        // })
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
