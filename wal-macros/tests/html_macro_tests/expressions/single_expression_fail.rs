use wal_macros::html;

struct TestNonDisplayStruct {
    field: i32,
}

fn main() {
    html! { () };
    html! { invalid_reference };
    html! { TestNonDisplayStruct { field: 15 } };
}
