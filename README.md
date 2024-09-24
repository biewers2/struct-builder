Derive builders for your structs.

Put `#[derive(StructBuilder)]` on your structs to derive a builder pattern for that struct. The builder
can be used to create the struct from only required fields (those without the `Option` type) and modify
the content of the struct.

A struct builder enforces required fields to be specified and allows optional arguments to be specified
post-construction. This is done by defining a "params" struct that the builder depends on to be initialized.
This struct defines all the fields in the original struct that don't have the "Option" type. Once the builder
is initialized with the params, both required and optional fields can be updated by calling builder methods
(using the identifiers `with_<field>`).

# Examples

## Using StructBuilder to build a request with named fields.

```rust
use struct_builder::StructBuilder;

#[derive(StructBuilder)]
pub struct CreateUserRequest {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<u64>
}

fn main() {
    // New "params" struct that defines required fields for [CreateUserRequest].
    let params = CreateUserRequestParams {
        email: "john.doe@email.com".to_owned()
    };
    
    // Create a builder using the [builder] function by passing the params to it.
    // All optional fields are set to [None] by default.
    let request = CreateUserRequest::builder(params)
        .with_first_name(Some("John".to_owned()))
        .with_age(Some(35))
        .build();

    assert_eq!(request.email, "john.doe@email.com".to_owned());
    assert_eq!(request.first_name, Some("John".to_owned()));
    assert_eq!(request.last_name, None);
    assert_eq!(request.age, Some(35));
}
```
