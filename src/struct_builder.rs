use crate::components::{BuilderStruct, ImplBuilderFns, ImplFromBuilderForSubject, ImplFromParamsForSubject, ImplFromSubjectForBuilder, ImplSubjectFnBuilder, ParamsStruct};
use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, GenericParam, Generics, Ident, ItemStruct, TypeParam};

const PARAMS_ARGUMENT_NAME: &str = "params";
const BUILDER_SUBJECT_FIELD_NAME: &str = "inner";

pub struct StructBuilder(pub ItemStruct);

pub struct BuilderContext {
    pub subject: Ident,
    pub params: Ident,
    pub params_argument: Ident,
    pub builder: Ident,
    pub builder_subject_field: Ident,
    pub generics: GenericsContext,
    pub fields_metadata: FieldsMetadata
}

pub struct FieldsMetadata {
    required_fields_count: usize,
    optional_fields_count: usize,
    generic_required_fields_count: usize,
    generic_optional_fields_count: usize
}

pub struct GenericsContext {
    generics_def: Generics,
    generics_expr: Generics,
    where_expr: Generics
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
        }
        
        meta
    }
}

impl From<&ItemStruct> for BuilderContext {
    fn from(item: &ItemStruct) -> Self {
        BuilderContext {
            subject: format_ident!("{}", &item.ident),
            params: format_ident!("{}Params", &item.ident),
            params_argument: format_ident!("{}", PARAMS_ARGUMENT_NAME),
            builder: format_ident!("{}Builder", &item.ident),
            builder_subject_field: format_ident!("{}", BUILDER_SUBJECT_FIELD_NAME),
            generics: GenericsContext::from(&item.generics),
            fields_metadata: FieldsMetadata::from(item)
        }
    }
}

impl From<&Generics> for GenericsContext {
    fn from(value: &Generics) -> Self {
        let mut generics_def = value.clone();
        generics_def.where_clause = None;

        let mut generics_expr = value.clone();
        generics_expr.params.into_iter().map(|p| match p {
            GenericParam::Lifetime(_) => todo!(),
            GenericParam::Type(TypeParam { ident, .. }) => parse_quote! { #ident }),
            GenericParam::Const(_) => panic!("not supported!")
        })
    }
}

impl BuilderContext {
    pub fn are_params_generic(&self) -> bool {
        self.fields_metadata.generic_required_fields_count > 0
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
