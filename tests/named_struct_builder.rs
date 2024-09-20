use struct_builder::StructBuilder;

#[derive(StructBuilder)]
pub struct Platypus<T>
where
    T: Copy
{
    pub age: u8,
    pub color: (u8, u8, u8),
    pub name: Option<String>,
    pub is_perry: T
}

#[test]
fn test_named_struct_builder() {
    let params: PlatypusParams<bool> = PlatypusParams {
        age: 3,
        color: (36, 167, 161),
        is_perry: false
    };

    let platypus = Platypus::builder(params)
        .with_name(Some(String::from("Perry")))
        .with_age(4)
        .with_is_perry(true)
        .build();
    
    assert_eq!(platypus.age, 4);
    assert_eq!(platypus.color, (36, 167, 161));
    assert_eq!(platypus.name, Some(String::from("Perry")));
    assert_eq!(platypus.is_perry, true);
}