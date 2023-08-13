struct TestStruct {
    field: i32,
}

impl std::fmt::Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestStruct {{ field: {} }}", self.field)
    }
}

fn main() {
    html! {""};
    html! {"Hello world!"};
    html! {'a'};
    html! {15};
    html! {15.0};
    html! {String::from("Hello world!")};
    let val = "Hello world!";
    html! {val};
    let t = TestStruct { field: 15 };
    html! {t};
}
