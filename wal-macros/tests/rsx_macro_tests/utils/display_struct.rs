pub struct DisplayStruct;

impl std::fmt::Display for DisplayStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DisplayStruct")
    }
}
