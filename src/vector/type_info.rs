pub trait TypeInfo {
    fn type_of(&self) -> &'static str;
}

impl TypeInfo for i32 {
    fn type_of(&self) -> &'static str {
        "i32"
    }
}

impl TypeInfo for bool {
    fn type_of(&self) -> &'static str {
        "bool"
    }
}
