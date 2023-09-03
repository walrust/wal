use wal_macros::html;

include!("../utils/non_display_struct.rs");

fn main() {
    html! { for };
    html! { for () };
    html! { for {} };
    html! { for { () } };
    html! { for Vec::<()>::new() };
    html! { for { Vec::<NonDisplayStruct>::new() } };
    html! {
        <div>
            for vec![1, 2, 3]
        </div>
    };
}
