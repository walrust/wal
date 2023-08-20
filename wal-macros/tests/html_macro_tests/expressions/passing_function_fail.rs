use wal_macros::html;

struct TestNonDisplayStruct {
    field: i32,
}

fn main() {
    let empty = || ();
    html! {
        empty()
    };

    let not_node = || TestNonDisplayStruct { field: 0 };
    html! {
        not_node()
    };
}
