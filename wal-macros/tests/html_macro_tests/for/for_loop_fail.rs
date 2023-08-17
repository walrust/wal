use wal_macros::html;

struct TestNonDisplayStruct {
    field: i32,
}

fn main() {
    html! { for };
    html! { for () };
    html! { for {} };
    html! { for { () } };
    html! { for Vec::<()>::new() };
    html! { for { Vec::<TestNonDisplayStruct>::new() } };
}
