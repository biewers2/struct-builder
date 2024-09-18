Derive builders for your structs.

The `#[derive(StructBuilder)]` macro creates new structs that can be used to build the struct being derived from that follow the builder pattern.
The builder can be used to create the struct from only required fields (those without the `Option` type) and modify the content of the struct.

A struct builder enforces required fields to be specified and allows optional arguments to be specified post-construction.
This is done by defining a "params" struct that the builder depends on to be initialized. This struct defines all the fields
in the original struct that don't have the "Option" type. Once the builder is initialized with the params, both required and optional fields
can be updated by calling builder methods (using the identifiers `with_<field>`).

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
    let params = CreateUserRequestParams {
        email: "john.doe@email.com".to_owned()
    };
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
