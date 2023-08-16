use wal_macros::html;

struct TestStruct {
    field: i32,
}

fn main() {
    let empty = || ();
    html! {
        empty()
    };

    let not_node = || TestStruct { field: 0 };
    html! {
        not_node()
    };
}
