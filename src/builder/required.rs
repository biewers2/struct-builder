use syn::{Field, Type};

pub fn is_required(field: &Field) -> bool {
    if let Type::Path(path_type) = &field.ty {
        path_type.path.segments.last()
            .map(|seg| seg.ident != "Option")
            .unwrap_or(true)
    } else {
        true
    }
}