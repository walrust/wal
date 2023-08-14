use wal_macros::html;

struct TestStruct {
    field: i32,
}

fn main() {
    html! { "valid value" "invalid value" };
    html! { () };
    html! { TestStruct { field: 15 } };
    html! { invalid_reference };
}
