//! Derive builders for your structs.
//! 
//! The derive macro [StructBuilder] creates new structs that can be used to build the struct being derived from that follow the builder pattern.
//! The builder can be used to create the struct from only required fields (those without the [Option] type) and modify the content of the struct.
//!
//! A struct builder enforces required fields to be specified and allows optional arguments to be specified post-construction.
//! This is done by defining a "params" struct that the builder depends on to be initialized. This struct defines all the fields
//! in the original struct that don't have the "Option" type. Once the builder is initialized with the params, both required and optional fields
//! can be updated by calling builder methods (using the identifiers `with_<field>`).
//!
//! # Examples
//!
//! ## Using [StructBuilder] to build a request with named fields.
//! ```
//! use struct_builder::StructBuilder;
//!
//! #[derive(StructBuilder)]
//! pub struct CreateUserRequest {
//!     pub email: String,
//!     pub first_name: Option<String>,
//!     pub last_name: Option<String>,
//!     pub age: Option<u64>
//! }
//!
//! let params = CreateUserRequestParams {
//!     email: "john.doe@email.com".to_owned()
//! };
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
//! ## Using [StructBuilder] to build a tuple (unnamed) struct.
//! ```
//! use struct_builder::StructBuilder;
//!
//! /// First, Middle, and Last names.
//! #[derive(StructBuilder)]
//! pub struct FullName(pub String, pub Option<String>, pub String);
//!
//! let params = FullNameParams("John".to_owned(), "Doe".to_owned());
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
//! use struct_builder::StructBuilder;
//!
//! #[derive(StructBuilder)]
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
//! use struct_builder::StructBuilder;
//!
//! #[derive(StructBuilder)]
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
#[cfg(test)]
mod test_util;

use quote::quote;
use syn::{parse_macro_input, ItemStruct};
use crate::struct_builder::StructBuilder;

/// Derive a struct builder for a struct.
///
///
#[proc_macro_derive(StructBuilder)]
pub fn derive_builder(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let struct_builder = StructBuilder(item);

    proc_macro::TokenStream::from(quote! { #struct_builder })
}
