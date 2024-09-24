use struct_builder::StructBuilder;

#[derive(StructBuilder)]
pub struct Platypus<S>
where
    S: Into<String>
{
    pub age: u8,
    pub color: (u8, u8, u8),
    pub name: Option<S>,
    pub is_perry: bool
}

#[test]
fn test_structs_are_defined() {
    let params = PlatypusParams {
        age: 3,
        color: (36, 167, 161),
        is_perry: false
    };

    let platypus = Platypus::builder(params)
        .with_name(Some("Perry"))
        .with_age(4)
        .with_is_perry(true)
        .build();
    
    assert_eq!(platypus.age, 4);
    assert_eq!(platypus.color, (36, 167, 161));
    assert_eq!(platypus.name, Some("Perry"));
    assert_eq!(platypus.is_perry, true);
}

#[test]
fn test_subject_from_params() {
    let params = PlatypusParams {
        age: 0,
        color: (0, 0, 0),
        is_perry: ,
    };
}

#[test]
fn test_subject_from_builder() {
    
}

#[test]
fn test_builder_from_subject() {
    
}
