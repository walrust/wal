use wal_macros::html;

struct TestStruct {
    field: i32,
}

impl std::fmt::Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestStruct {{ field: {} }}", self.field)
    }
}

fn main() {
    html! { <div></div> };
    html! { <div> "" </div> };
    html! { <div> "Hello world!" </div> };
    html! { <div> 'a' </div> };
    html! { <div> 15 </div> };
    html! { <div> 15.0 </div> };

    html! { <div> { String::from("Hello world!") } </div> };
    let val = "Hello world!";
    html! { <div> { val } </div> };
    let t = TestStruct { field: 15 };
    html! { <div> { t } </div> };
    html! { <div> { TestStruct { field: 15 } } </div> };
}
