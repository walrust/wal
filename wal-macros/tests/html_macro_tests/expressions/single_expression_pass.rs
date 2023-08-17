use wal_macros::html;

struct TestDisplayStruct {
    field: i32,
}

impl std::fmt::Display for TestDisplayStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestStruct {{ field: {} }}", self.field)
    }
}

fn main() {
    html! { String::from("Hello world!") };
    let val = "Hello world!";
    html! { val };
    let t = TestDisplayStruct { field: 15 };
    html! { t };
    html! { TestDisplayStruct { field: 15 } };
}
