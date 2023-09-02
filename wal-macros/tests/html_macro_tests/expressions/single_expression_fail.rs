use wal_macros::html;

struct TestNonDisplayStruct {
    field: i32,
}

fn main() {
    html! { () };
    html! { invalid_reference };
    html! { TestNonDisplayStruct { field: 15 } };
    let node = || TestNonDisplayStruct { field: 15 };
    html! { node() };
}
