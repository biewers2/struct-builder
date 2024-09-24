use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::{Field, GenericParam, Generics, ReturnType, Token, Type};

struct SearchIdents {
    types: Vec<Ident>,
    lifetimes: Vec<Ident>
}

#[inline]
pub fn field_has_generic(generics: &Generics, field: &Field) -> bool {
    generic_params_contain_type(&generics.params, &field.ty)
}

fn generic_params_contain_type(generic_params: &Punctuated<GenericParam, Token![,]>, ty: &Type) -> bool {
    let mut search_idents = SearchIdents {
        types: vec![],
        lifetimes: vec![],
    };

    for param in generic_params {
        match param {
            GenericParam::Lifetime(lp) => search_idents.lifetimes.push(lp.lifetime.ident.clone()),
            GenericParam::Type(tp) => search_idents.types.push(tp.ident.clone()),
            GenericParam::Const(cp) => search_idents.types.push(cp.ident.clone())
        }
    }

    search_idents_contain_type(&search_idents, &ty)
}

fn search_idents_contain_type(search_idents: &SearchIdents, ty: &Type) -> bool {
    #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]
    match ty {
        Type::Array(array) => search_idents_contain_type(&search_idents, &array.elem),

        Type::BareFn(bare_fn) => {
            let mut contains = false;
            if let Some(lts) = &bare_fn.lifetimes {
                contains = contains || generic_params_contain_type(&lts.lifetimes, &ty)
            }
            for input in &bare_fn.inputs {
                contains = contains || search_idents_contain_type(&search_idents, &input.ty)
            }
            if let ReturnType::Type(_, inner_ty) = &bare_fn.output {
                contains = contains || search_idents_contain_type(&search_idents, &inner_ty);
            }
            contains
        }

        Type::Group(group) => search_idents_contain_type(&search_idents, &group.elem),

        Type::Paren(paren) => search_idents_contain_type(&search_idents, &paren.elem),

        Type::Path(path) => {
            if let Some(qself) = &path.qself {
                search_idents_contain_type(&search_idents, &qself.ty)
            } else {
                let p = &path.path;
                search_idents.types.iter().any(|ty| p.is_ident(ty))
            }
        },

        Type::Ptr(ptr) => search_idents_contain_type(&search_idents, &ptr.elem),

        Type::Reference(reference) => {
            let mut contains = false;
            
            if let Some(lt) = &reference.lifetime {
                contains = contains || search_idents.lifetimes.contains(&lt.ident)
            }
            
            contains || search_idents_contain_type(&search_idents, &reference.elem)
        },

        Type::Slice(slice) => search_idents_contain_type(&search_idents, &slice.elem),
        
        Type::Tuple(tuple) => tuple.elems.iter().any(|el| search_idents_contain_type(&search_idents, &el)),

        // todo - unknown
        Type::ImplTrait(_) => false,

        // todo - unknown
        Type::TraitObject(_) => false,

        _ => false
    }
}

#[cfg(test)]
mod tests {
    use crate::generic_resolution::field_has_generic;
    use syn::{parse_quote, Field, Generics};

    macro_rules! test_cases {
        ($($name:tt| $generics:tt | $field:tt |$res:literal),*$(,)?) => {
            $(
                #[test]
                fn $name() {
                    let generics: Generics = parse_quote! $generics ;
                    let field: Field = parse_quote! $field ;
                    
                    assert_eq!(field_has_generic(&generics, &field), $res);
                }
            )*
        };
    }
    
    test_cases! {
        test_array_no_generics |{      }|{ values: [A; 3]  }| false,
        test_array_has_type    |{ <T>  }|{ values: [T; 2]       }| true,
        test_array_has_liftime |{ <'a> }|{ values: [&'a B; 1] }| true,
        
        test_bare_fn_no_generics        |{      }|{ block: fn(B) -> u64         }| false,
        test_bare_fn_has_input_type     |{ <T>  }|{ block: fn(T) -> u64         }| true,
        test_bare_fn_has_output_type    |{ <T>  }|{ block: fn(u32) -> T         }| true,
        test_bare_fn_has_input_liftime  |{ <'a> }|{ block: fn(&'a str) -> usize }| true,
        test_bare_fn_has_output_liftime |{ <'a> }|{ block: fn() -> &'a str      }| true,
        
        test_paren_no_generics |{      }|{ value: (String)  }| false,
        test_paren_has_type    |{ <T>  }|{ value: (T)       }| true,
        test_paren_has_liftime |{ <'a> }|{ value: (&'a str) }| true,
        
        test_path_no_generics        |{      }|{ value: ::std::option::Option<A> }| false,
        test_path_qself_has_type     |{ <T>  }|{ value: <T>::Type                }| true,
        test_path_qself_has_lifetime |{ <'a> }|{ value: <&'a Self as Send>::Type }| true,
        test_path_has_type           |{ <T>  }|{ value: T                        }| true,
        
        test_ptr_no_generics  |{      }|{ value: *const A }| false,
        test_ptr_has_type     |{ <T>  }|{ value: *mut T   }| true,
        
        test_reference_no_generics  |{      }|{ value: &'static A }| false,
        test_reference_has_type     |{ <T>  }|{ value: &'static T }| false,
        test_reference_has_lifetime |{ <'a> }|{ value: &'a T      }| false,
    }
}
