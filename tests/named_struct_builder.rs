use struct_builder::StructBuilder;

#[derive(StructBuilder)]
pub struct Platypus {
    pub age: u8,
    pub color: (u8, u8, u8),
    pub name: Option<String>
}

#[test]
fn test_named_struct_builder() {
    let params = PlatypusParams {
        age: 3,
        color: (36, 167, 161)
    };

    let platypus = Platypus::builder(params)
        .with_name(Some(String::from("Perry")))
        .with_age(4)
        .build();
    
    assert_eq!(platypus.age, 4);
    assert_eq!(platypus.color, (36, 167, 161));
    assert_eq!(platypus.name, Some(String::from("Perry")));
}