use struct_builder::builder;

#[builder]
pub struct Platypus<T>(
    pub u8,
    pub (u8, u8, u8),
    pub Option<String>,
    pub T
);

#[test]
fn test_unnamed_struct_builder() {
    let params = PlatypusParams(3, (36, 167, 161), true);

    let platypus = Platypus::builder(params)
        .with_2(Some(String::from("Perry")))
        .with_0(4)
        .with_3(false)
        .build();
    
    assert_eq!(platypus.0, 4);
    assert_eq!(platypus.1, (36, 167, 161));
    assert_eq!(platypus.2, Some(String::from("Perry")));
    assert_eq!(platypus.3, false);
}