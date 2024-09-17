use struct_builder::StructBuilder;

#[derive(StructBuilder)]
pub struct Platypus {
    pub age: u8,
    pub color: (u8, u8, u8),
    pub name: Option<String>
}

#[test]
fn test_named_struct_builder() {
    
}