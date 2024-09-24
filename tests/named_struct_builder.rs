use struct_builder::builder;

#[builder]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Platypus<S>
where
    S: Into<String>,
{
    pub age: u8,
    pub color: (u8, u8, u8),
    pub name: Option<S>,
    pub is_perry: bool,
}

#[test]
fn test_structs_are_defined() {
    let params = PlatypusParams {
        age: 3,
        color: (36, 167, 161),
        is_perry: false,
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
        age: 2,
        color: (1, 2, 3),
        is_perry: false,
    };

    let subject: Platypus<&'static str> = params.into();

    assert_eq!(subject.age, 2);
    assert_eq!(subject.color, (1, 2, 3));
    assert_eq!(subject.name, None);
    assert_eq!(subject.is_perry, false);
}

#[test]
fn test_subject_from_builder() {
    let builder = Platypus::builder(
        PlatypusParams {
            age: 2,
            color: (1, 2, 3),
            is_perry: false,
        })
        .with_name(Some("perry"));

    let subject: Platypus<&'static str> = builder.into();

    assert_eq!(subject.age, 2);
    assert_eq!(subject.color, (1, 2, 3));
    assert_eq!(subject.name, Some("perry"));
    assert_eq!(subject.is_perry, false);
}

#[test]
fn test_builder_from_subject() {
    let subject = Platypus {
        age: 2,
        color: (1, 2, 3),
        name: Some("perry"),
        is_perry: true
    };

    let builder: PlatypusBuilder<&str> = subject.into();

    assert_eq!(builder.inner.age, 2);
    assert_eq!(builder.inner.color, (1, 2, 3));
    assert_eq!(builder.inner.name, Some("perry"));
    assert_eq!(builder.inner.is_perry, true);   
}

#[test]
fn test_params_has_attributes() {
    let params = PlatypusParams {
        age: 1,
        color: (23, 45, 56),
        is_perry: false,
    };
    
    // Test clone
    let new_params = params.clone();
    
    // Test PartialEq
    assert_eq!(params, new_params);
}
