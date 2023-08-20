use wal_macros::html;

struct TestNonDisplayStruct {
    field: i32,
}

fn main() {
    html! { { () } };
    html! { { invalid_reference } };
    let s = TestNonDisplayStruct { field: 15 };
    html! { { s } };
    html! { { TestNonDisplayStruct { field: 15 } } };
}
