use struct_builder::StructBuilder;

#[derive(StructBuilder)]
pub struct NamedTest {
    opt0: Option<usize>,
    opt1: Option<String>
}

#[derive(StructBuilder)]
pub struct UnnamedTest(String, Option<usize>);

// #[derive(StructBuilder)]
// pub struct User {
//     pub email: String,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>
// }

fn main() {
    // let user_builder = UserBuilder {
    //     inner: User {
    //         email: "test@email.com".to_string(),
    //         first_name: None,
    //         last_name: None,
    //     }
    // };
    //
    // let user = user_builder
    //     .with_first_name(Some(String::from("Bob")))
    //     .with_last_name(Some(String::from("Wood")));
}
