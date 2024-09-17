use crate::builder::{BuildStruct, BuildStructStats};
use proc_macro2::Ident;
use syn::{parse_quote, Expr, ImplItemFn, ItemStruct};

#[derive(Default)]
pub struct UnitStructBuilder {
    stats: BuildStructStats
}

impl BuildStruct for UnitStructBuilder {
    fn stats(&self) -> &BuildStructStats {
        &self.stats
    }
    
    fn initialized_struct(&self, ident: Ident, _required_field_source: Expr) -> Expr {
        parse_quote! {
            #ident
        }
    }

    fn params_struct(&self, ident: Ident) -> ItemStruct {
        parse_quote! {
            pub struct #ident;
        }
    }

    fn builder_functions(&self, _item_ident: Ident) -> Vec<ImplItemFn> {
        Vec::new()
    }
}
