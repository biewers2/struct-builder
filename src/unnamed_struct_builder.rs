use crate::build_struct::{BuildStruct, BuildStructStats};
use proc_macro2::Ident;
use syn::{Expr, ExprStruct, Field, FieldsUnnamed, ImplItemFn, Index, ItemStruct, Type};

pub struct UnnamedStructBuilder {
    field_builders: Vec<UnnamedFieldBuilder>
}

struct UnnamedFieldBuilder {
    pub index: Index,
    pub field: Field,
    pub required: bool
}

impl BuildStruct for UnnamedStructBuilder {
    fn stats(&self) -> &BuildStructStats {
        todo!()
    }
    
    fn initialized_struct(&self, ident: Ident, required_field_source: Expr) -> ExprStruct {
        todo!()
    }

    fn params_struct(&self, ident: Ident) -> ItemStruct {
        todo!()
    }

    fn builder_functions(&self, item_ident: Ident) -> Vec<ImplItemFn> {
        todo!()
    }
}

impl From<FieldsUnnamed> for UnnamedStructBuilder {
    fn from(value: FieldsUnnamed) -> Self {
        let field_builders = value.unnamed.into_iter()
            .enumerate()
            .filter_map(|(index, field)| {
                if let Type::Path(path_type) = &field.ty {
                    let index = Index::from(index);
                    let required = path_type.path.segments.last()
                        .map(|seg| seg.ident != "Option")
                        .unwrap_or(true);

                    Some(UnnamedFieldBuilder { index, field, required })
                } else {
                    None
                }
            })
            .collect();

        Self { field_builders }
    }
}
