//! Derive builders for your structs.
//! 
//! Putting `#[builder]` on your struct will derive the builder pattern for it. A new "params" struct will be defined 
//! derived from that follow the builder pattern. The builder can be used to create the struct from only
//! required fields (those without the [Option] type) and modify the content of the struct.
//!
//! A struct builder enforces required fields to be specified and allows optional arguments to be specified post-construction.
//! This is done by defining a "params" struct that the builder depends on to be initialized. This struct defines all the fields
//! in the original struct that don't have the "Option" type. Once the builder is initialized with the params, both required and optional fields
//! can be updated by calling builder methods (using the identifiers `with_<field>`).
//!
//! # Examples
//!
//! ## Using [macro@builder] to build a request with named fields.
//! ```
//! use struct_builder::builder;
//!
//! #[builder]
//! #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
//! pub struct CreateUserRequest<P> {
//!     pub email: String,
//!     pub first_name: Option<String>,
//!     pub last_name: Option<String>,
//!     pub age: Option<u64>,
//!     pub payload: P
//! }
//!
//! // Inherits attributes and generics from `CreateUserRequest`
//! let params: CreateUserRequestParams<String> = CreateUserRequestParams {
//!     email: "john.doe@email.com".to_owned(),
//!     payload: "John Doe's User".to_owned()
//! };
//!
//! let request = CreateUserRequest::builder(params)
//!     .with_first_name(Some("John".to_owned()))
//!     .with_age(Some(35))
//!     .build();
//!
//! assert_eq!(request.email, "john.doe@email.com".to_owned());
//! assert_eq!(request.first_name, Some("John".to_owned()));
//! assert_eq!(request.last_name, None);
//! assert_eq!(request.age, Some(35));
//! ```
//!
//! ## Using [macro@builder] to build a tuple (unnamed) struct.
//! ```
//! use struct_builder::builder;
//!
//! /// First, Middle, and Last names.
//! #[builder]
//! pub struct FullName(pub String, pub Option<String>, pub String);
//!
//! let params = FullNameParams("John".to_owned(), "Doe".to_owned());
//!
//! let request = FullName::builder(params)
//!     .with_1(Some("Harold".to_owned()))
//!     .build();
//!
//! assert_eq!(request.0, "John".to_owned());
//! assert_eq!(request.1, Some("Harold".to_owned()));
//! assert_eq!(request.2, "Doe".to_owned());
//! ```
//!
//! ## Converting a params struct directly with no builder.
//! ```
//! use struct_builder::builder;
//!
//! #[builder]
//! pub struct CreateUserRequest {
//!     pub email: String,
//!     pub first_name: Option<String>,
//!     pub last_name: Option<String>,
//! }
//!
//! let request = CreateUserRequest::from(CreateUserRequestParams {
//!     email: "john.doe@email.com".to_owned()
//! });
//!
//! assert_eq!(request.email, "john.doe@email.com".to_owned());
//! assert_eq!(request.first_name, None);
//! assert_eq!(request.last_name, None);
//! ```
//!
//! ## Creating a builder directly
//! ```
//! use struct_builder::builder;
//!
//! #[builder]
//! pub struct CreateUserRequest {
//!     pub email: String,
//!     pub first_name: Option<String>,
//!     pub last_name: Option<String>,
//! }
//!
//! let request = CreateUserRequest {
//!     email: "john.doe@email.com".to_owned(),
//!     first_name: Some("John".to_owned()),
//!     last_name: None
//! };
//!
//! let rebuilt_request = CreateUserRequestBuilder::from(request)
//!     .with_last_name(Some("Doe".to_owned()))
//!     .build();
//!
//! assert_eq!(rebuilt_request.email, "john.doe@email.com".to_owned());
//! assert_eq!(rebuilt_request.first_name, Some("John".to_owned()));
//! assert_eq!(rebuilt_request.last_name, Some("Doe".to_owned()));
//! ```

extern crate proc_macro;

mod components;
mod struct_builder;
mod generic_resolution;
#[cfg(test)]
mod test_util;

use crate::struct_builder::StructBuilder;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// Derive the builder pattern for a struct.
///
/// A struct builder enforces required fields to be specified and allows optional arguments to be specified post-construction.
/// This is done by defining a "params" struct that the builder depends on to be initialized. This struct defines all the fields
/// in the original struct that don't have the "Option" type. Once the builder is initialized with the params, both required and optional fields
/// can be updated by calling builder methods (using the identifiers `with_<field>`).
///
#[proc_macro_attribute]
pub fn builder(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let original_item = parse_macro_input!(item as ItemStruct);
    let struct_builder = StructBuilder(original_item.clone());

    proc_macro::TokenStream::from(quote! { 
        #original_item
        #struct_builder
    })
}


#[deprecated(
    since = "0.3.0",
    note = r#"
        Please use `#[builder]` macro instead.
        This macro type does not support inheriting existing attributes, such as other derived traits.
    "#
)]
#[proc_macro_derive(StructBuilder)]
pub fn derive_builder(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let struct_builder = StructBuilder(item);

    proc_macro::TokenStream::from(quote! { #struct_builder })
}
